# Wickle

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | **Français** | [Deutsch](README.de.md) | [Русский](README.ru.md)

**Un moteur d’agents extensible développé par EpsilonDelta.**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="La mascotte de Wickle, un hérisson courant dans une roue d’exercice" width="420" />
</p>

Wickle est un moteur d’agents qu’EpsilonDelta développe en Rust. Il vise à fournir une bibliothèque intégrable aux applications, qui exécute une boucle de décisions du modèle et d’appels d’outils.

Sa conception repose sur des profils d’agent (Agent Profiles) pour configurer le comportement des agents et sur des adaptateurs pour connecter différents modèles et outils.

**État :** En développement. Les agents prennent en charge les appels séquentiels au modèle et aux outils, la séparation des entrées système, la persistance des résultats, la relecture des événements, l’annulation et la reprise des attentes enregistrées d’approbation, de données ou de confirmation d’effets externes. Les commandes de reprise sont dédupliquées ; les effets externes doivent être vérifiés par le Host. Les hooks du cycle de vie permettent des transformations limitées du contexte et des arguments, ainsi que l’observation des résultats enregistrés. Les exports Tool/Hook/ContextSource des adaptateurs utilisent une configuration figée et des instances par segment, libérées à sa fin. [Runtime des adaptateurs](docs/adapters.md). Les exécutions interrompues peuvent être reprises après vérification du propriétaire, en conservant les entrées et en vérifiant les effets externes. [Récupération](docs/recovery.md). [Exécuter un agent](docs/agents.md) · [Contrats de données](docs/contracts.md).

Les [sources de contexte](docs/context-sources.md) en lecture seule permettent la collecte par Run ou étape du modèle, la conservation des lots et la vérification des droits actuels lors de leur réutilisation.

[Skills](docs/skills.md) charge les instructions complètes d’une version fixée via un outil enregistré. [Artifacts](docs/artifacts.md) conserve les originaux dans leur périmètre, des aperçus limités et les références aux sources.

La [sélection et compression du contexte](docs/context-compaction.md) conserve la conversation originale et applique des aperçus limités et des résumés validés. La compression par modèle utilise le même budget du Run.

La [vérification des résultats](docs/verification.md) prend en charge les schémas JSON, les critères versionnés, les révisions limitées et l’approbation d’un candidat figé. Les vérifications par modèle partagent le budget du Run.

L’[adaptateur OpenAI Responses](docs/openai.md) propose la génération HTTP/SSE avec limites, les appels de fonctions, les sorties structurées et la vérification des versions, avec les identifiants fournis par l’application.

Pour les déploiements Azure et l’authentification par clé API ou Entra gérée par le Host, consultez le [guide de l’adaptateur Azure OpenAI](docs/azure-openai.md).

Consultez le [guide de l’adaptateur Anthropic](docs/anthropic.md) pour Claude Messages, la conservation des signatures thinking et les identifiants par workspace.

Consultez le [guide de l’adaptateur Bedrock](docs/bedrock.md) pour la signature AWS, les profils d’inférence et les protocoles de streaming.

Consultez le [guide de l’adaptateur Gemini](docs/gemini.md) pour les versions d’API, les schémas de fonctions et la conservation des signatures de raisonnement.

Consultez le [guide de l’adaptateur Vertex AI](docs/vertex.md) pour OAuth Google Cloud et les connexions globales ou régionales.
