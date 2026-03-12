Last updated: 2026-03-13

# Development Status

## 現在のIssues
- 現在、プロジェクトにおいて追跡中のオープンIssueは存在しません。
- 直近では、[Issue #117](../issue-notes/117.md)でMMLのデフォルトノート長が`l4`から`l8`へ変更されました。
- また、[Issue #116]（issue-noteは未作成ですが、コミット履歴から推測）で和音表記 `cg;e` のバグが修正され、MMLパーサーの正確性が向上しています。

## 次の一手候補
1.  デフォルトノート長 `l8` 変更 ([Issue #117](../issue-notes/117.md)) の影響検証とテスト拡充
    -   最初の小さな一歩: `src/mml_preprocessor.rs` および関連するパーサーステージでの `l8` のデフォルト値の扱いの確認と、既存テスト `tests/test_length.rs` に `l8` がデフォルトとして正しく適用されるシナリオを追加する。
    -   Agent実行プロンプト:
        ```
        対象ファイル: `src/mml_preprocessor.rs`, `src/pass1_parser.rs`, `tests/test_length.rs`, `tests/test_pass1.rs`

        実行内容: [Issue #117](../issue-notes/117.md)でデフォルトノート長が`l8`に変更されたことによる、MMLパーサー（特に`mml_preprocessor`と`pass1_parser`）への影響を分析してください。この変更が意図通りに動作し、予期せぬ副作用がないことを確認するため、`tests/test_length.rs`および`tests/test_pass1.rs`に、`l8`がデフォルトとして適用されるMML文字列のテストケースを追加することを検討してください。

        確認事項: `calculate_duration`関数の変更が他のノート長計算に影響を与えていないか、また`mmlabc`方言における`l8`の一般的な解釈と整合しているかを確認してください。

        期待する出力: 既存のテストファイル(`tests/test_length.rs`, `tests/test_pass1.rs`)に`l8`デフォルト適用に関するテストケースを追加するための具体的なコードスニペット（Rust言語）と、追加テストの妥当性を説明するmarkdown形式のコメント。
        ```

2.  大規模ファイルチェック (`.github/check-large-files.toml`) の効率化と設定柔軟化 ([Issue #44](../issue-notes/44.md))
    -   最初の小さな一歩: `check-large-files.toml` の設定オプションと、関連スクリプト `check_large_files.py` を分析し、現在のプロジェクトでの大規模ファイル検出が適切に機能しているか、また設定の柔軟性を高める余地がないかを確認する。
    -   Agent実行プロンプト:
        ```
        対象ファイル: `.github/check-large-files.toml`, `.github/actions-tmp/.github_automation/check-large-files/scripts/check_large_files.py`

        実行内容: プロジェクトにおける大規模ファイル検出ワークフロー（`.github/workflows/call-check-large-files.yml`）の効率性と設定の柔軟性について分析してください。特に、`.github/check-large-files.toml`がプロジェクトのニーズに合致しているか、`check_large_files.py`スクリプトが効率的に動作しているかを確認し、必要に応じて設定項目やスクリプトの改善案を提示してください。例えば、特定のディレクトリを無視する機能や、ファイルサイズの閾値を動的に設定する機能などが考えられます。

        確認事項: 既存のワークフローに影響を与えないこと。また、`check-large-files.toml`の変更が`.github/check-large-files.toml.default`との整合性を保っていることを確認してください。

        期待する出力: 大規模ファイルチェックの効率化および設定の柔軟化のための具体的な`check-large-files.toml`の変更案、または`check_large_files.py`スクリプトの修正案をmarkdown形式で出力してください。
        ```

3.  WASMモジュール `mmlabc-to-smf-wasm` のデモ連携と機能拡張 ([Issue #103](../issue-notes/103.md))
    -   最初の小さな一歩: `mmlabc-to-smf-wasm` クレートの現在の機能を確認し、`demo/src/mmlConverter.ts` がWASMモジュールを利用する際にどのようなAPIが必要となるかを洗い出す。
    -   Agent実行プロンプト:
        ```
        対象ファイル: `mmlabc-to-smf-wasm/src/lib.rs`, `mmlabc-to-smf-wasm/Cargo.toml`, `demo/src/mmlConverter.ts`, `demo/index.html`

        実行内容: `mmlabc-to-smf-wasm`クレートのWebAssemblyモジュールを`demo`プロジェクトでより効果的に利用するための機能拡張を検討してください。具体的には、既存の`mmlConverter.ts`でのWASMモジュールの利用を想定し、MML変換機能以外の、例えば特定のMML記法のエラーチェックや、SMF生成前のプレビュー機能など、WASMで提供可能な新機能の候補を洗い出してください。そして、これらの機能をデモで統合するためのロードマップと、必要な`mmlabc-to-smf-wasm`のAPI変更案を分析してください。

        確認事項: WASMモジュールのファイルサイズやパフォーマンスへの影響を最小限に抑えること。また、`demo`プロジェクトの既存のUI/UXデザインとの整合性を考慮してください。

        期待する出力: `mmlabc-to-smf-wasm`にWASMとしてエクスポートすべき新規APIの具体的なシグネチャ例と、`demo`プロジェクトでそれらを統合するための`mmlConverter.ts`および`index.html`の変更案をmarkdown形式で出力してください。

---
Generated at: 2026-03-13 07:07:37 JST
