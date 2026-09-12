//! Bounded lifecycle transformations and observations. Hooks receive selected
//! data, never a mutable Run, credentials, or the system-input map.

use crate::*;
use serde::{Deserialize, Serialize};
use std::{fmt, sync::Arc};
use tokio_util::sync::CancellationToken;

mod records;
mod runtime;
pub(crate) use records::validate_application_chain;

/// Exact callback contract pinned for one Run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HookDefinition {
    /// Registered identity and exact version.
    pub hook: VersionedRef,
    /// Single permitted lifecycle position.
    pub position: HookPosition,
    /// Lower priorities execute first; IDs break ties.
    pub priority: i32,
    /// Only optional before_run callback failures may continue as warnings.
    pub required: bool,
    /// Positive finite callback timeout, at most one day.
    pub timeout_ms: u64,
    /// Positive byte bound on serialized callback output.
    pub max_output_bytes: usize,
}
impl HookDefinition {
    /// Identity of the complete callback contract, including its bounds.
    pub fn digest(&self) -> JsonDigest {
        crate::serialization::data_digest(self)
    }
    /// Reject unbounded callback contracts before admission.
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.timeout_ms == 0
            || self.timeout_ms > 86_400_000
            || self.max_output_bytes == 0
            || self.max_output_bytes > 16_777_216
        {
            return Err(hook_error(ErrorCode::InvalidContract, "hooks.definition"));
        }
        Ok(())
    }
}

/// A stable logical lifecycle target, independent of physical retry attempts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum HookTarget {
    /// Initial admitted Run preparation.
    BeforeRun,
    /// One logical step, reused across transport retry and route fallback.
    BeforeModel {
        /// Logical step identity.
        model_step_id: Id,
    },
    /// One original planned call, before system binding.
    BeforeTool {
        /// Core logical call identity.
        call_id: Id,
    },
    /// Already committed tool observation.
    AfterTool {
        /// Core logical call identity.
        call_id: Id,
        /// Exact protected ToolResult.
        result_ref: RecordRef,
    },
    /// Already committed terminal outcome.
    AfterRun {
        /// Exact protected RunOutcome.
        outcome_ref: RecordRef,
        /// Terminal snapshot revision.
        revision: u64,
    },
}
impl HookTarget {
    /// Lifecycle position fixed by this target.
    pub fn position(&self) -> HookPosition {
        match self {
            Self::BeforeRun => HookPosition::BeforeRun,
            Self::BeforeModel { .. } => HookPosition::BeforeModel,
            Self::BeforeTool { .. } => HookPosition::BeforeTool,
            Self::AfterTool { .. } => HookPosition::AfterTool,
            Self::AfterRun { .. } => HookPosition::AfterRun,
        }
    }
}

/// Selected safe data. Raw tool receipts, opaque continuations and hidden inputs
/// are deliberately absent. Data payloads accept only text and JSON content.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum HookInput {
    /// Original user request and prior additions in this transform chain.
    BeforeRun {
        /// Original user content.
        user_input: Vec<InputContent>,
        /// Accumulated safe context.
        context_items: Vec<ContextItem>,
    },
    /// Route-independent data for a logical model step.
    BeforeModel {
        /// Original user content.
        user_input: Vec<InputContent>,
        /// Already supplied context and chain additions.
        context_items: Vec<ContextItem>,
    },
    /// Model-visible contract and original/effective argument maps.
    BeforeTool {
        /// Model-facing descriptor only.
        tool: ModelTool,
        /// Pinned full-descriptor identity, without its hidden schema.
        descriptor_digest: JsonDigest,
        /// Pinned compiler contract identity.
        compiled_digest: JsonDigest,
        /// Unchanged original model proposal.
        original_model_inputs: JsonObject,
        /// Current transform-chain value.
        model_inputs: JsonObject,
    },
    /// Safe committed observation, excluding receipt and diagnostic references.
    AfterTool {
        /// Original logical call identity.
        call_id: Id,
        /// Committed completion status.
        status: ToolResultStatus,
        /// Committed effect status.
        effect: ToolEffect,
        /// Safe text/JSON output only.
        content: Vec<InputContent>,
        /// Safe classified failure, if any.
        error_code: Option<Id>,
    },
    /// Safe terminal summary, excluding artifact and protected-record references.
    AfterRun {
        /// Terminal status.
        status: RunStatus,
        /// Safe text/JSON response only.
        output: Vec<InputContent>,
        /// Charged execution usage.
        usage: BudgetUsage,
    },
}
impl fmt::Debug for HookInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("HookInput(<protected>)")
    }
}
impl HookInput {
    /// Derive a reference-free summary from a committed tool result.
    pub fn tool_observed(call_id: &Id, result: &ToolResult) -> Self {
        Self::AfterTool {
            call_id: call_id.clone(),
            status: result.status,
            effect: result.effect,
            content: safe_summary(&result.content),
            error_code: result.error.as_ref().map(|error| error.code.clone()),
        }
    }
    /// Derive a reference-free summary from an authoritative terminal outcome.
    pub fn run_observed(outcome: &RunOutcome) -> Self {
        Self::AfterRun {
            status: outcome.result.status(),
            output: safe_summary(&outcome.output),
            usage: outcome.usage.clone(),
        }
    }
    /// Digest of the exact safe callback input.
    pub fn digest(&self) -> JsonDigest {
        crate::serialization::data_digest(self)
    }
    fn validate(&self, target: &HookTarget) -> Result<(), ContractError> {
        let valid = match (self, target) {
            (
                Self::BeforeRun {
                    user_input,
                    context_items,
                },
                HookTarget::BeforeRun,
            )
            | (
                Self::BeforeModel {
                    user_input,
                    context_items,
                },
                HookTarget::BeforeModel { .. },
            ) => {
                safe_content(user_input)
                    && context_items.iter().all(|item| safe_content(&item.content))
            }
            (
                Self::BeforeTool {
                    tool,
                    original_model_inputs,
                    model_inputs,
                    ..
                },
                HookTarget::BeforeTool { .. },
            ) => {
                let validator = crate::tool_schema::compile_validator(&tool.model_input_schema)?;
                validator.is_valid(
                    &serde_json::to_value(original_model_inputs)
                        .map_err(|_| hook_error(ErrorCode::InvalidJson, "hooks.input"))?,
                ) && validator.is_valid(
                    &serde_json::to_value(model_inputs)
                        .map_err(|_| hook_error(ErrorCode::InvalidJson, "hooks.input"))?,
                )
            }
            (
                Self::AfterTool {
                    call_id, content, ..
                },
                HookTarget::AfterTool {
                    call_id: target, ..
                },
            ) => call_id == target && safe_content(content),
            (Self::AfterRun { status, output, .. }, HookTarget::AfterRun { .. }) => {
                status.is_terminal() && safe_content(output)
            }
            _ => false,
        };
        if !valid {
            return Err(hook_error(ErrorCode::InvalidContract, "hooks.input"));
        }
        Ok(())
    }
}
fn safe_content(content: &[InputContent]) -> bool {
    content.iter().all(|content| {
        matches!(
            content,
            InputContent::Text { .. } | InputContent::Json { .. }
        )
    })
}
fn safe_summary(content: &[InputContent]) -> Vec<InputContent> {
    content
        .iter()
        .filter(|item| matches!(item, InputContent::Text { .. } | InputContent::Json { .. }))
        .cloned()
        .collect()
}

/// Additional data; the core assigns provenance, scope, ID and lifetime.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HookContextAddition {
    /// Text/JSON data, never raw message or opaque blocks.
    pub content: Vec<InputContent>,
    /// Required data must fit in full; importance grants no authority.
    pub priority: ContextPriority,
}
/// Position-specific output, never an internal-state patch or a next callback.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum HookOutput {
    /// Append data during before_run or before_model.
    Context {
        /// Complete additions to validate and persist.
        additions: Vec<HookContextAddition>,
    },
    /// Replace only model-owned arguments, optionally denying this call.
    Tool {
        /// Must still satisfy the model-visible input schema.
        model_inputs: JsonObject,
        /// Safe denial code; absence grants no permission.
        deny: Option<Id>,
    },
    /// Observer completed; cannot change a tool or Run result.
    Observed {},
}
impl fmt::Debug for HookOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("HookOutput(<protected>)")
    }
}

/// Current authorization and finite callback controls, without system inputs.
#[derive(Debug, Clone)]
pub struct HookContext {
    /// Exact namespace.
    pub scope: Scope,
    /// Owning Run identity.
    pub run_id: Id,
    /// Exact selected callback identity.
    pub hook: VersionedRef,
    /// Logical lifecycle target.
    pub target: HookTarget,
    /// Current authenticated actor.
    pub principal_ref: Id,
    /// Current Host grant.
    pub capability_grant_ref: Id,
    /// Cancelled on timeout, cancellation or callback completion.
    pub cancellation: CancellationToken,
    /// Finite callback deadline.
    pub deadline: tokio::time::Instant,
}
/// Trusted Host callback. It must not hide required business writes or dispatch
/// another execution; in-process Rust code is not an isolation sandbox.
pub trait HookHandler: Send + Sync {
    /// Apply one bounded transformation or read-only observation.
    fn call<'a>(
        &'a self,
        input: &'a HookInput,
        context: &'a HookContext,
    ) -> PortFuture<'a, HookOutput>;
}
/// Exact contract associated with an existing callback.
#[derive(Clone)]
pub struct HookRegistration {
    /// Immutable selected contract.
    pub definition: HookDefinition,
    /// Existing Host-owned implementation.
    pub handler: Arc<dyn HookHandler>,
}
/// Scope-bound immutable callback catalog.
pub struct HookRegistry {
    scope: Scope,
    entries: Vec<HookRegistration>,
}
impl HookRegistry {
    /// Register existing callbacks; this performs no callback or external I/O.
    pub fn new(scope: Scope, mut entries: Vec<HookRegistration>) -> Result<Self, ContractError> {
        if entries.len() > 64 {
            return Err(hook_error(ErrorCode::InvalidContract, "hooks.count"));
        }
        let mut seen = std::collections::BTreeSet::new();
        for entry in &entries {
            entry.definition.validate()?;
            if !seen.insert(entry.definition.hook.id.clone()) {
                return Err(hook_error(ErrorCode::InvalidReference, "hooks.duplicate"));
            }
        }
        entries.sort_by(|a, b| {
            a.definition
                .priority
                .cmp(&b.definition.priority)
                .then(a.definition.hook.id.cmp(&b.definition.hook.id))
        });
        Ok(Self { scope, entries })
    }
    /// Exact registered namespace.
    pub fn scope(&self) -> &Scope {
        &self.scope
    }
    /// Pin only exact profile-selected catalog definitions in execution order.
    pub fn plan(&self, profile: &AgentProfile) -> Result<HookPlan, ContractError> {
        let mut definitions = Vec::new();
        for selected in profile.hooks.iter().flatten() {
            let HookRef::Catalog(selected) = selected else {
                return Err(hook_error(ErrorCode::CapabilityUnsupported, "hooks.export"));
            };
            let entry = self
                .entries
                .iter()
                .find(|entry| {
                    entry.definition.hook.id == selected.hook_id
                        && entry.definition.hook.version == selected.version
                        && entry.definition.position == selected.position
                })
                .ok_or_else(|| hook_error(ErrorCode::ComponentUnavailable, "hooks.selection"))?;
            definitions.push(entry.definition.clone());
        }
        definitions.sort_by(|a, b| a.priority.cmp(&b.priority).then(a.hook.id.cmp(&b.hook.id)));
        let plan = HookPlan {
            scope: self.scope.clone(),
            definitions,
        };
        plan.validate(profile)?;
        Ok(plan)
    }
}

/// Immutable selected definitions, serialized for protected Run storage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HookPlan {
    scope: Scope,
    definitions: Vec<HookDefinition>,
}
impl HookPlan {
    /// Exact owning namespace.
    pub fn scope(&self) -> &Scope {
        &self.scope
    }
    /// Selected definitions in their stable execution order.
    pub fn definitions(&self) -> &[HookDefinition] {
        &self.definitions
    }
    /// Canonical identity of scope, definitions, order and bounds.
    pub fn digest(&self) -> JsonDigest {
        crate::serialization::data_digest(self)
    }
    /// Restore only the exact trusted protected plan identity.
    pub fn restore(
        json: &str,
        scope: &Scope,
        expected_digest: &JsonDigest,
    ) -> Result<Self, ContractError> {
        let plan: Self = serde_json::from_str(json)
            .map_err(|_| hook_error(ErrorCode::InvalidSnapshot, "hooks.plan"))?;
        if plan.scope() != scope || &plan.digest() != expected_digest {
            return Err(hook_error(
                ErrorCode::InvalidSnapshot,
                "hooks.plan_identity",
            ));
        }
        plan.validate_order()?;
        Ok(plan)
    }
    /// Confirm the plan is exactly the profile's selected catalog hooks.
    pub fn validate(&self, profile: &AgentProfile) -> Result<(), ContractError> {
        self.validate_order()?;
        let selections: Vec<_> = profile.hooks.iter().flatten().collect();
        let mut selected_ids = std::collections::BTreeSet::new();
        if selections.len() != self.definitions.len()
            || selections.iter().any(|selection| match selection {
                HookRef::Catalog(selected) => {
                    !selected_ids.insert(&selected.hook_id)
                        || !self.definitions.iter().any(|definition| {
                            definition.hook.id == selected.hook_id
                                && definition.hook.version == selected.version
                                && definition.position == selected.position
                        })
                }
                HookRef::Export(_) => true,
            })
        {
            return Err(hook_error(
                ErrorCode::InvalidSnapshot,
                "hooks.plan_selection",
            ));
        }
        Ok(())
    }
    fn validate_order(&self) -> Result<(), ContractError> {
        if self.definitions.len() > 64 {
            return Err(hook_error(ErrorCode::InvalidSnapshot, "hooks.plan_count"));
        }
        let mut seen = std::collections::BTreeSet::new();
        for (index, definition) in self.definitions.iter().enumerate() {
            definition.validate()?;
            if !seen.insert(definition.hook.id.clone())
                || (index > 0 && {
                    let prior = &self.definitions[index - 1];
                    (prior.priority, &prior.hook.id) > (definition.priority, &definition.hook.id)
                })
            {
                return Err(hook_error(ErrorCode::InvalidSnapshot, "hooks.plan_order"));
            }
        }
        Ok(())
    }
}

/// One durably applied transform. The result record retains input and output so
/// restart never substitutes freshly transformed arguments for the saved ones.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HookApplication {
    /// Exact selected hook.
    pub hook: VersionedRef,
    /// Complete pinned definition identity.
    pub definition_digest: JsonDigest,
    /// Logical lifecycle target.
    pub target: HookTarget,
    /// Digest of the safe input seen by this callback.
    pub input_digest: JsonDigest,
    /// Exact stored transformation or classified optional failure.
    pub result_ref: RecordRef,
}
/// Protected body referenced by HookApplication.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HookApplicationRecord {
    /// Exact namespace.
    pub scope: Scope,
    /// Owning Run.
    pub run_id: Id,
    /// Exact selected hook.
    pub hook: VersionedRef,
    /// Definition identity.
    pub definition_digest: JsonDigest,
    /// Logical target.
    pub target: HookTarget,
    /// Input before this callback's transformation.
    pub input: HookInput,
    /// Validated output; absent only for a permitted optional callback failure.
    pub output: Option<HookOutput>,
    /// Core-stamped data created from Context additions.
    pub context_items: Vec<ContextItem>,
    /// Safe classified optional failure.
    pub failure: Option<Id>,
}
impl fmt::Debug for HookApplicationRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("HookApplicationRecord(<protected>)")
    }
}
/// Current chain result, including its stored application prefix.
#[derive(Clone)]
pub struct HookTransform {
    /// Accumulated supplied context and newly added data.
    pub context_items: Vec<ContextItem>,
    /// Final model-owned arguments, only for before_tool.
    pub model_inputs: Option<JsonObject>,
    /// A persisted denial, never overridden by another Hook.
    pub deny: Option<Id>,
    /// Applied definitions in deterministic chain order.
    pub applications: Vec<HookApplication>,
}
impl fmt::Debug for HookTransform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HookTransform")
            .field("applications", &self.applications.len())
            .field("denied", &self.deny.is_some())
            .finish_non_exhaustive()
    }
}
/// Observer result that cannot alter an already committed outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum HookObservationStatus {
    /// Read-only callback completed.
    Completed,
    /// Callback or output contract failed safely.
    Failed {
        /// Safe error code, never raw callback text.
        code: Id,
    },
}
/// Immutable observation report stored outside the Run snapshot/event endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HookObservation {
    /// Exact namespace.
    pub scope: Scope,
    /// Owning Run.
    pub run_id: Id,
    /// Exact selected observer.
    pub hook: VersionedRef,
    /// Complete contract identity.
    pub definition_digest: JsonDigest,
    /// Exact committed ToolResult or terminal outcome.
    pub target: HookTarget,
    /// Digest of the reference-free observer input.
    pub input_digest: JsonDigest,
    /// Classified callback result.
    pub status: HookObservationStatus,
    /// Report time; it does not advance Run time or revision.
    pub timestamp_ms: i64,
}

/// Executes selected callbacks, with authority and persistence owned by the core.
pub struct HookRuntime {
    store: Arc<dyn StateStore>,
    policy: Arc<PolicyGate>,
    clock: Arc<dyn Clock>,
    ids: Arc<dyn IdSource>,
    registry: Arc<HookRegistry>,
}
fn hook_error(code: ErrorCode, path: &str) -> ContractError {
    ContractError::new(code, path)
}
