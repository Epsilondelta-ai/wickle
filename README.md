# Wickle

**English** | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [Русский](README.ru.md)

**An extensible agent engine by EpsilonDelta.**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="Wickle's hedgehog mascot running on an exercise wheel" width="420" />
</p>

Wickle is an agent engine being developed in Rust by EpsilonDelta. It aims to provide an embeddable library that runs the loop of model decisions and tool calls.

Its design uses Agent Profiles to configure agent behavior and adapters to connect different models and tools.

**Status:** In development. Agents support serial model/tool loops, separate system inputs, persisted outcomes, event replay, cancellation, and resuming saved approval, input, or external-effect waits. Resume commands are deduplicated; external effects require Host verification. General interruption recovery and candidate verification remain unsupported. [Run an agent](docs/agents.md) · [Data contracts](docs/contracts.md).
