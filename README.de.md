# Wickle

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md) | **Deutsch** | [Русский](README.ru.md)

**Eine erweiterbare Agenten-Engine von EpsilonDelta.**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="Das Wickle-Maskottchen, ein Igel, der in einem Laufrad läuft" width="420" />
</p>

Wickle ist eine Agenten-Engine, die EpsilonDelta in Rust entwickelt. Ziel ist eine in Anwendungen einbettbare Bibliothek, die den Zyklus aus Modellentscheidungen und Werkzeugaufrufen ausführt.

Der Entwurf verwendet Agentenprofile (Agent Profiles) zur Konfiguration des Agentenverhaltens und Adapter zur Anbindung verschiedener Modelle und Werkzeuge.

**Status:** In Entwicklung. Agenten unterstützen sequenzielle Schleifen aus Modell- und Werkzeugaufrufen, gespeicherte Ergebnisse, Ereigniswiedergabe und ausdrücklichen Abbruch. Registrierte `ToolExecutor`-Implementierungen erhalten validierte Modelleingaben, die mit separat vom Host verwalteten `SystemInputs` zusammengeführt werden. Bei erforderlicher Genehmigung oder unklarer Wirkung eines Schreibvorgangs stoppt die Ausführung in `Waiting`; `resume` wird noch nicht unterstützt. [Agenten ausführen](docs/agents.md) · [Datenverträge](docs/contracts.md).
