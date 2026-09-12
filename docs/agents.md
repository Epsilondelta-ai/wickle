# Run an agent

`create_agent` builds a scope-bound facade from an `AgentProfile` and existing
Host components. The runtime executes text requests through admission, context
preparation, model routing, and serial tool calls until a turn ends or execution
stops. Outcomes, tool observations, and events are saved; callers can replay events
and explicitly cancel execution.

```rust
use wickle::*;

pub async fn run_once(
    profile: AgentProfile,
    bindings: AgentBindings,
    request: RunRequest,
    context: ExecutionContext,
) -> Result<Guarded<RunOutcome>, ContractError> {
    let agent = create_agent(profile, bindings)?;
    match agent.start(request, context.clone()).await? {
        Guarded::Completed(handle) => handle.outcome(&context).await,
        Guarded::ApprovalRequired(challenge) => Ok(Guarded::ApprovalRequired(challenge)),
    }
}
```

The [text consumer](../tests/support/agent_consumer.rs) and
[tool-loop consumer](../tests/support/tool_loop_consumer.rs) configure the Host
components, execute requests with synthetic models, reconnect to their events,
and reopen SQLite state. Run the extracted-package examples with
`python3 scripts/check-package.py --allow-dirty`.

## Supply Host components

`AgentBindings` contains the exact scope, `StateStore`, current `PolicyGate`,
`ProfileResolver`, pinned `ModelRouter`, configured `ModelExchange`, trusted Host
instructions, `SystemInputRegistry`, optional `ToolRegistry` and
`SystemInputResolver`, clock, ID source, token estimator, and `AgentSettings`.
A single Agent instance owns one scope; use separately configured instances for
other scopes.

Constructing the Agent validates profile structure and finite settings. It does
not invoke component callbacks, open connections, create a runtime, spawn tasks,
or read environment files. The Host supplies Tokio before calling `start`.
Metadata resolution and model work begin only during execution.

`ModelTokenEstimator` estimates the final request for its selected route. It is a
synchronous Host callback with no I/O. Its estimate is used for capacity checks
and remains separate from provider-reported usage. `AgentSettings` bounds output,
request/response/context sizes, admission preparation, lease renewal, and observer
polling. Run-wide model, tool, recovery, and elapsed limits come from the profile.

The driver supports text instructions, text output, the bounded context strategy,
registered catalog tools, and `turn_end` completion. Skills, Hooks, connector and
adapter execution, automatic context sources, asset loading, and verified
completion are not connected to this driver. Profiles requiring these components
are rejected explicitly.

## Register tools and separate their inputs

The Host implements `ToolExecutor` and registers it with a compiled
`ToolDescriptor`. `AgentProfile.tools` selects the exact catalog tool ID and
version; only selected tools are offered to the model. The following function adds
one tool to a profile with no existing tool selections. All other Host bindings,
including the system-input registry and current authorization policy, must already
be configured.

```rust
use std::sync::Arc;
use wickle::*;

pub fn with_catalog_tool(
    mut profile: AgentProfile,
    mut bindings: AgentBindings,
    descriptor: ToolDescriptor,
    executor: Arc<dyn ToolExecutor>,
) -> Result<Agent, ContractError> {
    let compiled = SchemaCompiler::new().compile(descriptor, &bindings.system_inputs)?;
    let tool = compiled.descriptor().tool.clone();
    profile.tools.push(ToolBindingRef::Catalog(CatalogToolRef {
        tool_id: tool.id,
        version: tool.version,
        bindings: None,
        config: None,
    }));
    bindings.tools = Some(Arc::new(ToolRegistry::new(
        bindings.scope.clone(),
        vec![ToolRegistration { compiled, executor }],
    )?));
    create_agent(profile, bindings)
}
```

For a tool requiring `query`, `limit`, and `workspace_id`, set
`descriptor.agent_parameters` to `["query", "limit"]`. Register `workspace_id`
in `bindings.system_inputs` with its schema and `SystemInputSource::Run {}`;
supply its authenticated value through `ExecutionContextData.system_inputs` as
`Some(SystemInputs::new(values))`. The model supplies `query` and `limit`, while
the binder supplies `workspace_id`. The executor receives the final approved
`JsonObject`. A missing optional model parameter can receive its declared
top-level default; the binder never invents a missing system identifier.

System fields are absent from the model's input schema. A model-supplied hidden
field is rejected even when its value happens to match the Host's value. Only
bindings selected for that tool reach the executor, not the whole system-input map. For
resolver-owned keys, supply `bindings.system_input_resolver`; those values cannot
be overridden by the Run input map. See [tool schemas](tool-inputs.md) and
[input binding](input-binding.md) for aliases, defaults, and resolver contracts.

`ToolExecutor::execute` receives the arguments and a `ToolExecutionContext`
containing scope, principal, call/attempt IDs, an idempotency key, cancellation,
and a deadline. Credentials stay in the Host's executor instance. The executor
must perform one physical attempt without hidden retries or detached work.

## Execute a saved tool round

The driver saves the complete model tool plan, then processes calls serially in
their original order. Before entering an executor, it validates the model inputs,
binds and saves system inputs, checks current policy, reserves tool budget, and
saves dispatch identity. It checks current policy again immediately before the
callback. Validation, policy, or prerequisite storage failures cannot start the
executor. UUID format validation does not establish ownership: the Host policy
must check the actual target and the caller's permission.

Results, receipts, paired messages, and events are committed together. The next
model step sees the original model arguments and bounded, validated observations,
including safe status and effect fields. Bound system inputs, raw receipts, and
raw diagnostic payloads are not added to model context. Unknown tool names,
invalid arguments, and denied calls become error observations so the model can
respond or propose a corrected call within the Run's limits.

`ToolExecutionResult.effect` is independent of result validation:

| Effect | Meaning |
| --- | --- |
| `NotApplied` | No external business write occurred; a read-only result uses this value |
| `Applied` | A write is confirmed; return its receipt even if the output later fails schema validation |
| `Unknown` | A write may have occurred, but its outcome is not established |

A confirmed write with invalid output remains `Applied` with a failed observation
and its protected receipt. The same settled call is not executed again. If an
entered write times out, is cancelled, panics, or fails without a conclusive
result, its effect remains `Unknown`; stopping a Future does not prove a remote
write was rolled back. Oversized receipts retain an omission marker and digest
rather than claiming the full receipt was saved.

Approval of a fixed tool binding and unknown effects stop later tools and model calls. The
Agent saves a `Waiting` outcome with the fixed approval target or unresolved
effect reference. If approval becomes necessary after dispatch was reserved but
before the executor entered, `ApprovalPending` preserves the bound inputs,
attempt, idempotency key, and charged reservation. Cancellation or deadline
exhaustion instead produces its own outcome while retaining unresolved effects
and settling unstarted calls as `NotApplied`.

Approval required to read a system-input resolver is a separate, unsupported
binding operation: it produces a binding failure observation without entering
the tool executor or constructing an approval candidate from unresolved values.

Use `AgentSettings.tool_execution_limits` for the per-attempt timeout and receipt
bound. Profile limits still bound total model calls, tool attempts, and elapsed
time. Tool execution is serial; declaring tool metadata does not enable parallel
dispatch or automatic retry. There is no automatic replay after an unknown
effect. `Agent::resume` is still unsupported, so a saved wait can be inspected but
cannot yet be continued through an approval or reconciliation command. This saved
tool wait is distinct from `Guarded::ApprovalRequired`, which reports a policy
challenge on a facade operation such as starting or observing a Run.

## Start, replay, and observe

`start` checks current admission permission and looks up the request under its
scope, session, and request ID before resolving current metadata. An identical
request returns the original Run. Changed input, model options, or effective
system values conflicts with the stored request. For `start`, omitted system
inputs mean an empty map. Atomic admission decides concurrent requests; only the
newly created Run starts a driver.

The driver owns its lease, heartbeat, cancellation signal, and state transitions.
Dropping the start Future after it has been polled, a RunHandle, an outcome Future,
or an event Stream does not cancel accepted execution. Observer context
cancellation stops observation separately from the run's execution token.

`get_run` returns an authorized `RunView`. `get_run_details` requires the separate
details permission and returns the protected checkpoint. `RunHandle.outcome`
returns a `Guarded<RunOutcome>` read from the store; a text delta or finish
notification is not completion authority. If completion cannot be saved, the
observer receives an execution/storage error rather than a fabricated success.

`RunHandle.events(after_seq, context)` returns durable `EventView` metadata after
the cursor. It rechecks current permission while yielding and does not publish
protected record references. A slow or disconnected subscriber cannot restart or
block the driver. Reconnecting replays the remaining committed sequence.

## Cancel and finish

`handle.cancel(reason, &context)` checks `CancelRun` permission and returns a
`Guarded<CancelReceipt>`:

| Receipt | Meaning |
| --- | --- |
| `Requested` | The local driver received the cancellation signal; observe its saved outcome for completion |
| `AlreadyTerminal` | The stored result is already terminal and stays unchanged |
| `NotLocal` | This instance owns no live driver for the Run; no remote cancellation was accepted |

Success records `completion_basis=turn_ended`. This says the model completed its
turn under the configured output contract; it does not claim external business
verification. Failure, cancellation, and budget exhaustion have distinct outcomes.
Already stored response text can be retained as partial output without becoming
a successful assistant transcript. Unfinished stream deltas are not guaranteed
to survive an interrupted collector.

Opaque continuation returned by a successful model is stored in protected records
with the assistant transcript. A later Run can reuse it only under the exact
matching route. Changing providers or route identity fails before transmitting
foreign continuation. The original session prompt remains pinned.

`resume` has a public command shape, but waiting-command consumption and recovery
of interrupted external effects are not implemented by this initial driver.
Replaying `start` is not a substitute for that recovery operation. Durable state
can still be inspected after a process stops.
