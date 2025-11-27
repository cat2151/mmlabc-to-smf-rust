Last updated: 2025-11-28

# Development Status

## 現在のIssues
- [Issue #39](../issue-notes/39.md)は、mmlabcフォーマット準拠のktコマンド（key transpose）の実装に焦点を当てています。
- [Issue #37](../issue-notes/37.md)は、`@128`を含むトラックをMIDIチャンネル9（ドラムチャンネル）として扱うmmlabcフォーマット準拠の機能追加を目指しています。
- これらのissueは、mmlabcフォーマットへの準拠を強化し、MIDI変換の正確性を向上させるための機能拡張です。

## 次の一手候補
1. [Issue #39](../issue-notes/39.md) ktコマンドの実装
   - 最初の小さな一歩: `tree-sitter-mml/grammar.js`に`kt`コマンドの構文規則を追加し、`kt`コマンドが数値と音名を取る形式に対応させる。
   - Agent実行プロンプト:
     ```
     対象ファイル: tree-sitter-mml/grammar.js, src/pass1_parser.rs, src/pass2_ast.rs

     実行内容: `tree-sitter-mml/grammar.js`に`kt`コマンドの構文規則を追加し、`kt`コマンドが数値と音名を取る形式（例: `kt1 c`）に対応させる。その後、`src/pass1_parser.rs`でこの新しいトークンを認識し、`src/pass2_ast.rs`で適切なASTノードを生成できるように変更してください。

     確認事項: 既存のMML文法との衝突がないか、`kt`コマンドの引数（数値と音名）の解釈がmmlabcフォーマットに準拠しているかを確認してください。既存のテストが壊れないことを確認してください。

     期待する出力: `grammar.js`の更新内容、`pass1_parser.rs`と`pass2_ast.rs`における変更点の概要と関連するコードスニペットをmarkdown形式で出力してください。
     ```

2. [Issue #37](../issue-notes/37.md) `@128`トラックのMIDIチャンネル9割り当て
   - 最初の小さな一歩: `src/pass2_ast.rs`において、各トラックのASTノードを走査し、`@128`が存在するかどうかを検出して、その情報（例: `is_drum_track: true`のようなフラグ）をAST構造に追加することを検討する。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/pass2_ast.rs, src/pass3_events.rs, src/pass4_midi.rs

     実行内容: `src/pass2_ast.rs`において、各トラックのASTノードを走査し、`@128`が存在するかどうかを検出して、その情報（例: `is_drum_track: true`のようなフラグ）をAST構造に追加してください。次に、`src/pass3_events.rs`でMIDIイベントを生成する際にこのフラグを参照し、`is_drum_track`がtrueの場合にそのトラックのすべてのイベントをMIDIチャンネル9（0-indexed）に設定するように`src/pass4_midi.rs`の処理を調整してください。

     確認事項: `@128`が正しく検出されること、他のチャンネル設定と競合しないこと、および既存のMIDI生成ロジックが意図せず変更されないことを確認してください。mmlabcの「track」の定義とMIDIチャンネル割り当ての関連性を確認してください。

     期待する出力: `@128`検出ロジックの実装方法、ASTへのフラグ追加方法、およびMIDIチャンネル割り当て変更に関する`pass2_ast.rs`, `pass3_events.rs`, `pass4_midi.rs`の変更点をmarkdown形式で出力してください。
     ```

3. 既存MML構文のテストカバレッジ拡充と安定化
   - 最初の小さな一歩: `tests/test_pass1.rs`と`tests/test_pass2.rs`に、mmlabcフォーマットにおける複雑な音符、修飾子、制御コマンドの組み合わせを網羅する新しいテストケースを追加する。
   - Agent実行プロンプト:
     ```
     対象ファイル: tests/test_pass1.rs, tests/test_pass2.rs, src/pass1_parser.rs, src/pass2_ast.rs

     実行内容: `tests/test_pass1.rs`と`tests/test_pass2.rs`に、mmlabcフォーマットにおける様々なMML構文（特に複雑な音符、修飾子、制御コマンドの組み合わせ）を網羅する新しいテストケースを追加してください。これにより、パーサーとAST変換のロバスト性を向上させます。新しいテストケースは、成功ケースだけでなく、エラーハンドリングが必要な不正な構文のケースも考慮してください。

     確認事項: 新しいテストケースが既存の正常な動作を妨げないこと、およびテストカバレッジが実際に向上することを確認してください。追加するテストケースがmmlabcフォーマットの公式仕様または慣習に準拠していることを確認してください。

     期待する出力: `test_pass1.rs`と`test_pass2.rs`に追加するテストケースの例と、それらがカバーするMML構文の特徴をmarkdown形式で記述してください。
     ```

---
Generated at: 2025-11-28 07:05:32 JST
