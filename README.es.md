# Wickle

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | **Español** | [Français](README.fr.md) | [Deutsch](README.de.md) | [Русский](README.ru.md)

**Un motor de agentes extensible de EpsilonDelta.**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="La mascota de Wickle, un erizo corriendo en una rueda de ejercicio" width="420" />
</p>

Wickle es un motor de agentes que EpsilonDelta está desarrollando en Rust. Su objetivo es ofrecer una biblioteca integrable en aplicaciones que ejecute el ciclo de decisiones del modelo y llamadas a herramientas.

Su diseño utiliza perfiles de agente (Agent Profiles) para configurar el comportamiento de los agentes y adaptadores para conectar distintos modelos y herramientas.

**Estado:** En desarrollo. Los agentes de texto permiten ejecutar solicitudes, guardar resultados, reproducir eventos y cancelar explícitamente la ejecución. La ejecución de herramientas y otras extensiones siguen en desarrollo. [Ejecutar un agente](docs/agents.md) · [Contratos de datos](docs/contracts.md).
