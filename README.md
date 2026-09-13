# Wickle

**English** | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [Русский](README.ru.md)

**An extensible agent engine by EpsilonDelta.**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="Wickle's hedgehog mascot running on an exercise wheel" width="420" />
</p>

Wickle is an agent engine being developed in Rust by EpsilonDelta. It aims to provide an embeddable library that runs the loop of model decisions and tool calls.

Its design uses Agent Profiles to configure agent behavior and adapters to connect different models and tools.

**Status:** In development. Agents support serial model/tool loops, separate system inputs, persisted outcomes, event replay, cancellation, and resuming saved approval, input, or external-effect waits. Resume commands are deduplicated; external effects require Host verification. Lifecycle hooks support bounded context and argument transformations plus observations after commit. Adapter Tool/Hook/ContextSource exports support pinned assemblies and scoped instances that close when each execution segment ends. [Adapter runtime](docs/adapters.md). Interrupted runs support explicit recovery with fenced ownership, preserved inputs, and external-effect reconciliation. [Recovery](docs/recovery.md). [Run an agent](docs/agents.md) · [Data contracts](docs/contracts.md).

Read-only [context sources](docs/context-sources.md) support Run and model-step collection, saved batches, and current access checks on reuse.

[Skills](docs/skills.md) load complete, versioned instructions through a registered Tool. [Artifacts](docs/artifacts.md) preserve scoped originals, bounded previews, and source evidence.

[Context rewriting](docs/context-compaction.md) preserves the original conversation while applying bounded previews and validated summaries. Model-based compression shares the Run budget.

[Output verification](docs/verification.md) supports JSON schemas, versioned criteria, bounded repair, and approval of a fixed candidate. Model-based reviews share the Run budget.

The [model provider guide](docs/model-providers.md) covers OpenAI, Azure OpenAI, Anthropic, AWS Bedrock, Gemini API, Vertex AI and xAI adapters. They use the same core contracts while retaining explicit credentials, API versions and provider-specific behavior.

See [MCP tools](docs/mcp.md) for connecting reviewed tools over stdio.
