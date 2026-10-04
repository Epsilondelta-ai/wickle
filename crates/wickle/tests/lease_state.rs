//! Trusted adapter lease metadata cannot change business state or ownership.
mod support;
use support::{admission, finished, id, prepared, scope};
use wickle::*;

#[tokio::test]
async fn expiry_hydration_preserves_business_state_and_current_fencing() {
    let source = MemoryStateStore::new();
    source
        .admit(
            &scope(),
            admission("run", "request", "session", "private input", "1").await,
        )
        .await
        .unwrap();
    let lease = source
        .acquire_lease(&scope(), &id("run"), &id("owner"), 1000, 100)
        .await
        .unwrap();
    let original = source.load(&scope(), &id("run")).await.unwrap();
    let mut copy = MemoryStateStore::from_checkpoint(source.export_checkpoint(&scope()).unwrap());
    let renewed = source
        .renew_lease(&scope(), &id("run"), &lease, 1050, 200)
        .await
        .unwrap();
    copy.hydrate_lease_states(&scope(), &source.export_lease_states(&scope()).unwrap())
        .unwrap();
    assert_eq!(copy.load(&scope(), &id("run")).await.unwrap(), original);
    assert_eq!(
        copy.check_lease(&scope(), &id("run"), &lease, 1150)
            .await
            .unwrap(),
        renewed
    );
    assert_eq!(
        copy.acquire_lease(&scope(), &id("run"), &id("competitor"), 1150, 100)
            .await
            .unwrap_err()
            .code,
        ErrorCode::LeaseBusy
    );
    let checkpoint = copy.export_checkpoint(&scope()).unwrap();
    let encoded = serde_json::to_string(&checkpoint).unwrap();
    let decoded =
        StateStoreCheckpoint::from_json(&encoded, &scope(), &checkpoint.digest()).unwrap();
    let reopened = MemoryStateStore::from_checkpoint(decoded);
    assert_eq!(reopened.load(&scope(), &id("run")).await.unwrap(), original);
    assert_eq!(
        reopened
            .check_lease(&scope(), &id("run"), &lease, 1150)
            .await
            .unwrap(),
        renewed
    );
    assert_eq!(
        reopened
            .check_lease(&scope(), &id("run"), &lease, 1250)
            .await
            .unwrap_err()
            .code,
        ErrorCode::LeaseLost
    );
}

#[tokio::test]
async fn terminal_checkpoint_cannot_receive_a_live_lease() {
    let mut store = MemoryStateStore::new();
    store
        .admit(
            &scope(),
            admission("run", "request", "session", "data", "1").await,
        )
        .await
        .unwrap();
    let lease = store
        .acquire_lease(&scope(), &id("run"), &id("owner"), 1000, 100)
        .await
        .unwrap();
    let initial = store.load(&scope(), &id("run")).await.unwrap();
    let ready = store
        .commit(
            &scope(),
            &id("run"),
            prepared(&initial.snapshot, lease.clone(), 1001),
        )
        .await
        .unwrap();
    store
        .commit(
            &scope(),
            &id("run"),
            finished(&ready.snapshot, lease.clone(), 1002),
        )
        .await
        .unwrap();
    let exported = store.export_lease_states(&scope()).unwrap();
    assert_eq!(exported[0].status, RunStatus::Succeeded);
    assert_eq!(exported[0].last_fencing_token, lease.fencing_token);
    assert!(exported[0].lease.is_none());
    let mut injected = exported.clone();
    injected[0].lease = Some(lease);
    assert_eq!(
        store
            .hydrate_lease_states(&scope(), &injected)
            .unwrap_err()
            .code,
        ErrorCode::InvalidSnapshot
    );
    assert_eq!(store.export_lease_states(&scope()).unwrap(), exported);
    let business = store.load(&scope(), &id("run")).await.unwrap();
    store.hydrate_lease_states(&scope(), &exported).unwrap();
    assert_eq!(store.load(&scope(), &id("run")).await.unwrap(), business);
    assert_eq!(
        store
            .acquire_lease(&scope(), &id("run"), &id("new-owner"), 1003, 100)
            .await
            .unwrap_err()
            .code,
        ErrorCode::InvalidTransition
    );
}

#[tokio::test]
async fn invalid_lease_sets_are_rejected_before_any_expiry_is_applied() {
    let mut store = MemoryStateStore::new();
    for (run, request, session) in [("a", "qa", "sa"), ("b", "qb", "sb")] {
        store
            .admit(
                &scope(),
                admission(run, request, session, "data", "1").await,
            )
            .await
            .unwrap();
        store
            .acquire_lease(&scope(), &id(run), &id("owner"), 1000, 100)
            .await
            .unwrap();
    }
    let original = store.export_lease_states(&scope()).unwrap();
    let mut candidates = Vec::new();
    candidates.push(original[..1].to_vec());
    candidates.push(vec![original[0].clone(), original[0].clone()]);
    let mut unknown = original.clone();
    unknown[1].run_id = id("unknown");
    candidates.push(unknown);
    let mut status = original.clone();
    status[1].status = RunStatus::Succeeded;
    candidates.push(status);
    let mut generation = original.clone();
    generation[1].last_fencing_token += 1;
    candidates.push(generation);
    let mut released = original.clone();
    released[1].lease = None;
    candidates.push(released);
    for field in ["owner", "scope", "run", "generation", "expiry"] {
        let mut invalid = original.clone();
        let lease = invalid[1].lease.as_mut().unwrap();
        match field {
            "owner" => lease.owner = id("other-owner"),
            "scope" => lease.scope.tenant_id = id("other-tenant"),
            "run" => lease.run_id = id("other-run"),
            "generation" => lease.fencing_token += 1,
            "expiry" => lease.expires_at_ms -= 1,
            _ => unreachable!(),
        }
        candidates.push(invalid);
    }
    for mut candidate in candidates {
        candidate[0].lease.as_mut().unwrap().expires_at_ms += 100;
        assert_eq!(
            store
                .hydrate_lease_states(&scope(), &candidate)
                .unwrap_err()
                .code,
            ErrorCode::InvalidSnapshot
        );
        assert_eq!(store.export_lease_states(&scope()).unwrap(), original);
    }
}

#[tokio::test]
async fn hydration_cannot_take_over_release_or_resurrect_ownership() {
    let source = MemoryStateStore::new();
    source
        .admit(
            &scope(),
            admission("run", "request", "session", "data", "1").await,
        )
        .await
        .unwrap();
    let lease = source
        .acquire_lease(&scope(), &id("run"), &id("owner"), 1000, 100)
        .await
        .unwrap();
    let mut copy = MemoryStateStore::from_checkpoint(source.export_checkpoint(&scope()).unwrap());
    source
        .release_lease(&scope(), &id("run"), &lease, 1050)
        .await
        .unwrap();
    assert!(
        copy.hydrate_lease_states(&scope(), &source.export_lease_states(&scope()).unwrap())
            .is_err()
    );
    let mut released =
        MemoryStateStore::from_checkpoint(source.export_checkpoint(&scope()).unwrap());
    assert!(
        released
            .hydrate_lease_states(&scope(), &copy.export_lease_states(&scope()).unwrap())
            .is_err()
    );
    let successor = source
        .acquire_lease(&scope(), &id("run"), &id("new-owner"), 1200, 100)
        .await
        .unwrap();
    assert!(
        copy.hydrate_lease_states(&scope(), &source.export_lease_states(&scope()).unwrap())
            .is_err()
    );
    assert!(
        source
            .check_lease(&scope(), &id("run"), &lease, 1201)
            .await
            .is_err()
    );
    assert!(
        source
            .check_lease(&scope(), &id("run"), &successor, 1201)
            .await
            .is_ok()
    );
    copy.hydrate_lease_states(&scope(), &copy.export_lease_states(&scope()).unwrap())
        .unwrap();
}
