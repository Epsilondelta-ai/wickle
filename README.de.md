# Wickle

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md) | **Deutsch** | [Русский](README.ru.md)

**Eine erweiterbare Agenten-Engine von EpsilonDelta.**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="Das Wickle-Maskottchen, ein Igel, der in einem Laufrad läuft" width="420" />
</p>

Wickle ist eine Agenten-Engine, die EpsilonDelta in Rust entwickelt. Ziel ist eine in Anwendungen einbettbare Bibliothek, die den Zyklus aus Modellentscheidungen und Werkzeugaufrufen ausführt.

Der Entwurf verwendet Agentenprofile (Agent Profiles) zur Konfiguration des Agentenverhaltens und Adapter zur Anbindung verschiedener Modelle und Werkzeuge.

**Status:** In Entwicklung. Agenten unterstützen sequenzielle Modell- und Werkzeugaufrufe, getrennte Systemeingaben, gespeicherte Ergebnisse, Ereigniswiedergabe, Abbruch und die Fortsetzung gespeicherter Wartezustände für Genehmigungen, Eingaben oder externe Wirkungen. Fortsetzungsbefehle werden dedupliziert; externe Wirkungen müssen vom Host geprüft werden. Lifecycle-Hooks unterstützen begrenzte Kontext- und Argumenttransformationen sowie die Beobachtung gespeicherter Ergebnisse. Tool-/Hook-/ContextSource-Exporte von Adaptern verwenden festgelegte Konfigurationen und segmentgebundene Instanzen, deren Ressourcen am Segmentende freigegeben werden. [Adapter-Laufzeit](docs/adapters.md). Unterbrochene Läufe können mit geprüfter Ausführungsberechtigung, gespeicherten Eingaben und Abgleich externer Wirkungen wiederhergestellt werden. [Wiederherstellung](docs/recovery.md). [Agenten ausführen](docs/agents.md) · [Datenverträge](docs/contracts.md).

Schreibgeschützte [Kontextquellen](docs/context-sources.md) unterstützen Abrufe pro Run oder Modellschritt, gespeicherte Datenpakete und die Prüfung aktueller Zugriffsrechte bei erneuter Nutzung.

[Skills](docs/skills.md) lädt vollständige Anweisungen einer festgelegten Version über registrierte Tools. [Artifacts](docs/artifacts.md) bewahrt Originale innerhalb ihres Geltungsbereichs, begrenzte Vorschauen und Quellenbelege auf.

Die [Kontextauswahl und -komprimierung](docs/context-compaction.md) bewahrt den ursprünglichen Gesprächsverlauf und verwendet begrenzte Vorschauen sowie geprüfte Zusammenfassungen. Modellbasierte Komprimierung nutzt dasselbe Run-Budget.

Die [Ausgabeprüfung](docs/verification.md) unterstützt JSON-Schemas, versionierte Kriterien, begrenzte Überarbeitungen und die Genehmigung eines festgelegten Kandidaten. Modellbasierte Prüfungen nutzen dasselbe Run-Budget.

Der [OpenAI-Responses-Adapter](docs/openai.md) unterstützt begrenzte HTTP/SSE-Generierung, Funktionsaufrufe, strukturierte Ausgaben und Modellversionsprüfungen mit von der Anwendung bereitgestellten Zugangsdaten.

Informationen zu Azure-Bereitstellungen und vom Host verwalteten API-Schlüsseln oder Entra-Token stehen im [Leitfaden zum Azure-OpenAI-Adapter](docs/azure-openai.md).

Claude Messages, die Erhaltung von Thinking-Signaturen und Zugangsdaten pro Workspace beschreibt der [Anthropic-Adapter-Leitfaden](docs/anthropic.md).

AWS-Signaturen, Inferenzprofile und Streaming-Protokolle beschreibt der [Bedrock-Adapter-Leitfaden](docs/bedrock.md).

API-Versionen, Funktionsschemas und die Erhaltung von Gedankensignaturen beschreibt der [Gemini-Adapter-Leitfaden](docs/gemini.md).

Google-Cloud-OAuth und globale oder regionale Ziele beschreibt der [Vertex-AI-Adapter-Leitfaden](docs/vertex.md).
