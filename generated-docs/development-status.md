Last updated: 2026-01-21

# Development Status

## 現在のIssues
- [Issue #44](../issue-notes/44.md) は、ブラウザでMMLからSMFへのバイナリ変換をWASM (WASI Reactor FFI export) で実現し、Tree-sitterのC言語依存を解決することを目指しています。
- [Issue #39](../issue-notes/39.md) は、MMLabcフォーマット準拠の`kt`（key transpose）コマンドを実装し、音符番号の移調を可能にする機能追加を求めています。
- [Issue #37](../issue-notes/37.md) は、mmlabcフォーマットに従い、`@128`が指定されたトラックをMIDIチャンネル9（ドラムチャンネル）として扱う機能を実装する予定です。

## 次の一手候補
1. [Issue #44](../issue-notes/44.md): ブラウザで MML to SMF 変換を可能とするWASM版クレートをWASI Reactorで実装する
   - 最初の小さな一歩: Rustプロジェクトに`wasm32-wasi`ターゲットを追加し、`src/lib.rs`に`#[no_mangle]`と`pub extern "C"`を用いた最小限のFFI関数をエクスポートする。
   - Agent実行プロンプ:
     ```
     対象ファイル: `Cargo.toml`, `src/lib.rs`, `src/main.rs`

     実行内容: Rustプロジェクトに`wasm32-wasi`ターゲットを追加し、`mmlabc_to_smf_rust`クレートがWASI Reactorとしてコンパイル可能になるように設定してください。具体的には、`src/lib.rs`に`#[no_mangle]`と`pub extern "C"`を用いた最小限のFFI関数（例: `fn greet_wasm() -> i32`）を追加し、`Cargo.toml`に`crate-type = ["cdylib"]`を設定してください。

     確認事項: 既存のCLI機能がwasmターゲット追加後も正常にビルド・動作することを確認してください。また、`tree-sitter-mml/src/parser.c`のようなC言語依存のファイルがWASI Reactorの文脈でどのように扱われるか、その可能性を簡単に調査してください。

     期待する出力: WASI Reactorとしてビルド可能な`Cargo.toml`と`src/lib.rs`の変更内容。また、`wasm32-wasi`ターゲットの追加手順と、最小限のFFI関数の実装例およびそのコンパイル方法をmarkdown形式で出力してください。
     ```

2. [Issue #39](../issue-notes/39.md): ktコマンドを実装する。key transposeである。
   - 最初の小さな一歩: `tree-sitter-mml/grammar.js`に`kt`トークンと関連するルールを追加し、`src/tree_sitter_mml.rs`を更新して、Rust側で`kt`コマンドがパースツリーとして認識されるようにする。
   - Agent実行プロンプ:
     ```
     対象ファイル: `tree-sitter-mml/grammar.js`, `src/tree_sitter_mml.rs`

     実行内容: MMLの文法に`kt`コマンドを組み込むため、`tree-sitter-mml/grammar.js`に`kt`トークン（例: `anon_sym_kt`）と、それに続く数値引数（例: 符号付き整数）を認識するルールを追加してください。その後、`tree-sitter generate`を実行し、生成された`src/parser.c`に基づいて`src/tree_sitter_mml.rs`のバインディングを更新してください。最初の実装として、`kt`コマンドとその引数がAST（抽象構文木）に適切に表現されるまでを目標とします。

     確認事項: 既存のMML文法（ノート、オクターブ変更など）に影響を与えないことを確認してください。`tree-sitter-mml`のビルドプロセス（`tree-sitter generate`など）とRust側のバインディング更新手順を理解していることを前提とします。

     期待する出力: `tree-sitter-mml/grammar.js`と`src/tree_sitter_mml.rs`の変更内容。および、`kt`コマンドを認識する新しいAST構造の簡単な説明をmarkdown形式で出力してください。
     ```

3. [Issue #37](../issue-notes/37.md): `@128`のあるtrackはMIDI channel 9として扱う。
   - 最初の小さな一歩: `src/pass1_parser.rs`または`src/pass2_ast.rs`において、各MMLトラック（`;`で区切られる文字列グループ）内に`@128`コマンドが存在するかどうかを検出するロジックを実装し、検出結果をログ出力する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `src/pass1_parser.rs`, `src/pass2_ast.rs`, `src/pass3_events.rs`

     実行内容: MMLパース処理の初期段階（`src/pass1_parser.rs`または`src/pass2_ast.rs`）で、各MMLトラック（MML文字列が`;`で区切られたもの）のASTノードを走査し、そのトラック内に`@128`コマンドが存在するかどうかを検出する機能を実装してください。検出された場合、そのトラックはドラムトラックとみなされるため、最初のステップとして検出ロジックと結果をコンソールにログ出力するようにしてください。

     確認事項: `track`の定義が「;」で区切られるMML文字列のグループであることを考慮に入れてください。既存のパース・AST構築ロジックに大きな変更を加えないよう、既存のデータ構造を拡張するか、適切な場所で検出処理を行うように配慮してください。

     期待する出力: `@128`の検出ロジックが追加された`src/pass1_parser.rs`または`src/pass2_ast.rs`の具体的な変更内容。および、検出ロジックの実装方針と、簡単なテストケースで検出が機能することを示す説明をmarkdown形式で出力してください。
     ```

---
Generated at: 2026-01-21 07:06:13 JST
