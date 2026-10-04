//! Typed lease metadata for trusted external state-store adapters.
use super::*;

/// Lease ownership associated with one validated Run checkpoint.
///
/// This is storage metadata, not an authorization grant. Hosts must persist
/// ownership/status changes atomically with their full checkpoint and use their
/// current authoritative lease for all fencing checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunLeaseState {
    /// Run within the exported namespace.
    pub run_id: Id,
    /// Status from the validated business checkpoint.
    pub status: RunStatus,
    /// Last issued fencing generation, retained after lease release.
    pub last_fencing_token: u64,
    /// Current ownership, including its store-issued expiry.
    pub lease: Option<RunLease>,
}

impl MemoryStateStore {
    /// Export only lease metadata from a validated namespace.
    ///
    /// No transcript, protected record or business snapshot is returned.
    pub fn export_lease_states(&self, scope: &Scope) -> Result<Vec<RunLeaseState>, ContractError> {
        let scopes = self.lock()?;
        let state = namespace(&scopes, scope)?;
        Ok(state
            .runs
            .iter()
            .map(|(run_id, run)| RunLeaseState {
                run_id: run_id.clone(),
                status: run.snapshot.status,
                last_fencing_token: run.last_fencing_token,
                lease: run.lease.clone(),
            })
            .collect())
    }

    /// Hydrate expiries from a trusted adapter's authoritative lease metadata.
    ///
    /// Apply this only to a private working copy, never an immutable shared
    /// cache. The complete Run set, statuses and ownership generations must
    /// match. Apply the exact same-owner expiry, including a shorter TTL; this cannot acquire,
    /// release, take over or resurrect a lease. Validation precedes all changes.
    pub fn hydrate_lease_states(
        &mut self,
        scope: &Scope,
        leases: &[RunLeaseState],
    ) -> Result<(), ContractError> {
        let mut scopes = self.lock()?;
        let state = scopes.get_mut(&scope_key(scope)).ok_or_else(not_found)?;
        if leases.len() != state.runs.len() {
            return Err(error(ErrorCode::InvalidSnapshot, "lease_states.runs"));
        }
        let mut seen = BTreeSet::new();
        for entry in leases {
            if !seen.insert(entry.run_id.clone()) {
                return Err(error(ErrorCode::InvalidSnapshot, "lease_states.duplicate"));
            }
            let run = state
                .runs
                .get(&entry.run_id)
                .ok_or_else(|| error(ErrorCode::InvalidSnapshot, "lease_states.run"))?;
            if entry.status != run.snapshot.status
                || entry.last_fencing_token != run.last_fencing_token
            {
                return Err(error(ErrorCode::InvalidSnapshot, "lease_states.identity"));
            }
            match (&run.lease, &entry.lease) {
                (None, None) => {}
                (Some(current), Some(incoming))
                    if !entry.status.is_terminal()
                        && incoming.scope == *scope
                        && incoming.run_id == entry.run_id
                        && incoming.owner == current.owner
                        && incoming.fencing_token == current.fencing_token
                        && incoming.fencing_token == entry.last_fencing_token => {}
                _ => return Err(error(ErrorCode::InvalidSnapshot, "lease_states.ownership")),
            }
        }
        for entry in leases {
            state
                .runs
                .get_mut(&entry.run_id)
                .expect("validated Run")
                .lease = entry.lease.clone();
        }
        Ok(())
    }
}
