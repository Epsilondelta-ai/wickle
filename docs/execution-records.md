# Execution record contracts

The execution-record types separate submitted input, resolved model preparation,
execution ownership, and application business state. They are contracts for the
next storage/driver integration; their presence does not enable interruption or
new resume behavior in the existing driver.

`RequestSnapshot::capture` stores JSON text without rounding its number tokens.
The profile reference and protected system inputs participate in its digest.
Explicit empty model options and omitted model options compare equally, as do
absent and empty system inputs on Start. Original field presence and an explicit
system-input presence flag remain available in the snapshot. Null model options
or null system inputs are not empty objects. All other submitted fields retain
their explicit presence. The request-snapshot schema version fixes these
normalization rules; the canonicalization version selects the JSON encoding.

Call `validate` on decoded records before comparison. Request schema validation
and current authorization are still required at admission; capture is not a Run
start operation. `Debug` omits the request and system input payloads. Only trusted
storage/Host code should read their accessors.

`ExecutionSegment` identifies an accepted interval and its saved outcome.
`SegmentOutcome::Interrupted` requires matching, recoverable interruption evidence;
user cancellation and ownership loss cannot be represented as a recoverable pause.
`AppState` carries a Host namespace, business status and metadata; it cannot replace
core status. Validate metadata with the pinned Host schema before persistence.
`InterruptionPolicy` receives immutable evidence and returns a proposal. It has no
storage, Tool or model handles, and the driver must enforce protected causes,
timeouts and default behavior before committing a decision.

`PreparedStepRecord` references the exact model configuration, compiled Tool
contracts and context projection for a model purpose. Runtime clients and
credentials do not belong in these records. Protected record references must be
resolved in the same authorized scope.

`ExecutionTransactions` specifies atomic segment acceptance and durable control
command submission. It intentionally has no default implementation that simulates
a transaction with separate reads and writes. Storage integration must implement
these operations in the same transaction boundary as StateStore checkpoints and
events before the new segment driver can use them.

The legacy Run checkpoint validator rejects `Interrupted`, because that format
cannot carry the required execution record. `RunRequest.max_output_tokens` is a
positive optional contract field. Until purpose-specific routing integration is
available, the driver explicitly rejects a supplied override rather than silently
ignoring it. Omitted values retain existing request encoding.
