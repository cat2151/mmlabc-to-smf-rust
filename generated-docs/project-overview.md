Last updated: 2025-11-12

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) 形式の文字列をStandard MIDI File (SMF) へ変換するRust製のコマンドラインツールです。
- MMLのトークン化からMIDIファイル作成までを4つの処理パスで段階的に実行します。
- 基本的な音符変換、多チャンネル対応、そしてデバッグ用のJSON中間出力機能を備えています。

## 技術スタック
- フロントエンド: 該当なし (本プロジェクトはコマンドラインインターフェース(CLI)ツールです)
- 音楽・オーディオ:
    - Standard MIDI File (SMF): 世界的に広く使用されている電子楽器の演奏データを保存するための標準ファイル形式。
    - Music Macro Language (MML): 音楽をテキストベースで記述するための簡易的なプログラミング言語。
- 開発ツール:
    - Cargo: Rust言語の公式パッケージマネージャー兼ビルドシステム。プロジェクトの依存関係管理、ビルド、テスト実行に利用されます。
    - Git: ソースコードのバージョン管理システム。
    - Visual Studio Code (.vscode/settings.json): 開発環境のエディタ設定を統一するためのファイル。
    - tree-sitter: 将来的により高度なMML構文解析のために統合が検討されている、高速な構文解析フレームワーク。
- テスト:
    - `cargo test`: Rustに標準搭載されているテスト実行ツール。ユニットテストと統合テストを効率的に実行します。
- ビルドツール:
    - Cargo: Rustプロジェクトのビルドを管理する公式ツール。
    - `build.rs`: Rustのビルドプロセス中にカスタムスクリプトを実行するためのファイル。特にC言語のライブラリ（tree-sitterなど）をビルドする際に利用されます。
- 言語機能:
    - Rust: 高性能かつメモリ安全性を重視したシステムプログラミング言語。所有権システムにより安全なコード記述を支援します。
- 自動化・CI/CD:
    - `_config.yml`: GitHub ActionsなどのCI/CDパイプライン設定、または静的サイトジェネレータの設定ファイルとして利用されます。
    - `cargo clippy`: Rustコードの品質と慣用句をチェックする静的解析ツール（リンター）。
    - `cargo fmt`: Rustコードのフォーマットを自動的に整えるツール。
- 開発標準:
    - .editorconfig: 異なるエディタやIDEを使用する開発者間で、コーディングスタイル（インデント、改行コードなど）の一貫性を保つための設定ファイル。

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
├── src/
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
-   **src/main.rs**: コマンドラインインターフェース（CLI）アプリケーションのエントリーポイントです。コマンドライン引数の解析を行い、MML変換の主要な処理フローを管理します。
-   **src/lib.rs**: このプロジェクトのコアライブラリ機能を定義するファイルです。他のモジュールがこのファイルを通して主要な変換ロジックを公開します。
-   **src/pass1_parser.rs**: MML文字列を個々の意味を持つ最小単位（トークン）に分解する字句解析（単純パーサー）を担当します。
-   **src/pass2_ast.rs**: パス1で生成されたトークンの列を受け取り、それをプログラムの構造を階層的に表現する抽象構文木（AST）に変換します。
-   **src/pass3_events.rs**: パス2で構築されたASTを解釈し、最終的なMIDIファイルに書き込まれるべき一連のMIDIイベント（音符オン/オフ、テンポ変更など）を生成します。
-   **src/pass4_midi.rs**: パス3で生成されたMIDIイベントのリストから、Standard MIDI File形式に準拠したバイナリデータを生成し、ファイルとして保存します。
-   **src/tree_sitter_mml.rs**: tree-sitter構文解析フレームワークとMML文法を統合するためのモジュールです。将来的なより複雑なMML解析に対応するために準備されています。
-   **src/types.rs**: プロジェクト全体で共通して使用されるカスタムデータ型、構造体、列挙型などを定義し、コードの一貫性と可読性を高めます。
-   **tests/integration_test.rs**: アプリケーション全体のMMLからSMFへの変換フローが正しく機能するかを検証する統合テストが含まれています。
-   **tests/test_channel.rs**: MMLの多チャンネル機能が意図通りに動作するかを確認するためのユニットテストが含まれています。
-   **tests/test_cli.rs**: コマンドラインインターフェース（CLI）の挙動、引数処理、出力などが期待通りか検証するテストです。
-   **tests/test_pass1.rs**: パス1の字句解析機能がMML文字列を正しくトークン化できるかを検証するユニットテストです。
-   **tests/test_pass2.rs**: パス2のAST構築機能がトークン列から正しい抽象構文木を生成できるかを検証するユニットテストです。
-   **tests/test_pass3.rs**: パス3のMIDIイベント生成機能がASTから正確なMIDIイベント列を生成できるかを検証するユニットテストです。
-   **tests/test_pass4.rs**: パス4のMIDIファイル作成機能がMIDIイベントから有効なStandard MIDI Fileを生成できるかを検証するユニットテストです。
-   **tree-sitter-mml/grammar.js**: tree-sitterパーサーがMMLの構文を理解・解析するために使用する文法定義ファイル（JavaScript形式）です。
-   **tree-sitter-mml/package.json**: tree-sitter MMLパーサーモジュールのメタデータと依存関係を定義するnpmパッケージファイルです。
-   **tree-sitter-mml/src/grammar.json**: `grammar.js`からtree-sitterによって生成される、内部で使用されるJSON形式の文法定義です。
-   **tree-sitter-mml/src/node-types.json**: tree-sitterによって生成される、ASTノードの型定義を含むJSONファイルです。
-   **tree-sitter-mml/src/parser.c**: `grammar.js`からtree-sitterによって生成される、MML構文解析の中核を担うC言語のパーサー実装です。
-   **tree-sitter-mml/src/tree_sitter/\*.h**: tree-sitterパーサーのC言語実装が利用する各種ヘッダーファイル群です。
-   **Cargo.toml**: Rustプロジェクトのビルド設定、依存関係、パッケージのメタデータなどを記述するマニフェストファイルです。
-   **Cargo.lock**: `Cargo.toml`に基づいてCargoが解決した、プロジェクトが使用するすべての依存クレートの正確なバージョンとチェックサムを記録するファイルです。
-   **LICENSE**: 本プロジェクトのライセンス（MIT License）が記載されています。
-   **README.md**: プロジェクトの目的、機能、使い方、開発状況などの概要を英語で提供する主要なドキュメントファイルです。
-   **README.ja.md**: `README.md`の日本語版です。
-   **.editorconfig**: プロジェクト全体のコーディングスタイルを定義し、複数の開発環境で一貫したフォーマットを保つための設定ファイルです。
-   **.gitignore**: Gitのバージョン管理システムに含めないファイルやディレクトリを指定する設定ファイルです。
-   **.vscode/settings.json**: Visual Studio Codeエディタのワークスペース固有の設定を定義するファイルです。
-   **_config.yml**: YAML形式の設定ファイルで、主にCI/CDツール（例: GitHub Actions）や静的サイトジェネレータの設定に使用されます。
-   **build.rs**: Rustのビルドプロセス中に実行されるカスタムビルドスクリプトです。主に外部の非Rustコードのコンパイルや生成コードの組み込みに使用されます。

## 関数詳細説明
-   `run_mml_to_smf_conversion(mml_input: &str, output_path: Option<&str>, no_play: bool) -> Result<(), Box<dyn Error>>`:
    -   役割: MML文字列を受け取り、4パスアーキテクチャを通じてSMFに変換し、オプションでMIDIファイルを再生します。CLIの主要な実行ロジックをカプセル化しています。
    -   引数:
        -   `mml_input`: 変換対象のMML文字列。
        -   `output_path`: 生成されるSMFファイルのパス（オプション）。Noneの場合はデフォルト名を使用。
        -   `no_play`: MIDIファイル生成後の自動再生を無効にするフラグ。
    -   戻り値: 成功時は`Ok(())`、エラー発生時は`Err`を返します。

-   `pass1_parse_mml_to_tokens(mml_string: &str) -> Vec<Token>`:
    -   役割: 入力されたMML文字列を字句解析し、一連の`Token`構造体（音符、区切り文字など）のベクトルに変換します。
    -   引数: `mml_string`: Music Macro Language形式の入力文字列。
    -   戻り値: 解析されたトークンのリスト（`Vec<Token>`）。

-   `pass2_build_ast_from_tokens(tokens: Vec<Token>) -> AstNode`:
    -   役割: パス1で生成されたトークンのリストから、MMLの構造を表現する抽象構文木（AST）を構築します。
    -   引数: `tokens`: 字句解析によって得られたトークンのリスト。
    -   戻り値: MMLの構造を表すASTのルートノード（`AstNode`）。

-   `pass3_generate_midi_events_from_ast(ast: AstNode) -> Vec<MidiEvent>`:
    -   役割: パス2で構築されたASTを走査し、最終的なMIDIファイルに書き込まれるべき具体的なMIDIイベント（例: ノートオン、ノートオフ、テンポ設定）のリストを生成します。
    -   引数: `ast`: 抽象構文木。
    -   戻り値: 生成されたMIDIイベントのリスト（`Vec<MidiEvent>`）。

-   `pass4_create_smf_from_midi_events(events: Vec<MidiEvent>, output_path: Option<&str>) -> Result<(), Error>`:
    -   役割: パス3で生成されたMIDIイベントのリストを受け取り、Standard MIDI File（SMF）形式のバイナリデータを作成し、指定されたファイルパスに保存します。
    -   引数:
        -   `events`: 生成されたMIDIイベントのリスト。
        -   `output_path`: 保存するMIDIファイルのパス（オプション）。
    -   戻り値: ファイルの書き込みが成功した場合は`Ok(())`、エラーが発生した場合は`Err`。

## 関数呼び出し階層ツリー
```
main() (src/main.rs)
  ├─ run_mml_to_smf_conversion() (src/main.rs)
  │  ├─ pass1_parse_mml_to_tokens() (src/pass1_parser.rs)
  │  ├─ pass2_build_ast_from_tokens() (src/pass2_ast.rs)
  │  ├─ pass3_generate_midi_events_from_ast() (src/pass3_events.rs)
  │  ├─ pass4_create_smf_from_midi_events() (src/pass4_midi.rs)
  │  └─ play_midi_file() (src/main.rs - 外部コマンド呼び出し)
  └─ handle_cli_args() (src/main.rs)

---
Generated at: 2025-11-12 07:06:26 JST
