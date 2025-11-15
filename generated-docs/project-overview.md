Last updated: 2025-11-16

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) をStandard MIDI File (SMF) に変換するRust製のコマンドラインツールです。
- 4つのパス（トークン化、AST変換、MIDIイベント生成、SMF作成）に分かれた明確なアーキテクチャを採用しています。
- 多チャンネル対応、デバッグ用JSON出力、豊富なテストを備え、MML開発の効率化と音楽表現の可能性を広げます。

## 技術スタック
- フロントエンド: CLI (コマンドラインインターフェース) - ユーザーがMML文字列を直接入力し、オプションで出力ファイル名や再生制御を行うためのインターフェースを提供します。
- 音楽・オーディオ: Standard MIDI File (SMF), Music Macro Language (MML) - 入力と出力の主要なデータ形式です。また、生成されたMIDIファイルを自動再生するための外部プレイヤー（`cat-play-mml`がデフォルト）や、ユーザー設定で指定可能な`TiMidity++`、`FluidSynth`、`VLC`などのMIDIプレイヤーに対応しています。
- 開発ツール: Rust - プロジェクトの主要なプログラミング言語であり、その型システムと所有権モデルによりメモリ安全性と堅牢な設計を実現しています。また、tree-sitterパーサーの文法更新にはNode.jsが必要です。
- テスト: Rust標準テストフレームワーク - `cargo test`コマンドを通じて、ユニットテストと統合テストが合計35個実装されており、変換ロジックの正確性を保証します。
- ビルドツール: Cargo - Rustの公式パッケージマネージャー兼ビルドシステムで、プロジェクトのビルド、テスト、依存関係管理を担います。
- 言語機能: Rustの型システムと所有権モデル - これらはプロジェクトの「安全な設計」の基盤となっており、コンパイル時のエラー検出とランタイムの安全性を提供します。
- 自動化・CI/CD: (情報なし)
- 開発標準: `rustfmt` (`cargo fmt`) - コードの自動整形を行い、一貫したコードスタイルを維持します。`clippy` (`cargo clippy`) - コード品質と潜在的なバグをチェックするリンターツールです。

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
│   └── ... (他のissueメモ)
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
- **`.editorconfig`**: 異なるエディタやIDE間で一貫したコーディングスタイルを維持するための設定ファイル。
- **`.gitignore`**: Gitがバージョン管理の対象としないファイルやディレクトリを指定するファイル。
- **`.vscode/settings.json`**: Visual Studio Codeエディタ固有の設定ファイル。
- **`Cargo.lock`**: `Cargo.toml`で定義された依存関係の具体的なバージョンをロックするファイル。再現性のあるビルドを保証します。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイル。プロジェクト名、バージョン、依存クレート、ビルド設定などを定義します。
- **`LICENSE`**: プロジェクトのライセンス情報（MITライセンス）が記載されています。
- **`README.ja.md`**: プロジェクトの日本語による説明書。
- **`README.md`**: プロジェクトの主要な説明書（通常は英語、または多言語プロジェクトのデフォルト）。
- **`_config.yml`**: GitHub Pagesなどの静的サイトジェネレーターで利用される設定ファイル（推測）。
- **`build.rs`**: Cargoのビルドスクリプト。特に`tree-sitter-mml`クレートにおいて、`grammar.js`からC言語パーサーファイルを生成する処理を自動化します。
- **`generated-docs/`**: プロジェクトのドキュメントが生成されて格納されるディレクトリ（自動生成されるものと推測）。
- **`issue-notes/`**: 開発中の課題やメモが記述されたMarkdownファイルが格納されるディレクトリ。
- **`mmlabc-to-smf-rust.toml.example`**: ユーザーがカスタムMIDIプレイヤーを設定するためのサンプル設定ファイル。
- **`src/config.rs`**: カスタムMIDIプレイヤー設定などの構成情報を読み込み、管理するためのモジュール。
- **`src/lib.rs`**: ライブラリクレートのルートファイル。他のモジュールを公開し、MMLからSMFへの変換の中核ロジックを提供します。
- **`src/main.rs`**: コマンドラインインターフェース (CLI) のエントリーポイント。コマンドライン引数の解析、設定の読み込み、変換処理の呼び出し、結果の出力（MIDIファイル保存、自動再生）をオーケストレーションします。
- **`src/pass1_parser.rs`**: プロジェクトの「パス1」を担当するモジュール。MML文字列を入力として受け取り、構文解析を行い、トークンのシーケンスに変換します。
- **`src/pass2_ast.rs`**: プロジェクトの「パス2」を担当するモジュール。パス1で生成されたトークンのシーケンスから、MMLの論理構造を表す抽象構文木 (AST) を構築します。
- **`src/pass3_events.rs`**: プロジェクトの「パス3」を担当するモジュール。パス2で生成されたASTを走査し、MIDI標準に準拠したイベント（Note On/Offなど）のリストを生成します。
- **`src/pass4_midi.rs`**: プロジェクトの「パス4」を担当するモジュール。パス3で生成されたMIDIイベントのリストを受け取り、Standard MIDI File (SMF) 形式のバイナリデータを作成し、ファイルとして保存します。
- **`src/tree_sitter_mml.rs`**: tree-sitterパーサーとの統合を扱うモジュール。将来的により複雑なMML構文の解析を効率的に行うための基盤を提供します。
- **`src/types.rs`**: プロジェクト全体で共有されるカスタムデータ型（例: `Token`、`AstNode`、`MidiEvent`など）を定義するモジュール。
- **`tests/integration_test.rs`**: MMLからSMFへの変換プロセス全体を最初から最後までテストする統合テストスイート。
- **`tests/test_channel.rs`**: MMLのセミコロンによる多チャンネル分離機能の動作を検証するテスト。
- **`tests/test_chord.rs`**: MMLにおける和音記法（例: `<c,e,g>`) の解析とMIDIイベント生成をテストするモジュール。
- **`tests/test_cli.rs`**: コマンドライン引数のパース、オプションの適用、出力挙動など、CLIの機能に関するテスト。
- **`tests/test_config.rs`**: 外部設定ファイル`mmlabc-to-smf-rust.toml`の読み込み、解析、設定の適用を検証するテスト。
- **`tests/test_dotted_notes.rs`**: 付点音符（例: `c.`）の長さが正しく計算され、MIDIイベントに変換されるかをテストします。
- **`tests/test_length.rs`**: 音長指定（例: `c4`）が正しく解釈され、MIDIイベントに反映されるかをテストします。
- **`tests/test_modifier.rs`**: 音量（`v`）、パン（`p`）などのMML修飾子に関するテスト。
- **`tests/test_note_length.rs`**: 各音符のデフォルトおよび指定された長さが正しく処理されることを確認するテスト。
- **`tests/test_octave.rs`**: オクターブ変更コマンド（`>`、`<`）の動作を検証するテスト。
- **`tests/test_pass1.rs`**: パス1（トークン解析）モジュールのユニットテスト。MML文字列が正しくトークン化されるかを確認します。
- **`tests/test_pass2.rs`**: パス2（AST変換）モジュールのユニットテスト。トークンシーケンスが正しいAST構造に変換されるかを確認します。
- **`tests/test_pass3.rs`**: パス3（MIDIイベント生成）モジュールのユニットテスト。ASTから適切なMIDIイベントが生成されるかを確認します。
- **`tests/test_pass4.rs`**: パス4（MIDIファイル作成）モジュールのユニットテスト。MIDIイベントリストから有効なSMFファイルが作成されるかを確認します。
- **`tests/test_program_change.rs`**: プログラムチェンジコマンド（音色変更）のテスト。
- **`tests/test_rest.rs`**: 休符（`r`）の処理が正しく行われることを確認するテスト。
- **`tests/test_tempo.rs`**: テンポ変更コマンド（`t`）のテスト。
- **`tests/test_velocity.rs`**: ベロシティ（音の強さ）設定が正しく機能するかをテストします。
- **`tree-sitter-mml/grammar.js`**: tree-sitterパーサーの文法を定義するJavaScriptファイル。
- **`tree-sitter-mml/package.json`**: tree-sitterパーサーのビルドに必要なNode.jsパッケージの依存関係とスクリプトを定義します。
- **`tree-sitter-mml/src/grammar.json`**: `grammar.js`から生成されるMML文法のJSON形式表現。
- **`tree-sitter-mml/src/node-types.json`**: `grammar.js`から生成される、ASTノードのタイプ定義。
- **`tree-sitter-mml/src/parser.c`**: `grammar.js`から生成されるMML用のC言語パーサー本体。
- **`tree-sitter-mml/src/tree_sitter/alloc.h`, `array.h`, `parser.h`**: tree-sitterパーサーが内部的に利用するC言語のヘッダーファイル。

## 関数詳細説明
- **`run_cli()`** (src/main.rs):
  - 役割: コマンドラインインターフェースの主要な処理を担う関数。
  - 引数: なし (内部でコマンドライン引数をパース)。
  - 戻り値: `Result<(), Box<dyn Error>>` - 処理の成否を示す。
  - 機能: コマンドライン引数を解析し、MML文字列の取得、出力ファイル名の決定、自動再生の要否、カスタムプレイヤー設定の読み込みなどを行います。その後、`convert_mml_to_smf`関数を呼び出し、結果のMIDIファイルを保存・再生します。
- **`convert_mml_to_smf()`** (src/lib.rs):
  - 役割: MML文字列をStandard MIDI File形式のバイトデータに変換するコアロジックをまとめた関数。
  - 引数: `mml_string: &str` (変換対象のMML文字列), `options: ConversionOptions` (変換に関する設定)。
  - 戻り値: `Result<Vec<u8>, ConversionError>` - 変換されたSMFデータ（バイト配列）またはエラー。
  - 機能: 4パスアーキテクチャ（トークン化、AST構築、MIDIイベント生成、SMF作成）を順に実行し、MML文字列から最終的なSMFバイトデータを生成します。各パスの中間結果のデバッグ出力も制御します。
- **`tokenize_mml()`** (src/pass1_parser.rs):
  - 役割: MML文字列を個々の意味を持つトークン（音符、区切り文字など）に分割する関数。
  - 引数: `mml_string: &str` (トークン化するMML文字列)。
  - 戻り値: `Result<Vec<Token>, TokenizationError>` - トークンのリストまたはエラー。
  - 機能: MML文字列を走査し、定義されたMMLのルールに基づいて文字の並びを認識し、`Token`構造体のリストとして出力します。
- **`build_ast()`** (src/pass2_ast.rs):
  - 役割: トークンのシーケンスから抽象構文木 (AST) を構築する関数。
  - 引数: `tokens: Vec<Token>` (パス1で生成されたトークンのリスト)。
  - 戻り値: `Result<AstNode, AstBuildError>` - MMLの構造を表すASTのルートノードまたはエラー。
  - 機能: トークン間の関係性を解釈し、MMLの階層構造（例: チャンネル、音符、コマンドなど）をツリー状のデータ構造（AST）として表現します。
- **`generate_midi_events()`** (src/pass3_events.rs):
  - 役割: 抽象構文木 (AST) をMIDIイベントのシーケンスに変換する関数。
  - 引数: `ast: &AstNode` (パス2で構築されたAST), `initial_state: MidiState` (初期MIDI状態、テンポなど)。
  - 戻り値: `Result<Vec<MidiEvent>, MidiEventError>` - タイムスタンプ付きMIDIイベントのリストまたはエラー。
  - 機能: ASTを深さ優先または幅優先でトラバースし、各MML要素に対応するMIDIのノートオン/オフ、プログラムチェンジ、テンポ変更などのイベントを生成します。
- **`create_smf_file()`** (src/pass4_midi.rs):
  - 役割: MIDIイベントのリストを受け取り、Standard MIDI File (SMF) 形式のバイナリデータを作成する関数。
  - 引数: `midi_events: Vec<MidiEvent>` (パス3で生成されたMIDIイベントのリスト), `header_info: SmfHeader` (SMFヘッダー情報、トラック数など)。
  - 戻り値: `Result<Vec<u8>, SmfCreationError>` - SMF形式のバイト配列またはエラー。
  - 機能: MIDIイベントをSMF仕様に沿って整理し、SMFヘッダーと各トラックのMIDIデータチャンクを構築して、完全なSMFバイナリデータとして出力します。
- **`load_config()`** (src/config.rs):
  - 役割: 外部設定ファイル `mmlabc-to-smf-rust.toml` から設定を読み込む関数。
  - 引数: なし (デフォルトのファイルパスまたは指定されたパスから読み込む)。
  - 戻り値: `Result<AppConfig, ConfigError>` - アプリケーション設定構造体またはエラー。
  - 機能: TOML形式の設定ファイルをパースし、カスタムMIDIプレイヤーのパスなどの設定項目をアプリケーションで利用可能な形式で提供します。
- **`play_midi_file()`** (src/main.rs またはユーティリティモジュール):
  - 役割: 指定されたMIDIファイルを外部のMIDIプレイヤーで再生する関数。
  - 引数: `file_path: &Path` (再生するMIDIファイルのパス), `player_command: &str` (使用するプレイヤーのコマンド)。
  - 戻り値: `Result<(), io::Error>` - 実行の成否。
  - 機能: 設定された外部MIDIプレイヤーのコマンドを子プロセスとして実行し、生成されたMIDIファイルを自動的に再生します。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした。

---
Generated at: 2025-11-16 07:05:05 JST
