# Wickle

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md) | **Deutsch** | [Русский](README.ru.md)

**Eine erweiterbare Agenten-Engine von EpsilonDelta.**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="Das Wickle-Maskottchen, ein Igel, der in einem Laufrad läuft" width="420" />
</p>

Wickle ist eine Agenten-Engine, die EpsilonDelta in Rust entwickelt. Ziel ist eine in Anwendungen einbettbare Bibliothek, die den Zyklus aus Modellentscheidungen und Werkzeugaufrufen ausführt.

Der Entwurf verwendet Agentenprofile (Agent Profiles) zur Konfiguration des Agentenverhaltens und Adapter zur Anbindung verschiedener Modelle und Werkzeuge.

**Status:** In Entwicklung. Agenten unterstützen sequenzielle Modell- und Werkzeugaufrufe, getrennte Systemeingaben, gespeicherte Ergebnisse, Ereigniswiedergabe, Abbruch und die Fortsetzung gespeicherter Wartezustände für Genehmigungen, Eingaben oder externe Wirkungen. Fortsetzungsbefehle werden dedupliziert; externe Wirkungen müssen vom Host geprüft werden. Lifecycle-Hooks unterstützen begrenzte Kontext- und Argumenttransformationen sowie die Beobachtung gespeicherter Ergebnisse. Tool-/Hook-/ContextSource-Exporte von Adaptern verwenden festgelegte Konfigurationen und segmentgebundene Instanzen, deren Ressourcen am Segmentende freigegeben werden. [Adapter-Laufzeit](docs/adapters.md). Allgemeine Wiederherstellung nach Ausführungsunterbrechungen und die Prüfung von Ergebniskandidaten werden noch nicht unterstützt. [Agenten ausführen](docs/agents.md) · [Datenverträge](docs/contracts.md).

Schreibgeschützte [Kontextquellen](docs/context-sources.md) unterstützen Abrufe pro Run oder Modellschritt, gespeicherte Datenpakete und die Prüfung aktueller Zugriffsrechte bei erneuter Nutzung.

[Skills](docs/skills.md) lädt vollständige Anweisungen einer festgelegten Version über registrierte Tools. [Artifacts](docs/artifacts.md) bewahrt Originale innerhalb ihres Geltungsbereichs, begrenzte Vorschauen und Quellenbelege auf.

Die [Kontextauswahl und -komprimierung](docs/context-compaction.md) bewahrt den ursprünglichen Gesprächsverlauf und verwendet begrenzte Vorschauen sowie geprüfte Zusammenfassungen. Modellbasierte Komprimierung nutzt dasselbe Run-Budget.
