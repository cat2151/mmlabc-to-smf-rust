Last updated: 2025-12-02

# Development Status

## 現在のIssues
- MIDI出力のmmlabcフォーマット準拠のため、`kt`コマンド ([Issue #39](../issue-notes/39.md)) の実装が進行中です。
- 同様に、`@128`を持つトラックをドラムチャンネル ([Issue #37](../issue-notes/37.md)) として扱う機能の実装も進められています。
- これらはMMLパーサーのセマンティック解析とMIDIイベント生成ロジックに影響を与えるため、テストと連携が重要です。

## 次の一手候補
1. `kt`コマンド（キー移調）の実装 [Issue #39](../issue-notes/39.md)
   - 最初の小さな一歩: `tree-sitter-mml/grammar.js`を更新し、`kt`コマンドをMML文法として認識できるようにする。
   - Agent実行プロンプ:
     ```
     対象ファイル: `tree-sitter-mml/grammar.js`, `src/pass1_parser.rs`

     実行内容: `kt`コマンドをMML文法に追加し、`tree-sitter-mml/grammar.js`を更新する。これに伴い、`src/pass1_parser.rs`で生成される`tree-sitter`のASTで`kt`コマンドが認識されるように、既存のパーサロジックを分析し、必要に応じて変更点を特定してください。

     確認事項: `kt`コマンドの引数（移調量と対象ノート）の構文がmmlabcフォーマットに準拠しているか確認し、既存のMML文法との競合がないか検証してください。`npm install && npm test`による文法テストの実行を確認してください。

     期待する出力: `kt`コマンドの文法定義が追加された`tree-sitter-mml/grammar.js`の更新内容と、`src/pass1_parser.rs`における必要な変更の提案をMarkdown形式で出力してください。
     ```

2. `@128`トラックのドラムチャンネル割り当て実装 [Issue #37](../issue-notes/37.md)
   - 最初の小さな一歩: `src/pass2_ast.rs`にトラック情報を保持するASTノードに`@128`ディレクティブの有無を記録するフラグを追加する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/pass2_ast.rs`, `src/types.rs`

     実行内容: `pass2_ast.rs`でASTを構築する際に、MMLトラック内に`@128`ディレクティブが存在するかを検出し、その情報をAST構造に保持するためのロジックを追加してください。具体的には、`src/types.rs`でトラックを表す構造体（例: `Track`や`MmlContext`）に`is_drum_track: bool`のようなフラグを追加し、`pass2_ast.rs`でパース時にそのフラグをセットする処理を実装してください。

     確認事項: `pass1_parser.rs`から`pass2_ast.rs`へのAST変換フローと、既存のチャネル割り当てロジック（`@channel`など）との優先順位を考慮してください。

     期待する出力: `src/types.rs`と`src/pass2_ast.rs`における変更内容を記述したプルリクエスト形式のMarkdown出力。
     ```

3. `pass2_ast`における`length`コマンドの解析強化
   - 最初の小さな一歩: `tests/test_length.rs`に、`length`コマンドが複数のノートに影響を与える場合のテストケースを追加する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `tests/test_length.rs`, `src/pass2_ast.rs`

     実行内容: `tests/test_length.rs`を分析し、`length`コマンド（例: `l4`, `l8.`）が複数の連続するノートに正しく適用されることを検証するテストケースを複数追加してください。特に、`l`コマンドの後に複数のノートや休符が続く場合、および途中で`l`コマンドが変更される場合のシナリオをカバーするようにテストを拡充してください。

     確認事項: `src/pass2_ast.rs`の`length`コマンド処理ロジックと、`src/pass3_events.rs`でのMIDIイベント生成に与える影響を確認してください。追加するテストケースが既存のテストを壊さないことを確認してください。

     期待する出力: 追加されたテストケースを含む`tests/test_length.rs`の変更内容と、もし必要であれば`src/pass2_ast.rs`の修正案をMarkdown形式で出力してください。

---
Generated at: 2025-12-02 07:04:58 JST
