Last updated: 2026-02-09

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) 形式の楽譜データをStandard MIDI File (SMF) に変換するRust製ライブラリです。
- 独自の4パスアーキテクチャに基づき、MMLの解析からMIDIイベント生成、最終的なSMF作成までを段階的に実行します。
- コマンドラインツールとしてMMLの変換・再生に利用できるほか、WASMライブラリとしてブラウザアプリケーションへの組み込みも可能です。

## 技術スタック
- フロントエンド: WebAssembly (WASM) (ブラウザ向けライブラリとしての利用想定), JavaScript (demoアプリケーションでの利用)
- 音楽・オーディオ: Music Macro Language (MML), Standard MIDI File (SMF), MIDIイベント, `cat-play-mml`, `timidity`, `fluidsynth`, `vlc` (MIDIプレイヤー設定)
- 開発ツール: Rust (プログラミング言語), Cargo (Rustのビルドシステム兼パッケージマネージャー), tree-sitter (構文解析のためのパーサー生成ツール), Node.js (tree-sitterパーサー生成に必要)
- テスト: Rustのテストフレームワーク (35個のテストケースが実装済み)
- ビルドツール: Cargo (Rustプロジェクトのビルド管理), tree-sitter-cli (tree-sitterパーサーの生成・更新)
- 言語機能: Rustの型システム, 所有権モデル (メモリ安全性と堅牢な設計のため)
- 自動化・CI/CD: ビルドスクリプト (`build.rs`, `scripts/`) と連携し、パーサーファイルの自動生成などを行うことでCI/CDを簡素化。
- 開発標準: `.editorconfig` (コードスタイルの一貫性), `cargo clippy` (Linterによるコード品質チェック), `cargo fmt` (Formatterによるコード整形)

## ファイル階層ツリー
```
.
├── .editorconfig
├── .gitignore
├── .vscode/
│   └── settings.json
├── Cargo.lock
├── Cargo.toml
├── IMPLEMENTATION_REPORT.md
├── LICENSE
├── OPTION_A_IMPLEMENTATION.md
├── README.ja.md
├── README.md
├── _config.yml
├── build.rs
├── demo/
│   ├── .gitignore
│   ├── FEATURES.md
│   ├── README.md
│   ├── index.html
│   └── package.json
├── demo-library/
│   ├── index.html
│   └── package.json
├── generated-docs/
├── googled947dc864c270e07.html
├── mmlabc-to-smf-rust.toml.example
├── mmlabc-to-smf-wasm/
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── package.json
├── scripts/
│   ├── README.md
│   ├── build-demo.sh
│   └── transform-demo-paths.sh
├── src/
│   ├── config.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── pass1_parser.rs
│   ├── pass2_ast.rs
│   ├── pass3_events.rs
│   ├── pass4_midi.rs
│   ├── tree_sitter_mml.rs
│   └── types.rs
├── tests/
│   ├── integration_test.rs
│   ├── test_channel.rs
│   ├── test_chord.rs
│   ├── test_cli.rs
│   ├── test_config.rs
│   ├── test_dotted_notes.rs
│   ├── test_drum_channel.rs
│   ├── test_key_transpose.rs
│   ├── test_length.rs
│   ├── test_modifier.rs
│   ├── test_note_length.rs
│   ├── test_octave.rs
│   ├── test_pass1.rs
│   ├── test_pass2.rs
│   ├── test_pass3.rs
│   ├── test_pass4.rs
│   ├── test_program_change.rs
│   ├── test_rest.rs
│   ├── test_tempo.rs
│   └── test_velocity.rs
└── tree-sitter-mml/
    ├── grammar.js
    ├── package.json
    ├── src/
    │   ├── grammar.json
    │   ├── node-types.json
    │   ├── parser.c
    │   └── tree_sitter/
    │       ├── alloc.h
    │       ├── array.h
    │       └── parser.h
    └── tree-sitter-mml.wasm
```

## ファイル詳細説明
- **`.editorconfig`**: さまざまなエディタやIDE間でコードのフォーマットを統一するための設定ファイル。
- **`.gitignore`**: Gitが追跡しないファイルやディレクトリを指定するファイル。
- **`.vscode/settings.json`**: Visual Studio Codeのワークスペース設定ファイル。
- **`Cargo.lock`**: Rustプロジェクトのビルド時に、依存関係の正確なバージョンを固定するためにCargoが生成するファイル。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイル。プロジェクト名、バージョン、依存関係、ビルド設定などを定義。
- **`IMPLEMENTATION_REPORT.md`**: 実装に関するレポートやメモを記述するMarkdownファイル。
- **`LICENSE`**: プロジェクトのライセンス情報（MIT License）が記述されたファイル。
- **`OPTION_A_IMPLEMENTATION.md`**: 別の実装オプションに関するドキュメント。
- **`README.ja.md`**: プロジェクトの概要、使い方、開発情報などを日本語で記述したドキュメント。
- **`README.md`**: プロジェクトの概要、使い方、開発情報などを英語で記述したドキュメント。
- **`_config.yml`**: Jekyllなどの静的サイトジェネレーターの設定ファイルである可能性があり、ドキュメント生成に関連するかもしれない。
- **`build.rs`**: Cargoのカスタムビルドスクリプト。tree-sitterパーサーファイルの自動生成など、ビルド時の特殊な処理を記述。
- **`demo/`**: Webブラウザで動作するデモンストレーション関連のファイル群。
    - **`demo/index.html`**: デモアプリケーションのHTMLエントリーポイント。
    - **`demo/package.json`**: デモのJavaScript依存関係を管理するファイル。
- **`demo-library/`**: Webブラウザで動作するライブラリデモ関連のファイル群。
    - **`demo-library/index.html`**: ライブラリのWebAssembly版のデモを示すHTMLファイル。
- **`generated-docs/`**: 自動生成されたドキュメントが格納されるディレクトリ。
- **`googled947dc864c270e07.html`**: Googleサイト認証用のファイル。
- **`mmlabc-to-smf-rust.toml.example`**: 外部MIDIプレイヤーのカスタム設定例を示すTOMLファイル。
- **`mmlabc-to-smf-wasm/`**: WebAssembly (WASM) 版ライブラリのクレート。
    - **`mmlabc-to-smf-wasm/src/lib.rs`**: WASMターゲット向けライブラリのエントリーポイント。
- **`package.json`**: ルートディレクトリのNode.jsプロジェクトの依存関係を管理するファイル。主にtree-sitter-cliに関連。
- **`scripts/`**: ビルドやデモ関連のスクリプトを格納するディレクトリ。
    - **`scripts/build-demo.sh`**: デモをビルドするためのシェルスクリプト。
    - **`scripts/transform-demo-paths.sh`**: デモのパスを変換するためのシェルスクリプト。
- **`src/`**: Rustのソースコードを格納する主要なディレクトリ。
    - **`src/config.rs`**: アプリケーション設定（例：外部MIDIプレイヤー）を処理するモジュール。
    - **`src/lib.rs`**: ライブラリのメインエントリーポイント。MML-SMF変換のパブリックAPIを提供。
    - **`src/main.rs`**: コマンドラインインターフェース (CLI) のエントリーポイント。コマンド引数を解析し、変換処理を呼び出す。
    - **`src/pass1_parser.rs`**: MML文字列をトークンに解析する最初のパスを実装するモジュール。tree-sitterパーサーを利用。
    - **`src/pass2_ast.rs`**: トークン列を抽象構文木 (AST) に変換する第2パスを実装するモジュール。
    - **`src/pass3_events.rs`**: ASTからMIDIイベントのリストを生成する第3パスを実装するモジュール。
    - **`src/pass4_midi.rs`**: MIDIイベントのリストから最終的なStandard MIDI File (SMF) を作成する第4パスを実装するモジュール。
    - **`src/tree_sitter_mml.rs`**: `tree-sitter-mml`パーサーをRustプロジェクトに統合するためのコード。
    - **`src/types.rs`**: プロジェクト全体で共有されるカスタムデータ構造や列挙型などの型定義。
- **`tests/`**: ユニットテストと統合テストのソースコードを格納するディレクトリ。
    - **`tests/integration_test.rs`**: プロジェクト全体の統合テスト。
    - **`tests/test_channel.rs`**: MMLのチャンネル機能に関するテスト。
    - **`tests/test_chord.rs`**: MMLの和音機能に関するテスト。
    - **`tests/test_cli.rs`**: コマンドラインインターフェースの動作に関するテスト。
    - **`tests/test_config.rs`**: 設定ファイルの読み込みと適用に関するテスト。
    - **`tests/test_dotted_notes.rs`**: 付点音符の処理に関するテスト。
    - **`tests/test_drum_channel.rs`**: ドラムチャンネルの処理に関するテスト。
    - **`tests/test_key_transpose.rs`**: キー（調）の移調に関するテスト。
    - **`tests/test_length.rs`**: 音長指定に関するテスト。
    - **`tests/test_modifier.rs`**: MML修飾子に関するテスト。
    - **`tests/test_note_length.rs`**: 音符の長さに関するテスト。
    - **`tests/test_octave.rs`**: オクターブ指定に関するテスト。
    - **`tests/test_pass1.rs`**: パス1 (トークン解析) のユニットテスト。
    - **`tests/test_pass2.rs`**: パス2 (AST変換) のユニットテスト。
    - **`tests/test_pass3.rs`**: パス3 (MIDIイベント生成) のユニットテスト。
    - **`tests/test_pass4.rs`**: パス4 (MIDIファイル作成) のユニットテスト。
    - **`tests/test_program_change.rs`**: プログラムチェンジ（音色変更）に関するテスト。
    - **`tests/test_rest.rs`**: 休符の処理に関するテスト。
    - **`tests/test_tempo.rs`**: テンポ変更に関するテスト。
    - **`tests/test_velocity.rs`**: ベロシティ（音量）に関するテスト。
- **`tree-sitter-mml/`**: MML用のtree-sitterパーサーの定義と生成されたファイル群。
    - **`tree-sitter-mml/grammar.js`**: MML言語のtree-sitter文法を定義するJavaScriptファイル。
    - **`tree-sitter-mml/package.json`**: tree-sitterパーサーのビルドに必要なNode.jsパッケージを管理。
    - **`tree-sitter-mml/src/`**: tree-sitterによって生成されたC言語のパーサーソースコードやJSON定義。
    - **`tree-sitter-mml/tree-sitter-mml.wasm`**: WebAssemblyにコンパイルされたMMLパーサー。

## 関数詳細説明
現状では、個々の関数の詳細な役割、引数、戻り値に関する具体的な情報はありません。ただし、プロジェクトの4パスアーキテクチャに基づき、各パスのモジュール（`src/pass1_parser.rs`、`src/pass2_ast.rs`、`src/pass3_events.rs`、`src/pass4_midi.rs`など）内にそれぞれの処理段階を担う関数が実装されています。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした

---
Generated at: 2026-02-09 07:08:05 JST
