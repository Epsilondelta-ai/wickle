use super::*;
use std::collections::BTreeSet;

impl Agent {
    /// Persist an authorized, deduplicated control. Receipt is acceptance, not
    /// completion; remote Worker notification remains the Host's responsibility.
    pub async fn submit_control_command(
        &self,
        run_id: Id,
        command: ControlCommand,
        context: ExecutionContext,
    ) -> Result<Guarded<ControlReceipt>, ContractError> {
        self.check_scope(&context)?;
        if command.principal_ref != context.data.principal_ref {
            return Err(fail(ErrorCode::AccessDenied, "control.principal"));
        }
        let agent = self.clone();
        tokio::runtime::Handle::try_current()
            .map_err(|_| fail(ErrorCode::RuntimeUnavailable, "agent.runtime"))?
            .spawn(crate::future::boxed(|| async move {
                agent.submit_control_owned(run_id, command, context).await
            }))
            .await
            .map_err(|_| fail(ErrorCode::InvalidContract, "control.coordinator"))?
    }

    async fn submit_control_owned(
        &self,
        run_id: Id,
        command: ControlCommand,
        context: ExecutionContext,
    ) -> Result<Guarded<ControlReceipt>, ContractError> {
        let bindings = &self.inner.bindings;
        let action = match command.action {
            ControlAction::Cancel { .. } => PolicyAction::CancelRun {},
            ControlAction::Stop { cause } => PolicyAction::StopExecution { cause },
            ControlAction::Expire => PolicyAction::ExpireRun {},
        };
        let request = PolicyRequest {
            owner_scope: bindings.scope.clone(),
            resource_id: run_id.clone(),
            action,
        };
        bindings
            .policy
            .guard(&request, &context, None, None, || async {
                let receipt = bindings
                    .state
                    .submit_control_command(&bindings.scope, &run_id, command.clone())
                    .await?;
                if receipt.processed_segment_id.is_some() {
                    return Ok(receipt);
                }
                let saved = bindings.state.load(&bindings.scope, &run_id).await?;
                if matches!(
                    saved.snapshot.status,
                    RunStatus::Waiting | RunStatus::Interrupted
                ) {
                    return self
                        .settle_idle_control(&run_id, &command, &context, &request)
                        .await;
                }
                let local = self
                    .inner
                    .runs
                    .lock()
                    .map_err(|_| fail(ErrorCode::InvalidContract, "agent.local_state"))?
                    .get(&run_id)
                    .cloned();
                if let Some(local) = local {
                    self.signal_pending_control(&run_id, &local, None).await?;
                }
                Ok(receipt)
            })
            .await
    }

    /// Read a control receipt without processing it, acquiring a lease or changing state.
    pub async fn get_control_receipt(
        &self,
        run_id: &Id,
        command_id: &Id,
        context: &ExecutionContext,
    ) -> Result<Guarded<ControlReceipt>, ContractError> {
        self.check_scope(context)?;
        let bindings = &self.inner.bindings;
        let request = PolicyRequest {
            owner_scope: bindings.scope.clone(),
            resource_id: run_id.clone(),
            action: PolicyAction::ReadRunDetails {},
        };
        bindings
            .policy
            .guard(&request, context, None, None, || async {
                let history = self
                    .resume_read(
                        context,
                        bindings.state.read_execution(&bindings.scope, run_id),
                    )
                    .await?;
                let control = history
                    .controls
                    .iter()
                    .find(|item| item.command.command_id == *command_id)
                    .ok_or_else(|| fail(ErrorCode::StateNotFound, "control.command"))?;
                match bindings
                    .policy
                    .guard(&request, context, None, None, || async { Ok(()) })
                    .await?
                {
                    Guarded::Completed(()) => Ok(ControlReceipt {
                        run_id: run_id.clone(),
                        command_id: command_id.clone(),
                        processed_segment_id: control.processed_segment_id.clone(),
                    }),
                    Guarded::ApprovalRequired(_) => {
                        Err(fail(ErrorCode::AccessDenied, "control.receipt_approval"))
                    }
                }
            })
            .await
    }

    /// Process a durable command from a Host-delivered notification. The Worker
    /// needs current processing permission; this method never steals a live lease.
    /// Cancel/expiry can settle a Running checkpoint after its worker's lease expires,
    /// without invoking the model, tool executor, or reconciliation handler.
    pub async fn process_control_command(
        &self,
        run_id: Id,
        command_id: Id,
        context: ExecutionContext,
    ) -> Result<Guarded<ControlReceipt>, ContractError> {
        self.check_scope(&context)?;
        let bindings = &self.inner.bindings;
        let history = self
            .resume_read(
                &context,
                bindings.state.read_execution(&bindings.scope, &run_id),
            )
            .await?;
        let control = history
            .controls
            .iter()
            .find(|item| item.command.command_id == command_id)
            .ok_or_else(|| fail(ErrorCode::StateNotFound, "control.command"))?;
        let request = PolicyRequest {
            owner_scope: bindings.scope.clone(),
            resource_id: run_id.clone(),
            action: PolicyAction::ProcessControl {
                command: Box::new(control.command.clone()),
            },
        };
        bindings
            .policy
            .guard(&request, &context, None, None, || async {
                if control.processed_segment_id.is_some() {
                    return Ok(ControlReceipt {
                        run_id: run_id.clone(),
                        command_id: command_id.clone(),
                        processed_segment_id: control.processed_segment_id.clone(),
                    });
                }
                let saved = bindings.state.load(&bindings.scope, &run_id).await?;
                if matches!(
                    saved.snapshot.status,
                    RunStatus::Waiting | RunStatus::Interrupted
                ) {
                    return self
                        .settle_idle_control(&run_id, &control.command, &context, &request)
                        .await;
                }
                if saved.snapshot.status.is_terminal() {
                    // The store records the terminal no-op without changing the outcome.
                    return bindings
                        .state
                        .submit_control_command(&bindings.scope, &run_id, control.command.clone())
                        .await;
                }
                let local = self
                    .inner
                    .runs
                    .lock()
                    .map_err(|_| fail(ErrorCode::InvalidContract, "agent.local_state"))?
                    .get(&run_id)
                    .cloned();
                if let Some(local) = local.filter(|local| !local.done.load(Ordering::Acquire)) {
                    self.signal_pending_control(&run_id, &local, None).await?;
                } else if saved.snapshot.status == RunStatus::Running
                    && matches!(
                        control.command.action,
                        ControlAction::Cancel { .. } | ControlAction::Expire
                    )
                {
                    return self
                        .prepare_idle_control(&run_id, &control.command, &context, &request)
                        .await;
                }
                Ok(ControlReceipt {
                    run_id,
                    command_id,
                    processed_segment_id: None,
                })
            })
            .await
    }

    pub(super) async fn signal_pending_control(
        &self,
        run_id: &Id,
        local: &Arc<LocalRun>,
        now_ms: Option<i64>,
    ) -> Result<Option<ControlCommand>, ContractError> {
        let bindings = &self.inner.bindings;
        let history = bindings
            .state
            .read_execution(&bindings.scope, run_id)
            .await?;
        let command = history
            .controls
            .iter()
            .filter(|item| item.processed_segment_id.is_none())
            .map(|item| &item.command)
            .find(|command| matches!(command.action, ControlAction::Cancel { .. }))
            .or_else(|| {
                history
                    .controls
                    .iter()
                    .filter(|item| item.processed_segment_id.is_none())
                    .map(|item| &item.command)
                    .find(|command| !matches!(command.action, ControlAction::Expire))
            })
            .cloned();
        let command = match command {
            Some(command) => Some(command),
            None if now_ms.is_some() => {
                let saved = bindings.state.load(&bindings.scope, run_id).await?;
                history
                    .controls
                    .iter()
                    .find(|item| {
                        item.processed_segment_id.is_none()
                            && matches!(item.command.action, ControlAction::Expire)
                            && now_ms.is_some_and(|now| now >= saved.snapshot.timing.deadline_at_ms)
                    })
                    .map(|item| item.command.clone())
            }
            None => None,
        };
        if let Some(command) = &command {
            match &command.action {
                ControlAction::Cancel { reason } => {
                    *local
                        .reason
                        .lock()
                        .map_err(|_| fail(ErrorCode::InvalidContract, "control.reason"))? =
                        Some(reason.clone())
                }
                ControlAction::Stop { cause } => {
                    *local
                        .stop_cause
                        .lock()
                        .map_err(|_| fail(ErrorCode::InvalidContract, "control.cause"))? =
                        Some(*cause)
                }
                ControlAction::Expire => {}
            }
            local.cancel.cancel();
        }
        Ok(command)
    }
}

impl Agent {
    async fn settle_idle_control(
        &self,
        run_id: &Id,
        command: &ControlCommand,
        context: &ExecutionContext,
        authorization: &PolicyRequest,
    ) -> Result<ControlReceipt, ContractError> {
        let deadline = tokio::time::Instant::now()
            + Duration::from_millis(self.inner.bindings.settings.start_timeout_ms);
        loop {
            match self
                .prepare_idle_control(run_id, command, context, authorization)
                .await
            {
                Err(error) if error.code == ErrorCode::LeaseBusy => {
                    tokio::select! { biased;
                        _ = context.cancellation.cancelled() => return Err(fail(ErrorCode::Cancelled, "control.caller")),
                        _ = tokio::time::sleep_until(deadline) => return Err(error),
                        _ = tokio::time::sleep(Duration::from_millis(self.inner.bindings.settings.observer_poll_ms.min(20))) => {},
                    }
                }
                Err(error)
                    if error.code == ErrorCode::RevisionConflict
                        && tokio::time::Instant::now() < deadline =>
                {
                    continue;
                }
                result => return result,
            }
        }
    }
    async fn prepare_idle_control(
        &self,
        run_id: &Id,
        command: &ControlCommand,
        context: &ExecutionContext,
        authorization: &PolicyRequest,
    ) -> Result<ControlReceipt, ContractError> {
        let bindings = &self.inner.bindings;
        let mut saved = self
            .resume_read(context, bindings.state.load(&bindings.scope, run_id))
            .await?;
        let history = self
            .resume_read(
                context,
                bindings.state.read_execution(&bindings.scope, run_id),
            )
            .await?;
        let stored = history
            .controls
            .iter()
            .find(|item| item.command == *command)
            .ok_or_else(|| fail(ErrorCode::RequestConflict, "control.command"))?;
        let receipt = ControlReceipt {
            run_id: run_id.clone(),
            command_id: command.command_id.clone(),
            processed_segment_id: stored.processed_segment_id.clone(),
        };
        if receipt.processed_segment_id.is_some() {
            return Ok(receipt);
        }
        if saved.snapshot.status.is_terminal()
            || matches!(command.action, ControlAction::Stop { .. })
        {
            return bindings
                .state
                .submit_control_command(&bindings.scope, run_id, command.clone())
                .await;
        }
        if !matches!(
            saved.snapshot.status,
            RunStatus::Running | RunStatus::Waiting | RunStatus::Interrupted
        ) {
            return Ok(receipt);
        }
        let mut clock =
            super::segment::PreparationClock::new(bindings.clock.clone(), &saved.snapshot)?;
        let (_, now) = clock.now()?;
        let result = match &command.action {
            ControlAction::Cancel { reason } => OutcomeResult::Cancelled {
                reason: reason.to_string(),
            },
            ControlAction::Expire if now >= saved.snapshot.timing.deadline_at_ms => {
                OutcomeResult::Exhausted {
                    budget: BudgetKind::Elapsed,
                }
            }
            _ => return Ok(receipt),
        };
        let execution = self.execution_context(run_id, context).await?;
        let metadata = self.metadata_segment(&saved, execution).await?;
        let round = self.tool_round_for_snapshot(&saved, &metadata).await?;
        let expected_revision = saved.snapshot.revision;
        let mut messages = vec![];
        let mut events = vec![];
        let mut records = vec![];
        let calls: Vec<_> = saved
            .snapshot
            .tool_ledger
            .iter()
            .filter(|entry| {
                matches!(
                    entry.state,
                    ToolCallState::Planned {}
                        | ToolCallState::ApprovalPending { .. }
                        | ToolCallState::InputPending { .. }
                )
            })
            .map(|entry| entry.call.call_id.clone())
            .collect();
        for call in calls {
            let prepared = round.prepare_unstarted(
                &saved,
                &call,
                ToolResultStatus::Cancelled,
                Id::new("execution_ended")?,
                now,
            )?;
            saved.session.transcript_revision = saved
                .session
                .transcript_revision
                .checked_add(1)
                .ok_or_else(|| fail(ErrorCode::InvalidSnapshot, "control.tool_message"))?;
            saved.messages.push(prepared.message.clone());
            super::resume::apply_resolution(
                &mut saved.snapshot,
                &mut messages,
                &mut events,
                &mut records,
                prepared,
            )?;
        }
        // A dead worker may already have applied a dispatched effect. Record an
        // unresolved observation without invoking the executor or reconciliation.
        let mut unresolved_effects = saved
            .snapshot
            .outcome
            .as_ref()
            .map(|outcome| outcome.unresolved_effects.clone())
            .unwrap_or_default();
        let dispatching = saved
            .snapshot
            .tool_ledger
            .iter()
            .filter_map(|entry| match &entry.state {
                ToolCallState::Dispatching {
                    attempt_id,
                    idempotency_key,
                } => Some((
                    entry.call.clone(),
                    attempt_id.clone(),
                    idempotency_key.clone(),
                )),
                _ => None,
            })
            .collect::<Vec<_>>();
        for (call, attempt_id, idempotency_key) in dispatching {
            let result = ToolResult {
                call_id: call.call_id.clone(),
                call_message_id: crate::tool_execution::call_message(&saved, &call)?,
                status: ToolResultStatus::Unknown,
                effect: ToolEffect::Unknown,
                content: vec![],
                effect_receipt_ref: None,
                skill_ref: None,
                error: Some(Failure {
                    code: Id::new("control_unconfirmed")?,
                    diagnostic_ref: None,
                }),
            };
            let record = ProtectedRecord::new(
                bindings.ids.next_id()?,
                1,
                serde_json::to_value(&result)
                    .map_err(|_| fail(ErrorCode::InvalidJson, "control.tool_result"))?,
            );
            saved
                .snapshot
                .tool_ledger
                .iter_mut()
                .find(|entry| entry.call.call_id == call.call_id)
                .expect("collected call")
                .state = ToolCallState::Unknown {
                attempt_id: attempt_id.clone(),
                idempotency_key: idempotency_key.clone(),
            };
            saved.snapshot.last_event_seq = saved
                .snapshot
                .last_event_seq
                .checked_add(1)
                .ok_or_else(|| fail(ErrorCode::InvalidEvent, "control.tool_event"))?;
            events.push(RunEvent {
                schema_version: RunEventSchemaVersion::V1,
                event_id: bindings.ids.next_id()?,
                scope: bindings.scope.clone(),
                run_id: run_id.clone(),
                session_id: saved.snapshot.request.session_id.clone(),
                seq: saved
                    .snapshot
                    .last_event_seq
                    .try_into()
                    .map_err(|_| fail(ErrorCode::InvalidEvent, "control.tool_event"))?,
                timestamp_ms: now,
                payload: RunEventPayload::ToolUnresolved {
                    result_ref: record.reference().clone(),
                    attempt_id,
                    idempotency_key,
                },
            });
            saved.session.transcript_revision = saved
                .session
                .transcript_revision
                .checked_add(1)
                .ok_or_else(|| fail(ErrorCode::InvalidSnapshot, "control.tool_message"))?;
            let message = Message {
                source_model_request_id: None,
                message_id: bindings.ids.next_id()?,
                run_id: run_id.clone(),
                sequence: saved
                    .session
                    .transcript_revision
                    .try_into()
                    .map_err(|_| fail(ErrorCode::InvalidSnapshot, "control.tool_message"))?,
                role: MessageRole::Tool,
                content: vec![ContentBlock::ToolResult { result }],
                origin: MessageOrigin::Tool,
                visibility: Visibility::UserAndModel,
            };
            saved.messages.push(message.clone());
            messages.push(message);
            unresolved_effects.push(record.reference().clone());
            records.push(record);
        }
        let unknown = saved
            .snapshot
            .tool_ledger
            .iter()
            .filter_map(|entry| match &entry.state {
                ToolCallState::Unknown {
                    attempt_id,
                    idempotency_key,
                } => Some((attempt_id.clone(), idempotency_key.clone())),
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        if !unknown.is_empty() {
            let mut after = 0;
            loop {
                let page = bindings
                    .state
                    .read_events(&bindings.scope, run_id, after, MAX_EVENT_PAGE_SIZE)
                    .await?;
                for event in &page.events {
                    if let RunEventPayload::ToolUnresolved {
                        result_ref,
                        attempt_id,
                        idempotency_key,
                    } = &event.payload
                    {
                        if unknown.contains(&(attempt_id.clone(), idempotency_key.clone()))
                            && !unresolved_effects.contains(result_ref)
                        {
                            unresolved_effects.push(result_ref.clone());
                        }
                    }
                }
                if !page.has_more {
                    break;
                }
                after = page.next_after_seq;
            }
        }
        let output = match &saved.snapshot.outcome {
            Some(previous) => previous.output.clone(),
            None => self.saved_partial_output(&saved.snapshot).await?,
        };
        let artifacts = match &saved.snapshot.outcome {
            Some(previous) => previous.artifacts.clone(),
            None => super::artifacts::produced(&saved.snapshot),
        };
        if let Guarded::ApprovalRequired(_) = bindings
            .policy
            .guard(authorization, context, None, None, || async { Ok(()) })
            .await?
        {
            return Err(fail(ErrorCode::AccessDenied, "control.processing_approval"));
        }
        let (elapsed, now) = clock.now()?;
        let mut snapshot = saved.snapshot;
        snapshot.revision = snapshot
            .revision
            .checked_add(1)
            .ok_or_else(|| fail(ErrorCode::RevisionConflict, "control.revision"))?;
        snapshot.status = result.status();
        snapshot.phase = RunPhase::Finish;
        snapshot.wait = None;
        snapshot.usage.elapsed_ms = elapsed;
        snapshot.timing.last_observed_at_ms = now;
        let outcome = RunOutcome {
            result,
            output,
            artifacts,
            usage: snapshot.usage.clone(),
            checkpoint_revision: snapshot.revision,
            verification: None,
            unresolved_effects,
            app_state: snapshot.app_state.clone(),
        };
        let record = ProtectedRecord::new(
            bindings.ids.next_id()?,
            1,
            serde_json::to_value(&outcome)
                .map_err(|_| fail(ErrorCode::InvalidJson, "control.outcome"))?,
        );
        snapshot.last_event_seq = snapshot
            .last_event_seq
            .checked_add(1)
            .ok_or_else(|| fail(ErrorCode::InvalidEvent, "control.event"))?;
        events.push(RunEvent {
            schema_version: RunEventSchemaVersion::V1,
            event_id: bindings.ids.next_id()?,
            scope: bindings.scope.clone(),
            run_id: run_id.clone(),
            session_id: snapshot.request.session_id.clone(),
            seq: snapshot
                .last_event_seq
                .try_into()
                .map_err(|_| fail(ErrorCode::InvalidEvent, "control.event"))?,
            timestamp_ms: now,
            payload: RunEventPayload::RunFinished {
                outcome_ref: record.reference().clone(),
            },
        });
        records.push(record);
        snapshot.outcome = Some(outcome);
        let started =
            bindings
                .state
                .begin_segment(
                    &bindings.scope,
                    BeginSegmentRequest {
                        run_id: run_id.clone(),
                        expected_revision,
                        segment_id: bindings.ids.next_id()?,
                        owner: bindings.ids.next_id()?,
                        now_ms: now,
                        lease_ttl_ms: bindings.settings.lease_ttl_ms.try_into().map_err(|_| {
                            fail(ErrorCode::InvalidConfiguration, "agent.lease_ttl")
                        })?,
                        start: SegmentStart::Control(command.command_id.clone()),
                        transition: Some(SegmentTransition {
                            snapshot,
                            messages,
                            events,
                            records,
                        }),
                    },
                )
                .await?;
        Ok(ControlReceipt {
            run_id: run_id.clone(),
            command_id: command.command_id.clone(),
            processed_segment_id: Some(started.segment.segment_id),
        })
    }
}
