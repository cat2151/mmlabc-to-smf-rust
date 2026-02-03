Last updated: 2026-02-04

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) 形式の文字列を、Standard MIDI File (SMF) へ変換するRust製のライブラリです。
- 構文解析からSMF生成までを4段階の明確なパスで処理し、高い拡張性と保守性を実現しています。
- コマンドラインツールとしてMMLの入力・再生・MIDIファイル出力機能を提供し、WebAssembly版のデモも用意されています。

## 技術スタック
- フロントエンド: 
  - HTML/JavaScript: WebAssembly版デモ (`demo/index.html`) を実行するための基本的なウェブ技術。
- 音楽・オーディオ: 
  - Standard MIDI File (SMF): 最終的な出力フォーマット。
  - Music Macro Language (MML): 入力として受け取る音楽記述言語。
  - `cat-play-mml`, TiMidity++, FluidSynth, VLC: 生成されたMIDIファイルを自動再生するための外部プレイヤー。
- 開発ツール: 
  - Rust: プロジェクトの主要なプログラミング言語 (バージョン1.70.0以上を要求)。
  - Cargo: Rustのビルドシステムおよびパッケージマネージャ。
  - tree-sitter-cli: MML言語の構文解析パーサーを生成するために使用。
  - Node.js, npx: tree-sitterパーサーの生成に必要な実行環境。
  - git: ソースコード管理システム。
- テスト: 
  - Rustのテストフレームワーク: `cargo test` を使用した包括的なユニットテストおよび統合テスト。
- ビルドツール: 
  - Cargo: Rustクレートのビルド、依存関係管理。
  - WebAssembly (wasm): `mmlabc-to-smf-wasm` サブプロジェクトでWeb向けビルドをサポート。
- 言語機能: 
  - Rustの型システムと所有権モデル: メモリ安全性を確保し、堅牢なアプリケーション開発を促進。
- 自動化・CI/CD: 
  - `cargo fmt`: コードの自動フォーマットツール。
  - `cargo clippy`: コードの品質をチェックし、一般的な間違いや非効率なコードを指摘するリンター。
- 開発標準: 
  - EditorConfig: 複数のエディタやIDE間で一貫したコーディングスタイルを維持するための設定。

## ファイル階層ツリー
```
.editorconfig
.gitignore
.vscode/
  settings.json
Cargo.lock
Cargo.toml
IMPLEMENTATION_REPORT.md
LICENSE
OPTION_A_IMPLEMENTATION.md
README.ja.md
README.md
_config.yml
build.rs
demo/
  .gitignore
  README.md
  index.html
  package.json
generated-docs/
  development-status-generated-prompt.md
googled947dc864c270e07.html
index.html
issue-notes/
  14.md
  17.md
  18.md
  19.md
  20.md
  21.md
  22.md
  23.md
  24.md
  30.md
  36.md
  37.md
  39.md
  40.md
  42.md
  44.md
  46.md
  48.md
  50.md
  52.md
  54.md
  55.md
  56.md
mmlabc-to-smf-rust.toml.example
mmlabc-to-smf-wasm/
  Cargo.lock
  Cargo.toml
  src/
    lib.rs
package.json
scripts/
  README.md
  build-demo.sh
src/
  config.rs
  lib.rs
  main.rs
  pass1_parser.rs
  pass2_ast.rs
  pass3_events.rs
  pass4_midi.rs
  tree_sitter_mml.rs
  types.rs
tests/
  integration_test.rs
  test_channel.rs
  test_chord.rs
  test_cli.rs
  test_config.rs
  test_dotted_notes.rs
  test_length.rs
  test_modifier.rs
  test_note_length.rs
  test_octave.rs
  test_pass1.rs
  test_pass2.rs
  test_pass3.rs
  test_pass4.rs
  test_program_change.rs
  test_rest.rs
  test_tempo.rs
  test_velocity.rs
tree-sitter-mml/
  grammar.js
  package.json
  src/
    grammar.json
    node-types.json
    parser.c
    tree_sitter/
      alloc.h
      array.h
      parser.h
  tree-sitter-mml.wasm
```

## ファイル詳細説明
- **.editorconfig**: 異なるエディタやIDE間で一貫したコーディングスタイル（インデントサイズ、エンコードなど）を維持するための設定ファイル。
- **.gitignore**: Gitがバージョン管理の対象としないファイルやディレクトリを指定するファイル。
- **.vscode/settings.json**: Visual Studio Codeエディタでこのプロジェクトに固有の設定を定義するファイル。
- **Cargo.lock**: Rustプロジェクトのビルド時に使用された全ての依存クレートの正確なバージョンを記録するファイル。再現性のあるビルドを保証します。
- **Cargo.toml**: Rustプロジェクトのマニフェストファイル。プロジェクト名、バージョン、著者、依存クレート、ビルド設定などが記述されます。
- **IMPLEMENTATION_REPORT.md**: プロジェクトの実装状況や設計に関する詳細なレポートを記述したMarkdownファイル。
- **LICENSE**: プロジェクトのライセンス情報（MIT License）が記載されています。
- **OPTION_A_IMPLEMENTATION.md**: 開発中に検討された代替実装案に関するドキュメント。
- **README.ja.md**: プロジェクトの日本語版説明書。
- **README.md**: プロジェクトの英語版説明書。
- **_config.yml**: 静的サイトジェネレーター（例: Jekyll）の設定ファイルとして使用され、プロジェクトのドキュメントサイトなどを構築する際に利用される可能性があります。
- **build.rs**: Rustプロジェクトのビルドスクリプト。特定の条件（例: `tree-sitter-mml/grammar.js`の変更）に基づいて`tree-sitter`パーサーファイルを自動的に再生成する役割を担います。
- **demo/.gitignore**: デモディレクトリに固有のGit無視ファイル。
- **demo/README.md**: デモアプリケーションに関する説明書。
- **demo/index.html**: WebAssembly版デモのメインとなるHTMLファイル。ユーザーインターフェースを提供し、Wasmモジュールをロードします。
- **demo/package.json**: デモプロジェクトのNode.js関連の依存関係とスクリプトを定義するファイル。
- **generated-docs/development-status-generated-prompt.md**: 自動生成された開発状況に関するドキュメント。
- **googled947dc864c270e07.html**: Googleサイト所有権確認のためのファイル。
- **index.html**: プロジェクトのウェブサイトのルートファイル。
- **issue-notes/**: 開発中に議論された特定の課題や機能に関するメモが個別のMarkdownファイルとして保存されています。
- **mmlabc-to-smf-rust.toml.example**: カスタムMIDIプレイヤー設定の例を示すTOMLファイル。
- **mmlabc-to-smf-wasm/Cargo.lock**: WebAssembly版サブプロジェクトの依存関係の正確なバージョンを記録するファイル。
- **mmlabc-to-smf-wasm/Cargo.toml**: WebAssembly版サブプロジェクトのマニフェストファイル。
- **mmlabc-to-smf-wasm/src/lib.rs**: WebAssembly版ライブラリのルートファイル。JavaScriptから呼び出されるAPIを定義します。
- **package.json**: プロジェクト全体のNode.js関連の依存関係を定義します。主に`tree-sitter-cli`などの開発ツール用。
- **scripts/README.md**: スクリプトディレクトリ内のファイルに関する説明。
- **scripts/build-demo.sh**: デモアプリケーションをビルドするためのシェルスクリプト。
- **src/config.rs**: プロジェクトの設定（例: 外部MIDIプレイヤーの指定）を読み込み、管理するためのロジックが含まれています。
- **src/lib.rs**: `mmlabc-to-smf-rust` クレートのライブラリ部分のルート。MMLからSMFへの変換ロジックの主要な公開APIを定義します。
- **src/main.rs**: コマンドラインインターフェース (CLI) のエントリーポイント。引数を解析し、ライブラリの変換機能を呼び出し、結果のMIDIファイルを処理します。
- **src/pass1_parser.rs**: MML文字列を構文解析し、トークンのストリームを生成する「パス1」のロジックを実装しています。`tree-sitter-mml`パーサーを利用します。
- **src/pass2_ast.rs**: パス1で生成されたトークンストリームから抽象構文木 (AST) を構築する「パス2」のロジックを実装しています。
- **src/pass3_events.rs**: ASTを走査し、具体的なMIDIイベント（ノートオン、ノートオフ、テンポ変更など）のシーケンスを生成する「パス3」のロジックを実装しています。
- **src/pass4_midi.rs**: パス3で生成されたMIDIイベントのシーケンスを受け取り、Standard MIDI Fileの構造として最終的に出力する「パス4」のロジックを実装しています。
- **src/tree_sitter_mml.rs**: `tree-sitter-mml`パーサーとRustコード間の統合レイヤーを提供します。パーサーの初期化やMML文字列の解析に利用されます。
- **src/types.rs**: プロジェクト全体で共有されるデータ構造や型エイリアスを定義します。MMLトークン、ASTノード、MIDIイベントの表現などが含まれます。
- **tests/integration_test.rs**: プロジェクト全体のエンドツーエンドの機能を検証する統合テスト。
- **tests/test_channel.rs**: MMLの多チャンネル機能に関するテスト。
- **tests/test_chord.rs**: 和音（コード）の変換に関するテスト。
- **tests/test_cli.rs**: コマンドラインインターフェースの動作に関するテスト。
- **tests/test_config.rs**: 設定ファイルの読み込みと適用に関するテスト。
- **tests/test_dotted_notes.rs**: 付点音符の変換に関するテスト。
- **tests/test_length.rs**: 音符の長さ指定に関するテスト。
- **tests/test_modifier.rs**: MMLの修飾子（例: 音量、音色）に関するテスト。
- **tests/test_note_length.rs**: 音符の長さ変換の特定の側面に関するテスト。
- **tests/test_octave.rs**: オクターブ指定に関するテスト。
- **tests/test_pass1.rs**: パス1 (トークン解析) の単体テスト。
- **tests/test_pass2.rs**: パス2 (AST変換) の単体テスト。
- **tests/test_pass3.rs**: パス3 (MIDIイベント生成) の単体テスト。
- **tests/test_pass4.rs**: パス4 (MIDIファイル作成) の単体テスト。
- **tests/test_program_change.rs**: プログラムチェンジ（音色変更）に関するテスト。
- **tests/test_rest.rs**: 休符の変換に関するテスト。
- **tests/test_tempo.rs**: テンポ変更に関するテスト。
- **tests/test_velocity.rs**: ベロシティ（音の強さ）に関するテスト。
- **tree-sitter-mml/grammar.js**: MML言語の構文ルールを定義するJavaScriptファイル。`tree-sitter-cli`はこのファイルからパーサーを生成します。
- **tree-sitter-mml/package.json**: `tree-sitter-mml`サブプロジェクトのNode.js関連の依存関係とスクリプトを定義するファイル。
- **tree-sitter-mml/src/grammar.json**: `grammar.js`から自動生成されたMML文法のJSON表現。
- **tree-sitter-mml/src/node-types.json**: `grammar.js`から自動生成されたASTノード型の定義。
- **tree-sitter-mml/src/parser.c**: `grammar.js`から自動生成されたC言語によるMMLパーサーのソースコード。
- **tree-sitter-mml/src/tree_sitter/**: `tree-sitter`パーサーのC言語ヘッダーファイルが含まれるディレクトリ。
- **tree-sitter-mml/tree-sitter-mml.wasm**: WebAssembly形式でコンパイルされたMMLパーサー。ウェブブラウザなどの環境で利用可能です。

## 関数詳細説明
（提供されたプロジェクト情報および標準的なRustプロジェクトの構造から推測）

- **`main::run(args: Args) -> Result<(), Box<dyn Error>>`**:
  - **役割**: コマンドラインアプリケーションの主要な実行フローを制御します。引数解析、MMLからSMFへの変換、およびオプションのMIDI再生処理を行います。
  - **引数**: `args` - コマンドライン引数を格納した構造体。
  - **戻り値**: 処理が成功した場合は空の`Result`、エラーが発生した場合はエラーオブジェクト。
  - **機能**: 入力MML文字列の取得、カスタムプレイヤー設定の読み込み、`lib::convert_mml_to_smf` の呼び出し、生成されたMIDIデータのファイル保存、そして指定された外部プレイヤーでのMIDI再生を調整します。

- **`lib::convert_mml_to_smf(mml_string: &str, config: &Config) -> Result<Vec<u8>, MmlConversionError>`**:
  - **役割**: MML文字列をStandard MIDI File形式のバイト配列に変換する、ライブラリのコア機能を提供します。4パスアーキテクチャの各ステップをオーケストレートします。
  - **引数**:
    - `mml_string` - 変換対象のMMLコード文字列。
    - `config` - 変換処理に影響を与える設定情報。
  - **戻り値**: 変換されたSMFバイト配列、または変換エラー。
  - **機能**: `pass1_parser::parse_mml` でトークン化、`pass2_ast::build_ast` でAST構築、`pass3_events::generate_midi_events` でMIDIイベント生成、最後に `pass4_midi::create_smf_file` でSMFバイト列を組み立てます。

- **`pass1_parser::parse_mml(mml_string: &str) -> Result<Vec<Token>, ParserError>`**:
  - **役割**: 入力MML文字列を`tree-sitter`パーサーを使用して構文解析し、一連のトークン（構文要素）のリストを生成します。
  - **引数**: `mml_string` - 構文解析対象のMML文字列。
  - **戻り値**: 構文解析されたトークンのリスト、またはパーサーエラー。
  - **機能**: `tree_sitter_mml::init_parser` を使用してパーサーを初期化し、MML文字列を解析して抽象的な構文要素を抽出します。

- **`pass2_ast::build_ast(tokens: Vec<Token>) -> Result<AstNode, AstBuildError>`**:
  - **役割**: パス1で生成されたトークンのリストから、MMLの構造を表す抽象構文木（AST）を構築します。
  - **引数**: `tokens` - パス1で生成されたトークンのリスト。
  - **戻り値**: 構築されたASTのルートノード、またはAST構築エラー。
  - **機能**: トークンを結合し、階層的な構造を持つASTノードを形成することで、MMLの論理的な意味を表現します。

- **`pass3_events::generate_midi_events(ast: &AstNode) -> Result<Vec<MidiEvent>, EventGenerationError>`**:
  - **役割**: 構築されたASTを走査し、音符のオン/オフ、テンポ変更、プログラムチェンジなどの具体的なMIDIイベントのシーケンスを生成します。
  - **引数**: `ast` - パス2で構築された抽象構文木。
  - **戻り値**: 生成されたMIDIイベントのリスト、またはイベント生成エラー。
  - **機能**: ASTのノードをMIDIメッセージにマッピングし、MMLの音楽的指示をMIDIフォーマットで表現可能なイベントに変換します。

- **`pass4_midi::create_smf_file(midi_events: Vec<MidiEvent>) -> Result<Vec<u8>, SmfCreationError>`**:
  - **役割**: パス3で生成されたMIDIイベントのシーケンスを受け取り、Standard MIDI File (SMF) の仕様に従ってバイナリデータを構築します。
  - **引数**: `midi_events` - パス3で生成されたMIDIイベントのリスト。
  - **戻り値**: 完成したSMFのバイナリデータ（`Vec<u8>`）、またはSMF作成エラー。
  - **機能**: MIDIイベントをMIDIトラックチャンク、ヘッダーチャンクなどのSMF構造に組み込み、ファイルとして保存できる形式にシリアライズします。

- **`config::load_config() -> Config`**:
  - **役割**: アプリケーションの設定ファイル（例: `mmlabc-to-smf-rust.toml`）を読み込み、`Config`構造体として提供します。
  - **引数**: なし。
  - **戻り値**: 読み込まれた設定情報を含む`Config`構造体。
  - **機能**: 設定ファイルが存在しない場合はデフォルト値を適用し、外部MIDIプレイヤーのパスなどの設定項目を解析します。

- **`tree_sitter_mml::init_parser() -> Parser`**:
  - **役割**: `tree-sitter`のMMLパーサーを初期化します。
  - **引数**: なし。
  - **戻り値**: 初期化された`Parser`インスタンス。
  - **機能**: `tree-sitter`ライブラリを使用して、MML言語の文法定義に基づいたパーサーをセットアップし、構文解析に使用できるように準備します。

## 関数呼び出し階層ツリー
```
main::run()
├── config::load_config()
├── lib::convert_mml_to_smf()
│   ├── pass1_parser::parse_mml()
│   │   └── tree_sitter_mml::init_parser()
│   ├── pass2_ast::build_ast()
│   ├── pass3_events::generate_midi_events()
│   └── pass4_midi::create_smf_file()
└── (External MIDI Player Command Execution - OSレベルの呼び出し)

---
Generated at: 2026-02-04 07:10:34 JST
