//! Serial tool execution validates and persists each boundary before external effects.

#[allow(dead_code)]
mod support;
#[path = "support/tool_execution.rs"]
#[allow(dead_code)]
mod tool_support;
use serde_json::json;
use std::sync::atomic::Ordering;
use support::{id, scope};
use tool_support::*;
use wickle::*;

#[tokio::test]
async fn stored_calls_execute_in_order_using_only_the_frozen_handler_arguments() {
    let fixture = Fixture::new(
        &[
            ("read", ToolSideEffect::ReadOnly, Action::Success),
            ("write", ToolSideEffect::Write, Action::Success),
        ],
        Some(OWNED),
    )
    .await;
    fixture
        .plan(&[
            ("first", "read", object(json!({"query":"first"}))),
            (
                "second",
                "write",
                object(json!({"query":"second","limit":2})),
            ),
        ])
        .await;
    assert!(matches!(
        fixture.execute().await.unwrap(),
        ToolRoundOutcome::Completed
    ));
    assert_eq!(
        *fixture.order.lock().unwrap(),
        vec![id("first"), id("second")]
    );
    assert_eq!(
        fixture.executors[0].observed.lock().unwrap()[0].args,
        object(json!({"query":"first","limit":10,"workspace_id":OWNED}))
    );
    assert_eq!(
        fixture.executors[1].observed.lock().unwrap()[0].args,
        object(json!({"query":"second","limit":2,"workspace_id":OWNED}))
    );
    let saved = fixture.saved().await;
    assert_eq!(saved.snapshot.usage.tool_attempts, 2);
    assert_eq!(
        saved.snapshot.tool_ledger[0].call.model_inputs,
        object(json!({"query":"first"}))
    );
    let results: Vec<_> = saved
        .messages
        .iter()
        .flat_map(|message| &message.content)
        .filter_map(|content| match content {
            ContentBlock::ToolResult { result } => Some(result),
            _ => None,
        })
        .collect();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].call_id, id("first"));
    assert_eq!(results[1].call_id, id("second"));
    for result in results {
        assert_eq!(result.call_message_id, id("call-message"));
        assert_eq!(result.status, ToolResultStatus::Succeeded);
    }
    assert!(matches!(
        fixture.execute().await.unwrap(),
        ToolRoundOutcome::Completed
    ));
    assert_eq!(fixture.executors[0].calls.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.executors[1].calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn invalid_model_arguments_and_unknown_tools_settle_without_executor_dispatch() {
    let fixture = Fixture::new(
        &[("read", ToolSideEffect::ReadOnly, Action::Success)],
        Some(OWNED),
    )
    .await;
    fixture
        .plan(&[
            (
                "hidden",
                "read",
                object(json!({"query":"x","workspace_id":OWNED})),
            ),
            ("unknown", "unregistered", object(json!({"query":"x"}))),
            ("valid", "read", object(json!({"query":"good"}))),
        ])
        .await;
    assert!(matches!(
        fixture.execute().await.unwrap(),
        ToolRoundOutcome::Completed
    ));
    assert_eq!(fixture.executors[0].calls.load(Ordering::SeqCst), 1);
    assert_eq!(*fixture.order.lock().unwrap(), vec![id("valid")]);
    let saved = fixture.saved().await;
    assert_eq!(saved.snapshot.usage.tool_attempts, 1);
    assert!(
        saved.snapshot.tool_ledger[1]
            .call
            .descriptor_digest
            .is_none()
    );
    for entry in &saved.snapshot.tool_ledger[..2] {
        let ToolCallState::Settled { result } = &entry.state else {
            panic!("rejected call remains unsettled")
        };
        assert_ne!(result.status, ToolResultStatus::Succeeded);
        assert_eq!(result.effect, ToolEffect::NotApplied);
        assert!(entry.call.bound_input_ref.is_none());
    }
}

#[tokio::test]
async fn missing_system_inputs_and_valid_foreign_targets_never_reach_the_executor() {
    for value in [None, Some(FOREIGN)] {
        let fixture =
            Fixture::new(&[("write", ToolSideEffect::Write, Action::Success)], value).await;
        fixture.policy.reject_foreign.store(1, Ordering::SeqCst);
        fixture
            .plan(&[("call", "write", object(json!({"query":"x"})))])
            .await;
        let _outcome = fixture.execute().await;
        assert_eq!(fixture.executors[0].calls.load(Ordering::SeqCst), 0);
        assert_eq!(fixture.executors[0].applied.load(Ordering::SeqCst), 0);
        assert_eq!(fixture.saved().await.snapshot.usage.tool_attempts, 0);
    }
}

#[tokio::test]
async fn policy_is_checked_again_after_binding_and_can_revoke_the_final_call() {
    for check in [2, 3] {
        let fixture = Fixture::new(
            &[("write", ToolSideEffect::Write, Action::Success)],
            Some(OWNED),
        )
        .await;
        fixture
            .plan(&[("call", "write", object(json!({"query":"x"})))])
            .await;
        fixture
            .policy
            .revoke_on_check
            .store(check, Ordering::SeqCst);
        assert!(matches!(
            fixture.execute().await.unwrap(),
            ToolRoundOutcome::Completed
        ));
        assert_eq!(fixture.policy.calls.lock().unwrap().len(), check);
        assert_eq!(fixture.executors[0].calls.load(Ordering::SeqCst), 0);
        let saved = fixture.saved().await;
        assert!(saved.snapshot.tool_ledger[0].call.bound_input_ref.is_some());
        assert_eq!(saved.snapshot.usage.tool_attempts, u64::from(check == 3));
        let ToolCallState::Settled { result } = &saved.snapshot.tool_ledger[0].state else {
            panic!("denied call must settle without an effect")
        };
        assert_eq!(result.status, ToolResultStatus::Denied);
        assert_eq!(result.effect, ToolEffect::NotApplied);
    }
}

#[tokio::test]
async fn applied_write_with_invalid_output_keeps_its_receipt_and_is_never_reexecuted() {
    let fixture = Fixture::new(
        &[("write", ToolSideEffect::Write, Action::WrongOutput)],
        Some(OWNED),
    )
    .await;
    fixture
        .plan(&[("call", "write", object(json!({"query":"x"})))])
        .await;
    assert!(matches!(
        fixture.execute().await.unwrap(),
        ToolRoundOutcome::Completed
    ));
    let saved = fixture.saved().await;
    let ToolCallState::Settled { result } = &saved.snapshot.tool_ledger[0].state else {
        panic!("result not settled")
    };
    assert_eq!(result.status, ToolResultStatus::Failed);
    assert_eq!(result.effect, ToolEffect::Applied);
    let receipt = result
        .effect_receipt_ref
        .as_ref()
        .expect("applied effect receipt is retained");
    let record = fixture.store.read_record(&scope(), receipt).await.unwrap();
    assert_eq!(
        &record.value()["receipt"],
        &json!({"effect_id":"external-effect","value":"x"})
    );
    assert!(matches!(
        fixture.execute().await.unwrap(),
        ToolRoundOutcome::Completed
    ));
    assert_eq!(fixture.executors[0].calls.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.executors[0].applied.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn an_approval_candidate_is_persisted_and_stops_later_calls_without_dispatch() {
    let fixture = Fixture::new(
        &[
            ("write", ToolSideEffect::Write, Action::Success),
            ("read", ToolSideEffect::ReadOnly, Action::Success),
        ],
        Some(OWNED),
    )
    .await;
    fixture
        .plan(&[
            ("review", "write", object(json!({"query":"x"}))),
            ("later", "read", object(json!({"query":"y"}))),
        ])
        .await;
    *fixture.policy.approval.lock().unwrap() = Some(id("review"));
    let outcome = fixture.execute().await.unwrap();
    assert!(
        matches!(outcome,ToolRoundOutcome::ApprovalRequired{call_id,..} if call_id==id("review"))
    );
    let saved = fixture.saved().await;
    assert!(saved.snapshot.tool_ledger[0].call.bound_input_ref.is_some());
    assert!(saved.snapshot.tool_ledger[1].call.bound_input_ref.is_none());
    assert_eq!(saved.snapshot.usage.tool_attempts, 0);
    assert!(fixture.order.lock().unwrap().is_empty());
}

#[tokio::test]
async fn late_approval_preserves_the_reserved_attempt_and_stops_before_executor_entry() {
    let fixture = Fixture::new(
        &[
            ("write", ToolSideEffect::Write, Action::Success),
            ("read", ToolSideEffect::ReadOnly, Action::Success),
        ],
        Some(OWNED),
    )
    .await;
    fixture
        .plan(&[
            ("review", "write", object(json!({"query":"x"}))),
            ("later", "read", object(json!({"query":"y"}))),
        ])
        .await;
    // Binding and pre-reservation authorization allow; only the stored dispatch gate changes.
    fixture.policy.approve_on_check.store(3, Ordering::SeqCst);
    let ToolRoundOutcome::ApprovalRequired {
        call_id,
        bound_input_ref,
        binding_digest,
        ..
    } = fixture.execute().await.unwrap()
    else {
        panic!("late approval must pause the round")
    };
    assert_eq!(fixture.policy.calls.lock().unwrap().len(), 3);
    let before = fixture
        .policy
        .before_approval
        .lock()
        .unwrap()
        .clone()
        .unwrap();
    let ToolCallState::Dispatching {
        attempt_id,
        idempotency_key,
    } = before.state
    else {
        panic!("the approval policy must run after the dispatch reservation is stored")
    };
    let saved = fixture.saved().await;
    let entry = &saved.snapshot.tool_ledger[0];
    assert_eq!(entry.call, before.call);
    assert_eq!(
        entry.state,
        ToolCallState::ApprovalPending {
            attempt_id: attempt_id.clone(),
            idempotency_key,
        }
    );
    assert_eq!(call_id, entry.call.call_id);
    assert_eq!(entry.call.bound_input_ref.as_ref(), Some(&bound_input_ref));
    let record = fixture
        .store
        .read_record(&scope(), &bound_input_ref)
        .await
        .unwrap();
    let bound = BoundToolInput::restore(
        &record,
        &fixture.compiled[0],
        &scope(),
        &id("run"),
        &entry.call,
        saved.snapshot.system_inputs.as_ref(),
    )
    .unwrap();
    assert_eq!(bound.binding_digest(), &binding_digest);
    assert_eq!(bound.original_model_inputs(), &object(json!({"query":"x"})));
    assert_eq!(
        bound.execution_args(),
        &object(json!({"query":"x","limit":10,"workspace_id":OWNED}))
    );
    assert_eq!(saved.snapshot.usage.tool_attempts, 1);
    let reservations: Vec<_> = saved
        .snapshot
        .reservations
        .iter()
        .filter(|reservation| matches!(reservation.kind, ReservationKind::Tool { .. }))
        .collect();
    assert_eq!(reservations.len(), 1);
    assert_eq!(reservations[0].attempt_id, attempt_id);
    assert_eq!(reservations[0].kind, ReservationKind::Tool { call_id });
    assert!(matches!(
        saved.snapshot.tool_ledger[1].state,
        ToolCallState::Planned {}
    ));
    assert!(saved.snapshot.tool_ledger[1].call.bound_input_ref.is_none());
    assert!(fixture.order.lock().unwrap().is_empty());
    assert!(
        fixture
            .executors
            .iter()
            .all(|executor| executor.calls.load(Ordering::SeqCst) == 0)
    );
}

#[tokio::test(start_paused = true)]
async fn a_write_timeout_preserves_unknown_attempt_identity_and_stops_following_tools() {
    let fixture = Fixture::new(
        &[
            ("write", ToolSideEffect::Write, Action::Pending),
            ("read", ToolSideEffect::ReadOnly, Action::Success),
        ],
        Some(OWNED),
    )
    .await;
    fixture
        .plan(&[
            ("uncertain", "write", object(json!({"query":"x"}))),
            ("later", "read", object(json!({"query":"y"}))),
        ])
        .await;
    let round = fixture
        .round()
        .with_limits(ToolExecutionLimits {
            timeout_ms: 20,
            max_receipt_bytes: 4096,
        })
        .unwrap();
    let outcome = round
        .execute(
            &id("model-request"),
            &fixture.context,
            &fixture.budget(fixture.store.clone()).await,
        )
        .await
        .unwrap();
    assert!(matches!(outcome,ToolRoundOutcome::Unresolved{call_id,..} if call_id==id("uncertain")));
    let saved = fixture.saved().await;
    let ToolCallState::Unknown {
        attempt_id,
        idempotency_key,
    } = &saved.snapshot.tool_ledger[0].state
    else {
        panic!("lost write must remain unknown")
    };
    let invocation = fixture.executors[0].observed.lock().unwrap()[0].clone();
    assert_eq!(attempt_id, &invocation.attempt_id);
    assert_eq!(idempotency_key, &invocation.idempotency_key);
    assert_eq!(fixture.executors[0].applied.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.executors[1].calls.load(Ordering::SeqCst), 0);
    assert!(matches!(
        fixture.execute().await.unwrap(),
        ToolRoundOutcome::Unresolved { .. }
    ));
    assert_eq!(fixture.executors[0].calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn write_panic_transport_error_or_explicit_unknown_never_claims_no_effect() {
    for action in [Action::Panic, Action::Error, Action::DeclaredUnknown] {
        let fixture = Fixture::new(
            &[
                ("write", ToolSideEffect::Write, action),
                ("read", ToolSideEffect::ReadOnly, Action::Success),
            ],
            Some(OWNED),
        )
        .await;
        fixture
            .plan(&[
                ("uncertain", "write", object(json!({"query":"x"}))),
                ("later", "read", object(json!({"query":"y"}))),
            ])
            .await;
        assert!(matches!(
            fixture.execute().await.unwrap(),
            ToolRoundOutcome::Unresolved { .. }
        ));
        let saved = fixture.saved().await;
        assert!(matches!(
            saved.snapshot.tool_ledger[0].state,
            ToolCallState::Unknown { .. }
        ));
        let result = saved
            .messages
            .iter()
            .flat_map(|message| &message.content)
            .find_map(|content| match content {
                ContentBlock::ToolResult { result } if result.call_id == id("uncertain") => {
                    Some(result)
                }
                _ => None,
            })
            .unwrap();
        assert_eq!(result.effect, ToolEffect::Unknown);
        assert_eq!(fixture.executors[0].applied.load(Ordering::SeqCst), 1);
        assert_eq!(fixture.executors[1].calls.load(Ordering::SeqCst), 0);
    }
}

#[tokio::test]
async fn checkpoint_restore_rejects_a_changed_unknown_effect_key() {
    let fixture = Fixture::new(
        &[("write", ToolSideEffect::Write, Action::DeclaredUnknown)],
        Some(OWNED),
    )
    .await;
    fixture
        .plan(&[("uncertain", "write", object(json!({"query":"x"})))])
        .await;
    fixture.execute().await.unwrap();
    let checkpoint = fixture.store.export_checkpoint(&scope()).unwrap();
    let original = serde_json::to_value(&checkpoint).unwrap();
    let restored =
        StateStoreCheckpoint::from_json(&original.to_string(), &scope(), &checkpoint.digest())
            .unwrap();
    let restored = MemoryStateStore::from_checkpoint(restored);
    assert_eq!(
        restored.load(&scope(), &id("run")).await.unwrap(),
        fixture.saved().await
    );

    let mut changed = original;
    let event = changed["runs"][0]["events"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|event| event["payload"]["type"] == "tool.unresolved")
        .unwrap();
    event["payload"]["idempotency_key"] = json!("different-effect-key");
    // Recomputing the container checksum must not bypass cross-record validation.
    let error = StateStoreCheckpoint::from_json(
        &changed.to_string(),
        &scope(),
        &canonical_digest(&changed),
    )
    .unwrap_err();
    assert_eq!(error.path, "checkpoint.tool_unresolved_key");
}

#[tokio::test]
async fn read_only_transport_failure_has_no_effect_and_does_not_block_the_next_read() {
    let fixture = Fixture::new(
        &[
            ("read", ToolSideEffect::ReadOnly, Action::Error),
            ("next", ToolSideEffect::ReadOnly, Action::Success),
        ],
        Some(OWNED),
    )
    .await;
    fixture
        .plan(&[
            ("failed", "read", object(json!({"query":"x"}))),
            ("next", "next", object(json!({"query":"y"}))),
        ])
        .await;
    assert!(matches!(
        fixture.execute().await.unwrap(),
        ToolRoundOutcome::Completed
    ));
    let saved = fixture.saved().await;
    let ToolCallState::Settled { result } = &saved.snapshot.tool_ledger[0].state else {
        panic!("read failure should settle")
    };
    assert_eq!(result.effect, ToolEffect::NotApplied);
    assert_eq!(fixture.executors[1].calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn failed_binding_reservation_or_dispatch_persistence_prevents_executor_entry() {
    for stage in [
        FailStage::Binding,
        FailStage::Reservation,
        FailStage::Dispatch,
    ] {
        let fixture = Fixture::new(
            &[("write", ToolSideEffect::Write, Action::Success)],
            Some(OWNED),
        )
        .await;
        fixture
            .plan(&[("call", "write", object(json!({"query":"x"})))])
            .await;
        let store = std::sync::Arc::new(FaultStore {
            inner: fixture.store.clone(),
            stage,
            lose_ack: false,
            failures: std::sync::atomic::AtomicUsize::new(0),
        });
        let result = fixture
            .round()
            .execute(
                &id("model-request"),
                &fixture.context,
                &fixture.budget(store.clone()).await,
            )
            .await;
        assert_eq!(result.unwrap_err().code, ErrorCode::PersistenceUnavailable);
        assert_eq!(store.failures.load(Ordering::SeqCst), 1);
        assert_eq!(fixture.executors[0].calls.load(Ordering::SeqCst), 0);
        assert_eq!(fixture.executors[0].applied.load(Ordering::SeqCst), 0);
        let saved = fixture.saved().await;
        assert!(matches!(
            saved.snapshot.tool_ledger[0].state,
            ToolCallState::Planned {}
        ));
    }
}

#[tokio::test]
async fn failed_result_storage_keeps_dispatch_uncertainty_and_blocks_later_effects() {
    let fixture = Fixture::new(
        &[
            ("write", ToolSideEffect::Write, Action::Success),
            ("read", ToolSideEffect::ReadOnly, Action::Success),
        ],
        Some(OWNED),
    )
    .await;
    fixture
        .plan(&[
            ("written", "write", object(json!({"query":"x"}))),
            ("later", "read", object(json!({"query":"y"}))),
        ])
        .await;
    let store = std::sync::Arc::new(FaultStore {
        inner: fixture.store.clone(),
        stage: FailStage::Result,
        lose_ack: false,
        failures: std::sync::atomic::AtomicUsize::new(0),
    });
    let result = fixture
        .round()
        .execute(
            &id("model-request"),
            &fixture.context,
            &fixture.budget(store.clone()).await,
        )
        .await;
    assert_eq!(result.unwrap_err().code, ErrorCode::PersistenceUnavailable);
    assert_eq!(fixture.executors[0].applied.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.executors[1].calls.load(Ordering::SeqCst), 0);
    let saved = fixture.saved().await;
    assert!(matches!(
        saved.snapshot.tool_ledger[0].state,
        ToolCallState::Dispatching { .. }
    ));
    assert!(
        !fixture
            .store
            .read_events(&scope(), &id("run"), 0, 100)
            .await
            .unwrap()
            .events
            .iter()
            .any(|event| matches!(event.payload, RunEventPayload::ToolSettled { .. }))
    );
}

#[tokio::test]
async fn result_commit_acknowledgement_loss_does_not_repeat_an_applied_write() {
    let fixture = Fixture::new(
        &[("write", ToolSideEffect::Write, Action::Success)],
        Some(OWNED),
    )
    .await;
    fixture
        .plan(&[("call", "write", object(json!({"query":"x"})))])
        .await;
    let store = std::sync::Arc::new(FaultStore {
        inner: fixture.store.clone(),
        stage: FailStage::Result,
        lose_ack: true,
        failures: std::sync::atomic::AtomicUsize::new(0),
    });
    let _first = fixture
        .round()
        .execute(
            &id("model-request"),
            &fixture.context,
            &fixture.budget(store.clone()).await,
        )
        .await;
    assert_eq!(store.failures.load(Ordering::SeqCst), 1);
    let saved = fixture.saved().await;
    let ToolCallState::Settled { result } = &saved.snapshot.tool_ledger[0].state else {
        panic!("committed result must survive its lost acknowledgement")
    };
    assert_eq!(result.effect, ToolEffect::Applied);
    assert!(matches!(
        fixture.execute().await.unwrap(),
        ToolRoundOutcome::Completed
    ));
    assert_eq!(fixture.executors[0].calls.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.executors[0].applied.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn scope_or_lease_mismatch_prevents_all_executor_calls() {
    let fixture = Fixture::new(
        &[("write", ToolSideEffect::Write, Action::Success)],
        Some(OWNED),
    )
    .await;
    fixture
        .plan(&[("call", "write", object(json!({"query":"x"})))])
        .await;
    let budget = fixture.budget(fixture.store.clone()).await;
    let mut foreign = fixture.context.clone();
    foreign.data.scope.tenant_id = id("foreign");
    assert!(
        fixture
            .round()
            .execute(&id("model-request"), &foreign, &budget)
            .await
            .is_err()
    );
    fixture
        .store
        .release_lease(
            &scope(),
            &id("run"),
            &fixture.lease,
            fixture.clock.now().unwrap().utc_ms,
        )
        .await
        .unwrap();
    assert_eq!(
        fixture
            .round()
            .execute(&id("model-request"), &fixture.context, &budget)
            .await
            .unwrap_err()
            .code,
        ErrorCode::LeaseLost
    );
    assert_eq!(fixture.executors[0].calls.load(Ordering::SeqCst), 0);
    assert_eq!(fixture.saved().await.snapshot.usage.tool_attempts, 0);
}

#[tokio::test]
async fn cancellation_after_executor_entry_retains_an_unknown_write_and_stops_the_round() {
    let fixture = Fixture::new(
        &[
            ("write", ToolSideEffect::Write, Action::Cancel),
            ("read", ToolSideEffect::ReadOnly, Action::Success),
        ],
        Some(OWNED),
    )
    .await;
    *fixture.executors[0].cancel.lock().unwrap() = Some(fixture.context.cancellation.clone());
    fixture
        .plan(&[
            ("uncertain", "write", object(json!({"query":"x"}))),
            ("later", "read", object(json!({"query":"y"}))),
        ])
        .await;
    assert!(
        matches!(fixture.execute().await.unwrap(),ToolRoundOutcome::Unresolved{call_id,..} if call_id==id("uncertain"))
    );
    let saved = fixture.saved().await;
    assert!(matches!(
        saved.snapshot.tool_ledger[0].state,
        ToolCallState::Unknown { .. }
    ));
    assert_eq!(fixture.executors[0].applied.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.executors[1].calls.load(Ordering::SeqCst), 0);
    let result = saved
        .messages
        .iter()
        .flat_map(|message| &message.content)
        .find_map(|content| match content {
            ContentBlock::ToolResult { result } if result.call_id == id("uncertain") => {
                Some(result)
            }
            _ => None,
        })
        .unwrap();
    assert_eq!(result.effect, ToolEffect::Unknown);
    assert_eq!(saved.snapshot.usage.tool_attempts, 1);
}

#[tokio::test]
async fn cancellation_before_the_round_starts_never_reserves_or_enters_an_executor() {
    let fixture = Fixture::new(
        &[("write", ToolSideEffect::Write, Action::Success)],
        Some(OWNED),
    )
    .await;
    fixture
        .plan(&[("call", "write", object(json!({"query":"x"})))])
        .await;
    fixture.context.cancellation.cancel();
    assert_eq!(
        fixture.execute().await.unwrap_err().code,
        ErrorCode::Cancelled
    );
    assert_eq!(fixture.executors[0].calls.load(Ordering::SeqCst), 0);
    assert_eq!(fixture.saved().await.snapshot.usage.tool_attempts, 0);
}

#[tokio::test]
async fn registry_requires_unique_names_and_the_exact_selected_tool_version() {
    let fixture = Fixture::new(
        &[("read", ToolSideEffect::ReadOnly, Action::Success)],
        Some(OWNED),
    )
    .await;
    let entry = ToolRegistration {
        compiled: fixture.compiled[0].clone(),
        executor: fixture.executors[0].clone(),
    };
    assert!(ToolRegistry::new(scope(), vec![entry.clone(), entry]).is_err());
    let saved = fixture.saved().await;
    let mut selected = saved.snapshot.profile.profile().clone();
    let ToolBindingRef::Catalog(reference) = &mut selected.tools[0] else {
        unreachable!()
    };
    reference.version = id("not-installed");
    assert!(fixture.registry.prompt_bindings(&selected).is_err());
    assert_eq!(fixture.executors[0].calls.load(Ordering::SeqCst), 0);
    let ordinary = fixture
        .registry
        .prompt_bindings(saved.snapshot.profile.profile())
        .unwrap();
    assert_eq!(ordinary.len(), 1);
    assert_eq!(ordinary[0].compiled.digest(), fixture.compiled[0].digest());
}

#[tokio::test]
async fn a_saved_failed_result_with_unknown_effect_still_blocks_following_tools() {
    let fixture = Fixture::new(
        &[
            ("write", ToolSideEffect::Write, Action::Success),
            ("read", ToolSideEffect::ReadOnly, Action::Success),
        ],
        Some(OWNED),
    )
    .await;
    fixture
        .plan(&[
            ("historical", "write", object(json!({"query":"x"}))),
            ("later", "read", object(json!({"query":"y"}))),
        ])
        .await;
    let saved = fixture.saved().await;
    // The stored effect field remains authoritative even in a checkpoint whose
    // failed status does not use the newer dedicated Unknown classification.
    let result = ToolResult {
        call_id: id("historical"),
        call_message_id: id("call-message"),
        status: ToolResultStatus::Failed,
        effect: ToolEffect::Unknown,
        content: vec![],
        effect_receipt_ref: None,
        error: Some(Failure {
            code: id("historical_unknown"),
            diagnostic_ref: None,
        }),
    };
    let record = ProtectedRecord::new(
        id("historical-result"),
        1,
        serde_json::to_value(&result).unwrap(),
    );
    let mut update = support::prepared(
        &saved.snapshot,
        fixture.lease.clone(),
        fixture.clock.now().unwrap().utc_ms,
    );
    update.snapshot.tool_ledger[0].state = ToolCallState::Settled {
        result: result.clone(),
    };
    update.snapshot.last_event_seq += 1;
    update.events.push(support::event(
        &id("run"),
        &id("session"),
        &scope(),
        update.snapshot.last_event_seq,
        RunEventPayload::ToolSettled {
            result_ref: record.reference().clone(),
        },
    ));
    update.messages.push(Message {
        message_id: id("historical-result-message"),
        run_id: id("run"),
        sequence: (saved.session.transcript_revision + 1).try_into().unwrap(),
        role: MessageRole::Tool,
        origin: MessageOrigin::Tool,
        visibility: Visibility::UserAndModel,
        content: vec![ContentBlock::ToolResult { result }],
    });
    update.records.push(record);
    fixture
        .store
        .commit(&scope(), &id("run"), update)
        .await
        .unwrap();
    assert!(!matches!(
        fixture.execute().await,
        Ok(ToolRoundOutcome::Completed)
    ));
    assert_eq!(fixture.executors[0].calls.load(Ordering::SeqCst), 0);
    assert_eq!(fixture.executors[1].calls.load(Ordering::SeqCst), 0);
    assert_eq!(fixture.saved().await.snapshot.usage.tool_attempts, 0);
}
