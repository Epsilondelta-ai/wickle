//! Agent-level model/tool loops preserve binding boundaries, waits, and effect outcomes.

#[path = "support/agent.rs"]
#[allow(dead_code)]
mod agent_support;
use agent_support::{completed, context, id, profile, reference, request, scope};
use futures_util::stream;
use serde_json::{Value, json};
use std::{
    collections::BTreeSet,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::sync::Notify;
use wickle::*;

const WORKSPACE: &str = "11111111-1111-4111-8111-111111111111";
fn object(value: Value) -> JsonObject {
    value
        .as_object()
        .unwrap()
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

struct Catalog;
impl ProfileResolver for Catalog {
    fn resolve<'a>(
        &'a self,
        reference: &'a ComponentRef,
        _: &'a Scope,
    ) -> PortFuture<'a, ComponentMetadata> {
        Box::pin(async move {
            Ok(ComponentMetadata {
                reference: ComponentRef {
                    version: Some(id("1")),
                    ..reference.clone()
                },
                contract_version: 1,
                manifest_digest: canonical_digest(&json!("tool-loop-catalog")),
                config_schema: json!({"type":"object","additionalProperties":false}),
                dependencies: vec![],
                capabilities: BTreeSet::new(),
                required_capabilities: BTreeSet::new(),
                required_connections: BTreeSet::new(),
                model_name: (reference.kind == ComponentKind::Tool).then(|| reference.id.clone()),
                hook_position: None,
                exports: vec![],
            })
        })
    }
}
#[derive(Default)]
struct Policy {
    mode: AtomicUsize,
    tool_checks: AtomicUsize,
}
impl PolicyPort for Policy {
    fn authorize<'a>(
        &'a self,
        request: &'a PolicyRequest,
        _: PolicyContext<'a>,
    ) -> PortFuture<'a, PolicyDecision> {
        Box::pin(async move {
            if let PolicyAction::ExecuteTool { .. } = &request.action {
                let check = self.tool_checks.fetch_add(1, Ordering::SeqCst) + 1;
                match self.mode.load(Ordering::SeqCst) {
                    1 => {
                        return Ok(PolicyDecision::Deny {
                            reason: id("denied"),
                        });
                    }
                    2 => {
                        return Ok(PolicyDecision::RequireApproval {
                            reason: id("review"),
                        });
                    }
                    3 if check >= 3 => {
                        return Ok(PolicyDecision::RequireApproval {
                            reason: id("late_review"),
                        });
                    }
                    _ => {}
                }
            }
            Ok(PolicyDecision::Allow {})
        })
    }
}

struct Model {
    plans: Vec<(&'static str, JsonObject)>,
    calls: AtomicUsize,
    requests: Mutex<Vec<ModelRequest>>,
}
impl ModelPort for Model {
    fn binding(&self) -> ModelPortBinding {
        ModelPortBinding {
            provider: id("fixture"),
            adapter: reference("adapter"),
            connection_ref: reference("connection"),
        }
    }
    fn generate<'a>(
        &'a self,
        request: &'a ModelRequest,
        _: &'a ModelCallContext,
    ) -> PortStream<'a, ModelEvent> {
        let attempt = self.calls.fetch_add(1, Ordering::SeqCst);
        self.requests.lock().unwrap().push(request.clone());
        let events = if attempt == 0 {
            let mut events: Vec<_> = self
                .plans
                .iter()
                .enumerate()
                .map(|(index, (name, arguments))| {
                    Ok(ModelEvent::ToolArgumentsDelta {
                        index: index as u32,
                        provider_call_id: Some(format!("provider-{index}")),
                        name: Some((*name).into()),
                        delta: serde_json::to_string(arguments).unwrap(),
                    })
                })
                .collect();
            events.push(Ok(ModelEvent::ResponseCompleted {
                finish: ModelFinish::ToolCalls,
                metadata: ModelResponseMetadata::default(),
                continuation: vec![],
            }));
            events
        } else {
            vec![
                Ok(ModelEvent::TextDelta {
                    text: "All observations processed".into(),
                }),
                Ok(ModelEvent::ResponseCompleted {
                    finish: ModelFinish::Stop,
                    metadata: ModelResponseMetadata::default(),
                    continuation: vec![],
                }),
            ]
        };
        Box::pin(stream::iter(events))
    }
}
#[derive(Clone, Copy)]
enum Behavior {
    Success,
    InvalidOutput,
    Unknown,
    Pending,
}
struct Tool {
    name: &'static str,
    behavior: Behavior,
    effect: ToolEffect,
    calls: AtomicUsize,
    applied: AtomicUsize,
    arguments: Mutex<Vec<JsonObject>>,
    order: Arc<Mutex<Vec<&'static str>>>,
    entered: Notify,
}
impl ToolExecutor for Tool {
    fn execute<'a>(
        &'a self,
        arguments: &'a JsonObject,
        _: &'a ToolExecutionContext,
    ) -> PortFuture<'a, ToolExecutionResult> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.arguments.lock().unwrap().push(arguments.clone());
            self.order.lock().unwrap().push(self.name);
            if self.effect == ToolEffect::Applied {
                self.applied.fetch_add(1, Ordering::SeqCst);
            }
            self.entered.notify_one();
            match self.behavior {
            Behavior::Pending=>std::future::pending().await,
            Behavior::Unknown=>Ok(ToolExecutionResult{outcome:ToolExecutionOutcome::Failed{code:id("lost_response")},effect:ToolEffect::Unknown,receipt:None}),
            Behavior::Success|Behavior::InvalidOutput=>Ok(ToolExecutionResult{
                outcome:ToolExecutionOutcome::Succeeded{value:if matches!(self.behavior,Behavior::InvalidOutput){json!(42)}else{json!(format!("{} observation",self.name))}},effect:self.effect,
                receipt:(self.effect==ToolEffect::Applied).then(||json!({"private_receipt":"only-for-storage","target":arguments["workspace_id"]})),
            }),
        }
        })
    }
}

struct Fixture {
    base: agent_support::Fixture,
    model: Arc<Model>,
    policy: Arc<Policy>,
    tools: Vec<Arc<Tool>>,
    registry: Arc<ToolRegistry>,
    inputs: SystemInputRegistry,
    profile: AgentProfile,
    order: Arc<Mutex<Vec<&'static str>>>,
}
impl Fixture {
    fn new(plans: Vec<(&'static str, JsonObject)>, write_behavior: Behavior) -> Self {
        let base = agent_support::Fixture::new(agent_support::Response::Text, false);
        let inputs = SystemInputRegistry::new(vec![SystemInputDefinition {
            key: id("workspace_id"),
            version: id("1"),
            value_schema: json!({"type":"string","format":"uuid"}),
            source: SystemInputSource::Run {},
        }])
        .unwrap();
        let order = Arc::new(Mutex::new(vec![]));
        let mut tools = vec![];
        let mut registrations = vec![];
        let mut profile = profile();
        profile.limits.max_tool_attempts = 4;
        for (name, effect, behavior) in [
            ("read", ToolSideEffect::ReadOnly, Behavior::Success),
            ("write", ToolSideEffect::Write, write_behavior),
        ] {
            let compiled=SchemaCompiler::new().compile(ToolDescriptor{tool:reference(name),name:id(name),description:format!("{name} records"),input_schema:json!({"type":"object","properties":{"query":{"type":"string"},"limit":{"type":"integer","minimum":1,"default":10},"workspace_id":{"type":"string","format":"uuid"}},"required":["query","workspace_id"],"additionalProperties":false}),agent_parameters:vec!["query".into(),"limit".into()],system_bindings:None,output_schema:json!({"type":"string"}),side_effect:effect,concurrency:ToolConcurrency::Serial,retry:ToolRetryPolicy::Never,reconcile:false,max_output_bytes:4096.try_into().unwrap()},&inputs).unwrap();
            let executor = Arc::new(Tool {
                name,
                behavior,
                effect: if effect == ToolSideEffect::ReadOnly {
                    ToolEffect::NotApplied
                } else {
                    ToolEffect::Applied
                },
                calls: AtomicUsize::new(0),
                applied: AtomicUsize::new(0),
                arguments: Mutex::new(vec![]),
                order: order.clone(),
                entered: Notify::new(),
            });
            registrations.push(ToolRegistration {
                compiled,
                executor: executor.clone(),
            });
            tools.push(executor);
            profile.tools.push(ToolBindingRef::Catalog(CatalogToolRef {
                tool_id: id(name),
                version: id("1"),
                bindings: None,
                config: None,
            }));
        }
        Self {
            base,
            model: Arc::new(Model {
                plans,
                calls: AtomicUsize::new(0),
                requests: Mutex::new(vec![]),
            }),
            policy: Arc::new(Policy::default()),
            tools,
            registry: Arc::new(ToolRegistry::new(scope(), registrations).unwrap()),
            inputs,
            profile,
            order,
        }
    }
    fn agent(&self) -> Agent {
        let mut bindings = self.base.bindings();
        let mut router = agent_support::Router::new();
        let mut catalog = router.snapshot.catalog().clone();
        catalog.models[0]
            .capabilities
            .features
            .insert(id("tool_calling"));
        catalog.bindings[0]
            .capabilities
            .features
            .insert(id("tool_calling"));
        catalog.bindings[0].evidence[0].binding_digest = catalog.bindings[0]
            .contract_digest(&catalog.models[0])
            .unwrap();
        router.snapshot = RoutingSnapshot::new(catalog, router.snapshot.policy().clone()).unwrap();
        let policy =
            Arc::new(PolicyGate::new(self.policy.clone(), Duration::from_secs(1)).unwrap());
        bindings.router = Arc::new(router);
        bindings.profile_resolver = Arc::new(Catalog);
        bindings.policy = policy.clone();
        bindings.model_exchange = Arc::new(
            ModelExchange::new(self.model.clone(), policy)
                .with_route_inspector(self.base.inspector.clone(), Duration::from_secs(1))
                .unwrap(),
        );
        bindings.tools = Some(self.registry.clone());
        bindings.system_inputs = self.inputs.clone();
        bindings.system_input_resolver = None;
        bindings.settings.tool_execution_limits = ToolExecutionLimits {
            timeout_ms: 30,
            max_receipt_bytes: 4096,
        };
        create_agent(self.profile.clone(), bindings).unwrap()
    }
    async fn start(&self, agent: &Agent) -> RunHandle {
        let mut context = context();
        context.data.system_inputs =
            Some(SystemInputs::new(object(json!({"workspace_id":WORKSPACE}))));
        completed(agent.start(request("request"), context).await.unwrap())
    }
    async fn outcome(&self, handle: &RunHandle) -> RunOutcome {
        completed(handle.outcome(&context()).await.unwrap())
    }
}

fn default_plans() -> Vec<(&'static str, JsonObject)> {
    vec![
        ("read", object(json!({"query":"first"}))),
        ("write", object(json!({"query":"second","limit":2}))),
    ]
}
fn observations(request: &ModelRequest) -> Vec<(&Id, &Value)> {
    request
        .messages
        .iter()
        .flat_map(|message| &message.content)
        .filter_map(|content| match content {
            ModelContent::ToolResult {
                provider_call_id,
                content,
            } => Some((provider_call_id, content)),
            _ => None,
        })
        .collect()
}

#[tokio::test]
async fn an_agent_executes_two_calls_then_receives_only_the_safe_observations_and_original_arguments()
 {
    let fixture = Fixture::new(default_plans(), Behavior::Success);
    let agent = fixture.agent();
    let handle = fixture.start(&agent).await;
    assert_eq!(
        fixture.outcome(&handle).await.result.status(),
        RunStatus::Succeeded
    );
    assert_eq!(*fixture.order.lock().unwrap(), vec!["read", "write"]);
    assert_eq!(fixture.model.calls.load(Ordering::SeqCst), 2);
    assert_eq!(
        fixture.tools[0].arguments.lock().unwrap()[0],
        object(json!({"query":"first","limit":10,"workspace_id":WORKSPACE}))
    );
    assert_eq!(
        fixture.tools[1].arguments.lock().unwrap()[0],
        object(json!({"query":"second","limit":2,"workspace_id":WORKSPACE}))
    );
    let requests = fixture.model.requests.lock().unwrap();
    for tool in &requests[0].tools {
        assert_eq!(
            tool.model_input_schema["properties"]
                .as_object()
                .unwrap()
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            ["query".to_owned(), "limit".to_owned()]
                .into_iter()
                .collect()
        );
    }
    let calls: Vec<_> = requests[1]
        .messages
        .iter()
        .flat_map(|message| &message.content)
        .filter_map(|content| match content {
            ModelContent::ToolCall {
                provider_call_id,
                name,
                arguments,
            } => Some((provider_call_id, name, arguments)),
            _ => None,
        })
        .collect();
    assert_eq!(
        calls,
        vec![
            (
                &id("provider-0"),
                &id("read"),
                &object(json!({"query":"first"}))
            ),
            (
                &id("provider-1"),
                &id("write"),
                &object(json!({"query":"second","limit":2}))
            )
        ]
    );
    assert_eq!(
        observations(&requests[1]),
        vec![
            (
                &id("provider-0"),
                &json!({"status":"succeeded","effect":"not_applied","content":[{"type":"json","value":"read observation"}]})
            ),
            (
                &id("provider-1"),
                &json!({"status":"succeeded","effect":"applied","content":[{"type":"json","value":"write observation"}]})
            )
        ]
    );
}

#[tokio::test]
async fn unknown_invalid_and_denied_calls_return_errors_to_the_model_without_executing() {
    for case in 0..3 {
        let plan = match case {
            0 => ("unregistered", object(json!({"query":"x"}))),
            1 => (
                "read",
                object(json!({"query":"x","workspace_id":WORKSPACE})),
            ),
            _ => ("read", object(json!({"query":"x"}))),
        };
        let fixture = Fixture::new(vec![plan], Behavior::Success);
        if case == 2 {
            fixture.policy.mode.store(1, Ordering::SeqCst);
        }
        let agent = fixture.agent();
        let handle = fixture.start(&agent).await;
        let outcome = fixture.outcome(&handle).await;
        assert_eq!(outcome.result.status(), RunStatus::Succeeded);
        assert_eq!(outcome.usage.tool_attempts, 0);
        assert_eq!(fixture.model.calls.load(Ordering::SeqCst), 2);
        assert!(fixture.order.lock().unwrap().is_empty());
        let requests = fixture.model.requests.lock().unwrap();
        let results = observations(&requests[1]);
        assert_eq!(results.len(), 1);
        assert_ne!(results[0].1["status"], json!("succeeded"));
        assert_eq!(results[0].1["effect"], json!("not_applied"));
        assert!(results[0].1.get("error").is_some());
    }
}

#[tokio::test]
async fn an_applied_write_with_invalid_output_reaches_the_model_as_failure_and_is_not_replayed() {
    let fixture = Fixture::new(
        vec![("write", object(json!({"query":"x"})))],
        Behavior::InvalidOutput,
    );
    let agent = fixture.agent();
    let handle = fixture.start(&agent).await;
    assert_eq!(
        fixture.outcome(&handle).await.result.status(),
        RunStatus::Succeeded
    );
    assert_eq!(fixture.tools[1].applied.load(Ordering::SeqCst), 1);
    let saved = fixture
        .base
        .store
        .load(&scope(), handle.run_id())
        .await
        .unwrap();
    let ToolCallState::Settled { result } = &saved.snapshot.tool_ledger[0].state else {
        panic!("write result missing")
    };
    assert_eq!(result.status, ToolResultStatus::Failed);
    assert_eq!(result.effect, ToolEffect::Applied);
    let record = fixture
        .base
        .store
        .read_record(&scope(), result.effect_receipt_ref.as_ref().unwrap())
        .await
        .unwrap();
    assert_eq!(
        record.value()["receipt"],
        json!({"private_receipt":"only-for-storage","target":WORKSPACE})
    );
    {
        let requests = fixture.model.requests.lock().unwrap();
        assert_eq!(observations(&requests[1])[0].1["effect"], json!("applied"));
        assert_eq!(observations(&requests[1])[0].1["status"], json!("failed"));
    }
    let replay = fixture.start(&agent).await;
    assert_eq!(replay.run_id(), handle.run_id());
    assert_eq!(fixture.tools[1].calls.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.model.calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn approval_waits_with_a_fixed_binding_before_later_tools_or_model_calls() {
    let fixture = Fixture::new(default_plans(), Behavior::Success);
    fixture.policy.mode.store(2, Ordering::SeqCst);
    let agent = fixture.agent();
    let handle = fixture.start(&agent).await;
    let outcome = fixture.outcome(&handle).await;
    assert_eq!(outcome.result.status(), RunStatus::Waiting);
    assert_eq!(fixture.model.calls.load(Ordering::SeqCst), 1);
    assert!(fixture.order.lock().unwrap().is_empty());
    let saved = fixture
        .base
        .store
        .load(&scope(), handle.run_id())
        .await
        .unwrap();
    assert!(saved.snapshot.tool_ledger[0].call.bound_input_ref.is_some());
    assert!(saved.snapshot.tool_ledger[1].call.bound_input_ref.is_none());
    let wait = saved.snapshot.wait.as_ref().unwrap();
    assert!(matches!(wait.target, WaitTarget::Approval { .. }));
    assert_eq!(saved.session.active_run_id.as_ref(), Some(handle.run_id()));
}

#[tokio::test]
async fn approval_required_after_dispatch_reservation_becomes_a_fixed_agent_wait() {
    let fixture = Fixture::new(default_plans(), Behavior::Success);
    fixture.policy.mode.store(3, Ordering::SeqCst);
    let agent = fixture.agent();
    let handle = fixture.start(&agent).await;
    let outcome = fixture.outcome(&handle).await;
    assert_eq!(outcome.result.status(), RunStatus::Waiting);
    assert_eq!(fixture.policy.tool_checks.load(Ordering::SeqCst), 3);
    assert_eq!(fixture.model.calls.load(Ordering::SeqCst), 1);
    assert!(fixture.order.lock().unwrap().is_empty());
    assert!(
        fixture
            .tools
            .iter()
            .all(|tool| tool.calls.load(Ordering::SeqCst) == 0)
    );
    let saved = fixture
        .base
        .store
        .load(&scope(), handle.run_id())
        .await
        .unwrap();
    let entry = &saved.snapshot.tool_ledger[0];
    let ToolCallState::ApprovalPending { attempt_id, .. } = &entry.state else {
        panic!("an unexecuted reserved call must remain pending approval")
    };
    let bound_ref = entry.call.bound_input_ref.as_ref().unwrap();
    let record = fixture
        .base
        .store
        .read_record(&scope(), bound_ref)
        .await
        .unwrap();
    let compiled = &fixture.registry.get(&id("read")).unwrap().compiled;
    let bound = BoundToolInput::restore(
        &record,
        compiled,
        &scope(),
        handle.run_id(),
        &entry.call,
        saved.snapshot.system_inputs.as_ref(),
    )
    .unwrap();
    assert_eq!(
        bound.execution_args(),
        &object(json!({"query":"first","limit":10,"workspace_id":WORKSPACE}))
    );
    assert_eq!(
        saved.snapshot.wait.as_ref().unwrap().target,
        WaitTarget::Approval {
            target: ApprovalTarget::Tool {
                call_id: entry.call.call_id.clone(),
                binding_digest: bound.binding_digest().clone(),
            },
        }
    );
    assert_eq!(saved.snapshot.usage.tool_attempts, 1);
    let reservations: Vec<_> = saved
        .snapshot
        .reservations
        .iter()
        .filter(|reservation| matches!(reservation.kind, ReservationKind::Tool { .. }))
        .collect();
    assert_eq!(reservations.len(), 1);
    assert_eq!(&reservations[0].attempt_id, attempt_id);
    assert_eq!(
        reservations[0].kind,
        ReservationKind::Tool {
            call_id: entry.call.call_id.clone()
        }
    );
    assert!(matches!(
        saved.snapshot.tool_ledger[1].state,
        ToolCallState::Planned {}
    ));
    assert!(saved.snapshot.tool_ledger[1].call.bound_input_ref.is_none());
    assert_eq!(saved.session.active_run_id.as_ref(), Some(handle.run_id()));
}

#[tokio::test]
async fn uncertain_write_waits_and_does_not_run_the_later_tool_or_next_model() {
    let fixture = Fixture::new(
        vec![
            ("write", object(json!({"query":"x"}))),
            ("read", object(json!({"query":"y"}))),
        ],
        Behavior::Unknown,
    );
    let agent = fixture.agent();
    let handle = fixture.start(&agent).await;
    let outcome = fixture.outcome(&handle).await;
    assert_eq!(outcome.result.status(), RunStatus::Waiting);
    assert_eq!(fixture.model.calls.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.tools[0].calls.load(Ordering::SeqCst), 0);
    assert_eq!(fixture.tools[1].applied.load(Ordering::SeqCst), 1);
    let saved = fixture
        .base
        .store
        .load(&scope(), handle.run_id())
        .await
        .unwrap();
    assert!(matches!(
        saved.snapshot.tool_ledger[0].state,
        ToolCallState::Unknown { .. }
    ));
    assert!(matches!(
        saved.snapshot.wait.as_ref().unwrap().target,
        WaitTarget::External { .. }
    ));
    assert!(!outcome.unresolved_effects.is_empty());
}

#[tokio::test]
async fn using_the_last_model_slot_still_executes_its_saved_tool_plan_before_exhaustion() {
    let mut fixture = Fixture::new(default_plans(), Behavior::Success);
    fixture.profile.limits.max_model_calls = 1.try_into().unwrap();
    let agent = fixture.agent();
    let handle = fixture.start(&agent).await;
    let outcome = fixture.outcome(&handle).await;
    assert_eq!(
        outcome.result,
        OutcomeResult::Exhausted {
            budget: BudgetKind::ModelCalls
        }
    );
    assert_eq!(outcome.usage.tool_attempts, 2);
    assert_eq!(fixture.tools[1].applied.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.model.calls.load(Ordering::SeqCst), 1);
    assert!(fixture.base.store.load(&scope(),handle.run_id()).await.unwrap().snapshot.tool_ledger.iter().all(|entry|matches!(&entry.state,ToolCallState::Settled{result} if result.status==ToolResultStatus::Succeeded)));
}

#[tokio::test]
async fn tool_budget_exhaustion_settles_the_unstarted_plan_without_a_second_model_call() {
    let mut fixture = Fixture::new(default_plans(), Behavior::Success);
    fixture.profile.limits.max_tool_attempts = 1;
    let agent = fixture.agent();
    let handle = fixture.start(&agent).await;
    let outcome = fixture.outcome(&handle).await;
    assert_eq!(
        outcome.result,
        OutcomeResult::Exhausted {
            budget: BudgetKind::ToolAttempts
        }
    );
    assert_eq!(fixture.tools[0].calls.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.tools[1].calls.load(Ordering::SeqCst), 0);
    assert_eq!(fixture.model.calls.load(Ordering::SeqCst), 1);
    let saved = fixture
        .base
        .store
        .load(&scope(), handle.run_id())
        .await
        .unwrap();
    let ToolCallState::Settled { result } = &saved.snapshot.tool_ledger[1].state else {
        panic!("unstarted call remains orphaned")
    };
    assert_eq!(result.effect, ToolEffect::NotApplied);
    assert_ne!(result.status, ToolResultStatus::Succeeded);
}

#[tokio::test]
async fn cancelling_an_entered_write_retains_its_unknown_effect_and_closes_unstarted_calls() {
    let fixture = Fixture::new(
        vec![
            ("write", object(json!({"query":"x"}))),
            ("read", object(json!({"query":"y"}))),
        ],
        Behavior::Pending,
    );
    let agent = fixture.agent();
    let handle = fixture.start(&agent).await;
    tokio::time::timeout(Duration::from_secs(5), fixture.tools[1].entered.notified())
        .await
        .expect("write must enter before cancellation is requested");
    assert_eq!(
        completed(handle.cancel(id("stop"), &context()).await.unwrap()),
        CancelReceipt::Requested
    );
    let outcome = fixture.outcome(&handle).await;
    assert_eq!(outcome.result.status(), RunStatus::Cancelled);
    assert!(!outcome.unresolved_effects.is_empty());
    let saved = fixture
        .base
        .store
        .load(&scope(), handle.run_id())
        .await
        .unwrap();
    assert!(matches!(
        saved.snapshot.tool_ledger[0].state,
        ToolCallState::Unknown { .. }
    ));
    let ToolCallState::Settled { result } = &saved.snapshot.tool_ledger[1].state else {
        panic!("unstarted call not closed")
    };
    assert_eq!(result.effect, ToolEffect::NotApplied);
    assert_eq!(fixture.tools[0].calls.load(Ordering::SeqCst), 0);
    assert_eq!(fixture.model.calls.load(Ordering::SeqCst), 1);
}

#[tokio::test(start_paused = true)]
async fn the_run_deadline_keeps_an_entered_write_unknown_and_closes_the_remaining_plan() {
    let mut fixture = Fixture::new(
        vec![
            ("write", object(json!({"query":"x"}))),
            ("read", object(json!({"query":"y"}))),
        ],
        Behavior::Pending,
    );
    fixture.profile.limits.max_elapsed_ms = 20.try_into().unwrap();
    let agent = fixture.agent();
    let handle = fixture.start(&agent).await;
    let outcome = fixture.outcome(&handle).await;
    assert_eq!(
        outcome.result,
        OutcomeResult::Exhausted {
            budget: BudgetKind::Elapsed
        }
    );
    assert!(!outcome.unresolved_effects.is_empty());
    let saved = fixture
        .base
        .store
        .load(&scope(), handle.run_id())
        .await
        .unwrap();
    assert!(matches!(
        saved.snapshot.tool_ledger[0].state,
        ToolCallState::Unknown { .. }
    ));
    let ToolCallState::Settled { result } = &saved.snapshot.tool_ledger[1].state else {
        panic!("unstarted call not closed")
    };
    assert_eq!(result.effect, ToolEffect::NotApplied);
    assert_eq!(fixture.tools[1].applied.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.tools[0].calls.load(Ordering::SeqCst), 0);
    assert_eq!(fixture.model.calls.load(Ordering::SeqCst), 1);
}
