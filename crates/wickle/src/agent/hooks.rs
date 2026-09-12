use super::*;

impl Agent {
    pub(super) async fn before_run(
        &self,
        budget: &RunBudget,
        context: &ExecutionContext,
    ) -> Result<Vec<ContextItem>, ContractError> {
        let Some(hooks) = &self.inner.bindings.hooks else {
            return Ok(vec![]);
        };
        let saved = self
            .inner
            .bindings
            .state
            .load(budget.scope(), budget.run_id())
            .await?;
        let transformed = hooks
            .transform(
                HookTarget::BeforeRun,
                HookInput::BeforeRun {
                    user_input: saved.snapshot.request.input.clone(),
                    context_items: vec![],
                },
                context,
                budget,
            )
            .await?;
        Ok(transformed.context_items)
    }
    pub(super) async fn before_model(
        &self,
        step: &Id,
        user_input: Vec<InputContent>,
        context_items: Vec<ContextItem>,
        context: &ExecutionContext,
        budget: &RunBudget,
    ) -> Result<Vec<ContextItem>, ContractError> {
        let Some(hooks) = &self.inner.bindings.hooks else {
            return Ok(context_items);
        };
        let transformed = hooks
            .transform(
                HookTarget::BeforeModel {
                    model_step_id: step.clone(),
                },
                HookInput::BeforeModel {
                    user_input,
                    context_items,
                },
                context,
                budget,
            )
            .await?;
        Ok(transformed.context_items)
    }
    pub(super) fn remember_observer_error(&self, local: &LocalRun, error: Option<ContractError>) {
        if let Some(error) = error {
            if let Ok(mut slot) = local.observer_error.lock() {
                *slot = Some(fail(error.code, "hooks.observer_report"));
            }
        }
    }
    pub(super) async fn after_run(
        &self,
        saved: &StoredRun,
        context: &ExecutionContext,
        local: &LocalRun,
    ) {
        let Some(hooks) = &self.inner.bindings.hooks else {
            return;
        };
        let Some(outcome) = &saved.snapshot.outcome else {
            return;
        };
        if !saved.snapshot.status.is_terminal() {
            return;
        }
        let mut data = context.data.clone();
        data.system_inputs = None;
        let cleanup = ExecutionContext::new(data, CancellationToken::new());
        let observed = caller_read(&cleanup, Some(Duration::from_secs(30)), async {
            let page = self
                .inner
                .bindings
                .state
                .read_events(
                    &saved.snapshot.scope,
                    &saved.snapshot.run_id,
                    saved.snapshot.last_event_seq.saturating_sub(1),
                    1,
                )
                .await?;
            let Some(RunEvent {
                payload: RunEventPayload::RunFinished { outcome_ref },
                ..
            }) = page.events.last()
            else {
                return Err(fail(ErrorCode::InvalidSnapshot, "hooks.terminal_event"));
            };
            hooks
                .observe(
                    &saved.snapshot.run_id,
                    HookTarget::AfterRun {
                        outcome_ref: outcome_ref.clone(),
                        revision: saved.snapshot.revision,
                    },
                    HookInput::run_observed(outcome),
                    &cleanup,
                )
                .await
        })
        .await;
        self.remember_observer_error(local, observed.err());
    }
    pub(super) async fn after_tool(
        &self,
        run_id: &Id,
        target: HookTarget,
        input: HookInput,
        context: &ExecutionContext,
    ) -> Option<ContractError> {
        let Some(hooks) = &self.inner.bindings.hooks else {
            return None;
        };
        let mut data = context.data.clone();
        data.system_inputs = None;
        let cleanup = ExecutionContext::new(data, CancellationToken::new());
        hooks
            .observe(run_id, target, input, &cleanup)
            .await
            .err()
            .map(|error| fail(error.code, "hooks.observer_report"))
    }
}
