//! Privileged record partitioning preserves a complete validated checkpoint.
//! Checkpoint metadata remains private; assertion failures never print bodies.
#[allow(dead_code)]
mod support;

use serde_json::{Value, json};
use support::{admission, event, finished, id, prepared, scope};
use wickle::*;

const LARGE_MARKER: &str = "immutable synthetic record body;";

struct Fixture {
    store: MemoryStateStore,
    lease: RunLease,
    large_ref: RecordRef,
}

impl Fixture {
    async fn new() -> Self {
        let store = MemoryStateStore::new();
        store
            .admit(
                &scope(),
                admission("run", "request", "session", "synthetic user input", "1").await,
            )
            .await
            .unwrap();
        let lease = store
            .acquire_lease(&scope(), &id("run"), &id("worker"), 100, 30_000)
            .await
            .unwrap();
        let saved = store.load(&scope(), &id("run")).await.unwrap();
        let large = ProtectedRecord::new(
            id("large-record"),
            7,
            json!({"body":LARGE_MARKER.repeat(32_768)}),
        );
        let large_ref = large.reference().clone();
        let mut change = prepared(&saved.snapshot, lease.clone(), 101);
        change.records.push(large);
        // Distinct revisions of the same record ID must remain distinct keys.
        change.records.push(ProtectedRecord::new(
            id("large-record"),
            8,
            json!({"revision":8}),
        ));
        store.commit(&scope(), &id("run"), change).await.unwrap();
        Self {
            store,
            lease,
            large_ref,
        }
    }

    fn split(&self) -> (CheckpointRecordIndex, Vec<ProtectedRecord>) {
        self.store
            .export_checkpoint(&scope())
            .unwrap()
            .split_records()
            .unwrap()
    }
}

fn parse_saved(index: &CheckpointRecordIndex) -> CheckpointRecordIndex {
    CheckpointRecordIndex::from_json(index.as_json(), &scope(), index.digest()).unwrap()
}

// A revised outer checksum cannot waive the index's structural or full-graph
// checks. Callers still supply the original trusted complete checkpoint digest.
fn parse_rehashed(value: &Value) -> Result<CheckpointRecordIndex, ContractError> {
    CheckpointRecordIndex::from_json(&value.to_string(), &scope(), &canonical_digest(value))
}

#[tokio::test]
async fn partition_roundtrip_preserves_full_checkpoint_records_session_and_lease() {
    let f = Fixture::new().await;
    let before = f.store.load(&scope(), &id("run")).await.unwrap();
    let session = f
        .store
        .load_session(&scope(), &id("session"))
        .await
        .unwrap();
    let checkpoint = f.store.export_checkpoint(&scope()).unwrap();
    let full_json = serde_json::to_string(&checkpoint).unwrap();
    let full_digest = checkpoint.digest();
    let full: Value = serde_json::from_str(&full_json).unwrap();
    let expected_refs: Vec<RecordRef> = full["records"]
        .as_array()
        .unwrap()
        .iter()
        .map(|record| serde_json::from_value(record["reference"].clone()).unwrap())
        .collect();
    let (index, mut records) = checkpoint.split_records().unwrap();
    assert_eq!(index.complete_digest(), &full_digest);
    assert_eq!(index.record_refs(), expected_refs);
    assert!(index.record_refs().contains(&f.large_ref));
    let metadata: Value = serde_json::from_str(index.as_json()).unwrap();
    assert!(
        metadata["checkpoint"]["records"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(
        !index.as_json().contains(LARGE_MARKER),
        "record values must not be in the index"
    );
    assert!(!format!("{index:?}").contains("synthetic user input"));
    assert!(!format!("{index:?}").contains(LARGE_MARKER));
    // Delivery order is not the manifest's stable key order.
    records.reverse();
    let restored_checkpoint = parse_saved(&index).restore(records).unwrap();
    assert_eq!(restored_checkpoint.digest(), full_digest);
    assert!(
        serde_json::to_string(&restored_checkpoint).unwrap() == full_json,
        "partitioning must preserve the exact full checkpoint bytes"
    );
    let restored = MemoryStateStore::from_checkpoint(restored_checkpoint);
    assert!(
        restored.load(&scope(), &id("run")).await.unwrap() == before,
        "the restored Run must retain its complete state"
    );
    assert!(
        restored
            .load_session(&scope(), &id("session"))
            .await
            .unwrap()
            == session,
        "the restored session must retain its transcript and identity"
    );
    assert_eq!(
        restored
            .check_lease(&scope(), &id("run"), &f.lease, 102)
            .await
            .unwrap(),
        f.lease
    );
    assert!(
        restored.read_record(&scope(), &f.large_ref).await.unwrap()
            == f.store.read_record(&scope(), &f.large_ref).await.unwrap(),
        "the restored immutable record must retain its value and identity"
    );
    assert!(
        restored
            .read_record(
                &scope(),
                &RecordRef {
                    record_id: id("large-record"),
                    revision: 8,
                    digest: canonical_digest(&json!({"revision":8})),
                }
            )
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn record_set_rejects_missing_extra_duplicate_changed_value_and_changed_revision() {
    let f = Fixture::new().await;
    let (index, records) = f.split();
    let large = records
        .iter()
        .position(|record| record.reference() == &f.large_ref)
        .unwrap();
    let mut missing = records.clone();
    missing.remove(large);
    let mut extra = records.clone();
    extra.push(ProtectedRecord::new(
        id("unmanifested"),
        1,
        json!({"extra":true}),
    ));
    let mut duplicate = records.clone();
    // Preserve cardinality so a duplicate cannot be rejected by length alone.
    let another = (large + 1) % records.len();
    duplicate[another] = records[large].clone();
    let mut changed = records.clone();
    changed[large] = ProtectedRecord::new(
        f.large_ref.record_id.clone(),
        f.large_ref.revision,
        json!({"body":"changed immutable value"}),
    );
    let mut revised = records.clone();
    revised[large] = ProtectedRecord::new(
        f.large_ref.record_id.clone(),
        99,
        records[large].value().clone(),
    );
    for supplied in [missing, extra, duplicate, changed, revised] {
        assert!(
            parse_saved(&index).restore(supplied).is_err(),
            "an incompatible immutable record set must be rejected"
        );
    }
}

#[tokio::test]
async fn trusted_scope_index_digest_version_and_inline_record_guards_remain_independent() {
    let f = Fixture::new().await;
    let (index, records) = f.split();
    let mut foreign = scope();
    foreign.user_id = Some(id("other-user"));
    assert_eq!(
        CheckpointRecordIndex::from_json(index.as_json(), &foreign, index.digest())
            .unwrap_err()
            .code,
        ErrorCode::AccessDenied
    );
    assert!(
        CheckpointRecordIndex::from_json(
            index.as_json(),
            &scope(),
            &canonical_digest(&json!("wrong trusted index digest"))
        )
        .is_err()
    );
    let original: Value = serde_json::from_str(index.as_json()).unwrap();
    let mut changed_bytes = original.clone();
    changed_bytes["checkpoint"]["runs"][0]["lease"]["expires_at_ms"] = json!(50_000);
    assert!(
        CheckpointRecordIndex::from_json(&changed_bytes.to_string(), &scope(), index.digest())
            .is_err()
    );
    // The changed lease is structurally valid but is not the original graph.
    assert!(
        parse_rehashed(&changed_bytes)
            .unwrap()
            .restore(records.clone())
            .is_err()
    );
    let mut version = original.clone();
    version["schema_version"] = json!("wickle.state-store-record-index.v99");
    assert_eq!(
        parse_rehashed(&version).unwrap_err().code,
        ErrorCode::UnsupportedSchemaVersion
    );
    let mut changed_scope = original.clone();
    changed_scope["checkpoint"]["scope"]["workspace_id"] = json!("another-workspace");
    assert_eq!(
        parse_rehashed(&changed_scope).unwrap_err().code,
        ErrorCode::AccessDenied
    );
    let mut inline = original.clone();
    inline["checkpoint"]["records"] = json!([{
        "reference":records[0].reference(), "value":records[0].value(),
    }]);
    assert!(
        parse_rehashed(&inline).is_err(),
        "inline values cannot accompany a record index"
    );
    let mut graph_version = original.clone();
    graph_version["checkpoint"]["schema_version"] = json!("wickle.state-store.v99");
    assert!(
        parse_rehashed(&graph_version)
            .unwrap()
            .restore(records)
            .is_err()
    );
}

#[tokio::test]
async fn revised_index_checksums_cannot_bypass_manifest_full_digest_or_graph_validation() {
    let f = Fixture::new().await;
    let (index, records) = f.split();
    let original: Value = serde_json::from_str(index.as_json()).unwrap();
    let large = records
        .iter()
        .position(|record| record.reference() == &f.large_ref)
        .unwrap();
    let mut reordered = original.clone();
    reordered["records"].as_array_mut().unwrap().swap(0, 1);
    assert!(
        parse_rehashed(&reordered).is_err(),
        "manifest key order must be canonical"
    );
    let mut duplicate = original.clone();
    let first = duplicate["records"][0].clone();
    duplicate["records"]
        .as_array_mut()
        .unwrap()
        .insert(1, first);
    assert!(
        parse_rehashed(&duplicate).is_err(),
        "duplicate manifest identities must be rejected"
    );
    let mut missing = original.clone();
    missing["records"].as_array_mut().unwrap().remove(large);
    let mut missing_records = records.clone();
    missing_records.remove(large);
    assert!(
        parse_rehashed(&missing)
            .unwrap()
            .restore(missing_records)
            .is_err(),
        "a matching reduced record set must not change the trusted full checkpoint"
    );
    let mut forged = original.clone();
    let replacement = ProtectedRecord::new(
        f.large_ref.record_id.clone(),
        f.large_ref.revision,
        json!({"body":"substituted value with a valid new digest"}),
    );
    forged["records"][large] = serde_json::to_value(replacement.reference()).unwrap();
    let mut substituted = records.clone();
    substituted[large] = replacement;
    assert!(
        parse_rehashed(&forged)
            .unwrap()
            .restore(substituted)
            .is_err(),
        "consistent forged manifest/value pairs must not change the trusted full checkpoint"
    );
    let mut full_digest = original.clone();
    full_digest["complete_digest"] =
        serde_json::to_value(canonical_digest(&json!("different full checkpoint"))).unwrap();
    assert!(
        parse_rehashed(&full_digest)
            .unwrap()
            .restore(records.clone())
            .is_err()
    );
    // Even recomputing both checksums cannot validate an internally invalid
    // lease graph. Restoration must retain the existing full-graph guards.
    let mut invalid_full =
        serde_json::to_value(f.store.export_checkpoint(&scope()).unwrap()).unwrap();
    invalid_full["runs"][0]["last_fencing_token"] = json!(0);
    let mut invalid_graph = original;
    invalid_graph["checkpoint"]["runs"][0]["last_fencing_token"] = json!(0);
    invalid_graph["complete_digest"] =
        serde_json::to_value(canonical_digest(&invalid_full)).unwrap();
    assert!(
        parse_rehashed(&invalid_graph)
            .unwrap()
            .restore(records)
            .is_err()
    );
}

#[tokio::test]
async fn partition_preserves_historical_wait_resume_and_terminal_records_after_a_new_run() {
    let store = MemoryStateStore::new();
    store
        .admit(
            &scope(),
            admission("run-a", "request-a", "session", "first input", "1").await,
        )
        .await
        .unwrap();
    let lease = store
        .acquire_lease(&scope(), &id("run-a"), &id("worker"), 100, 100)
        .await
        .unwrap();
    let snapshot = store.load(&scope(), &id("run-a")).await.unwrap().snapshot;
    let candidate = ProtectedRecord::new(id("candidate"), 1, json!({"source":"saved selection"}));
    let target = ApprovalTarget::Candidate {
        candidate_ref: candidate.reference().clone(),
        verifier_ref: VersionedRef {
            id: id("verifier"),
            version: id("1"),
        },
    };
    let wait = WaitState {
        wait_id: id("wait"),
        target: WaitTarget::Approval {
            target: target.clone(),
        },
        expires_at_ms: Some(180),
    };
    let wait_record = ProtectedRecord::new(id("old-wait"), 1, serde_json::to_value(&wait).unwrap());
    let mut waiting = prepared(&snapshot, lease.clone(), 101);
    waiting.snapshot.status = RunStatus::Waiting;
    waiting.snapshot.phase = RunPhase::Waiting;
    waiting.snapshot.wait = Some(wait);
    waiting.snapshot.outcome = Some(RunOutcome {
        app_state: None,
        result: OutcomeResult::Waiting {
            wait: waiting.snapshot.wait.clone().unwrap(),
        },
        output: vec![],
        artifacts: vec![],
        usage: waiting.snapshot.usage.clone(),
        checkpoint_revision: waiting.snapshot.revision,
        verification: None,
        unresolved_effects: vec![],
    });
    let prior_outcome = ProtectedRecord::new(
        id("old-outcome"),
        1,
        serde_json::to_value(waiting.snapshot.outcome.as_ref().unwrap()).unwrap(),
    );
    let prior_outcome_ref = prior_outcome.reference().clone();
    waiting.records.push(prior_outcome);
    waiting.records.push(candidate);
    waiting.snapshot.last_event_seq += 1;
    waiting.events.push(event(
        &id("run-a"),
        &id("session"),
        &scope(),
        2,
        RunEventPayload::RunWaiting {
            outcome_ref: Some(prior_outcome_ref.clone()),
            wait_ref: wait_record.reference().clone(),
        },
    ));
    waiting.records.push(wait_record);
    waiting.events[0].timestamp_ms = 101;
    let waiting = store.commit(&scope(), &id("run-a"), waiting).await.unwrap();
    let command = ResumeCommand {
        run_id: id("run-a"),
        expected_revision: waiting.snapshot.revision,
        command_id: id("answer"),
        action: ResumeAction::Approve {
            wait_id: id("wait"),
            target,
        },
    };
    let command_record = ProtectedRecord::new(
        id("old-command"),
        1,
        serde_json::to_value(&command).unwrap(),
    );
    let mut resumed = prepared(&waiting.snapshot, lease.clone(), 102);
    resumed.snapshot.status = RunStatus::Running;
    resumed.snapshot.wait = None;
    resumed.snapshot.outcome = None;
    resumed.snapshot.timing.last_observed_at_ms = 102;
    resumed.snapshot.usage.elapsed_ms = (102 - resumed.snapshot.timing.started_at_ms) as u64;
    resumed.snapshot.resume_receipts.push(ResumeReceipt {
        command: command.clone(),
        command_ref: command_record.reference().clone(),
        accepted_revision: resumed.snapshot.revision,
        previous_segment_start_revision: 0,
        previous_outcome_ref: prior_outcome_ref,
        previous_last_event_seq: waiting.snapshot.last_event_seq,
        actor_ref: id("reviewer"),
        capability_grant_ref: id("reviewer-grant"),
        expired: false,
    });
    resumed.snapshot.last_event_seq += 1;
    resumed.events.push(event(
        &id("run-a"),
        &id("session"),
        &scope(),
        3,
        RunEventPayload::RunResumed {
            command_ref: command_record.reference().clone(),
        },
    ));
    resumed.events[0].timestamp_ms = 102;
    resumed.records.push(command_record);
    let resumed = store.commit(&scope(), &id("run-a"), resumed).await.unwrap();
    store
        .commit(
            &scope(),
            &id("run-a"),
            finished(&resumed.snapshot, lease, 103),
        )
        .await
        .unwrap();
    let mut next = admission("run-b", "request-b", "session", "next input", "2").await;
    next.messages[0].sequence = 2.try_into().unwrap();
    store.admit(&scope(), next).await.unwrap();

    let old_before = store.load(&scope(), &id("run-a")).await.unwrap();
    let new_before = store.load(&scope(), &id("run-b")).await.unwrap();
    let session_before = store.load_session(&scope(), &id("session")).await.unwrap();
    let events_before = store
        .read_events(&scope(), &id("run-a"), 0, 100)
        .await
        .unwrap();
    let checkpoint = store.export_checkpoint(&scope()).unwrap();
    let full_digest = checkpoint.digest();
    let (index, records) = checkpoint.split_records().unwrap();
    let wait_ref = events_before.events[1].payload.clone();
    let command_ref = events_before.events[2].payload.clone();
    let RunEventPayload::RunWaiting { wait_ref, .. } = wait_ref else {
        panic!("fixture must retain a historical waiting event");
    };
    let RunEventPayload::RunResumed { command_ref } = command_ref else {
        panic!("fixture must retain a historical resume event");
    };
    assert!(index.record_refs().contains(&wait_ref));
    assert!(index.record_refs().contains(&command_ref));
    let rebuilt = parse_saved(&index).restore(records).unwrap();
    assert_eq!(rebuilt.digest(), full_digest);
    let restored = MemoryStateStore::from_checkpoint(rebuilt);
    let old = restored.load(&scope(), &id("run-a")).await.unwrap();
    assert!(
        old == old_before,
        "completed Run history must survive record partitioning"
    );
    assert_eq!(old.snapshot.status, RunStatus::Succeeded);
    assert!(old.snapshot.wait.is_none());
    assert_eq!(old.session.active_run_id, Some(id("run-b")));
    assert!(
        restored.load(&scope(), &id("run-b")).await.unwrap() == new_before,
        "the next Run must retain its own state"
    );
    assert!(
        restored
            .load_session(&scope(), &id("session"))
            .await
            .unwrap()
            == session_before,
        "historical and current transcript identity must be preserved"
    );
    assert!(
        restored
            .read_events(&scope(), &id("run-a"), 0, 100)
            .await
            .unwrap()
            == events_before,
        "waiting, resume and terminal event replay must remain unchanged"
    );
    for reference in [&wait_ref, &command_ref] {
        assert!(
            restored.read_record(&scope(), reference).await.unwrap()
                == store.read_record(&scope(), reference).await.unwrap(),
            "retired historical records must retain exact values and identities"
        );
    }
}

#[tokio::test]
async fn partition_preserves_a_validated_legacy_v1_checkpoint() {
    let store = MemoryStateStore::new();
    store
        .admit(
            &scope(),
            admission("run", "request", "session", "legacy synthetic input", "1").await,
        )
        .await
        .unwrap();
    let lease = store
        .acquire_lease(&scope(), &id("run"), &id("legacy-owner"), 1000, 30_000)
        .await
        .unwrap();
    // Use the same authentic legacy fixture construction as execution_store:
    // v1 has no execution history or invented actor/segment evidence.
    let mut legacy: Value =
        serde_json::to_value(store.export_checkpoint(&scope()).unwrap()).unwrap();
    legacy["schema_version"] = json!("wickle.state-store.v1");
    legacy.as_object_mut().unwrap().remove("executions");
    legacy.as_object_mut().unwrap().remove("legacy_runs");
    let digest = canonical_digest(&legacy);
    let validated =
        StateStoreCheckpoint::from_json(&legacy.to_string(), &scope(), &digest).unwrap();
    let full_json = serde_json::to_string(&validated).unwrap();
    let baseline = MemoryStateStore::from_checkpoint(validated.clone());
    let (index, records) = validated.split_records().unwrap();
    let restored_checkpoint = parse_saved(&index).restore(records).unwrap();
    assert_eq!(restored_checkpoint.digest(), digest);
    assert!(
        serde_json::to_string(&restored_checkpoint).unwrap() == full_json,
        "legacy bytes must be preserved"
    );
    let restored = MemoryStateStore::from_checkpoint(restored_checkpoint);
    assert!(
        restored.load(&scope(), &id("run")).await.unwrap()
            == baseline.load(&scope(), &id("run")).await.unwrap(),
        "legacy state must be preserved"
    );
    assert_eq!(
        restored
            .check_lease(&scope(), &id("run"), &lease, 1001)
            .await
            .unwrap(),
        lease
    );
    assert!(
        restored.read_execution(&scope(), &id("run")).await.is_err(),
        "partitioning cannot invent legacy execution history"
    );
}
