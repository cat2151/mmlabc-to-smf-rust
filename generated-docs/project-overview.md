Last updated: 2026-02-16

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) 形式の音楽データをStandard MIDI File (SMF) へ変換するRustライブラリです。
- ネイティブアプリケーション (`cat-play-mml`など) やWebブラウザ向けのWebAssembly (WASM) アプリケーションで利用可能です。
- MMLの構文解析からSMFの生成までを4段階の処理パス（トークン化、AST構築、MIDIイベント生成、SMF作成）で実現します。

## 技術スタック
- フロントエンド: WebAssembly (WASM) を用いて、Rustで記述されたロジックをウェブブラウザ上で実行可能にします。デモ用のHTMLファイルが存在し、将来的にブラウザアプリからの利用が予定されています。
- 音楽・オーディオ: Music Macro Language (MML) の解析と、Standard MIDI File (SMF) 形式での音楽データ出力に特化しています。生成されたMIDIファイルを`cat-play-mml`などの外部MIDIプレイヤーで自動再生する機能も備わっています。
- 開発ツール: **Rust** (バージョン1.70.0以上) を主要なプログラミング言語として使用し、**Cargo** をビルドシステム兼パッケージマネージャーとして利用しています。構文解析には**tree-sitter**が統合されており、MMLパーサーの生成には**Node.js**と**npx**（`tree-sitter-cli`経由）が必要です。
- テスト: Rustの組み込みテストフレームワークを利用し、ユニットテストおよび統合テスト（計35個以上）が実装されています。特定の機能（チャンネル、和音、テンポなど）ごとにテストファイルが分かれています。
- ビルドツール: **Cargo** がプロジェクトのビルドと依存関係管理を担い、**build.rs** スクリプトがtree-sitterパーサーのC言語ソースファイルの自動生成を管理します。
- 言語機能: Rustの**型システム**と**所有権モデル**を活用し、メモリ安全性と堅牢な設計を追求しています。
- 自動化・CI/CD: `build.rs` を通じたtree-sitterパーサーの自動再生成機能があります。
- 開発標準: **cargo fmt** によるコードフォーマットと、**cargo clippy** によるコード品質チェックが推奨されており、`.editorconfig`でエディタの統一設定を定義しています。

## ファイル階層ツリー
```
.
├── .editorconfig
├── .gitignore
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
│   ├── 89.md
│   ├── 91.md
│   ├── 92.md
│   └── 93.md
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
        └── tree-sitter-mml.wasm
```

## ファイル詳細説明
-   `.editorconfig`: 異なる開発環境間でのコードスタイルの一貫性を保つための設定ファイルです。
-   `.gitignore`: Gitバージョン管理システムで追跡しないファイルやディレクトリを指定します。
-   `Cargo.lock`: Rustプロジェクトの依存関係の正確なバージョンを記録し、再現可能なビルドを保証します。
-   `Cargo.toml`: Rustプロジェクトのビルド設定、メタデータ、および依存関係を定義するマニフェストファイルです。
-   `IMPLEMENTATION_REPORT.md`: プロジェクトの実装に関する詳細なレポートやメモを記述するMarkdownファイルです。
-   `LICENSE`: プロジェクトのライセンス情報（MIT License）を記載しています。
-   `OPTION_A_IMPLEMENTATION.md`: 特定の実装方針に関する文書です。
-   `README.ja.md`: プロジェクトの日本語版説明書で、概要、使い方、開発状況などが記述されています。
-   `README.md`: プロジェクトの英語版説明書です。
-   `_config.yml`: Jekyllなどの静的サイトジェネレーターで利用される設定ファイルです。
-   `build.rs`: Rustプロジェクトのカスタムビルドスクリプトで、主にtree-sitterパーサーのC言語ソースファイルの自動生成に使用されます。
-   `demo/`: WebブラウザでMML変換を試すためのデモアプリケーション関連ファイル群です。
    -   `index.html`: デモのウェブページ本体です。
    -   `package.json`: デモのフロントエンド（JavaScript）の依存関係とスクリプトを定義します。
-   `demo-library/`: ライブラリとしての利用例を示すデモ関連ファイル群です。
    -   `index.html`: ライブラリデモのウェブページ本体です。
    -   `package.json`: ライブラリデモのフロントエンド（JavaScript）の依存関係とスクリプトを定義します。
-   `generated-docs/`: プロジェクトから自動生成されたドキュメントを格納します。
    -   `development-status-generated-prompt.md`: 開発状況に関する自動生成レポートです。
-   `googled947dc864c270e07.html`: Googleサイト認証のために配置されるファイルです。
-   `issue-notes/`: 開発中の課題や改善点に関するメモを個別のMarkdownファイルとして保存しています。
-   `mmlabc-to-smf-rust.toml.example`: 外部MIDIプレイヤーを設定するための設定ファイルのサンプルです。
-   `mmlabc-to-smf-wasm/`: RustコードをWebAssemblyにコンパイルし、Webブラウザから利用可能にするためのクレートです。
    -   `src/lib.rs`: WASMバインディングと、Rustコアライブラリへのブリッジを実装します。
-   `package.json`: プロジェクト全体のJavaScript依存関係（主に`tree-sitter-cli`）を管理します。
-   `scripts/`: ビルドプロセスやデモ環境の準備に用いるシェルスクリプト群です。
    -   `build-demo.sh`: デモをビルドするためのスクリプトです。
    -   `transform-demo-paths.sh`: デモ関連のファイルパスを調整するためのスクリプトです。
-   `src/`: プロジェクトの主要なRustソースコードが配置されています。
    -   `config.rs`: 外部MIDIプレイヤーやデバッグ出力などの設定を管理するモジュールです。
    -   `lib.rs`: `mmlabc-to-smf-rust` クレートの公開インターフェースを定義するライブラリルートです。
    -   `main.rs`: コマンドラインインターフェース (CLI) アプリケーションのエントリーポイントです。
    -   `pass1_parser.rs`: MML文字列を解析し、トークン列を生成する「パス1」のロジックを含みます。
    -   `pass2_ast.rs`: トークン列から抽象構文木 (AST) を構築する「パス2」のロジックを含みます。
    -   `pass3_events.rs`: ASTをMIDIイベントのリストに変換する「パス3」のロジックを含みます。
    -   `pass4_midi.rs`: MIDIイベントのリストからStandard MIDI Fileを生成する「パス4」のロジックを含みます。
    -   `tree_sitter_mml.rs`: tree-sitterによって生成されたMMLパーサーとRustコードを統合するモジュールです。
    -   `types.rs`: プロジェクト全体で利用される共通のデータ構造や列挙型を定義します。
-   `tests/`: プロジェクトのテストスイートが格納されています。各ファイルが特定の機能やパスのテストを担当します。
-   `tree-sitter-mml/`: MML文法のためのtree-sitterパーサー定義とその生成物を含みます。
    -   `grammar.js`: MMLの構文ルールを定義するJavaScriptファイルです。
    -   `package.json`: tree-sitter-mmlパーサー自体の依存関係を管理します。
    -   `src/`: tree-sitterによって`grammar.js`から生成されたC言語ソースファイルが含まれます。
        -   `parser.c`: 実際にMML文字列を解析するC言語パーサーのコードです。
        -   `tree_sitter/`: tree-sitterパーサーのランタイムに必要なヘッダーファイルが含まれます。
    -   `tree-sitter-mml.wasm`: MMLパーサーのWebAssemblyバイナリです。

## 関数詳細説明
プロジェクト情報には個々の関数の詳細なシグネチャは提供されていませんが、以下の主要な処理ロジックが存在すると考えられます。

-   `main` (CLI エントリーポイント):
    -   **役割**: コマンドライン引数を解釈し、MML変換処理を起動し、結果のMIDIファイルを管理します。
    -   **機能**: 引数解析（MML文字列、出力ファイル、再生オプションなど）、MMLからSMFへの変換実行、変換されたSMFの保存、設定された外部プレイヤーによるMIDI再生。
-   `convert_mml_to_smf` (MMLからSMFへの変換の中核):
    -   **役割**: MML文字列を受け取り、4パスアーキテクチャ全体をオーケストレーションしてStandard MIDI Fileのバイナリデータを生成します。
    -   **機能**: `parse_mml`、`build_ast`、`generate_midi_events`、`create_smf`の各パスを順に呼び出し、MML変換の主要な流れを制御します。
-   `parse_mml` (パス1: トークン解析):
    -   **役割**: 入力されたMML文字列をMML構文の基本的な要素（トークン）に分解します。
    -   **機能**: tree-sitterパーサーを利用してMML文字列を解析し、識別子、音符、コマンドなどの意味のある単位に構造化します。
-   `build_ast` (パス2: AST変換):
    -   **役割**: トークン列を解析し、MMLの構造を階層的に表現する抽象構文木（AST）を構築します。
    -   **機能**: パス1で生成されたトークンを用いて、MMLの論理的な構造（チャンネル、シーケンス、音符、コマンドなど）をツリー形式で表現します。
-   `generate_midi_events` (パス3: MIDIイベント生成):
    -   **役割**: ASTを解釈し、Standard MIDI Fileの仕様に基づいたMIDIイベントのシーケンスを生成します。
    -   **機能**: ASTのノードをトラバースしながら、ノートオン/オフ、テンポ変更、プログラムチェンジなどの具体的なMIDIイベントをタイムスタンプ付きで生成します。
-   `create_smf` (パス4: MIDIファイル作成):
    -   **役割**: 生成されたMIDIイベントのリストから、最終的なStandard MIDI File形式のバイナリデータを作成します。
    -   **機能**: MIDIイベントのリストをチャンク構造にまとめ、SMFのヘッダ、トラックなどのフォーマットに従ってバイナリデータを構築します。
-   `load_config` (設定読み込み):
    -   **役割**: プロジェクト固有の設定ファイル（例: `mmlabc-to-smf-rust.toml`）を読み込み、アプリケーションの動作に反映させます。
    -   **機能**: 外部MIDIプレイヤーのパスやデバッグ出力オプションなどを設定から取得します。
-   `play_midi` (外部プレイヤー呼び出し):
    -   **役割**: 生成されたStandard MIDI Fileを指定された外部のMIDIプレイヤーで再生します。
    -   **機能**: 設定ファイルで指定されたプレイヤーコマンドを呼び出し、一時的に保存されたMIDIファイルを再生します。

## 関数呼び出し階層ツリー
提供された情報から直接的な詳細な関数呼び出し階層は分析できませんでしたが、プロジェクトの「4パスアーキテクチャ」という論理的な処理フローに基づき、主要な処理ステップの呼び出し関係を以下に示します。

```
main (CLI エントリーポイント)
└── convert_mml_to_smf (MMLからSMFへの変換の中核)
    ├── parse_mml (パス1: MMLトークン解析)
    ├── build_ast (パス2: AST変換)
    ├── generate_midi_events (パス3: MIDIイベント生成)
    └── create_smf (パス4: SMFファイル作成)

---
Generated at: 2026-02-16 07:06:41 JST
