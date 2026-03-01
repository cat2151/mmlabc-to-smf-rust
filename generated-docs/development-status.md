Last updated: 2026-03-02

# Development Status

## 現在のIssues
- [Issue #100](../issue-notes/100.md)は、多くのファイルが500行を超過しており、特にCodeQLの報告に起因する大規模ファイルの検出が課題となっています。
- [Issue #99](../issue-notes/99.md)と[Issue #90](../issue-notes/90.md)は、デモのSMFからオーディオへの変換・再生パイプラインを既存のTone.jsから`web-ym2151`と`smf-to-ym2151log-rust`ライブラリへ移行する重要かつ密接に関連するタスクです。
- これらのIssueは、コードの品質向上と主要機能の置き換え、およびデモ環境の改善に焦点を当てています。

## 次の一手候補
1. [Issue #99](../issue-notes/99.md)と[Issue #90](../issue-notes/90.md)を完了させる - デモのオーディオ再生をweb-ym2151に移行する
   - 最初の小さな一歩: `demo/src/audioPlayback.ts`と`demo/src/audioRenderer.ts`のコードをレビューし、現在のTone.jsベースのロジックと、`web-ym2151`への移行に必要な変更点を特定する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `demo/src/audioPlayback.ts`, `demo/src/audioRenderer.ts`, `demo/src/smfToYm2151.ts`, `demo/src/main.ts`, `demo/package.json`, `scripts/build-demo.sh`

     実行内容:
     1. `demo/src/audioPlayback.ts`と`demo/src/audioRenderer.ts`の現在のTone.jsを使用したSMF演奏ロジックを分析し、`web-ym2151`および`smf-to-ym2151log-rust`ライブラリに置き換えるための具体的な変更点を特定してください。
     2. SMFデータを受け取り、Ym2151ログを生成し、それを`web-ym2151`に渡してオーディオを再生する新しいパイプラインの統合方法について調査してください。
     3. `demo/package.json`に`web-ym2151`と`smf-to-ym2151log-rust`が追加されているか確認し、まだであれば追加する変更案を提示してください。
     4. `scripts/build-demo.sh`が新しいライブラリを適切にバンドルまたはコピーするように更新されているか確認し、必要であれば修正案を提案してください。

     確認事項:
     - `web-ym2151`と`smf-to-ym2151log-rust`のAPIドキュメントや使用例が存在するか確認し、その利用方法を把握してください。
     - 既存のTone.js関連のコードが完全に置き換え可能か、または一部が残るのかを検討してください。
     - 新しいライブラリ導入がデモのビルドプロセスに与える影響（特に`esbuild`の設定）を考慮してください。

     期待する出力: `web-ym2151`および`smf-to-ym2151log-rust`への移行に必要なコード変更の概要、`demo/package.json`の修正案、`scripts/build-demo.sh`の修正案をMarkdown形式で出力してください。
     ```

2. [Issue #100](../issue-notes/100.md)に対応 - 500行を超える大規模ファイルの特定とリファクタリング計画の策定
   - 最初の小さな一歩: `_codeql_detected_source_root`というCodeQLによって検出された特殊なパスのファイルが実際にリファクタリング対象かを確認し、もしそうでない場合は、CodeQLの検出設定を見直す。同時に、プロジェクト内で最も行数の多い実際のファイル上位5つを特定する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `.github/actions-tmp/.github_automation/check-large-files/scripts/check_large_files.py`, `.github/actions-tmp/.github_automation/check-large-files/check-large-files.toml.default`, `src/lib.rs`, `src/main.rs`, `demo/src/main.ts`など、プロジェクト内の主要なソースファイル。

     実行内容:
     1. [Issue #100](../issue-notes/100.md)で言及されている`_codeql_detected_source_root`のような特殊なパスが、実際の開発対象ファイルではない可能性を調査してください。これがCodeQLの内部的な報告であれば、`check-large-files.yml`ワークフローや`check_large_files.py`スクリプトの除外設定を検討してください。
     2. 実際にリファクタリングが必要な500行を超えるファイルのうち、最も影響範囲が大きく、かつリファクタリングの優先度が高いと思われるファイルを3つ特定してください。この際、ファイルの種類（Rust, TypeScript, JavaScriptなど）も考慮に入れてください。
     3. 特定した各ファイルについて、どのような観点（関数の分離、責務の分割、モジュール化など）でリファクタリングを進めるべきかの初期計画を提案してください。

     確認事項:
     - `check_large_files.py`がどのようにファイルパスを解決しているか、特に`_codeql_detected_source_root`のようなパスがどのように生成されるかを理解する。
     - 各ファイルの現在の行数と複雑度を把握し、依存関係を考慮する。
     - リファクタリングによって発生しうる副作用（ビルドエラー、機能不全など）を最小限に抑える方法を検討する。

     期待する出力: `_codeql_detected_source_root`パスに関する調査結果と、今後の処理方針、そしてリファクタリングすべき上位3つのファイルとそのファイルごとのリファクタリング初期計画をMarkdown形式で出力してください。
     ```

3. デモのビルドプロセスと依存関係の最適化
   - 最初の小さな一歩: `scripts/build-demo.sh`スクリプトと`demo/package.json`、`demo/.gitignore`ファイルをレビューし、[Issue #99](../issue-notes/99.md)で言及されている`smf-to-ym2151log-rust`と`web-ym2151`が正しく統合されているかを確認する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `scripts/build-demo.sh`, `demo/package.json`, `demo/.gitignore`

     実行内容:
     1. `scripts/build-demo.sh`スクリプトを分析し、`smf-to-ym2151log-rust`と`web-ym2151`ライブラリが適切にビルドプロセスに組み込まれているかを確認してください。特に、`npm install`、`esbuild`によるバンドル、WASMモジュールのコピーに関して、問題がないかをレビューしてください。
     2. `demo/package.json`にこれらの新しい依存関係が正しく追加され、バージョンが適切に管理されているかを確認してください。
     3. `demo/.gitignore`が、新しいライブラリによって生成される可能性のあるビルド成果物や一時ファイルを適切に無視しているかを確認し、必要であれば追記を提案してください。

     確認事項:
     - 新しいライブラリのビルド要件や配置場所を把握する。
     - `esbuild`の設定が外部モジュールを正しく扱っているかを確認する。
     - ビルドスクリプトが冪等性を持っているか、および環境による差異が生じないかを確認する。

     期待する出力: `scripts/build-demo.sh`、`demo/package.json`、`demo/.gitignore`に対する確認結果と、もしあれば改善提案や修正案をMarkdown形式で出力してください。

---
Generated at: 2026-03-02 07:06:03 JST
