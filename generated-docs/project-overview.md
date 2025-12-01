Last updated: 2025-12-02

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) 形式の文字列をStandard MIDI File (SMF) へ変換するRustライブラリです。
- 4パスアーキテクチャを採用し、MMLの解析、抽象構文木の構築、MIDIイベント生成を経てSMFを作成します。
- コマンドラインインターフェース (CLI) を提供し、MMLによる楽曲を簡単に作成・再生できます。

## 技術スタック
- フロントエンド: CLI (コマンドラインインターフェース) - ユーザーはコマンドラインを通じてMML文字列を入力し、変換処理を指示します。
- 音楽・オーディオ: Standard MIDI File (SMF) 形式の生成。生成されたMIDIファイルは、設定された外部MIDIプレイヤー (`cat-play-mml`、`timidity` など) で自動再生可能です。
- 開発ツール:
    - Cargo: Rustプロジェクトのビルド、依存関係管理、テスト実行を統合するツールです。
    - npx: Node.jsパッケージランナー。tree-sitter文法定義ファイル (`grammar.js`) からパーサーコードを生成する際に利用されます。
- テスト: Cargo test - Rust標準のテストフレームワークを利用し、各機能（パス、チャンネル、CLI操作など）に対して包括的なユニットテストおよび統合テストが実装されています。
- ビルドツール:
    - Rustc: Rust言語のコンパイラ。ソースコードをネイティブ実行ファイルにコンパイルします。
    - tree-sitter-cli: MML文法定義 (`grammar.js`) からC言語のパーサーコードを自動生成するためのツールです。
- 言語機能: Rust - メモリ安全性、パフォーマンス、堅牢な型システムを特徴とし、安全で高速なコード開発を可能にします。
- 自動化・CI/CD:
    - cargo fmt: Rustコードの自動フォーマットツールで、コードスタイルの一貫性を保ちます。
    - cargo clippy: Rustコードの静的解析ツールで、潜在的なバグや非効率なコードパターンを検出します。
- 開発標準:
    - .editorconfig: 複数の開発者が異なるエディタを使用しても、コードのインデントや文字コードなどのスタイル設定を統一するためのファイルです。
    - .gitignore: Gitバージョン管理システムが追跡しないファイルやディレクトリを指定するためのファイルです。

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
├── googled947dc864c270e07.html
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
│   ├── test_chord.rs
│   ├── test_cli.rs
│   ├── test_config.rs
│   ├── test_dotted_notes.rs
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
- **`.editorconfig`**: さまざまなエディタやIDE間でコーディングスタイルの一貫性を保つための設定ファイルです。
- **`.gitignore`**: Gitがバージョン管理の対象から除外するファイルやディレクトリを指定します。
- **`.vscode/settings.json`**: Visual Studio Codeエディタのワークスペース固有の設定を定義します。
- **`Cargo.lock`**: `Cargo.toml` に基づいて解決された、すべての依存関係の正確なバージョンを記録します。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイルで、プロジェクトのメタデータ、依存関係、ビルド設定などが記述されています。
- **`LICENSE`**: プロジェクトのライセンス情報（MIT License）が記載されています。
- **`README.ja.md`**, **`README.md`**: プロジェクトの概要、機能、使い方、開発方法などを説明するドキュメントファイル（日本語版と英語版）。
- **`_config.yml`**: 通常、Jekyllなどの静的サイトジェネレーターで設定情報として使われるファイルですが、このプロジェクトでの具体的な用途は不明です。
- **`build.rs`**: カスタムビルドロジックを記述するためのRustスクリプトです。tree-sitterパーサーファイルの自動生成などを担います。
- **`generated-docs/`**: ドキュメント生成ツールによって生成されたドキュメントを格納するディレクトリです（現在は空の可能性があります）。
- **`googled947dc864c270e07.html`**: Googleサービスによるサイト認証などに使用される可能性のあるファイルです。
- **`mmlabc-to-smf-rust.toml.example`**: 外部MIDIプレイヤーの設定例が記述されたファイルで、ユーザーが設定をコピーして利用できます。
- **`src/config.rs`**: アプリケーションの実行時設定（例: 外部MIDIプレイヤーの指定、出力パスなど）を読み込み、管理するロジックが含まれます。
- **`src/lib.rs`**: ライブラリのルートファイルであり、MMLからSMFへの変換を行う主要な関数やモジュールを公開します。
- **`src/main.rs`**: コマンドラインインターフェース (CLI) のエントリーポイントです。コマンドライン引数をパースし、ライブラリの機能 (`lib.rs`) を呼び出してMML変換処理を実行します。
- **`src/pass1_parser.rs`**: MML文字列を最小単位のトークンに分解する「パス1」のロジックを実装しています。
- **`src/pass2_ast.rs`**: トークンのリストを受け取り、プログラムの構造を表す抽象構文木 (AST) を構築する「パス2」のロジックを実装しています。
- **`src/pass3_events.rs`**: ASTを走査し、MIDIイベント（音符のオン/オフ、テンポ変更など）のシーケンスを生成する「パス3」のロジックを実装しています。
- **`src/pass4_midi.rs`**: 生成されたMIDIイベントのシーケンスから、最終的なStandard MIDI File (.mid) 形式のデータを構築し、ファイルとして出力する「パス4」のロジックを実装しています。
- **`src/tree_sitter_mml.rs`**: tree-sitter製のMMLパーサーとRustコードを統合するためのインターフェースやヘルパー関数が含まれます。
- **`src/types.rs`**: プロジェクト全体で共通して使用されるカスタムデータ型、列挙型、構造体などを定義しています。
- **`tests/integration_test.rs`**: ライブラリ全体の結合テストや、複数のモジュールが連携するテストを記述します。
- **`tests/test_channel.rs`**: MMLの多チャンネル機能 (`c;e;g` など) の正しい動作を検証するテストです。
- **`tests/test_chord.rs`**: 和音機能の動作を検証するテストです。
- **`tests/test_cli.rs`**: コマンドラインインターフェースの引数処理や、様々なオプションが正しく機能するかを検証するテストです。
- **`tests/test_config.rs`**: 設定ファイル (`mmlabc-to-smf-rust.toml`) の読み込みと適用に関するテストです。
- **`tests/test_dotted_notes.rs`**: 付点音符の処理が正しく行われるかを検証するテストです。
- **`tests/test_length.rs`**: 音長指定 (`l4`, `l8` など) の動作を検証するテストです。
- **`tests/test_modifier.rs`**: 音符の修飾子 (シャープ、フラットなど) の動作を検証するテストです。
- **`tests/test_note_length.rs`**: 個々の音符の長さを指定する機能のテストです。
- **`tests/test_octave.rs`**: オクターブ変更コマンド (`>`, `<`, `o` など) の動作を検証するテストです。
- **`tests/test_pass1.rs`**: パス1 (トークン解析) の単体テストです。
- **`tests/test_pass2.rs`**: パス2 (AST構築) の単体テストです。
- **`tests/test_pass3.rs`**: パス3 (MIDIイベント生成) の単体テストです。
- **`tests/test_pass4.rs`**: パス4 (SMFファイル作成) の単体テストです。
- **`tests/test_program_change.rs`**: プログラムチェンジ（音色変更）コマンドの動作を検証するテストです。
- **`tests/test_rest.rs`**: 休符 (`r` など) の動作を検証するテストです。
- **`tests/test_tempo.rs`**: テンポ変更コマンド (`t` など) の動作を検証するテストです。
- **`tests/test_velocity.rs`**: 音量（ベロシティ）変更コマンドの動作を検証するテストです。
- **`tree-sitter-mml/grammar.js`**: tree-sitterツールキットでMML言語の文法を定義するJavaScriptファイルです。
- **`tree-sitter-mml/package.json`**: tree-sitter-mmlパーサーに関連するNode.jsパッケージのメタデータと依存関係を定義します。
- **`tree-sitter-mml/src/grammar.json`**: `grammar.js` から生成されるJSON形式の文法定義ファイルです。
- **`tree-sitter-mml/src/node-types.json`**: `grammar.js` から生成される、ASTノードの型情報を定義するファイルです。
- **`tree-sitter-mml/src/parser.c`**: `grammar.js` から生成される、MML文字列を解析するためのC言語のパーサー実装です。
- **`tree-sitter-mml/src/tree_sitter/alloc.h`**, **`tree-sitter-mml/src/tree_sitter/array.h`**, **`tree-sitter-mml/src/tree_sitter/parser.h`**: tree-sitterパーサーが内部的に使用するC言語のヘッダーファイル群です。

## 関数詳細説明
具体的な関数シグネチャが提供されていないため、主要な処理フローにおける概念的な関数について説明します。

- **`parse_mml_input(mml_string: &str) -> Result<Vec<Token>, Error>`**:
    - **役割**: 入力されたMML文字列を解析し、プログラムが処理できる最小単位の要素（トークン）のリストに変換します。
    - **引数**: `mml_string` (MML形式の音楽データ文字列)。
    - **戻り値**: 成功した場合はトークンのリスト、失敗した場合はエラー。
    - **機能**: 文字列を走査し、音符、コマンド、チャンネル分離子などを識別して構造化されたデータに変換します。

- **`build_abstract_syntax_tree(tokens: Vec<Token>) -> Result<AstNode, Error>`**:
    - **役割**: パス1で生成されたトークンのリストから、MMLコードの論理構造を表す抽象構文木 (AST) を構築します。
    - **引数**: `tokens` (パス1で生成されたトークンのリスト)。
    - **戻り値**: 成功した場合はASTのルートノード、失敗した場合はエラー。
    - **機能**: トークン間の関係性を解釈し、音符シーケンス、チャンネルグループ、コマンドブロックなどを階層的に表現します。

- **`generate_midi_events_from_ast(ast: AstNode) -> Result<Vec<MidiEvent>, Error>`**:
    - **役割**: ASTを走査し、Standard MIDI Fileの仕様に準拠したMIDIイベントのシーケンスを生成します。
    - **引数**: `ast` (パス2で構築された抽象構文木)。
    - **戻り値**: 成功した場合はMIDIイベントのリスト、失敗した場合はエラー。
    - **機能**: 各ASTノードをMIDI音符オン/オフ、テンポ変更、プログラムチェンジなどの具体的なMIDIメッセージに変換します。

- **`create_smf_output(events: Vec<MidiEvent>, output_path: Option<&str>) -> Result<(), Error>`**:
    - **役割**: 生成されたMIDIイベントのシーケンスを受け取り、実際にStandard MIDI File (.mid) を作成して指定されたファイルパスに保存します。
    - **引数**: `events` (パス3で生成されたMIDIイベントのリスト)、`output_path` (出力ファイルパス、任意)。
    - **戻り値**: 成功した場合はUnit型 (`()`)、失敗した場合はエラー。
    - **機能**: MIDIイベントをSMFヘッダー、トラックデータなどを含むバイナリ形式にエンコードし、ファイルシステムに書き込みます。

- **`handle_cli_commands()`**:
    - **役割**: コマンドライン引数をパースし、MML文字列の変換、出力ファイルの指定、自動再生の制御など、CLI全体の処理をオーケストレートします。
    - **引数**: なし（コマンドライン引数は環境から取得）。
    - **戻り値**: なし（直接結果を出力するか、エラーを報告）。
    - **機能**: `parse_mml_input` から `create_smf_output` までの各パスを順に呼び出し、必要に応じてデバッグ情報をJSON形式で出力したり、外部プレイヤーでMIDIファイルを再生したりします。

- **`load_application_config() -> Config`**:
    - **役割**: アプリケーションの実行時設定（例: デフォルトの外部MIDIプレイヤー）を、設定ファイル (`mmlabc-to-smf-rust.toml`) から読み込みます。
    - **引数**: なし。
    - **戻り値**: 設定情報を含む`Config`構造体。
    - **機能**: 設定ファイルを解析し、アプリケーションの動作に影響を与えるパラメータをロードします。

- **`play_generated_midi(file_path: &str, player_command: &str)`**:
    - **役割**: 生成されたMIDIファイルを、設定された外部MIDIプレイヤーコマンドを使って再生します。
    - **引数**: `file_path` (再生するMIDIファイルのパス)、`player_command` (使用するMIDIプレイヤーのコマンド名)。
    - **戻り値**: なし（再生の成否は内部で処理）。
    - **機能**: 外部プロセスとしてMIDIプレイヤーを起動し、指定されたMIDIファイルを渡します。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした。

---
Generated at: 2025-12-02 07:05:19 JST
