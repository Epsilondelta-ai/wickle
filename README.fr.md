# Wickle

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | **Français** | [Deutsch](README.de.md) | [Русский](README.ru.md)

**Un moteur d’agents extensible développé par EpsilonDelta.**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="La mascotte de Wickle, un hérisson courant dans une roue d’exercice" width="420" />
</p>

Wickle est un moteur d’agents qu’EpsilonDelta développe en Rust. Il vise à fournir une bibliothèque intégrable aux applications, qui exécute une boucle de décisions du modèle et d’appels d’outils.

Sa conception repose sur des profils d’agent (Agent Profiles) pour configurer le comportement des agents et sur des adaptateurs pour connecter différents modèles et outils.

**État :** En développement. Les agents prennent en charge les appels séquentiels au modèle et aux outils, la séparation des entrées système, la persistance des résultats, la relecture des événements, l’annulation et la reprise des attentes enregistrées d’approbation, de données ou de confirmation d’effets externes. Les commandes de reprise sont dédupliquées ; les effets externes doivent être vérifiés par le Host. La récupération générale après interruption et la vérification des résultats candidats ne sont pas encore prises en charge. [Exécuter un agent](docs/agents.md) · [Contrats de données](docs/contracts.md).
