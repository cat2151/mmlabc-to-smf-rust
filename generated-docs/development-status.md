Last updated: 2026-02-04

# Development Status

## 現在のIssues
- [Issue #55](../issue-notes/55.md): `README.ja.md`内の`grammar.js`への参照をGitHubリポジトリのURLリンクに改善し、読者が直接アクセスできるようにする。
- [Issue #39](../issue-notes/39.md): `mmlabc`フォーマット準拠の`kt`（key transpose）コマンドを実装し、その機能のデモを追加する。
- [Issue #37](../issue-notes/37.md): `@128`コマンドを含むMMLトラックを、MIDIチャンネル9（ドラムチャンネル）として扱う`mmlabc`準拠の機能を実装する。

## 次の一手候補
1. [Issue #55](../issue-notes/55.md): `README.ja.md`の`grammar.js`への参照をURLリンクに改善する
   - 最初の小さな一歩: `README.ja.md`内の「`tree-sitter-mml/grammar.js` をお読みください」という記述のファイルパス部分を、GitHubリポジトリ上の`grammar.js`ファイルへの直接URLリンクに置き換える。
   - Agent実行プロンプ:
     ```
     対象ファイル: `README.ja.md`

     実行内容: `README.ja.md`ファイルを開き、「実装されたMMLを知りたい場合、まず `tree-sitter-mml/grammar.js` をお読みください」という記述を見つけてください。この文中の`tree-sitter-mml/grammar.js`を、GitHubリポジトリの該当ファイル（`https://github.com/cat2151/mmlabc-to-smf-rust/blob/main/tree-sitter-mml/grammar.js`）へのMarkdownリンクに修正してください。

     確認事項: 変更後、生成されたリンクが正しく機能すること、および`README.ja.md`の全体的な日本語表現が自然であることを確認してください。また、`README.md`（英語バージョン）には影響を与えないことを確認してください。

     期待する出力: 変更が適用された`README.ja.md`の更新内容。
     ```

2. [Issue #39](../issue-notes/39.md): `kt`コマンド（key transpose）を実装する
   - 最初の小さな一歩: `tree-sitter-mml/grammar.js` に`kt`コマンドの構文（`kt`、それに続く符号付き整数、そして音符）を定義するルールを追加する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `tree-sitter-mml/grammar.js`

     実行内容: `tree-sitter-mml/grammar.js`ファイルに、`kt`コマンドをパースするための新しいルールを追加してください。このルールは、`"kt"`というキーワードの後に符号付きの整数（例: `+1`, `-2`, `3`）、そしてMMLの音符（`c`, `d`, `e`, `f`, `g`, `a`, `b`とその修飾子）が続く形式を認識するようにしてください。既存の音符やコマンドのルールと整合性が取れるように定義してください。

     確認事項: 新しい`kt`コマンドのルールが既存のMML文法と競合しないかを確認してください。また、変更後に`npx tree-sitter generate`コマンドを実行し、パーサーファイルがエラーなく再生成されることを確認してください。

     期待する出力: `kt`コマンドの文法ルールが追加された`tree-sitter-mml/grammar.js`の更新された内容。
     ```

3. [Issue #37](../issue-notes/37.md): `@128`のあるトラックをMIDIチャンネル9（ドラム）として扱う機能を実装する
   - 最初の小さな一歩: `tree-sitter-mml/grammar.js`で`@`コマンドがどのようにパースされているか確認し、`pass1_parser.rs`および`pass2_ast.rs`で`@128`情報がASTに適切に伝達されるよう、既存のAST構造体の確認と必要に応じた変更案を検討する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `tree-sitter-mml/grammar.js`, `src/pass1_parser.rs`, `src/pass2_ast.rs`, `src/types.rs`

     実行内容: まず`tree-sitter-mml/grammar.js`を分析し、`@`コマンドがどのようにトークン化されるかを確認してください。次に、`src/pass1_parser.rs`でそれがどのように処理され、`src/pass2_ast.rs`で抽象構文木（AST）に変換されるか、特に`@128`のような特定の数値が伴う場合にその情報がASTに保持されるかを調査してください。その結果に基づき、トラックごとに`@128`コマンドの存在を識別できるようにするために、`src/types.rs`内の既存のAST構造体（例: `AstNode`や関連する構造体）にどのような変更が必要か、または既存の構造で対応可能かを分析し、変更の必要性とその具体的な提案をmarkdown形式で記述してください。

     確認事項: `@`コマンド全般の既存のパースロジックに影響を与えないこと。提案される変更が、MMLの他の要素（音符、長さ、テンポなど）のパースやAST表現に予期せぬ副作用をもたらさないことを確認してください。

     期待する出力: `@128`コマンドの検出と、それがトラックのMIDIチャンネル割り当てに影響を与えるための、`grammar.js`、`pass1_parser.rs`、`pass2_ast.rs`における必要な変更点および`src/types.rs`のAST構造体への提案される変更を詳細に説明したmarkdown形式の分析レポート。
     ```

---
Generated at: 2026-02-04 07:10:13 JST
