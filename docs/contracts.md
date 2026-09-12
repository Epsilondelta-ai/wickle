# Data contracts and profile validation

Wickle currently provides profile validation and the data contracts used by an
agent runtime. The model/tool driver and runtime component assembly are not yet
implemented.

`AgentProfile` contains data and registered references. A `ProfileResolver` is
Host code that supplies approved metadata for a scope. `ProfileValidator` checks
that metadata and configuration without opening an adapter or invoking a model.

## Decode a profile

```rust
use wickle::AgentProfile;

fn main() -> Result<(), wickle::ContractError> {
let profile = AgentProfile::from_json(r#"{
  "schema_version": "wickle.agent-profile.v1",
  "agent_id": "information-assistant",
  "version": "1.0.0",
  "name": "Information assistant",
  "description": "Organize information",
  "instructions": {"text": "Use available evidence."},
  "model_binding": "primary",
  "tools": [], "skills": [], "connectors": [],
  "context_policy": {"strategy": "bounded"},
  "output_contract": {"type": "text"},
  "limits": {
    "max_model_calls": 8, "max_tool_attempts": 0,
    "max_repair_attempts": 0, "max_recovery_attempts": 0,
    "max_elapsed_ms": 30000
  }
}"#)?;
println!("Validated {}", profile.agent_id);
Ok(())
}
```

`from_json` rejects duplicate keys, unknown fields and format versions, invalid
types, and invalid local binding references. Model-call and elapsed-time limits
must be positive integers. Zero tool, repair, or recovery attempts disables that
action. Completion defaults to `turn_end`; `verified` requires `verifier_ref`.

Optional adapter, source, hook, and extension collections preserve omission versus
an explicit empty collection. Explicit `null` is rejected. Context sources require
finite limits in the profile; a Host using presets must expand them before parsing.
The built-in context strategy is `bounded`. Custom strategies require an exact
versioned reference.

## Resolve approved metadata

Implement the async, dynamically dispatchable `ProfileResolver` trait in Host
code, then call `ProfileValidator::new(&resolver).validate(&profile, &scope).await`.
The resolver must return the requested kind, identifier, and exact version. Model
binding and extension-schema lookups resolve a Host-selected revision, which is
then retained in `ResolvedProfile`.

The validator checks:

- Selected components and their explicitly selected dependencies are available.
- Metadata/export contract versions are supported; their current version is `1`.
- Required connector mappings exist, export kinds match, and selected tool names
  do not collide. The Host supplies normalized descriptor names; provider-specific
  wire name validation belongs to the corresponding adapter.
- Required capabilities are supplied by selected components and exports. Merely
  declaring an unused export does not activate it or supply its capabilities.
- Configuration satisfies its registered Draft 2020-12 schema, including formats.
  Schema resolution is local: external `$ref`, dynamic/recursive references, and
  other declared dialects are rejected. Schema annotation data is not executed.
- Extension keys use dotted namespaces and have registered schemas.

Raw credentials, SDK clients, and executable module fields are not profile
properties. `config` and extension values are nonsecret data constrained by
Host-approved schemas. Validation is not a secret scanner for arbitrary text;
the Host must keep credentials in its connection bindings.

`ResolvedProfile` exposes immutable accessors and records the complete profile,
scope, exact component versions, and definition digests. Its deserializer checks
the saved digest consistency. Use `ensure_matches` to reject a different profile
or scope and `ensure_same_resolution` to reject changed component metadata.
These checks do not replace authentication or the authoritative store.

## Execution and storage data

| Contract | Purpose |
| --- | --- |
| `RunRequest`, `ResumeCommand` | Caller data with request/command identity and typed input or decisions |
| `ExecutionContextData`, `ExecutionContext` | Owned Host data and nonserializable runtime context with cancellation |
| `Message`, `ToolCall`, `ToolResult` | Transcript provenance and paired calls/results; tool model inputs remain separate from bound input references |
| `RouteRequest`, `ResolvedModelRoute`, `ModelInvocationRecord` | Model selection data, independent model/API/deployment/adapter versions, and physical-attempt provenance |
| `WaitState`, `RunOutcome` | Fixed approval/input/effect targets and explicit completion basis |
| `SessionSnapshot`, `RunSnapshot` | Pinned session identity and checkpoint data, without executing recovery |
| `RunEvent`, `EphemeralEvent` | Durable committed record references versus candidate deltas without durable sequence numbers |

Use `RunSnapshot::from_json`, or call `validate` after constructing a checkpoint,
to check request identity, limits, status/phase/outcome consistency, tool-ledger
pairing, and attempt uniqueness. A verified success requires a passing verifier
record. Success cannot retain unsettled tools or unresolved effects. State
transitions, current authorization, call dispatch, and duplicate resume-command
enforcement belong to the runtime/store implementation.

`ExecutionContextData.system_inputs` preserves three distinct inputs: omission,
an empty map, and a nonempty map. `null` is invalid. On start, omission means the
empty map; on resume, omission means reuse the saved inputs. `SystemInputs` owns
its values and redacts them from Debug output. Its serialization is for protected
storage. It is not automatically copied into messages or events.

## Digests and versions

`canonical_digest_json` accepts strict JSON; `canonical_digest` accepts an already
constructed `serde_json::Value`. The `sorted-json-v1:sha256:` encoding recursively
sorts object keys and preserves arrays and values. It is not RFC 8785. With the
supported JSON number representation, `1` differs from `1.0`, and `0` from `-0.0`.

Profile, run snapshot, session snapshot, and durable event formats have separate
`wickle.*.v1` identifiers. Model release identifiers are opaque strings, not parsed
as SemVer or dates. Requested and provider-reported model versions remain separate;
missing provider usage/version reports are not filled with inferred values.

## Run the consumer example

```sh
python3 scripts/check-package.py --allow-dirty
```

This builds the `.crate` outside its source workspace and runs
[`tests/support/consumer.rs`](../tests/support/consumer.rs) in an independent Rust
application. The Host supplies Tokio, a scoped metadata resolver, and a JSON
profile. The program validates the profile, persists and restores its resolved
identity, and reports the result. The consumer also accepts a profile JSON file
and an optional second file to compare against the pinned profile.
