Last updated: 2026-02-09

# Development Status

## 現在のIssues
- 現在、プロジェクトにはオープンなIssueが存在しません。
- これは、最近のデモ関連の修正が順調に進み、主要な問題が解決されたことを示しています。
- 今後は、機能の安定性向上や品質改善に焦点を当てることが推奨されます。

## 次の一手候補
1. デモプロジェクトのセットアップと利用ガイドの整備 [新規Issue]
   - 最初の小さな一歩: `demo`および`demo-library`ディレクトリ内のファイルを参照し、ユーザーがローカルでデモをセットアップして実行するための手順のドラフトを作成する。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo/package.json, demo/index.html, demo/.gitignore, demo-library/package.json, demo-library/index.html, scripts/build-demo.sh, scripts/transform-demo-paths.sh

     実行内容: `demo`および`demo-library`のセットアップ、ビルド、実行方法に関する既存の情報を収集し、ユーザーがデモをローカルで動かすための具体的な手順をmarkdown形式で整理してください。特に、`npm install`やビルドスクリプトの実行、ブラウザでの表示手順に焦点を当ててください。

     確認事項: 最近のコミット（特にデモ関連の修正）で導入された新しい手順や依存関係が反映されていることを確認してください。また、`demo-library`が`demo`にどのように統合されているかを明確にしてください。

     期待する出力: `docs/demo-setup-guide.md`として保存できるような、Markdown形式のセットアップガイド。
     ```

2. CI/CDワークフローにおけるデモの自動テスト導入の検討 [新規Issue]
   - 最初の小さな一歩: 現在のCI/CDワークフローファイルを確認し、デモのビルドが成功していることを確認する既存の仕組みがあるか調査する。また、デモが正しく動作するかを検証するための簡単なエンドツーエンドテスト（例: 生成されたHTMLファイルの存在チェックやJavaScriptバンドルの検証など）を導入する可能性について検討する。
   - Agent実行プロンプト:
     ```
     対象ファイル: .github/workflows/call-daily-project-summary.yml, scripts/build-demo.sh, demo/package.json, demo-library/package.json

     実行内容: デモプロジェクト（`demo`および`demo-library`）のビルドと、その後の簡単な動作確認をCI/CDワークフローに組み込むための実現可能性を分析してください。どのようなツールやアクションが利用可能か、どのようなテストステップが適切かを検討し、提案してください。

     確認事項: 既存のワークフローに不必要な変更を加えないよう、依存関係と実行順序を慎重に確認してください。また、テスト実行時に必要な環境（Node.jsなど）がCI環境で利用可能か考慮してください。

     期待する出力: `README.md`または`docs/ci-demo-testing-proposal.md`に追加できるような、Markdown形式の提案書。提案書には、導入するテストの種類、想定されるステップ、および関連するワークフローファイルへの変更案を含めてください。
     ```

3. プロジェクト全体の依存関係の棚卸しと最適化の検討 [新規Issue]
   - 最初の小さな一歩: `Cargo.toml`, `package.json` (ルート、demo、demo-library, tree-sitter-mml) および `package-lock.json` ファイルから、現在使用されている全ての直接的・間接的な依存関係をリストアップする。
   - Agent実行プロンプト:
     ```
     対象ファイル: Cargo.toml, Cargo.lock, package.json (ルート, demo/, demo-library/, tree-sitter-mml/), package-lock.json

     実行内容: プロジェクト全体で使用されている依存関係（Rustクレート、Node.jsパッケージ）を洗い出し、それぞれの依存関係が本当に必要か、またはより軽量な代替品がないかを分析してください。特に、最近のデモ関連の変更で追加されたJavaScriptの依存関係に注目し、不要なものがないか、バージョンは最適かを確認してください。

     確認事項: 依存関係を削除または変更する前に、それがプロジェクトのビルド、テスト、実行に与える影響を十分に評価してください。特に、`mmlabc-to-smf-wasm`などのWebAssembly関連の依存関係は注意深く扱う必要があります。

     期待する出力: Markdown形式で、現在の依存関係のリスト、各依存関係の評価（必要性、代替案の可能性）、および最適化の提案をまとめたレポート。

---
Generated at: 2026-02-09 07:08:00 JST
