# Wickle

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | **简体中文** | [Español](README.es.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [Русский](README.ru.md)

**由 EpsilonDelta 开发的可扩展智能体引擎。**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="Wickle 的刺猬吉祥物正在跑轮上奔跑" width="420" />
</p>

Wickle 是 EpsilonDelta 正在使用 Rust 开发的智能体引擎，旨在以可嵌入应用程序的库形式，运行模型决策与工具调用的循环。

其设计通过 Agent Profile 配置智能体行为，并通过适配器连接不同的模型和工具。

**开发状态：** 已支持模型与工具的串行调用、系统输入分离、结果持久化、事件重放、取消，以及从已保存的审批、输入或外部操作确认等待中恢复执行。恢复命令会去重，外部操作是否生效须由 Host 验证。生命周期 Hook 支持受限的上下文和参数转换，以及提交后的结果观察。适配器 Tool/Hook/ContextSource export 使用固定的装配配置和按执行阶段隔离的实例，阶段结束后释放资源。[适配器运行时](docs/adapters.md)。尚不支持通用的执行中断恢复及候选结果验证。[运行智能体](docs/agents.md) · [数据契约](docs/contracts.md)。

只读 [ContextSource](docs/context-sources.md) 支持按 Run 或模型步骤查询、保存数据批次，并在复用时重新检查当前访问权限。
