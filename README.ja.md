# Wickle

[English](README.md) | [한국어](README.ko.md) | **日本語** | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [Русский](README.ru.md)

**EpsilonDelta が開発する、拡張可能なエージェントエンジン。**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="回し車を走るハリネズミ、Wickle のマスコット" width="420" />
</p>

Wickle は、EpsilonDelta が Rust で開発しているエージェントエンジンです。モデルによる判断とツール呼び出しのループを、アプリケーションに組み込めるライブラリとして提供することを目指しています。

Agent Profile でエージェントの振る舞いを設定し、アダプターを通じてさまざまなモデルやツールに接続する構成を設計しています。

**開発状況:** モデルとツールの順次呼び出し、システム入力の分離、結果の保存、イベントの再取得、キャンセル、保存済みの承認・入力・外部作用確認待ちからの再開に対応しています。同じ再開コマンドは重複して処理せず、外部作用の確定には Host による検証を必要とします。Lifecycle Hook により、制限付きのコンテキスト・引数変換と保存後の観察にも対応しています。アダプターの Tool・Hook・ContextSource export は固定された構成と実行区間ごとのインスタンスで接続し、区間終了時にリソースを解放します。[アダプターランタイム](docs/adapters.md)。一般的な実行中断からの復旧と候補結果の検証は、まだ未対応です。[エージェントの実行](docs/agents.md) · [データ契約](docs/contracts.md)。

読み取り専用の [ContextSource](docs/context-sources.md) は、Run・モデルステップごとの取得、データの保存、再利用時の現在のアクセス権確認に対応しています。

[Skills](docs/skills.md) は登録済みツールから固定バージョンの指示全文を読み込みます。[Artifacts](docs/artifacts.md) はスコープ付きの原文、サイズ制限付きプレビュー、出典情報を保持します。
