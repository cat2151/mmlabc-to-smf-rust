Last updated: 2025-11-28

# Project Overview

## プロジェクト概要
- Music Macro Language（MML）形式の楽譜テキストを、Standard MIDI File（SMF）形式の音楽ファイルに変換するライブラリです。
- Rust言語で開発されており、安全な設計とパフォーマンスを重視した4パスアーキテクチャを採用しています。
- MMLの解析からMIDIイベント生成、最終的なSMF作成までを一貫して処理し、多チャンネル対応やデバッグ出力機能を備えています。

## 技術スタック
- フロントエンド: CLI (コマンドラインインターフェース) - ユーザーがMML文字列を直接入力し、変換処理を指示するためのインターフェースを提供します。
- 音楽・オーディオ:
    - Music Macro Language (MML): 入力として受け付ける音楽記述言語。
    - Standard MIDI File (SMF): 変換後の出力形式となる標準的な音楽ファイルフォーマット。
    - `cat-play-mml`, `timidity`, `fluidsynth`, `vlc`: 生成されたMIDIファイルを再生するための外部オーディオプレイヤー。
- 開発ツール:
    - Rust: プロジェクトの主要な開発言語。メモリ安全性とパフォーマンスに優れています。
    - `tree-sitter-cli`: より高度なMML構文解析のために、tree-sitterパーサーを生成する際に使用されるCLIツール。
    - Node.js, `npx`: `tree-sitter`パーサー生成に必要なJavaScript環境とパッケージランナー。
    - .editorconfig: 開発者間でコーディングスタイルを統一するための設定ファイル。
    - .vscode/settings.json: VS Code利用者向けのIDE設定ファイル。
- テスト:
    - `cargo test`: Rustの組み込みテストフレームワークを利用し、ユニットテストおよび統合テストを実行します。
- ビルドツール:
    - Cargo: Rustの公式ビルドシステムおよびパッケージマネージャー。依存関係管理、ビルド、テスト、ドキュメント生成などを担当します。
    - `build.rs`: `tree-sitter`パーサーのC言語ソースファイルを自動生成するためのカスタムビルドスクリプト。
- 言語機能:
    - Rustの型システムと所有権モデル: メモリ安全性を確保し、堅牢なコードを記述するためのRust言語の主要な機能。
- 自動化・CI/CD:
    - `build.rs`によるパーサー自動生成: `grammar.js`の変更時にパーサーのC言語ソースが自動で再生成される仕組みが、継続的インテグレーションをサポートします。
- 開発標準:
    - `cargo fmt`: Rustコードの自動フォーマットツール。コードの一貫性を保ちます。
    - `cargo clippy`: Rustコードのリンター。潜在的なバグや非効率なコードパターンを指摘し、コード品質を向上させます。

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
│   └── development-status-generated-prompt.md
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
-   **`.editorconfig`**: さまざまなエディタやIDEで一貫したコーディングスタイル（インデント、改行コードなど）を維持するための設定ファイル。
-   **`.gitignore`**: Gitでバージョン管理システムに含めないファイルやディレクトリを指定するファイル。ビルド成果物や一時ファイルなどが含まれます。
-   **`.vscode/settings.json`**: Visual Studio Codeエディタ固有の設定ファイル。ワークスペースの設定や拡張機能の設定が記述されます。
-   **`Cargo.lock`**: `Cargo.toml`で指定された依存関係の具体的なバージョンを記録するファイル。再現性のあるビルドを保証します。
-   **`Cargo.toml`**: Rustプロジェクトのメタデータ（プロジェクト名、バージョン、著者など）と、外部ライブラリの依存関係、ビルド設定を定義するファイル。
-   **`LICENSE`**: 本プロジェクトのライセンス情報（MIT License）を記載したファイル。
-   **`README.ja.md` / `README.md`**: プロジェクトの目的、使い方、インストール方法、開発ガイドラインなどを説明するドキュメント（日本語版と英語版）。
-   **`_config.yml`**: プロジェクトのメタ設定やビルド設定（GitHub PagesのJekyll設定など）を記述する可能性があるファイル。
-   **`build.rs`**: Rustプロジェクトのカスタムビルドスクリプト。主に`tree-sitter`パーサーのC言語ソースファイル（`parser.c`など）を`grammar.js`から自動生成する際に使用されます。
-   **`generated-docs/development-status-generated-prompt.md`**: 自動生成されたドキュメントやプロンプト関連のメモが格納されるディレクトリ。
-   **`mmlabc-to-smf-rust.toml.example`**: カスタムMIDIプレイヤーの設定例を示すTOMLファイル。ユーザーが自身の環境に合わせてコピー・編集して使用できます。
-   **`src/config.rs`**: アプリケーションの動作に必要な設定（例: 外部MIDIプレイヤーのコマンド）を定義し、設定ファイルから読み込むロジックを管理するモジュール。
-   **`src/lib.rs`**: プロジェクトのライブラリクレートのエントリポイント。MML-SMF変換の主要な公開関数やデータ構造を提供します。
-   **`src/main.rs`**: コマンドラインインターフェース（CLI）アプリケーションのエントリポイント。CLI引数の解析、MML文字列の処理、変換処理の実行、結果のファイル出力と自動再生を担当します。
-   **`src/pass1_parser.rs`**: MML文字列を解析し、基本的なトークン（音符、記号、コマンドなど）のシーケンスに変換する「パス1」のロジックを実装したモジュール。
-   **`src/pass2_ast.rs`**: パス1で生成されたトークン列を、MMLの論理構造を表現する抽象構文木（AST）に変換する「パス2」のロジックを実装したモジュール。
-   **`src/pass3_events.rs`**: パス2で生成されたASTを基に、MIDIの具体的なイベント（音符オン/オフ、テンポ変更、プログラムチェンジなど）のシーケンスを生成する「パス3」のロジックを実装したモジュール。
-   **`src/pass4_midi.rs`**: パス3で生成されたMIDIイベントのシーケンスから、最終的なStandard MIDI File（SMF）のバイナリデータを作成する「パス4」のロジックを実装したモジュール。
-   **`src/tree_sitter_mml.rs`**: `tree-sitter`パーサーをRustプロジェクトに統合するためのモジュール。C言語で生成されたパーサーのバインディングや利用ロジックを提供します。
-   **`src/types.rs`**: プロジェクト全体で共有される共通のデータ型、列挙型、構造体などを定義するモジュール。MMLトークン、ASTノード、MIDIイベントなどが含まれます。
-   **`tests/` ディレクトリ内のファイル群**: プロジェクトのテストコードを格納するディレクトリ。
    -   `tests/integration_test.rs`: ライブラリ全体の機能が連携して正しく動作するかを確認する結合テスト。
    -   `tests/test_channel.rs` / `test_chord.rs` / `test_cli.rs` / `test_config.rs` / `test_dotted_notes.rs` / `test_length.rs` / `test_modifier.rs` / `test_note_length.rs` / `test_octave.rs` / `test_pass1.rs` / `test_pass2.rs` / `test_pass3.rs` / `test_pass4.rs` / `test_program_change.rs` / `test_rest.rs` / `test_tempo.rs` / `test_velocity.rs`: それぞれ特定のMMLコマンド、機能、または変換パスの単体テストや機能テストを定義。
-   **`tree-sitter-mml/grammar.js`**: `tree-sitter`がMMLを解析するための文法定義ファイル。JavaScriptで記述され、MMLのルールを定義します。
-   **`tree-sitter-mml/package.json`**: `tree-sitter-mml`サブプロジェクト（Node.jsモジュール）のメタデータと依存関係を定義するファイル。
-   **`tree-sitter-mml/src/grammar.json`**: `grammar.js`から自動生成されるJSON形式の文法定義ファイル。`tree-sitter`ランタイムが利用します。
-   **`tree-sitter-mml/src/node-types.json`**: `grammar.js`から自動生成されるASTノードの型定義ファイル。
-   **`tree-sitter-mml/src/parser.c`**: `grammar.js`から自動生成される、MML文字列を解析するためのC言語ソースコード。
-   **`tree-sitter-mml/src/tree_sitter/alloc.h` / `array.h` / `parser.h`**: `tree-sitter`パーサーのC言語実装に必要な共通ヘッダファイル群。

## 関数詳細説明
（提供されたプロジェクト情報から具体的な関数シグネチャを特定することは困難なため、4パスアーキテクチャの主要な処理フローに基づいて推測される役割を記述します。）

-   **`main`関数** (src/main.rs):
    -   役割: コマンドラインアプリケーションの開始点。
    -   引数: 通常は無し（内部で`std::env::args`を使って引数を解析）。
    -   戻り値: `Result<(), Box<dyn std::error::Error>>` (処理の成功またはエラー)。
    -   機能: ユーザーからのMML入力や各種オプションを処理し、MMLからMIDIへの変換、結果のファイル出力、および外部プレイヤーでの自動再生をオーケストレーションします。

-   **`parse_mml_to_tokens`関数** (src/pass1_parser.rs内の主要関数と推測):
    -   役割: MML文字列を解析し、より基本的な構文要素（トークン）のリストに変換します。
    -   引数: `mml_string: &str` (Music Macro Language形式の入力文字列)。
    -   戻り値: `Result<Vec<Token>, Error>` (トークンのリスト、または解析エラー)。
    -   機能: 入力されたMMLテキストを読み込み、音符、休符、制御コマンド、チャンネル区切りなど、MMLの構成要素を識別して構造化されたトークンとして出力します。

-   **`build_ast_from_tokens`関数** (src/pass2_ast.rs内の主要関数と推測):
    -   役割: トークン列からMMLの抽象構文木（AST）を構築します。
    -   引数: `tokens: Vec<Token>` (パス1で生成されたトークンのリスト)。
    -   戻り値: `Result<AstNode, Error>` (MMLの階層構造を表すASTのルートノード、またはエラー)。
    -   機能: トークン間の関係性（例：音符とそれに続く音長指定、チャンネルごとのMMLブロックなど）を解釈し、MMLの論理的な構造を木構造で表現します。

-   **`generate_midi_events_from_ast`関数** (src/pass3_events.rs内の主要関数と推測):
    -   役割: ASTを基に、Standard MIDI Fileの基本的な構成要素であるMIDIイベントのシーケンスを生成します。
    -   引数: `ast: &AstNode` (パス2で生成された抽象構文木)。
    -   戻り値: `Result<Vec<MidiEvent>, Error>` (時間軸上のMIDIイベントのリスト、またはエラー)。
    -   機能: ASTの各ノード（音符、テンポ、音量、プログラムチェンジなど）を、MIDI標準で定義された具体的なメッセージ（Note On, Note Off, Tempo Changeなど）に変換し、イベント発生時刻を計算します。

-   **`create_smf_from_midi_events`関数** (src/pass4_midi.rs内の主要関数と推測):
    -   役割: MIDIイベントのシーケンスから、最終的なStandard MIDI File（SMF）のバイナリデータを生成します。
    -   引数: `events: Vec<MidiEvent>` (パス3で生成されたMIDIイベントのリスト)。
    -   戻り値: `Result<Vec<u8>, Error>` (SMF形式のバイナリデータ、またはエラー)。
    -   機能: MIDIイベントをSMFの仕様（ヘッダチャンク、トラックチャンク、イベントフォーマットなど）に従って整形し、再生可能なMIDIファイルとしてバイト列を構築します。

-   **`read_app_config`関数** (src/config.rs内の主要関数と推測):
    -   役割: 設定ファイル（例: `mmlabc-to-smf-rust.toml`）からアプリケーション固有の設定を読み込みます。
    -   引数: `config_path: Option<&Path>` (設定ファイルのパス、省略可能)。
    -   戻り値: `Result<AppConfig, Error>` (パースされたアプリケーション設定、またはエラー)。
    -   機能: ユーザーがカスタマイズしたMIDIプレイヤーのパスやその他の設定をロードし、アプリケーション全体で利用できるようにします。

-   **`run_external_midi_player`関数** (src/main.rs または src/config.rsに関連するヘルパー関数と推測):
    -   役割: 生成されたMIDIファイルを指定された外部MIDIプレイヤーアプリケーションで再生します。
    -   引数: `file_path: &Path`, `player_command: &str` (再生するMIDIファイルのパス、実行するプレイヤーコマンド)。
    -   戻り値: `Result<(), Error>` (コマンド実行の成功またはエラー)。
    -   機能: OSのシェルコマンドを介して、設定されたMIDIプレイヤーを実行し、生成されたMIDIファイルを自動的に開いて再生させます。

## 関数呼び出し階層ツリー
（提供された情報では詳細な関数呼び出し階層の分析が困難なため、プロジェクトの4パスアーキテクチャに基づいた主要な処理フローを示します。）

```
main()
└── (CLI引数解析、設定読み込み)
    ├── read_app_config()
    └── execute_conversion_and_playback() (内部的なオーケストレーション関数と仮定)
        ├── parse_mml_to_tokens() (パス1: MML文字列をトークンに)
        ├── build_ast_from_tokens() (パス2: トークンをASTに)
        ├── generate_midi_events_from_ast() (パス3: ASTからMIDIイベントに)
        ├── create_smf_from_midi_events() (パス4: MIDIイベントからSMFファイルに)
        └── (SMFファイル保存)
        └── run_external_midi_player() (オプションでMIDIを再生)

---
Generated at: 2025-11-28 07:05:56 JST
