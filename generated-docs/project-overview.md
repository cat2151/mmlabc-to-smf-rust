Last updated: 2025-11-14

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) をStandard MIDI File (SMF) に変換するRust製CLIツールです。
- 4パスアーキテクチャを用いてMMLを効率的に解析し、MIDIイベントを生成します。
- 基本音符変換、多チャンネル対応、デバッグ出力、外部プレイヤーによる自動再生機能を備えています。

## 技術スタック
- フロントエンド: CLI (コマンドラインインターフェース) - ユーザーはターミナルを通じてMML文字列を直接入力し、変換処理を指示します。
- 音楽・オーディオ: Music Macro Language (MML) - 入力フォーマットとして使用。Standard MIDI File (SMF) - 出力フォーマットとして使用。外部MIDIプレイヤー (cat-play-mml, timidity, fluidsynth, vlc) - 生成されたMIDIファイルの自動再生に利用されます。
- 開発ツール: Rust - 高性能で安全なシステムプログラミング言語。Cargo - Rustのパッケージマネージャー兼ビルドシステム。tree-sitter - より複雑なMML構文解析のため、将来的に統合を計画しているパーサー生成ツール。
- テスト: Rustの組み込みテストフレームワーク (cargo test) - 35個の包括的なテストケースで機能の正確性を保証します。
- ビルドツール: Cargo - プロジェクトのコンパイル、依存関係の管理、テスト実行に使用されます。
- 言語機能: Rustの型システムと所有権モデル - メモリ安全性を確保し、堅牢なアプリケーション開発を可能にします。
- 自動化・CI/CD: rustfmt - コードのフォーマットを自動的に適用し、コーディングスタイルを統一します。clippy - コードの品質と一般的な間違いをチェックするLintツールです。
- 開発標準: rustfmt (コードフォーマット規則), clippy (静的解析ルール) - プロジェクト全体のコードの一貫性と品質を維持するために利用されます。

## ファイル階層ツリー
```
.
├── .editorconfig
├── .gitignore
├── .vscode/
│   └── settings.json
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.ja.md
├── README.md
├── _config.yml
├── build.rs
├── generated-docs/
├── issue-notes/
│   └── 14.md
├── mmlabc-to-smf-rust.toml.example
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
│   ├── test_cli.rs
│   ├── test_config.rs
│   ├── test_pass1.rs
│   ├── test_pass2.rs
│   ├── test_pass3.rs
│   └── test_pass4.rs
└── tree-sitter-mml/
    ├── grammar.js
    ├── package.json
    └── src/
        ├── grammar.json
        ├── node-types.json
        ├── parser.c
        └── tree_sitter/
            ├── alloc.h
            ├── array.h
            └── parser.h
```

## ファイル詳細説明
- **`.editorconfig`**: 各エディタでコードスタイル（インデント、改行コードなど）を統一するための設定ファイルです。
- **`.gitignore`**: Gitがバージョン管理の対象としないファイルやディレクトリ（例: ビルド成果物、一時ファイル）を指定します。
- **`.vscode/settings.json`**: Visual Studio Codeのワークスペース固有の設定を定義し、プロジェクトチーム間での開発環境の一貫性を保ちます。
- **`Cargo.lock`**: `Cargo.toml`で指定された依存関係に基づいて、ビルド時に実際に使用されたクレートとその正確なバージョンをロックするファイルです。
- **`Cargo.toml`**: Rustプロジェクトの構成ファイル。プロジェクトのメタデータ、ビルド設定、外部依存クレートを定義します。
- **`LICENSE`**: プロジェクトがMIT Licenseで配布されることを示し、利用条件を定めます。
- **`README.ja.md`**: プロジェクトの目的、機能、使用方法、開発方法などを日本語で説明する主要なドキュメントです。
- **`README.md`**: プロジェクトの目的、機能、使用方法、開発方法などを英語で説明する主要なドキュメントです。
- **`_config.yml`**: GitHub Pagesなど、特定のツールやプラットフォームの設定を定義するためのファイル（プロジェクト情報に詳細な言及がないため一般的な説明）。
- **`build.rs`**: カスタムビルドロジックを含むRustスクリプト。特定の依存関係を処理したり、コードを生成したりするためにビルドプロセス中に実行されます。
- **`generated-docs/`**: 自動生成されたドキュメントやレポート、その他の出力ファイルを格納するために用意されたディレクトリです。
- **`issue-notes/14.md`**: 開発プロセスにおける特定の課題（Issue #14）に関するメモや詳細情報が記述されたファイル（来訪者向けには具体的な内容は省略）。
- **`mmlabc-to-smf-rust.toml.example`**: カスタムMIDIプレイヤーの設定方法を示すための設定ファイル例です。
- **`src/config.rs`**: アプリケーションの設定（例: 外部MIDIプレイヤーのコマンド名）を読み込み、管理するためのモジュールです。
- **`src/lib.rs`**: プロジェクトのライブラリクレートのエントリーポイント。他のモジュールを公開し、ライブラリ全体のインターフェースを定義します。
- **`src/main.rs`**: CLIアプリケーションのメインエントリーポイント。コマンドライン引数の解析とMML変換の主要な処理フローを制御します。
- **`src/pass1_parser.rs`**: MML文字列を解析し、基本的な音符やチャンネル区切りなどの「トークン」に分解する、変換プロセスの最初のパスを実装します。
- **`src/pass2_ast.rs`**: パス1で生成されたトークンリストから、MMLの構造を階層的に表現する抽象構文木（AST）を構築するモジュールです。
- **`src/pass3_events.rs`**: パス2で構築されたASTを解釈し、Standard MIDI Fileで表現されるMIDIイベント（ノートオン、ノートオフなど）のリストを生成するモジュールです。
- **`src/pass4_midi.rs`**: パス3で生成されたMIDIイベントのリストを基に、最終的なStandard MIDI Fileフォーマットのバイトストリームを組み立て、ファイルとして出力するモジュールです。
- **`src/tree_sitter_mml.rs`**: より高度なMML構文解析のため、tree-sitterパーサーをMML変換ワークフローに統合するためのモジュールです（現在開発中）。
- **`src/types.rs`**: プロジェクト全体で利用される共通のデータ構造、列挙型、トレイトなどを定義し、型の一貫性と再利用性を促進します。
- **`tests/integration_test.rs`**: プロジェクトの各パスや機能が連携して正しく動作するかを検証する統合テストを含みます。
- **`tests/test_channel.rs`**: MMLの多チャンネル機能（セミコロン`;`によるチャンネル分離）が正しく処理されることを確認するためのテストです。
- **`tests/test_cli.rs`**: コマンドラインインターフェースの引数解析、オプション処理、出力などが正しく行われることを確認するテストです。
- **`tests/test_config.rs`**: 設定ファイル（`mmlabc-to-smf-rust.toml`）の読み込み、解析、アプリケーションへの適用が正しく機能することを確認するテストです。
- **`tests/test_pass1.rs`**: パス1（MML文字列のトークン化）の単体テストを定義し、トークン生成の正確性を検証します。
- **`tests/test_pass2.rs`**: パス2（トークンからASTへの変換）の単体テストを定義し、AST構築の論理的な正確性を検証します。
- **`tests/test_pass3.rs`**: パス3（ASTからMIDIイベント生成）の単体テストを定義し、MIDIイベント生成の正確性を検証します。
- **`tests/test_pass4.rs`**: パス4（MIDIイベントからStandard MIDI File作成）の単体テストを定義し、最終的なMIDIファイル出力の正確性を検証します。
- **`tree-sitter-mml/grammar.js`**: tree-sitterがMML言語を解析するための文法ルールを定義したJavaScriptファイルです。
- **`tree-sitter-mml/package.json`**: `tree-sitter-mml`パーサーモジュールのメタデータと依存関係を定義します。
- **`tree-sitter-mml/src/grammar.json`**: `grammar.js`から自動生成された、JSON形式の文法定義ファイルです。
- **`tree-sitter-mml/src/node-types.json`**: tree-sitterパーサーが生成する抽象構文木（AST）のノード型に関する情報を提供します。
- **`tree-sitter-mml/src/parser.c`**: tree-sitterによって生成された、MMLパーサーのC言語実装のソースコードです。
- **`tree-sitter-mml/src/tree_sitter/alloc.h`**: tree-sitterパーサーが内部で使用するメモリ割り当て関数に関するヘッダーファイルです。
- **`tree-sitter-mml/src/tree_sitter/array.h`**: tree-sitterパーサーが内部で使用する動的配列構造に関するヘッダーファイルです。
- **`tree-sitter-mml/src/tree_sitter/parser.h`**: tree-sitterパーサーの公開インターフェースや主要な定義を含むヘッダーファイルです。

## 関数詳細説明
- **`main()`** (src/main.rs)
    - 役割: CLIアプリケーションの開始点。コマンドライン引数の解析、MMLからSMFへの変換処理の管理、結果の出力とオプションでのMIDI再生を実行します。
    - 引数: なし（内部でコマンドライン引数を処理）。
    - 戻り値: `Result<(), Box<dyn Error>>`。成功時にはユニット型 `()`、エラー発生時には動的エラーオブジェクトを返します。
    - 機能: コマンドライン引数をパースし、MML文字列と各種オプション（出力パス、再生の有無など）を取得します。その後、`pass1_parser::parse`から`pass4_midi::create_smf`までの4パスの変換処理を順に呼び出します。各パスの中間結果をJSONファイルとして保存するデバッグ機能も持ちます。変換が完了すると、設定に応じて生成されたMIDIファイルを外部プレイヤーで再生します。

- **`pass1_parser::parse(mml_string: &str) -> Result<Vec<Token>, ParserError>`** (src/pass1_parser.rs)
    - 役割: 入力されたMML文字列を、処理しやすい最小単位である「トークン」のベクターに分解（字句解析）します。
    - 引数: `mml_string` (`&str`) - MML形式の入力文字列。
    - 戻り値: `Result<Vec<Token>, ParserError>`。成功時には`Token`のベクター、エラー発生時には`ParserError`を返します。
    - 機能: MML文字列を一文字ずつ走査し、認識された音符（`c`, `d`など）やチャンネル区切り（`;`）を対応する`Token`型に変換します。未定義の記号や不正な形式を検出した場合はエラーを報告します。

- **`pass2_ast::build_ast(tokens: Vec<Token>) -> Result<AstNode, AstBuilderError>`** (src/pass2_ast.rs)
    - 役割: パス1で生成されたトークンのベクターを基に、MMLの構文構造を表現する抽象構文木（AST）を構築します。
    - 引数: `tokens` (`Vec<Token>`) - パス1で生成されたトークンのベクター。
    - 戻り値: `Result<AstNode, AstBuilderError>`。成功時にはASTのルートノード、エラー発生時には`AstBuilderError`を返します。
    - 機能: トークン列の並び順から文法的な意味を解釈し、MMLの要素（例: 各チャンネルの音符列）を階層的なASTノードとして表現します。これにより、後続のパスで音楽的な意味を容易に抽出できるようになります。

- **`pass3_events::generate_midi_events(ast: AstNode) -> Result<Vec<MidiEvent>, EventGeneratorError>`** (src/pass3_events.rs)
    - 役割: パス2で構築された抽象構文木（AST）を、Standard MIDI Fileのフォーマットに沿ったMIDIイベントのベクターに変換します。
    - 引数: `ast` (`AstNode`) - パス2で生成されたASTのルートノード。
    - 戻り値: `Result<Vec<MidiEvent>, EventGeneratorError>`。成功時には`MidiEvent`のベクター、エラー発生時には`EventGeneratorError`を返します。
    - 機能: ASTを深さ優先または幅優先でトラバースし、各MML要素（音符、休符、チャンネルなど）に対応するMIDIメッセージ（ノートオン、ノートオフ、テンポ変更など）を生成します。各イベントに正確なタイムスタンプとチャンネル番号を割り当て、音楽的な順序を保持します。

- **`pass4_midi::create_smf(midi_events: Vec<MidiEvent>, output_path: &Path) -> Result<(), MidiWriterError>`** (src/pass4_midi.rs)
    - 役割: パス3で生成されたMIDIイベントのベクターから、最終的なStandard MIDI File (SMF) を作成し、指定されたファイルパスに保存します。
    - 引数: `midi_events` (`Vec<MidiEvent>`) - パス3で生成されたMIDIイベントのベクター。`output_path` (`&Path`) - 出力するMIDIファイルのパス。
    - 戻り値: `Result<(), MidiWriterError>`。成功時にはユニット型 `()`、エラー発生時には`MidiWriterError`を返します。
    - 機能: MIDIイベントをSMFフォーマットのバイトデータに変換します。MIDIファイルのヘッダーチャンク（ファイル形式、トラック数、タイムベースなど）と、イベントデータを格納するトラックチャンクを構築します。構築されたMIDIデータを指定されたファイルに書き込みます。

- **`config::load_config(path: Option<&Path>) -> AppConfig`** (src/config.rs)
    - 役割: アプリケーションの設定ファイル（`mmlabc-to-smf-rust.toml`）を読み込み、アプリケーション全体で利用可能な設定オブジェクトを返します。
    - 引数: `path` (`Option<&Path>`) - 設定ファイルへの任意のパス。`None`の場合、デフォルトの場所から読み込みます。
    - 戻り値: `AppConfig` - 読み込まれた設定を保持する構造体。
    - 機能: 指定されたパス、または実行ディレクトリ内のデフォルトパスから`mmlabc-to-smf-rust.toml`ファイルを検索し、外部MIDIプレイヤーのコマンド名などの設定項目を解析して`AppConfig`構造体に格納します。ファイルが存在しない場合や解析エラーの場合は、デフォルト設定を適用します。

- **`main::play_midi_file(file_path: &Path, player: &str)`** (src/main.rs)
    - 役割: 生成されたStandard MIDI Fileを、ユーザーが設定した外部MIDIプレイヤーで自動的に再生します。
    - 引数: `file_path` (`&Path`) - 再生するMIDIファイルへのパス。`player` (`&str`) - 使用するMIDIプレイヤーのコマンド名（例: "timidity", "cat-play-mml"）。
    - 戻り値: なし（実際にはコマンド実行結果を返す可能性もある）。
    - 機能: 設定されたMIDIプレイヤーのコマンドとMIDIファイルパスを引数として、オペレーティングシステム上で子プロセスとして実行します。これにより、MML変換後すぐに結果の音を確認できます。指定されたプレイヤーが見つからない場合は、警告メッセージを出力します。

## 関数呼び出し階層ツリー
```
main()
├── config::load_config(path)
├── pass1_parser::parse(mml_string)
├── pass2_ast::build_ast(tokens)
├── pass3_events::generate_midi_events(ast)
├── pass4_midi::create_smf(midi_events, output_path)
└── main::play_midi_file(file_path, player) (条件付き実行)

---
Generated at: 2025-11-14 07:06:45 JST
