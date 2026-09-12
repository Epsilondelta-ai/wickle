# Wickle

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | **Español** | [Français](README.fr.md) | [Deutsch](README.de.md) | [Русский](README.ru.md)

**Un motor de agentes extensible de EpsilonDelta.**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="La mascota de Wickle, un erizo corriendo en una rueda de ejercicio" width="420" />
</p>

Wickle es un motor de agentes que EpsilonDelta está desarrollando en Rust. Su objetivo es ofrecer una biblioteca integrable en aplicaciones que ejecute el ciclo de decisiones del modelo y llamadas a herramientas.

Su diseño utiliza perfiles de agente (Agent Profiles) para configurar el comportamiento de los agentes y adaptadores para conectar distintos modelos y herramientas.

**Estado:** En desarrollo. Los agentes admiten ciclos secuenciales de llamadas al modelo y a herramientas, resultados persistentes, reproducción de eventos y cancelación explícita. Las implementaciones registradas de `ToolExecutor` reciben los argumentos validados del modelo combinados con `SystemInputs` que el Host gestiona por separado. Si se requiere aprobación o no se puede confirmar el efecto de una escritura, la ejecución se detiene en `Waiting`; `resume` aún no está disponible. [Ejecutar un agente](docs/agents.md) · [Contratos de datos](docs/contracts.md).
