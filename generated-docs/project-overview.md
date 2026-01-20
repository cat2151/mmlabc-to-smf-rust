Last updated: 2026-01-21

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) 形式の楽譜をStandard MIDI File (SMF) に変換するRust製ライブラリです。
- 独自の4パスアーキテクチャを採用し、構文解析、抽象構文木(AST)構築、MIDIイベント生成、SMF作成の一連の処理を行います。
- コマンドラインツールとしても機能し、MML入力からMIDIファイル生成、さらには外部プレイヤーによる自動再生までをサポートします。

## 技術スタック
- フロントエンド: CLI (コマンドラインインターフェース) を提供しており、特定のWeb UIは存在しません。
- 音楽・オーディオ: Music Macro Language (MML) (入力形式), Standard MIDI File (SMF) (出力形式), `cat-play-mml` (デフォルト外部MIDIプレイヤー).
- 開発ツール: Rust (プログラミング言語), Cargo (Rustのビルドシステムとパッケージマネージャー), tree-sitter (MML構文解析のためのパーサーフレームワーク), Node.js / npx (tree-sitterパーサー生成用).
- テスト: Rustの組み込みテストフレームワーク (ユニットテスト、統合テスト).
- ビルドツール: Cargo (Rustプロジェクトのビルド管理).
- 言語機能: Rustの強力な型システムと所有権モデル (メモリ安全性と堅牢性を保証).
- 自動化・CI/CD: 明示的なCI/CDツールは記載されていませんが、開発標準の一部としてフォーマッターやリンターの設定が含まれます。
- 開発標準: `cargo clippy` (Lintツール), `cargo fmt` (コードフォーマッター).

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
├── googled947dc864c270e07.html
├── issue-notes/
│   ├── 14.md
│   ├── 17.md
│   ├── 18.md
│   ├── 19.md
│   ├── 20.md
│   ├── 21.md
│   ├── 22.md
│   ├── 23.md
│   ├── 24.md
│   ├── 30.md
│   ├── 36.md
│   ├── 37.md
│   ├── 39.md
│   ├── 40.md
│   ├── 42.md
│   └── 44.md
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
- **`.editorconfig`**: 異なるエディタやIDE間でコードのスタイル（インデント、改行など）を統一するための設定ファイルです。
- **`.gitignore`**: Gitのバージョン管理から除外するファイルやディレクトリを指定するファイルです。ビルド生成物や一時ファイルなどが含まれます。
- **`.vscode/settings.json`**: Visual Studio Codeエディタのワークスペース固有の設定ファイル。特定の拡張機能の設定やLint/フォーマットに関するルールが含まれることがあります。
- **`Cargo.lock`**: `Cargo.toml`に基づいてCargoが解決した、すべての依存関係の正確なバージョンとハッシュを記録するファイルです。再現性のあるビルドを保証します。
- **`Cargo.toml`**: Rustプロジェクトの依存関係、メタデータ、ビルド設定を定義するマニフェストファイルです。
- **`LICENSE`**: プロジェクトのライセンス（MIT License）が記載されています。
- **`README.ja.md`**: プロジェクトの概要、利用方法、開発状況などを日本語で説明する主要なドキュメントファイルです。
- **`README.md`**: プロジェクトの概要、利用方法、開発状況などを英語で説明する主要なドキュメントファイルです。
- **`_config.yml`**: GitHub Pagesなどの静的サイトジェネレータで使用される設定ファイルである可能性があります。
- **`build.rs`**: Rustのカスタムビルドスクリプト。特に`tree-sitter-mml/grammar.js`が変更された際に、tree-sitterパーサーのC言語ソースファイルを自動的に再生成するロジックを含みます。
- **`generated-docs/development-status-generated-prompt.md`**: 自動生成された開発状況に関するドキュメントやプロンプトを格納するディレクトリとファイルです。
- **`googled947dc864c270e07.html`**: Google Search Consoleなどのサイト所有権認証に使用されることが多いファイルです。
- **`issue-notes/*.md`**: 開発中の特定の問題や検討事項に関するメモファイル群です。
- **`mmlabc-to-smf-rust.toml.example`**: カスタムMIDIプレイヤー設定の例を示すファイル。ユーザーが`mmlabc-to-smf-rust.toml`を作成する際のテンプレートとなります。
- **`src/config.rs`**: 外部MIDIプレイヤーの設定など、アプリケーションの構成を管理するロジックが含まれています。
- **`src/lib.rs`**: このプロジェクトのライブラリ部分のルート。MMLからSMFへの変換を統合的に実行する主要なAPIを提供します。
- **`src/main.rs`**: コマンドラインインターフェース (CLI) のエントリーポイント。引数解析、設定の読み込み、MML変換フローのオーケストレーション、MIDIファイルの保存、外部プレイヤーでの再生処理を担います。
- **`src/pass1_parser.rs`**: MML文字列を解析し、構造化されたトークン列に変換する処理（パス1）を実装しています。tree-sitterパーサーを利用してMML構文を解析します。
- **`src/pass2_ast.rs`**: パス1で生成されたトークン列を抽象構文木（AST）に変換する処理（パス2）を実装しています。MMLの論理構造を表現します。
- **`src/pass3_events.rs`**: パス2で生成されたASTを元に、MIDIイベントのシーケンスを生成する処理（パス3）を実装しています。各音符や制御コマンドがMIDIメッセージにマッピングされます。
- **`src/pass4_midi.rs`**: パス3で生成されたMIDIイベントから、最終的なStandard MIDI File (.mid) を作成する処理（パス4）を実装しています。
- **`src/tree_sitter_mml.rs`**: Rustコードとtree-sitterで生成されたMMLパーサーとのブリッジングを担当します。MMLの構文解析機能を提供します。
- **`src/types.rs`**: プロジェクト全体で共有されるデータ構造や列挙型などの共通型定義をまとめています。
- **`tests/integration_test.rs`**: プロジェクト全体の統合的な動作を検証するテストケースが含まれています。
- **`tests/test_channel.rs`**: MMLの多チャンネル機能に関するテストケースです。
- **`tests/test_chord.rs`**: 和音機能に関するテストケースです。
- **`tests/test_cli.rs`**: コマンドラインインターフェースの動作に関するテストケースです。
- **`tests/test_config.rs`**: 設定ファイル（mmlabc-to-smf-rust.toml）の読み込みと適用に関するテストケースです。
- **`tests/test_dotted_notes.rs`**: 付点音符の処理に関するテストケースです。
- **`tests/test_length.rs`**: 音符の長さ指定に関するテストケースです。
- **`tests/test_modifier.rs`**: 音符の修飾子（シャープ、フラットなど）に関するテストケースです。
- **`tests/test_note_length.rs`**: 音符の長さに関する詳細なテストケースです。
- **`tests/test_octave.rs`**: オクターブ変更コマンドに関するテストケースです。
- **`tests/test_pass1.rs`**: パス1（トークン解析）の単体テストです。
- **`tests/test_pass2.rs`**: パス2（AST変換）の単体テストです。
- **`tests/test_pass3.rs`**: パス3（MIDIイベント生成）の単体テストです。
- **`tests/test_pass4.rs`**: パス4（MIDIファイル作成）の単体テストです。
- **`tests/test_program_change.rs`**: プログラムチェンジ（音色変更）に関するテストケースです。
- **`tests/test_rest.rs`**: 休符に関するテストケースです。
- **`tests/test_tempo.rs`**: テンポ変更に関するテストケースです。
- **`tests/test_velocity.rs`**: ベロシティ（音量）に関するテストケースです。
- **`tree-sitter-mml/grammar.js`**: MMLパーサーの文法定義ファイル。tree-sitter-cliによってC言語のパーサーソースファイルが生成されます。
- **`tree-sitter-mml/package.json`**: `tree-sitter-mml`ディレクトリのNode.jsパッケージ設定ファイルです。
- **`tree-sitter-mml/src/grammar.json`**: tree-sitterによって生成されたMMLパーサーの文法構造を記述するJSONファイルです。
- **`tree-sitter-mml/src/node-types.json`**: tree-sitterによって生成されたMMLパーサーのASTノードタイプを定義するJSONファイルです。
- **`tree-sitter-mml/src/parser.c`**: tree-sitterによって`grammar.js`から生成された、MMLを解析するためのC言語ソースコードです。
- **`tree-sitter-mml/src/tree_sitter/alloc.h`**: tree-sitterパーサーで使用されるメモリ割り当て関連のヘッダーファイルです。
- **`tree-sitter-mml/src/tree_sitter/array.h`**: tree-sitterパーサーで使用される配列操作関連のヘッダーファイルです。
- **`tree-sitter-mml/src/tree_sitter/parser.h`**: tree-sitterパーサーの主要なヘッダーファイルであり、パーサーのAPIを定義しています。

## 関数詳細説明
プロジェクト情報に具体的な関数シグネチャが提供されていないため、主要な処理フローに基づいて想定される役割を記述します。

- **`main`** (src/main.rs):
    - 役割: コマンドラインインターフェース (CLI) のエントリーポイント。アプリケーションの実行を制御します。
    - 引数: コマンドライン引数 (`env::args()` などから取得)。
    - 戻り値: 成功 (`Ok(())`) またはエラー (`Err(())`)。
    - 機能: コマンドライン引数の解析、設定の読み込み、MML変換処理の呼び出し、生成されたMIDIファイルの保存と必要に応じた自動再生を行います。
- **`load_config`** (src/config.rs):
    - 役割: アプリケーションの設定ファイル (`mmlabc-to-smf-rust.toml`) を読み込みます。
    - 引数: なし。
    - 戻り値: `Config` 構造体 (設定情報)。
    - 機能: 設定ファイルが存在しない場合はデフォルト値を適用し、外部MIDIプレイヤーの指定などを処理します。
- **`convert_mml_to_midi`** (src/lib.rs, または主要な変換関数):
    - 役割: MML文字列を受け取り、4パスアーキテクチャを通じてStandard MIDI File形式のバイトデータを生成します。
    - 引数: `mml_string: &str` (MML入力文字列)。
    - 戻り値: `Result<Vec<u8>, ConversionError>` (生成されたSMFバイトデータ、または変換エラー)。
    - 機能: パス1からパス4までの全処理を順次呼び出し、MMLをMIDIに変換する核心機能です。
- **`parse_mml_string`** (src/pass1_parser.rs):
    - 役割: MML文字列を構文解析し、構造化されたトークン列に変換します（パス1）。
    - 引数: `mml_string: &str` (MML入力文字列)。
    - 戻り値: `Result<MmlTokens, ParserError>` (解析されたMMLトークン列、または解析エラー)。
    - 機能: `tree-sitter-mml`パーサーを利用して、MMLの基本的な要素（音符、チャンネル区切りなど）を識別します。
- **`build_abstract_syntax_tree`** (src/pass2_ast.rs):
    - 役割: パス1で生成されたトークン列を抽象構文木（AST）に変換します（パス2）。
    - 引数: `tokens: MmlTokens` (MMLトークン列)。
    - 戻り値: `Result<MmlAST, ASTError>` (構築されたMMLのAST、またはAST構築エラー)。
    - 機能: トークン間の関係性を構築し、MMLの論理的な構造を階層的に表現します。
- **`generate_midi_events`** (src/pass3_events.rs):
    - 役割: パス2で生成されたASTを元に、MIDIイベントのシーケンスを生成します（パス3）。
    - 引数: `ast: MmlAST` (MML抽象構文木)。
    - 戻り値: `Result<Vec<MidiEvent>, EventGenerationError>` (MIDIイベントのリスト、またはイベント生成エラー)。
    - 機能: ASTの各ノードを巡回し、対応するMIDIノートオン/オフ、テンポ変更、チャンネル設定などのイベントを作成します。
- **`create_smf_file`** (src/pass4_midi.rs):
    - 役割: パス3で生成されたMIDIイベントリストから、Standard MIDI File形式のバイトデータを作成します（パス4）。
    - 引数: `midi_events: Vec<MidiEvent>` (MIDIイベントのリスト)。
    - 戻り値: `Result<Vec<u8>, MidiFileError>` (SMF形式のバイトデータ、またはファイル作成エラー)。
    - 機能: MIDIイベントをSMFの仕様に従ってフォーマットし、標準的なMIDIファイルを構築します。
- **`save_output_file`** (src/main.rs のヘルパー関数など):
    - 役割: 生成されたバイトデータを指定されたファイルパスに保存します。
    - 引数: `data: &[u8]` (保存するバイトデータ), `path: &Path` (出力ファイルパス)。
    - 戻り値: `Result<(), std::io::Error>` (ファイル保存の成功、またはI/Oエラー)。
    - 機能: デバッグ用JSONファイルや最終的なMIDIファイルをディスクに書き込みます。
- **`play_midi_file`** (src/main.rs のヘルパー関数など):
    - 役割: 生成されたMIDIファイルを、設定された外部MIDIプレイヤーを使用して再生します。
    - 引数: `midi_file_path: &Path` (MIDIファイルのパス), `config: &Config` (アプリケーション設定)。
    - 戻り値: `Result<(), PlayerError>` (再生の成功、またはプレイヤー起動エラー)。
    - 機能: 外部コマンドを実行してMIDIプレイヤーを呼び出し、生成された音楽を聴覚で確認できるようにします。

## 関数呼び出し階層ツリー
```
main (CLIエントリーポイント)
├── load_config()
│   └── (ファイルI/O、tomlパースライブラリ呼び出し)
├── convert_mml_to_midi() (MML変換ライブラリの公開API)
│   ├── parse_mml_string() (パス1: MML文字列 → トークン)
│   │   └── tree-sitter のパーサーコアを呼び出し
│   ├── build_abstract_syntax_tree() (パス2: トークン → AST)
│   ├── generate_midi_events() (パス3: AST → MIDIイベント)
│   └── create_smf_file() (パス4: MIDIイベント → SMFバイトデータ)
├── save_output_file(output.mid, ...)
├── save_output_file(pass1_tokens.json, ...) (デバッグ出力)
├── save_output_file(pass2_ast.json, ...) (デバッグ出力)
├── save_output_file(pass3_events.json, ...) (デバッグ出力)
└── play_midi_file(output.mid, config)
    └── (OSコマンド実行ライブラリを呼び出し、外部MIDIプレイヤーを起動)

---
Generated at: 2026-01-21 07:06:43 JST
