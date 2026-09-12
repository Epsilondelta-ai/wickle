# Wickle

[English](README.md) | [한국어](README.ko.md) | **日本語** | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [Русский](README.ru.md)

**EpsilonDelta が開発する、拡張可能なエージェントエンジン。**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="回し車を走るハリネズミ、Wickle のマスコット" width="420" />
</p>

Wickle は、EpsilonDelta が Rust で開発しているエージェントエンジンです。モデルによる判断とツール呼び出しのループを、アプリケーションに組み込めるライブラリとして提供することを目指しています。

Agent Profile でエージェントの振る舞いを設定し、アダプターを通じてさまざまなモデルやツールに接続する構成を設計しています。

**開発状況:** モデルとツールを順番に呼び出すループ、結果の保存、イベントの再取得、明示的なキャンセルに対応しています。登録された `ToolExecutor` には、検証済みのモデル入力と、別途管理する Host の `SystemInputs` を結合して渡します。承認が必要な場合や書き込みの適用状況が不明な場合は `Waiting` で停止します。`resume` はまだ未対応です。[エージェントの実行](docs/agents.md) · [データ契約](docs/contracts.md)。
