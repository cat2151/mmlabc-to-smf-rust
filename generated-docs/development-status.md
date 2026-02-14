Last updated: 2026-02-15

# Development Status

## 現在のIssues
オープン中のIssueはありません。
直近の作業はデモページの改善や内部の自動化スクリプトの調整に集中しています。
具体的な次のステップとして、デモの使いやすさ向上、自動生成プロンプトの精度検証、そしてコアライブラリのテスト強化が考えられます。

## 次の一手候補
1. デモページのユーザーエクスペリエンス向上とMML記法説明の追加 [関連Issue #88](../issue-notes/88.md)
   - 最初の小さな一歩: `demo/index.html` にMML記法の簡単な説明セクションを追加することを検討する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `demo/index.html`, `demo/FEATURES.md`

     実行内容: `demo/index.html` の現在のコンテンツと `demo/FEATURES.md` を分析し、MML記法の入門者向け説明や主要機能のハイライトを追加する最適な場所と形式を提案してください。特に、ユーザーがMMLを初めて使う際に役立つような説明を考慮してください。

     確認事項: 既存のレイアウトを大きく崩さないこと。追加する情報が冗長にならないように `demo/FEATURES.md` の内容と適切に連携させること。

     期待する出力: `demo/index.html` に追加すべきMML記法の説明コンテンツのmarkdown形式の提案と、その配置場所。
     ```

2. 開発状況レポート生成プロンプトの精度向上 [関連Issue #30](../issue-notes/30.md)
   - 最初の小さな一歩: `generated-docs/development-status.md` の最近の出力と、このプロンプト（`development-status-prompt.md`）の要件を比較し、改善点を特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `generated-docs/development-status.md`, `development-status-prompt.md`, `.github/actions-tmp/.github_automation/project_summary/prompts/development-status-prompt.md`

     実行内容: `generated-docs/development-status.md` の最近の出力内容が、本プロンプト `development-status-prompt.md` （及び `.github/actions-tmp/.github_automation/project_summary/prompts/development-status-prompt.md`）で定義されている要件（特に「生成しないもの」）をどの程度満たしているかを評価してください。特に、ハルシネーションや不適切な提案がないかを確認し、プロンプトの改善案を提案してください。

     確認事項: 生成されたレポートがユーザーにとって価値のある情報を提供しているか、また、不要な情報を排除できているか。

     期待する出力: 現在のプロンプトの課題点（もしあれば）と、それを解決するための `development-status-prompt.md` の具体的な修正案をmarkdown形式で出力してください。
     ```

3. MMLパーサーのテストカバレッジレビューと拡張 [関連Issue #85](../issue-notes/85.md)
   - 最初の小さな一歩: `src/pass1_parser.rs` の主要なパースロジックに対応する `tests/test_pass1.rs` のテストケースをレビューし、不足しているテストシナリオがないか確認する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/pass1_parser.rs`, `tests/test_pass1.rs`, `tests/integration_test.rs`

     実行内容: `src/pass1_parser.rs` で実装されているMMLのパース機能について、`tests/test_pass1.rs` および `tests/integration_test.rs` の既存のテストケースを分析し、特に複雑なMML記法（例: 同時発音、マクロ、変調など）やエッジケースに対するテストカバレッジのギャップを特定してください。

     確認事項: パースエラーを引き起こす可能性のある入力や、予期せぬ挙動につながるような記法の組み合わせがテストされているか。

     期待する出力: 特定されたテストカバレッジのギャップをリストアップし、それぞれについて新規追加すべきテストケースの概要（入力MMLと期待されるパース結果の概念）をmarkdown形式で提案してください。
     ```

---
Generated at: 2026-02-15 07:06:12 JST
