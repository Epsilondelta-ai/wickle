# Wickle

[English](README.md) | [한국어](README.ko.md) | **日本語** | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [Русский](README.ru.md)

**EpsilonDelta が開発する、拡張可能なエージェントエンジン。**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="回し車を走るハリネズミ、Wickle のマスコット" width="420" />
</p>

Wickle は、EpsilonDelta が Rust で開発しているエージェントエンジンです。モデルによる判断とツール呼び出しのループを、アプリケーションに組み込めるライブラリとして提供することを目指しています。

Agent Profile でエージェントの振る舞いを設定し、アダプターを通じてさまざまなモデルやツールに接続する構成を設計しています。

**開発状況:** モデルとツールの順次呼び出し、システム入力の分離、結果の保存、イベントの再取得、キャンセル、保存済みの承認・入力・外部作用確認待ちからの再開に対応しています。同じ再開コマンドは重複して処理せず、外部作用の確定には Host による検証を必要とします。Lifecycle Hook により、制限付きのコンテキスト・引数変換と保存後の観察にも対応しています。アダプターの Tool・Hook・ContextSource export は固定された構成と実行区間ごとのインスタンスで接続し、区間終了時にリソースを解放します。[アダプターランタイム](docs/adapters.md)。実行権限と保存済み入力を確認し、外部作用を照会して中断した実行を復旧できます。[実行の復旧](docs/recovery.md)。[エージェントの実行](docs/agents.md) · [データ契約](docs/contracts.md)。

読み取り専用の [ContextSource](docs/context-sources.md) は、Run・モデルステップごとの取得、データの保存、再利用時の現在のアクセス権確認に対応しています。

[Skills](docs/skills.md) は登録済みツールから固定バージョンの指示全文を読み込みます。[Artifacts](docs/artifacts.md) はスコープ付きの原文、サイズ制限付きプレビュー、出典情報を保持します。

[コンテキストの選択・圧縮](docs/context-compaction.md) は元の会話を保持し、制限付きプレビューと検証済みの要約を適用します。モデルによる圧縮も同じ Run の予算を使用します。

[出力検証](docs/verification.md)は JSON Schema、バージョンを固定した評価基準、上限付きの修正、固定された候補の承認に対応します。モデルによる検証も同じ Run 予算を使います。

[モデルプロバイダーガイド](docs/model-providers.md)では、OpenAI、Azure OpenAI、Anthropic、AWS Bedrock、Gemini API、Vertex AI、xAI アダプターの共通接続方法を説明しています。認証、API バージョン、各プロバイダーの動作の違いは個別に扱います。
