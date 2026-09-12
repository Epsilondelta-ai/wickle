use crate::{Id, JsonDigest, JsonObject, Scope, serialization::optional};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;

/// An authorized reference to immutable stored data, not the referenced payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordRef {
    /// Store-specific record identifier.
    pub record_id: Id,
    /// Exact stored revision.
    pub revision: u64,
    /// Digest of the referenced contract data.
    pub digest: JsonDigest,
}

/// Artifact metadata. Reading bytes still requires current scope authorization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactRef {
    /// Artifact identifier.
    pub artifact_id: Id,
    /// Scope that owns the artifact.
    pub scope: Scope,
    /// Media type of the stored bytes.
    pub media_type: Id,
    /// Original byte length.
    pub size_bytes: u64,
    /// Store-defined content hash, separate from JSON contract digests.
    pub content_hash: Id,
}

/// Provenance for a source passage or fact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceRef {
    /// Source identifier.
    pub source_id: Id,
    /// Exact source version/revision.
    pub version: Id,
    /// Source-specific passage location.
    pub location: Id,
    /// Original source content hash.
    pub content_hash: Id,
    /// Optional quoted passage.
    #[serde(
        default,
        deserialize_with = "optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote: Option<String>,
}

/// User-supplied or final-output content; cannot inject tool calls or provider state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum InputContent {
    /// Text content.
    Text {
        /// Text body.
        text: String,
    },
    /// JSON data, not executable objects.
    Json {
        /// JSON value; explicit JSON null is valid content.
        value: serde_json::Value,
    },
    /// Artifact metadata.
    Artifact {
        /// Artifact reference.
        reference: ArtifactRef,
    },
    /// Evidence metadata.
    Evidence {
        /// Source reference.
        reference: EvidenceRef,
    },
}

/// Model-owned tool arguments and provenance, separate from system execution inputs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolCall {
    /// Core call identifier.
    pub call_id: Id,
    /// Original model request identifier.
    pub model_request_id: Id,
    /// Provider-local call identifier, scoped by model_request_id.
    pub provider_call_id: Id,
    /// Model-facing tool name.
    pub tool_name: Id,
    /// Original model-supplied inputs, never replaced with execution_args.
    pub model_inputs: JsonObject,
    /// Pinned descriptor identity. None means the name was unregistered when planned.
    #[serde(
        default,
        deserialize_with = "optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub descriptor_digest: Option<JsonDigest>,
    /// Protected bound-input record, once binding succeeds.
    #[serde(
        default,
        deserialize_with = "optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub bound_input_ref: Option<RecordRef>,
}

/// Outcome of one tool dispatch or a pre-dispatch denial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolResultStatus {
    /// Tool completed successfully.
    Succeeded,
    /// Tool failed with a classified error.
    Failed,
    /// Policy or validation denied execution.
    Denied,
    /// Execution was cancelled.
    Cancelled,
    /// External effect status is not known.
    Unknown,
}

/// A safe structured failure; raw SDK errors belong in protected Host diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Failure {
    /// Registered error/reason code.
    pub code: Id,
    /// Optional protected diagnostic reference.
    #[serde(
        default,
        deserialize_with = "optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub diagnostic_ref: Option<RecordRef>,
}

/// Tool observation explicitly paired with its call message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolResult {
    /// Core call identifier.
    pub call_id: Id,
    /// Message containing the matching call.
    pub call_message_id: Id,
    /// Explicit execution status.
    pub status: ToolResultStatus,
    /// Confirmed external effect, separate from validation of the returned value.
    /// Missing legacy metadata is unknown, never evidence that a write did not occur.
    #[serde(default)]
    pub effect: crate::ToolEffect,
    /// Bounded model-visible observations or references.
    pub content: Vec<InputContent>,
    /// Protected receipt, retained even if output processing fails.
    #[serde(
        default,
        deserialize_with = "optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub effect_receipt_ref: Option<RecordRef>,
    /// Classified failure, when applicable.
    #[serde(
        default,
        deserialize_with = "optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub error: Option<Failure>,
}

/// Transcript content. Provider replay data is a scoped, protected reference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ContentBlock {
    /// Ordinary displayable content.
    Content {
        /// Text, JSON, artifact, or evidence.
        content: InputContent,
    },
    /// A complete model tool call.
    ToolCall {
        /// Call and model-only arguments.
        call: ToolCall,
    },
    /// Paired tool observation.
    ToolResult {
        /// Observation and receipt reference.
        result: ToolResult,
    },
    /// Opaque continuation data bound to one provider/route.
    ProviderOpaque {
        /// Registered provider key.
        provider: Id,
        /// Route that can interpret the protected block.
        route_digest: JsonDigest,
        /// Protected replay-data record.
        data_ref: RecordRef,
    },
}

/// Logical message role before provider-specific projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    /// Host/profile instruction role.
    System,
    /// User input.
    User,
    /// Model output.
    Assistant,
    /// Tool observation.
    Tool,
}

/// Provenance of content; a wire role does not grant authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageOrigin {
    /// Trusted Host instructions.
    Host,
    /// Pinned profile instructions.
    Profile,
    /// User input.
    User,
    /// Model response.
    Model,
    /// Loaded skill data.
    Skill,
    /// Tool observation.
    Tool,
    /// Retrieved external data.
    Retrieval,
    /// Retrieved memory.
    Memory,
    /// Verifier feedback.
    Verification,
    /// Synthetic recovery bookkeeping.
    Recovery,
}

/// Intended projection surfaces; authorization is still enforced at use time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    /// Protected execution data only.
    Internal,
    /// Model projection only.
    Model,
    /// User presentation only.
    User,
    /// Both model projection and user presentation.
    UserAndModel,
}

/// An original transcript record, not a provider request or UI event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Message {
    /// Unique message identity.
    pub message_id: Id,
    /// Owning run.
    pub run_id: Id,
    /// Monotonic stored message sequence.
    pub sequence: NonZeroU64,
    /// Logical role.
    pub role: MessageRole,
    /// Original content blocks.
    pub content: Vec<ContentBlock>,
    /// Source provenance.
    pub origin: MessageOrigin,
    /// Intended projections.
    pub visibility: Visibility,
}
