Last updated: 2026-02-07

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) 形式の楽譜を、Standard MIDI File (SMF) に変換するRust製ライブラリです。
- MMLの解析からMIDIファイル生成までを「4パスアーキテクチャ」で構造化し、高い拡張性とデバッグ容易性を実現しています。
- ネイティブアプリケーション (`cat-play-mml`) やWebAssemblyベースのブラウザアプリでMMLを音楽として扱うための基盤を提供します。

## 技術スタック
- フロントエンド: WebAssembly (WASM) を利用してブラウザでの動作を可能にし、シンプルなHTMLファイルでデモインターフェースを提供します。
- 音楽・オーディオ: Music Macro Language (MML) を入力とし、Standard MIDI File (SMF) を出力。外部MIDIプレイヤー (`cat-play-mml`, `timidity`, `fluidsynth`, `vlc`) と連携します。
- 開発ツール: 主にRust言語 (バージョン1.70.0以上) を使用し、ビルド管理にはCargoを採用。MML構文解析にはtree-sitterパーサーとその生成ツール (tree-sitter-cli, Node.js) を統合しています。
- テスト: Rustの標準テストフレームワークとCargo testコマンドを利用し、35個の包括的なテストケースで機能の正確性を検証しています。
- ビルドツール: Rustプロジェクトのビルド、依存関係管理にCargoを使用。tree-sitterパーサーのC言語ソースファイルは、`build.rs`スクリプトと`tree-sitter-cli`により自動生成されます。
- 言語機能: Rustの強力な型システムと所有権モデルを活用し、メモリ安全で堅牢な変換ライブラリを実現しています。
- 自動化・CI/CD: `cargo clippy`によるコード品質チェックと`cargo fmt`によるコードフォーマットを導入し、開発プロセスを効率化しています。
- 開発標準: `.editorconfig`でエディタのコードスタイルを統一し、一貫性のあるコードベースを維持しています。

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
│   ├── README.md
│   ├── index.html
│   └── package.json
├── demo-library/
│   ├── README.md
│   └── index.html
├── generated-docs/
│   └── development-status-generated-prompt.md
├── googled947dc864c270e07.html
├── issue-notes/
│   ├── 14.md
│   ├── 17.md
│   ├── ... (省略: 18-69.md)
│   └── 70.md
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
上記のファイル階層ツリーにおいて、CodeQL解析ツールが生成したと見られる冗長な`_codeql_detected_source_root/`の繰り返しは、プロジェクトの本来の構造ではないため、来訪者向けの概要として主要なディレクトリとファイルのみを抽出して記載しました。

## ファイル詳細説明
-   `.editorconfig`: 異なるエディタやIDE間で一貫したコーディングスタイルを強制するための設定ファイルです。
-   `.gitignore`: Gitバージョン管理システムが追跡しないファイルやディレクトリを指定します。
-   `.vscode/settings.json`: Visual Studio Codeエディタのワークスペース固有の設定を定義し、開発環境の統一に役立ちます。
-   `Cargo.lock`: Rustプロジェクトの依存関係ツリーの正確なバージョンを記録し、再現可能なビルドを保証します。
-   `Cargo.toml`: Rustプロジェクトのマニフェストファイルで、プロジェクトのメタデータ、依存関係、ビルド設定などを定義します。
-   `IMPLEMENTATION_REPORT.md`: 実装の進捗や決定事項に関する詳細な報告書です。
-   `LICENSE`: プロジェクトがMIT Licenseで公開されていることを示し、利用条件を定めます。
-   `OPTION_A_IMPLEMENTATION.md`: 実装に関する特定の設計選択肢や代替案についての詳細な説明です。
-   `README.ja.md`: プロジェクトの日本語での概要、使い方、開発状況などを記述したメインドキュメントです。
-   `README.md`: プロジェクトの英語での概要、使い方、開発状況などを記述したメインドキュメントです。
-   `_config.yml`: （存在する場合）Jekyllなどの静的サイトジェネレータの設定ファイルです。
-   `build.rs`: Rustのカスタムビルドスクリプトです。tree-sitterパーサーのC言語ソースファイルの自動生成など、ビルド時の特殊なタスクを実行します。
-   `demo/`: ブラウザ上でMML変換を試すことができるデモアプリケーションのファイル群を含みます。
    -   `.gitignore`: デモ固有の無視ファイルを設定します。
    -   `README.md`: デモの使用方法に関する説明書です。
    -   `index.html`: デモのユーザーインターフェースを提供するHTMLファイルです。
    -   `package.json`: デモが利用するJavaScript/Node.jsの依存関係を管理します。
-   `demo-library/`: ライブラリとしての利用例を示す別のデモアプリケーションのファイル群です。
    -   `README.md`: ライブラリデモの使用方法に関する説明書です。
    -   `index.html`: ライブラリデモのユーザーインターフェースを提供するHTMLファイルです。
-   `generated-docs/`: プロジェクトのドキュメントを自動生成する際に使用される、または生成されたファイルを格納します。
    -   `development-status-generated-prompt.md`: プロジェクトの開発状況に関する自動生成された情報です。
-   `googled947dc864c270e07.html`: Google Search Consoleなどのサイト認証に使用されるファイルです。
-   `issue-notes/`: 開発中に発生した課題や検討事項に関するメモが整理されています。（来訪者向けのため、個別のファイル詳細は省略）
-   `mmlabc-to-smf-rust.toml.example`: カスタムMIDIプレイヤー設定の例を示すファイルで、ユーザーがコピーして設定を変更できます。
-   `mmlabc-to-smf-wasm/`: `mmlabc-to-smf-rust`ライブラリをWebAssembly (WASM) にコンパイルするためのサブプロジェクト（クレート）です。
    -   `Cargo.lock`: WASMクレートの依存関係の正確なバージョンを記録します。
    -   `Cargo.toml`: WASMクレートのマニフェストファイルです。
    -   `src/lib.rs`: WASMクレートのライブラリコードが記述されており、JavaScriptからの呼び出しインターフェースを提供します。
-   `package.json`: 主に`tree-sitter-cli`など、Node.jsベースの開発ツールに関する依存関係を定義します。
-   `scripts/`: ビルド、デプロイ、テストなどの開発作業を自動化するためのシェルスクリプト群です。
    -   `README.md`: スクリプトの目的と使用方法を説明します。
    -   `build-demo.sh`: デモアプリケーションをビルドするスクリプトです。
    -   `transform-demo-paths.sh`: デモ関連ファイルのパスを調整するスクリプトです。
-   `src/`: プロジェクトの主要なRustソースコードが格納されています。
    -   `config.rs`: アプリケーションの実行時設定（例: 外部MIDIプレイヤーのパス）を管理するモジュールです。
    -   `lib.rs`: `mmlabc-to-smf-rust`クレートの公開APIを提供するライブラリのエントリーポイントです。
    -   `main.rs`: コマンドラインインターフェース (CLI) のエントリーポイントであり、引数解析とMML変換フローの実行を担います。
    -   `pass1_parser.rs`: MML文字列を構文解析し、トークン列を生成する「パス1」のロジックを含みます。tree-sitterパーサーを統合しています。
    -   `pass2_ast.rs`: パス1で生成されたトークン列を抽象構文木 (AST) に変換する「パス2」のロジックです。
    -   `pass3_events.rs`: ASTをトラバースし、MIDIイベントのシーケンスを生成する「パス3」のロジックです。MMLコマンドをMIDIメッセージにマッピングします。
    -   `pass4_midi.rs`: パス3で生成されたMIDIイベントから、最終的なStandard MIDI Fileのバイナリデータを構築する「パス4」のロジックです。
    -   `tree_sitter_mml.rs`: tree-sitter MMLパーサーとのインターフェースおよび関連ロジックをカプセル化しています。
    -   `types.rs`: プロジェクト全体で共有されるデータ構造や列挙型などの共通型定義を格納します。
-   `tests/`: プロジェクトの単体テストおよび統合テストファイル群です。
    -   `integration_test.rs`: 変換パイプライン全体を検証する統合テストが含まれます。
    -   `test_channel.rs`: MMLのチャンネル分離機能に関するテストです。
    -   `test_chord.rs`: MMLの和音機能に関するテストです。
    -   `test_cli.rs`: コマンドラインインターフェースの動作を検証するテストです。
    -   `test_config.rs`: 設定ファイルの読み込みと適用に関するテストです。
    -   `test_dotted_notes.rs`: 付点音符のMML記法に関するテストです。
    -   `test_drum_channel.rs`: ドラムチャンネルのMML記法に関するテストです。
    -   `test_key_transpose.rs`: キーの移調機能に関するテストです。
    -   `test_length.rs`: 音符の長さ指定機能に関するテストです。
    -   `test_modifier.rs`: 音符の修飾子（例:シャープ、フラット）に関するテストです。
    -   `test_note_length.rs`: 音符の長さを特に検証するテストです。
    -   `test_octave.rs`: オクターブ変更機能に関するテストです。
    -   `test_pass1.rs`: パス1（トークン解析）の機能と正確性を検証するテストです。
    -   `test_pass2.rs`: パス2（AST変換）の機能と正確性を検証するテストです。
    -   `test_pass3.rs`: パス3（MIDIイベント生成）の機能と正確性を検証するテストです。
    -   `test_pass4.rs`: パス4（MIDIファイル作成）の機能と正確性を検証するテストです。
    -   `test_program_change.rs`: プログラムチェンジ（音色変更）のMML記法に関するテストです。
    -   `test_rest.rs`: 休符のMML記法に関するテストです。
    -   `test_tempo.rs`: テンポ変更機能に関するテストです。
    -   `test_velocity.rs`: 音の強弱（ベロシティ）変更機能に関するテストです。
-   `tree-sitter-mml/`: MML言語用のtree-sitterパーサーを定義するサブディレクトリです。
    -   `grammar.js`: MMLの構文規則を記述したJavaScriptファイルで、tree-sitterパーサーの基になります。
    -   `package.json`: `tree-sitter-mml`パーサー固有のNode.js依存関係を管理します。
    -   `src/`: 生成されたパーサーのソースファイルとメタデータが格納されます。
        -   `grammar.json`: `grammar.js`から生成されるMML文法のJSON表現です。
        -   `node-types.json`: `grammar.js`から生成される抽象構文木のノードタイプ定義です。
        -   `parser.c`: `grammar.js`から自動生成されるMMLパーサーのC言語ソースコードです。
        -   `tree_sitter/`: tree-sitterライブラリのコア部分を構成するCヘッダーファイル群です。
            -   `alloc.h`: メモリ割り当てに関する宣言。
            -   `array.h`: 動的配列の実装に関する宣言。
            -   `parser.h`: tree-sitterパーサーの公開インターフェースに関する宣言。
    -   `tree-sitter-mml.wasm`: MMLパーサーをWebAssembly形式にコンパイルしたバイナリファイルで、ブラウザ環境での利用を可能にします。

## 関数詳細説明
提供された情報では個別のRust関数の詳細なシグネチャは明示されていませんが、プロジェクトの「4パスアーキテクチャ」に基づいて、主要な機能を提供する関数群の役割と概略を説明します。

-   `main()` (src/main.rs):
    -   **役割**: CLIアプリケーションのエントリーポイント。コマンドライン引数を解析し、MML変換プロセス全体を制御します。
    -   **引数**: `None` (コマンドライン引数は`clap`クレートなどで内部的に処理されます)
    -   **戻り値**: `Result<(), Box<dyn Error>>` (成功またはエラー)
    -   **機能**: 設定の読み込み、`mml_to_smf`関数を呼び出してMML変換を実行し、生成されたMIDIファイルの保存とオプションでの自動再生を管理します。

-   `convert_mml_to_smf()` (src/lib.rs):
    -   **役割**: ライブラリの主要な公開関数。MML文字列を入力として受け取り、Standard MIDI Fileのバイナリデータへの変換プロセス全体を実行します。WASMバインディングでも利用されます。
    -   **引数**: `mml_string: &str` (MML形式の文字列), `options: &ConversionOptions` (変換オプション、例えばデバッグ出力設定など)
    -   **戻り値**: `Result<Vec<u8>, ConversionError>` (SMFバイナリデータまたはエラー)
    -   **機能**: 内部的に4パスの各処理 (`parse_mml`, `build_ast`, `generate_midi_events`, `create_smf_file`) を順に呼び出し、MMLをSMFに変換します。

-   `parse_mml()` (src/pass1_parser.rs):
    -   **役割**: 「パス1」の処理を担い、入力されたMML文字列をtree-sitterパーサーを用いて解析し、構文要素を識別可能なトークン列に変換します。
    -   **引数**: `mml_string: &str` (MML形式の文字列)
    -   **戻り値**: `Result<Vec<MmlToken>, ParseError>` (MMLトークンのリストまたはエラー)
    -   **機能**: MML文法に基づいて文字列を走査し、音符、コマンド、チャンネル区切りなどを個別の意味を持つトークンに分割します。

-   `build_ast()` (src/pass2_ast.rs):
    -   **役割**: 「パス2」の処理を担い、`parse_mml`によって生成されたトークン列からMMLの抽象構文木 (AST) を構築します。MMLの論理構造を表現します。
    -   **引数**: `tokens: Vec<MmlToken>` (MMLトークンのリスト)
    -   **戻り値**: `Result<MmlAst, AstBuildError>` (MMLの抽象構文木またはエラー)
    -   **機能**: トークン間の関係性を解釈し、MMLの階層的な構造（例: チャンネル、シーケンス、音符グループ）をASTとして表現します。

-   `generate_midi_events()` (src/pass3_events.rs):
    -   **役割**: 「パス3」の処理を担い、ASTを走査して具体的なStandard MIDI Fileイベントのシーケンスを生成します。MMLの音楽的意味をMIDIメッセージに変換します。
    -   **引数**: `ast: MmlAst` (MMLの抽象構文木), `context: &mut MidiContext` (MIDI生成のための状態)
    -   **戻り値**: `Result<Vec<MidiEvent>, EventGenerationError>` (MIDIイベントのリストまたはエラー)
    -   **機能**: AST内の音符、テンポ、オクターブ、音量、プログラムチェンジ、チャンネルなどのMMLコマンドを、MIDIノートオン/オフ、テンポチェンジなどのイベントに変換します。

-   `create_smf_file()` (src/pass4_midi.rs):
    -   **役割**: 「パス4」の処理を担い、`generate_midi_events`によって生成されたMIDIイベントのリストから、Standard MIDI Fileフォーマットに準拠したバイナリデータを組み立てます。
    -   **引数**: `midi_events: Vec<MidiEvent>` (MIDIイベントのリスト)
    -   **戻り値**: `Result<Vec<u8>, MidiFileError>` (SMFバイナリデータまたはエラー)
    -   **機能**: MIDIイベントを適切なチャンク構造（ヘッダチャンク、トラックチャンク）に配置し、タイムスタンプをデルタタイムに変換して、標準的なSMFバイナリファイルを生成します。

-   `load_config()` (src/config.rs):
    -   **役割**: アプリケーションの実行時設定を管理するモジュール。特に外部MIDIプレイヤーのパスなどの設定をファイルから読み込みます。
    -   **引数**: `path: Option<&Path>` (設定ファイルパス、デフォルトは`mmlabc-to-smf-rust.toml`)
    -   **戻り値**: `Result<AppConfig, ConfigError>` (アプリケーション設定構造体またはエラー)
    -   **機能**: 指定されたパスまたはデフォルトパスからTOML形式の設定ファイルを解析し、`AppConfig`構造体として提供します。

-   `get_mml_parser()` (src/tree_sitter_mml.rs):
    -   **役割**: tree-sitter MMLパーサーのインスタンスを初期化し、MML文字列の解析に利用できるように提供します。
    -   **引数**: `None`
    -   **戻り値**: `tree_sitter::Parser` (初期化されたMMLパーサーインスタンス)
    -   **機能**: tree-sitterのランタイムライブラリをロードし、MML文法定義に基づいてパーサーを構築します。

## 関数呼び出し階層ツリー
提供された情報からは具体的な関数呼び出しの動的な階層は分析できませんでしたが、プロジェクトの「4パスアーキテクチャ」と各ファイルの役割に基づいて、論理的な処理フローを以下に示します。

```
`main` (src/main.rs) - CLIアプリケーションのエントリーポイント
├── `config::load_config` (設定読み込み)
└── `mml_to_smf::convert_mml_to_smf` (ライブラリの主要な変換API)
    ├── `tree_sitter_mml::get_mml_parser` (MMLパーサーの取得)
    ├── `pass1_parser::parse_mml` (MML文字列をトークンに解析 - パス1)
    │   └── (内部的にtree-sitterのパーサー機能を利用)
    ├── `pass2_ast::build_ast` (トークンからASTを構築 - パス2)
    ├── `pass3_events::generate_midi_events` (ASTからMIDIイベントを生成 - パス3)
    └── `pass4_midi::create_smf_file` (MIDIイベントからSMFを生成 - パス4)

`mmlabc_to_smf_wasm::lib::convert_mml_string` (mmlabc-to-smf-wasm/src/lib.rs) - WASM公開APIを仮定
└── `mml_to_smf::convert_mml_to_smf` (コアライブラリの変換APIを呼び出し)

---
Generated at: 2026-02-07 07:07:10 JST
