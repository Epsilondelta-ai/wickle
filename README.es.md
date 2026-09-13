# Wickle

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | **Español** | [Français](README.fr.md) | [Deutsch](README.de.md) | [Русский](README.ru.md)

**Un motor de agentes extensible de EpsilonDelta.**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="La mascota de Wickle, un erizo corriendo en una rueda de ejercicio" width="420" />
</p>

Wickle es un motor de agentes que EpsilonDelta está desarrollando en Rust. Su objetivo es ofrecer una biblioteca integrable en aplicaciones que ejecute el ciclo de decisiones del modelo y llamadas a herramientas.

Su diseño utiliza perfiles de agente (Agent Profiles) para configurar el comportamiento de los agentes y adaptadores para conectar distintos modelos y herramientas.

**Estado:** En desarrollo. Los agentes admiten llamadas secuenciales al modelo y a herramientas, entradas del sistema separadas, resultados persistentes, reproducción de eventos, cancelación y reanudación de esperas guardadas de aprobación, datos o confirmación de efectos externos. Los comandos de reanudación se deduplican; los efectos externos requieren verificación del Host. Los hooks del ciclo de vida permiten transformar contexto y argumentos con límites y observar resultados ya guardados. Los exports Tool/Hook/ContextSource de adaptadores usan una configuración fijada e instancias por segmento, que liberan sus recursos al finalizar. [Runtime de adaptadores](docs/adapters.md). Las ejecuciones interrumpidas pueden recuperarse validando la propiedad de ejecución, conservando las entradas y consultando los efectos externos. [Recuperación](docs/recovery.md). [Ejecutar un agente](docs/agents.md) · [Contratos de datos](docs/contracts.md).

Las [fuentes de contexto](docs/context-sources.md) de solo lectura permiten consultar por Run o paso del modelo, guardar lotes y comprobar los permisos actuales al reutilizarlos.

[Skills](docs/skills.md) carga instrucciones completas de una versión fija mediante una herramienta registrada. [Artifacts](docs/artifacts.md) conserva originales por ámbito, vistas previas limitadas y referencias de origen.

La [selección y compresión de contexto](docs/context-compaction.md) conserva la conversación original y aplica vistas previas limitadas y resúmenes validados. La compresión con modelos utiliza el mismo presupuesto del Run.

La [verificación de resultados](docs/verification.md) admite esquemas JSON, criterios con versión fija, revisiones limitadas y aprobación de un candidato fijo. Las revisiones mediante modelos comparten el presupuesto del Run.

El [adaptador OpenAI Responses](docs/openai.md) ofrece generación HTTP/SSE con límites, llamadas a funciones, salida estructurada y comprobación de versiones, con credenciales proporcionadas por la aplicación.
