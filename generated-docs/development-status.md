Last updated: 2025-11-16

# Development Status

## 現在のIssues
- [Issue #39](../issue-notes/39.md)は、MMLabcフォーマットに準拠した`kt`（key transpose）コマンドの実装を目指しています。
- この`kt`コマンドは、指定された値だけノート番号を増減させ、キーの移調を可能にします。
- [Issue #37](../issue-notes/37.md)では、mmlabcの`@128`指定があるトラックをMIDIチャンネル9（ドラムチャンネル）として処理する機能の実装が求められています。

## 次の一手候補
1. [Issue #39](../issue-notes/39.md) ktコマンド（キー移調）の実装
   - 最初の小さな一歩: `tree-sitter-mml/grammar.js` に `kt` コマンドの構文を追加し、パーサーがこれを認識できるようにする。
   - Agent実行プロンプト:
     ```
     対象ファイル: tree-sitter-mml/grammar.js, tree-sitter-mml/src/grammar.json, src/pass1_parser.rs

     実行内容: `tree-sitter-mml/grammar.js` に `kt` コマンド（例: `kt1 c`, `kt-1 c` の後にMMLシーケンスが続く形式）のパーシングルールを追加してください。その後、`tree-sitter-mml/src/grammar.json` を再生成し、`src/pass1_parser.rs` が新しいASTノードを適切に処理できるよう、変更の概要を提示してください。

     確認事項: 既存のノートやオクターブ変更などのMMLコマンドのパースに影響を与えないこと。`kt`コマンドは数値（正負両方）とそれに続くMMLシーケンスを取ることを考慮すること。

     期待する出力: 更新された `tree-sitter-mml/grammar.js` の関連部分と、`src/pass1_parser.rs` で`kt`コマンドをパースするために必要な変更点の概要をmarkdown形式で出力してください。
     ```

2. [Issue #37](../issue-notes/37.md) @128（ドラムチャンネル）の実装
   - 最初の小さな一歩: `src/pass1_parser.rs` で `@128` の構文を認識し、ASTにドラムチャンネル指定として記録する。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/pass1_parser.rs, src/pass2_ast.rs, src/types.rs

     実行内容: `src/pass1_parser.rs` にて、トラックの先頭に現れる `@128` コマンドを認識し、その情報をAST（`src/pass2_ast.rs` の関連構造体や、`src/types.rs` の定義）に適切に格納できるよう変更してください。具体的には、AST構造にドラムチャンネル指定のフラグや情報を追加することを検討してください。

     確認事項: `@128`はトラック全体のプロパティとして扱われるべきであり、他のチャンネル変更コマンド（例: `@n`）との相互作用を考慮すること。既存のパーシングロジックに予期せぬ影響を与えないこと。

     期待する出力: `@128`コマンドを処理するための `src/pass1_parser.rs`, `src/pass2_ast.rs`, `src/types.rs` の変更点を具体的にmarkdown形式で提示してください。
     ```

3. プログラムチェンジ (@n) コマンドの実装
   - 最初の小さな一歩: `tree-sitter-mml/grammar.js` に `@n` コマンドの構文（`n`は数値）を追加し、パーサーがこれを認識できるようにする。
   - Agent実行プロンプト:
     ```
     対象ファイル: tree-sitter-mml/grammar.js, tree-sitter-mml/src/grammar.json, src/pass1_parser.rs, src/pass2_ast.rs, src/pass3_events.rs, src/pass4_midi.rs

     実行内容: `tree-sitter-mml/grammar.js` に `@n` コマンド（例: `@0`, `@127`など）のパーシングルールを追加し、`src/pass1_parser.rs` がこれをASTに変換できるよう、また `src/pass3_events.rs` や `src/pass4_midi.rs` でMIDIプログラムチェンジイベントとして適切に処理できるよう、関連ファイルの変更の概要を提示してください。

     確認事項: `@n`コマンドはトラック内のどこにでも出現可能であり、その時点以降のノートに影響を与えることを考慮すること。また、既に存在する `@128`（ドラムチャンネル）との競合がないことを確認すること。

     期待する出力: `@n`コマンドをパースし、MIDIイベントとして生成するための `tree-sitter-mml/grammar.js` の関連部分と、`src/pass1_parser.rs`, `src/pass2_ast.rs`, `src/pass3_events.rs`, `src/pass4_midi.rs` で必要な変更点の概要をmarkdown形式で出力してください。
     ```

---
Generated at: 2025-11-16 07:04:45 JST
