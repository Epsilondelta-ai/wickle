# Wickle

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | **简体中文** | [Español](README.es.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [Русский](README.ru.md)

**由 EpsilonDelta 开发的可扩展智能体引擎。**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="Wickle 的刺猬吉祥物正在跑轮上奔跑" width="420" />
</p>

Wickle 是 EpsilonDelta 正在使用 Rust 开发的智能体引擎，旨在以可嵌入应用程序的库形式，运行模型决策与工具调用的循环。

其设计通过 Agent Profile 配置智能体行为，并通过适配器连接不同的模型和工具。

**开发状态：** 已支持模型与工具的串行调用循环、结果持久化、事件重放和显式取消。注册的 `ToolExecutor` 接收经验证的模型输入，以及独立管理并由 Host 提供的 `SystemInputs` 合并后的参数。需要审批或无法确认写入是否生效时，执行会暂停于 `Waiting`；尚不支持 `resume`。[运行智能体](docs/agents.md) · [数据契约](docs/contracts.md)。
