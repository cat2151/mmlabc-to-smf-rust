Last updated: 2026-02-15

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) 形式の文字列を、Standard MIDI File (SMF) へ変換するRustライブラリです。
- 4パスアーキテクチャを採用し、MMLの解析からMIDIイベント生成、SMF作成まで一貫して処理します。
- ネイティブアプリケーションだけでなく、WebAssembly (WASM) ライブラリとしても利用可能な汎用性の高い設計です。

## 技術スタック
- フロントエンド: WebAssembly (WASM): ブラウザ上で本ライブラリを利用するためのWeb技術。`demo/index.html`や`mmlabc-to-smf-wasm`クレートで利用されています。
- 音楽・オーディオ:
    - Music Macro Language (MML): 音楽をテキスト形式で記述するための言語。
    - Standard MIDI File (SMF): 電子楽器間で演奏情報をやり取りするための標準ファイルフォーマット。
    - `cat-play-mml`: 生成されたMIDIファイルを自動再生するための外部コマンド（デフォルト）。
    - `timidity`, `fluidsynth`, `vlc`: カスタムMIDIプレイヤーとして設定可能なソフトウェア。
- 開発ツール:
    - Rust: 安全性とパフォーマンスに優れたプログラミング言語。プロジェクトの主要言語です。
    - Cargo: Rustのビルドシステムおよびパッケージマネージャー。依存関係管理、ビルド、テストなどに使用されます。
    - tree-sitter-cli: 構文解析器(パーサー)を生成するためのコマンドラインツール。
    - Node.js, npx: tree-sitterパーサーの生成に必要なJavaScriptランタイムとパッケージ実行ツール。
- テスト: `cargo test`: Rustプロジェクトのユニットテストおよび統合テストを実行するためのコマンド。現在35個のテストケースが実装されています。
- ビルドツール: Cargo: Rustプロジェクトのビルドを管理します。`tree-sitter-cli`: MMLの構文解析器（パーサー）のC言語ソースファイルを生成します。
- 言語機能: Rustの型システムと所有権モデル: メモリ安全性を保証し、堅牢なアプリケーション開発を可能にします。
- 自動化・CI/CD:
    - `cargo clippy`: コードの品質とスタイルをチェックするリンター。
    - `cargo fmt`: コードの自動フォーマットツール。
    - `build.rs`: tree-sitterパーサーファイルの自動再生成を管理するビルドスクリプト。
- 開発標準:
    - .editorconfig: 異なるエディタやIDE間でコードのスタイルを統一するための設定ファイル。
    - .gitignore: Gitのバージョン管理から除外するファイルやディレクトリを指定するファイル。

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
│   └── development-status-generated-prompt.md
├── googled947dc864c270e07.html
├── issue-notes/
│   ├── 39.md
│   ├── 44.md
│   ├── 85.md
│   └── 87.md
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
- **`.editorconfig`**: コーディングスタイル（インデント、改行コードなど）を定義し、異なるエディタ間での一貫性を保つための設定ファイルです。
- **`.gitignore`**: Gitがバージョン管理から無視するファイルやディレクトリ（ビルド生成物、一時ファイルなど）を指定するファイルです。
- **`.vscode/settings.json`**: Visual Studio Codeエディタのワークスペース固有の設定を定義します。
- **`Cargo.lock`**: Cargoによって解決されたプロジェクトの依存関係の正確なバージョンを記録します。
- **`Cargo.toml`**: Rustプロジェクトのメタデータ、依存関係、ビルド設定を定義するマニフェストファイルです。
- **`IMPLEMENTATION_REPORT.md`**: 実装に関する詳細なレポートやメモを記述するMarkdownファイルです。
- **`LICENSE`**: プロジェクトのライセンス情報（MIT License）を記載したファイルです。
- **`OPTION_A_IMPLEMENTATION.md`**: 実装オプションAに関するドキュメントです。
- **`README.ja.md`**: プロジェクトの日本語版の概要、使い方、開発情報などを記述したメインドキュメントです。
- **`README.md`**: プロジェクトの英語版の概要、使い方、開発情報などを記述したメインドキュメントです。
- **`_config.yml`**: Jekyllなどの静的サイトジェネレータで使用される設定ファイルですが、このプロジェクトでは利用用途は不明です。
- **`build.rs`**: Rustのカスタムビルドスクリプトです。tree-sitterパーサーファイルの自動再生成などを担当します。
- **`demo/`**: ブラウザ向けデモに関連するファイル群を格納するディレクトリです。
    - **`demo/index.html`**: ブラウザデモのメインHTMLファイル。
    - **`demo/FEATURES.md`**: デモの機能に関するドキュメント。
- **`demo-library/`**: ライブラリ形式でデモを行うためのファイル群。
    - **`demo-library/index.html`**: ライブラリデモのHTMLファイル。
- **`generated-docs/development-status-generated-prompt.md`**: 自動生成された開発状況に関するドキュメント。
- **`googled947dc864c270e07.html`**: Googleサイト認証用のファイル。
- **`issue-notes/`**: 過去の課題や検討事項に関するノートファイル群です。
- **`mmlabc-to-smf-rust.toml.example`**: カスタムMIDIプレイヤーの設定例が記述されたファイルです。
- **`mmlabc-to-smf-wasm/`**: WebAssembly (WASM) 用のサブクレート。
    - **`mmlabc-to-smf-wasm/src/lib.rs`**: WASMライブラリのエントリーポイント。
- **`package.json`**: Node.jsプロジェクトのメタデータと依存関係を定義するファイル。主に`tree-sitter-cli`の依存関係に用いられます。
- **`scripts/`**: ビルドやデモ関連のスクリプトを格納するディレクトリです。
    - **`scripts/build-demo.sh`**: デモをビルドするためのシェルスクリプト。
    - **`scripts/transform-demo-paths.sh`**: デモのパスを変換するためのシェルスクリプト。
- **`src/`**: プロジェクトの主要なRustソースコードが格納されているディレクトリです。
    - **`src/config.rs`**: アプリケーションの設定を管理するコード。
    - **`src/lib.rs`**: ライブラリクレートのルートファイル。他のモジュールを公開し、主要なAPIを定義します。
    - **`src/main.rs`**: CLIアプリケーションのエントリーポイント。コマンドライン引数の解析と主要な処理フローを制御します。
    - **`src/pass1_parser.rs`**: MML文字列をトークンに解析する「パス1」の処理を担当します。tree-sitterパーサーを利用します。
    - **`src/pass2_ast.rs`**: トークンから抽象構文木（AST）への変換を行う「パス2」の処理を担当します。
    - **`src/pass3_events.rs`**: ASTからMIDIイベントを生成する「パス3」の処理を担当します。
    - **`src/pass4_midi.rs`**: 生成されたMIDIイベントからStandard MIDI Fileを作成する「パス4」の処理を担当します。
    - **`src/tree_sitter_mml.rs`**: tree-sitter MMLパーサーとの統合を扱うモジュール。
    - **`src/types.rs`**: プロジェクト全体で共有されるデータ構造や型定義を格納します。
- **`tests/`**: プロジェクトのテストコードが格納されているディレクトリです。
    - **`tests/integration_test.rs`**: 統合テスト。
    - **`tests/test_channel.rs`**: チャンネル機能のテスト。
    - **`tests/test_chord.rs`**: 和音機能のテスト。
    - **`tests/test_cli.rs`**: コマンドラインインターフェースのテスト。
    - **`tests/test_config.rs`**: 設定ファイル読み込みのテスト。
    - **`tests/test_dotted_notes.rs`**: 付点音符のテスト。
    - **`tests/test_drum_channel.rs`**: ドラムチャンネルのテスト。
    - **`tests/test_key_transpose.rs`**: キー転調のテスト。
    - **`tests/test_length.rs`**: 音長指定のテスト。
    - **`tests/test_modifier.rs`**: 各種修飾子（例：シャープ、フラット）のテスト。
    - **`tests/test_note_length.rs`**: 音符の長さに関するテスト。
    - **`tests/test_octave.rs`**: オクターブ機能のテスト。
    - **`tests/test_pass1.rs`**: パス1（トークン解析）のテスト。
    - **`tests/test_pass2.rs`**: パス2（AST変換）のテスト。
    - **`tests/test_pass3.rs`**: パス3（MIDIイベント生成）のテスト。
    - **`tests/test_pass4.rs`**: パス4（MIDIファイル作成）のテスト。
    - **`tests/test_program_change.rs`**: プログラムチェンジ（音色変更）のテスト。
    - **`tests/test_rest.rs`**: 休符のテスト。
    - **`tests/test_tempo.rs`**: テンポ変更のテスト。
    - **`tests/test_velocity.rs`**: 音量（ベロシティ）変更のテスト。
- **`tree-sitter-mml/`**: MML言語のtree-sitterパーサーを定義するディレクトリです。
    - **`tree-sitter-mml/grammar.js`**: MML言語の構文ルールを定義するJavaScriptファイル。
    - **`tree-sitter-mml/package.json`**: tree-sitterパーサーの依存関係を定義。
    - **`tree-sitter-mml/src/`**: 生成されたC言語ソースコードやJSON定義を格納。
        - **`tree-sitter-mml/src/grammar.json`**: 生成された文法定義のJSON形式。
        - **`tree-sitter-mml/src/node-types.json`**: 生成されたASTノードタイプのJSON形式。
        - **`tree-sitter-mml/src/parser.c`**: tree-sitterによって生成されたC言語パーサーのソースコード。
        - **`tree-sitter-mml/src/tree_sitter/`**: tree-sitterランタイムに必要なC言語ヘッダーファイル。
    - **`tree-sitter-mml/tree-sitter-mml.wasm`**: WebAssembly形式にコンパイルされたMMLパーサー。

## 関数詳細説明
※プロジェクト情報から具体的な関数名が直接確認できないため、READMEに記載されている「4パスアーキテクチャ」の各ステップを概念的な関数として説明します。

- **`process_mml_to_midi(mml_string: &str, output_path: Option<&str>, no_play: bool) -> Result<(), Error>`**
    - **役割**: MML文字列を解析し、Standard MIDI Fileを生成する全体のオーケストレーションを担う高レベル関数。コマンドラインインターフェースからの入力処理や、MIDIファイルの保存・再生オプションを処理します。
    - **引数**:
        - `mml_string`: 変換対象のMML文字列。
        - `output_path`: 生成されるMIDIファイルのパス（オプション）。Noneの場合はデフォルト名を使用。
        - `no_play`: MIDI生成後に自動再生を無効にするかどうかを示すフラグ。
    - **戻り値**: 成功した場合は空のResult、エラーが発生した場合は対応するError型。
    - **機能**: 内部で以下の各パスの処理を順次呼び出し、MMLからSMFへの変換を完了させます。

- **`pass1_tokenize_mml(mml_string: &str) -> Result<Vec<Token>, Error>`**
    - **役割**: 入力されたMML文字列を、tree-sitterパーサーを用いて一連のトークンに分解します。
    - **引数**:
        - `mml_string`: トークン化するMML文字列。
    - **戻り値**: 成功した場合はトークンのベクトル、エラーが発生した場合は対応するError型。
    - **機能**: MML構文の解析の最初のステップであり、後続のAST構築のための基礎データを提供します。

- **`pass2_build_ast(tokens: Vec<Token>) -> Result<AstNode, Error>`**
    - **役割**: パス1で生成されたトークンのリストから、抽象構文木 (AST) を構築します。
    - **引数**:
        - `tokens`: パス1で生成されたMMLトークンのベクトル。
    - **戻り値**: 成功した場合はASTのルートノード、エラーが発生した場合は対応するError型。
    - **機能**: MMLの構造をより高レベルなツリー形式で表現し、論理的な意味付けを行います。

- **`pass3_generate_midi_events(ast: AstNode) -> Result<Vec<MidiEvent>, Error>`**
    - **役割**: パス2で構築されたASTを走査し、対応するStandard MIDIイベントのシーケンスを生成します。
    - **引数**:
        - `ast`: パス2で構築されたMMLの抽象構文木。
    - **戻り値**: 成功した場合はMIDIイベントのベクトル、エラーが発生した場合は対応するError型。
    - **機能**: MMLの音楽的意味（音符、テンポ、チャンネルなど）をMIDIプロトコルが理解できる形式に変換します。

- **`pass4_create_smf(midi_events: Vec<MidiEvent>, output_path: &str) -> Result<(), Error>`**
    - **役割**: パス3で生成されたMIDIイベントのシーケンスから、最終的なStandard MIDI File (.mid) を作成し、指定されたパスに保存します。
    - **引数**:
        - `midi_events`: パス3で生成されたMIDIイベントのベクトル。
        - `output_path`: 保存するMIDIファイルのファイルパス。
    - **戻り値**: 成功した場合は空のResult、エラーが発生した場合は対応するError型。
    - **機能**: MIDIファイルフォーマットの仕様に従ってデータを構造化し、バイナリファイルとして出力します。

- **`play_midi_file(file_path: &str, player_command: &str) -> Result<(), Error>`**
    - **役割**: 生成されたMIDIファイルを外部のMIDIプレイヤーコマンド (`cat-play-mml`など) を使って再生します。
    - **引数**:
        - `file_path`: 再生するMIDIファイルのパス。
        - `player_command`: 使用する外部MIDIプレイヤーのコマンド名。
    - **戻り値**: 成功した場合は空のResult、エラーが発生した場合は対応するError型。
    - **機能**: ユーザーがMMLの出力結果をすぐに聴覚で確認できるようにします。

## 関数呼び出し階層ツリー
```
main (src/main.rs)
└── process_mml_to_midi
    ├── pass1_tokenize_mml
    │   └── (tree-sitterパーサーの内部呼び出し)
    ├── pass2_build_ast
    ├── pass3_generate_midi_events
    ├── pass4_create_smf
    └── play_midi_file (オプション)

---
Generated at: 2026-02-15 07:06:14 JST
