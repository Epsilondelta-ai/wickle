# Wickle

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | **Français** | [Deutsch](README.de.md) | [Русский](README.ru.md)

**Un moteur d’agents extensible développé par EpsilonDelta.**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="La mascotte de Wickle, un hérisson courant dans une roue d’exercice" width="420" />
</p>

Wickle est un moteur d’agents qu’EpsilonDelta développe en Rust. Il vise à fournir une bibliothèque intégrable aux applications, qui exécute une boucle de décisions du modèle et d’appels d’outils.

Sa conception repose sur des profils d’agent (Agent Profiles) pour configurer le comportement des agents et sur des adaptateurs pour connecter différents modèles et outils.

**État :** En développement. Les agents textuels peuvent exécuter des requêtes, conserver les résultats, relire les événements et annuler explicitement une exécution. L’exécution des outils et les autres extensions sont en cours de développement. [Exécuter un agent](docs/agents.md) · [Contrats de données](docs/contracts.md).
