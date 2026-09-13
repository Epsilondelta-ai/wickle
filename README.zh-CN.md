# Wickle

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | **简体中文** | [Español](README.es.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [Русский](README.ru.md)

**由 EpsilonDelta 开发的可扩展智能体引擎。**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="Wickle 的刺猬吉祥物正在跑轮上奔跑" width="420" />
</p>

Wickle 是 EpsilonDelta 正在使用 Rust 开发的智能体引擎，旨在以可嵌入应用程序的库形式，运行模型决策与工具调用的循环。

其设计通过 Agent Profile 配置智能体行为，并通过适配器连接不同的模型和工具。

**开发状态：** 已支持模型与工具的串行调用、系统输入分离、结果持久化、事件重放、取消，以及从已保存的审批、输入或外部操作确认等待中恢复执行。恢复命令会去重，外部操作是否生效须由 Host 验证。生命周期 Hook 支持受限的上下文和参数转换，以及提交后的结果观察。适配器 Tool/Hook/ContextSource export 使用固定的装配配置和按执行阶段隔离的实例，阶段结束后释放资源。[适配器运行时](docs/adapters.md)。支持在验证执行所有权和已保存输入后恢复中断的执行，并查询外部操作的结果。[执行恢复](docs/recovery.md)。[运行智能体](docs/agents.md) · [数据契约](docs/contracts.md)。

只读 [ContextSource](docs/context-sources.md) 支持按 Run 或模型步骤查询、保存数据批次，并在复用时重新检查当前访问权限。

[Skills](docs/skills.md) 通过已注册工具加载固定版本的完整指令。[Artifacts](docs/artifacts.md) 保存按 scope 隔离的原文、有大小限制的预览及来源依据。

[上下文选择与压缩](docs/context-compaction.md) 保留原始对话，并应用有大小限制的预览和经过验证的摘要。模型压缩也使用同一 Run 的预算。

[输出验证](docs/verification.md)支持 JSON Schema、固定版本的验证标准、限定次数的修正和固定候选结果的审批。模型验证也使用同一 Run 预算。
