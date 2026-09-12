# Run a text agent

`create_agent` builds a scope-bound facade from an `AgentProfile` and existing
Host components. The current runtime executes text requests through admission,
context preparation, model routing, and a persisted turn-ended outcome. It also
provides event replay and explicit cancellation.

```rust,ignore
let agent = create_agent(profile, bindings)?;
let handle = match agent.start(request, context.clone()).await? {
    Guarded::Completed(handle) => handle,
    Guarded::ApprovalRequired(challenge) => return handle_approval(challenge),
};
let outcome = handle.outcome(&context).await?;
```

The [complete standalone consumer](../tests/support/agent_consumer.rs) configures
the Host components, executes a request with synthetic models, reconnects to its
events, and reopens its SQLite state. Run the extracted-package examples with
`python3 scripts/check-package.py --allow-dirty`.

## Supply Host components

`AgentBindings` contains the exact scope, `StateStore`, current `PolicyGate`,
`ProfileResolver`, pinned `ModelRouter`, configured `ModelExchange`, trusted Host
instructions, `SystemInputRegistry`, clock, ID source, token estimator, and
`AgentSettings`. A single Agent instance owns one scope; use separately configured
instances for other scopes.

Constructing the Agent validates profile structure and finite settings. It does
not invoke component callbacks, open connections, create a runtime, spawn tasks,
or read environment files. The Host supplies Tokio before calling `start`.
Metadata resolution and model work begin only during execution.

`ModelTokenEstimator` estimates the final request for its selected route. It is a
synchronous Host callback with no I/O. Its estimate is used for capacity checks
and remains separate from provider-reported usage. `AgentSettings` bounds output,
request/response/context sizes, admission preparation, lease renewal, and observer
polling. Run-wide model, tool, recovery, and elapsed limits come from the profile.

The initial driver supports text instructions, text output, the bounded context
strategy without additional strategy version/configuration, and `turn_end` completion.
Profiles requiring Tool execution, Skills,
Hooks, automatic context sources, asset loading, or verified completion are
rejected explicitly until those execution components are connected. Their
configuration is never silently removed to make a request run.

## Start, replay, and observe

`start` checks current admission permission and looks up the request under its
scope, session, and request ID before resolving current metadata. An identical
request returns the original Run. Changed input, model options, or effective
system values conflicts with the stored request. For `start`, omitted system
inputs mean an empty map. Atomic admission decides concurrent requests; only the
newly created Run starts a driver.

The driver owns its lease, heartbeat, cancellation signal, and state transitions.
Dropping the start Future after it has been polled, a RunHandle, an outcome Future,
or an event Stream does not cancel accepted execution. Observer context
cancellation stops observation separately from the run's execution token.

`get_run` returns an authorized `RunView`. `get_run_details` requires the separate
details permission and returns the protected checkpoint. `RunHandle.outcome`
returns a `Guarded<RunOutcome>` read from the store; a text delta or finish
notification is not completion authority. If completion cannot be saved, the
observer receives an execution/storage error rather than a fabricated success.

`RunHandle.events(after_seq, context)` returns durable `EventView` metadata after
the cursor. It rechecks current permission while yielding and does not publish
protected record references. A slow or disconnected subscriber cannot restart or
block the driver. Reconnecting replays the remaining committed sequence.

## Cancel and finish

`handle.cancel(reason, &context)` checks `CancelRun` permission and returns a
`Guarded<CancelReceipt>`:

| Receipt | Meaning |
| --- | --- |
| `Requested` | The local driver received the cancellation signal; observe its saved outcome for completion |
| `AlreadyTerminal` | The stored result is already terminal and stays unchanged |
| `NotLocal` | This instance owns no live driver for the Run; no remote cancellation was accepted |

Success records `completion_basis=turn_ended`. This says the model completed its
turn under the configured output contract; it does not claim external business
verification. Failure, cancellation, and budget exhaustion have distinct outcomes.
Already stored response text can be retained as partial output without becoming
a successful assistant transcript. Unfinished stream deltas are not guaranteed
to survive an interrupted collector.

Opaque continuation returned by a successful model is stored in protected records
with the assistant transcript. A later Run can reuse it only under the exact
matching route. Changing providers or route identity fails before transmitting
foreign continuation. The original session prompt remains pinned.

`resume` has a public command shape, but waiting-command consumption and recovery
of interrupted external effects are not implemented by this initial driver.
Replaying `start` is not a substitute for that recovery operation. Durable state
can still be inspected after a process stops.
