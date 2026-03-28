Last updated: 2026-03-29

# Project Overview

## プロジェクト概要
- 本プロジェクトは、Music Macro Language（MML）形式の文字列をStandard MIDI File（SMF）へ変換するRust製ライブラリです。
- MMLの構文解析からMIDIファイル生成までを4パスアーキテクチャで処理し、高い変換精度と拡張性を提供します。
- ネイティブCLIツールおよびWebAssembly（WASM）対応のWebデモとして利用可能で、音楽制作者や開発者向けにMMLの可能性を広げます。

## 技術スタック
- フロントエンド:
    - **HTML**: Webデモのマークアップに使用されます。
    - **TypeScript**: WebデモのインタラクティブなUIとロジックを実装するために使用されます。
    - **WebAssembly (WASM)**: Rustで書かれたコア変換ロジックをWebブラウザで実行するために利用されます。
    - **Tone.js**: Webデモで生成されたMIDIデータをブラウザ上で即座に再生するために使われるオーディオフレームワークです。
- 音楽・オーディオ:
    - **Music Macro Language (MML)**: 音楽をテキストで記述するための言語で、本プロジェクトの入力形式です。
    - **Standard MIDI File (SMF)**: 電子楽器間で音楽データを交換するための標準フォーマットで、本プロジェクトの出力形式です。
- 開発ツール:
    - **Rust**: 高い安全性とパフォーマンスを特徴とするシステムプログラミング言語で、ライブラリのコア部分に使用されています。
    - **Cargo**: Rustの公式パッケージマネージャー兼ビルドシステムです。依存関係の管理、ビルド、テスト、ドキュメント生成などを担当します。
    - **Node.js/npm/npx**: `tree-sitter`パーサーの生成や、TypeScriptのビルド、Webデモのテストなど、JavaScript/TypeScript関連の開発フローに使用されます。
    - **git**: バージョン管理システムとして使用されます。
- テスト:
    - **`cargo test`**: Rustコードのユニットテストおよび統合テストを実行するためのコマンドです。
    - **Node.jsの`node:test`, `node:assert/strict`**: WebデモのTypeScript/JavaScriptコードのテストフレームワークとアサーションライブラリです。
- ビルドツール:
    - **`npx tree-sitter generate`**: MMLの文法定義からC言語のパーサーファイルを生成するコマンドです。
- 言語機能:
    - **Rust 1.70.0以上**: プロジェクトの主要な開発言語です。
    - **C言語**: `tree-sitter`によって生成されるパーサーのコードに使用されます。
- 自動化・CI/CD:
    - (特筆すべきCI/CDツールは明示されていませんが、`cargo clippy`や`cargo fmt`などの開発標準ツールはCIパイプラインに組み込まれることが想定されます。)
- 開発標準:
    - **`cargo clippy`**: Rustコードの品質と慣用的な記述をチェックするためのリンターです。
    - **`cargo fmt`**: Rustコードのフォーマットを自動的に整形し、コードスタイルを統一します。

## ファイル階層ツリー
```
.
├── .editorconfig                                # コードエディタの設定ファイル
├── .gitignore                                   # Git管理から除外するファイル指定
├── .vscode/                                     # Visual Studio Codeの設定ディレクトリ
│   └── settings.json                            # VS Codeワークスペース設定
├── Cargo.lock                                   # Cargoの依存関係ロックファイル
├── Cargo.toml                                   # Rustプロジェクトの設定と依存関係
├── IMPLEMENTATION_REPORT.md                     # 実装に関する詳細レポート
├── LICENSE                                      # プロジェクトのライセンス情報
├── OPTION_A_IMPLEMENTATION.md                   # 特定の実装オプションに関するドキュメント
├── README.ja.md                                 # 日本語版のREADMEファイル
├── README.md                                    # 英語版のREADMEファイル (このプロジェクトの概要)
├── _config.yml                                  # Jekyllなど、サイト設定に関するYAMLファイル
├── build.rs                                     # Rustのビルドスクリプト (主にtree-sitterパーサー生成に使用)
├── demo/                                        # Webブラウザ向けデモアプリケーションのルート
│   ├── .gitignore                               # デモ関連のGit無視ファイル
│   ├── FEATURES.md                              # デモの機能リスト
│   ├── README.md                                # デモのREADME
│   ├── index.html                               # デモのメインHTMLファイル
│   ├── package.json                             # デモのNode.jsパッケージ定義
│   ├── src/                                     # デモのTypeScriptソースコード
│   │   ├── audioPlayback.ts                     # オーディオ再生処理
│   │   ├── audioRenderer.ts                     # オーディオ波形レンダリングと生成
│   │   ├── main.ts                              # デモのメインエントリポイント
│   │   ├── midiReader.ts                        # MIDIバイナリ解析ユーティリティ
│   │   ├── mmlConverter.ts                      # MMLからSMFへのWASM変換呼び出し
│   │   ├── parseMidiNotes.ts                    # MIDIノートイベント解析
│   │   ├── smfToYm2151.ts                       # SMFからYM2151ログへの変換
│   │   ├── state.ts                             # アプリケーションの状態管理
│   │   ├── treeToJSON.ts                        # Tree-sitter ASTのJSON変換
│   │   ├── ui.ts                                # ユーザーインターフェース操作
│   │   ├── visualization.ts                     # オーディオ波形視覚化
│   │   └── wavExport.ts                         # WAVファイルエクスポート機能
│   ├── test-loader.mjs                          # Node.jsテストローダー
│   └── test-register.mjs                        # Node.jsテスト登録スクリプト
├── demo-library/                                # デモライブラリ関連ファイル
│   ├── index.html                               # デモライブラリのHTMLファイル
│   └── package.json                             # デモライブラリのNode.jsパッケージ定義
├── generated-docs/                              # 自動生成されたドキュメント
│   └── development-status-generated-prompt.md   # 開発ステータス生成プロンプトの記録
├── googled947dc864c270e07.html                   # Googleサイト認証用ファイル
├── issue-notes/                                 # 課題に関するメモ
│   ├── 103.md                                   # 課題103のメモ
│   ├── 123.md                                   # 課題123のメモ
│   ├── 131.md                                   # 課題131のメモ
│   ├── 39.md                                    # 課題39のメモ
│   └── 44.md                                    # 課題44のメモ
├── mmlabc-to-smf-rust.toml.example              # 外部SMFプレイヤー設定のサンプルファイル
├── mmlabc-to-smf-wasm/                          # WebAssembly (WASM) クレート
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                               # WASMバインディングを持つライブラリのルート
│       └── token_extractor.rs                   # WASM向けにトークン抽出機能を提供
├── package.json                                 # プロジェクト全体のNode.jsパッケージ定義 (root)
├── scripts/                                     # シェルスクリプトディレクトリ
│   ├── README.md                                # スクリプトのREADME
│   ├── build-demo.sh                            # デモをビルドするスクリプト
│   └── transform-demo-paths.sh                  # デモのパスを変換するスクリプト
├── src/                                         # Rustのコアライブラリソース
│   ├── attachment_json.rs                       # JSONデバッグ出力機能
│   ├── config.rs                                # アプリケーション設定の読み込みと管理
│   ├── lib.rs                                   # Rustライブラリのメインエントリーポイント
│   ├── main.rs                                  # CLIアプリケーションのメインエントリーポイント
│   ├── mml_preprocessor.rs                      # MML文字列の前処理ロジック
│   ├── pass1_parser.rs                          # MMLをトークン化する最初のパス
│   ├── pass2_ast.rs                             # トークンから抽象構文木(AST)への変換パス
│   ├── pass3_events.rs                          # ASTからMIDIイベントへの生成パス
│   ├── pass4_midi.rs                            # MIDIイベントからSMFへの最終変換パス
│   ├── tree_sitter_mml.rs                       # tree-sitter MMLパーサーとの統合
│   └── types.rs                                 # プロジェクト共通の型定義
├── tests/                                       # Rustのテストファイル
│   ├── integration_test.rs                      # 統合テスト
│   ├── test_attachment_json.rs                  # attachment_jsonモジュールのテスト
│   ├── test_c1_vs_c64.rs                        # C1とC64の音符関連テスト
│   ├── test_channel.rs                          # チャンネル機能のテスト
│   ├── test_chord.rs                            # 和音機能のテスト
│   ├── test_cli.rs                              # コマンドラインインターフェースのテスト
│   ├── test_config.rs                           # 設定読み込み機能のテスト
│   ├── test_dotted_notes.rs                     # 付点音符のテスト
│   ├── test_drum_channel.rs                     # ドラムチャンネルのテスト
│   ├── test_key_transpose.rs                    # キー転調のテスト
│   ├── test_length.rs                           # 音長指定のテスト
│   ├── test_modifier.rs                         # 音符修飾子のテスト
│   ├── test_note_length.rs                      # 音符長のテスト
│   ├── test_octave.rs                           # オクターブ指定のテスト
│   ├── test_pass1.rs                            # パス1 (トークン解析) のテスト
│   ├── test_pass2.rs                            # パス2 (AST変換) のテスト
│   ├── test_pass3.rs                            # パス3 (MIDIイベント生成) のテスト
│   ├── test_pass4.rs                            # パス4 (SMF生成) のテスト
│   ├── test_program_change.rs                   # プログラムチェンジのテスト
│   ├── test_rest.rs                             # 休符のテスト
│   ├── test_tempo.rs                            # テンポ指定のテスト
│   └── test_velocity.rs                         # ベロシティ指定のテスト
└── tree-sitter-mml/                             # MML用のtree-sitterパーサー関連
    ├── grammar.js                               # MML言語のtree-sitter文法定義
    ├── package.json                             # tree-sitterパーサーのNode.jsパッケージ定義
    └── src/                                     # 生成されたC言語パーサーソース
        ├── grammar.json                         # 生成されたtree-sitter文法JSON
        ├── node-types.json                      # 生成されたtree-sitterノードタイプJSON
        ├── parser.c                             # MMLパーサーのC言語実装
        └── tree_sitter/                         # tree-sitterランタイムCヘッダー
            ├── alloc.h
            ├── array.h
            └── parser.h
```

## ファイル詳細説明
- **`.editorconfig`**: 異なるエディタやIDE間で一貫したコーディングスタイルを維持するための設定ファイルです。
- **`.gitignore`**: Gitがバージョン管理の対象から除外するファイルやディレクトリを指定するファイルです。
- **`.vscode/settings.json`**: Visual Studio Codeのワークスペース固有の設定を定義し、プロジェクトチーム間での開発環境の統一を支援します。
- **`Cargo.lock`**: Rustプロジェクトのビルドに使用されるすべての依存関係の正確なバージョンとチェックサムを記録します。これにより、開発環境間でのビルドの再現性が保証されます。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイルで、プロジェクト名、バージョン、著者情報、依存クレート、ビルド設定などが記述されています。
- **`IMPLEMENTATION_REPORT.md`**: プロジェクトの実装に関する詳細な報告書や進捗状況が記述されているMarkdownファイルです。
- **`LICENSE`**: プロジェクトの利用条件を定めるMITライセンスの全文が記載されています。
- **`OPTION_A_IMPLEMENTATION.md`**: 実装の選択肢Aに関する詳細な説明や検討内容が記述されているMarkdownファイルです。
- **`README.ja.md`**: プロジェクトの日本語版概要と使用方法を説明するMarkdownファイルです。
- **`README.md`**: プロジェクトの英語版概要と使用方法を説明するメインのMarkdownファイルです。
- **`_config.yml`**: 主にGitHub Pagesなどの静的サイトジェネレータで使用される設定ファイルで、サイトのメタデータやテーマなどを定義します。
- **`build.rs`**: Rustのカスタムビルドスクリプトです。このプロジェクトでは、`tree-sitter-mml`パーサーのC言語ソースコードをビルドプロセス中にコンパイルするために使用されます。
- **`demo/`**: Webブラウザで動作するMML-to-SMF変換デモアプリケーションの関連ファイルが格納されています。
    - **`demo/.gitignore`**: デモ固有のGit無視設定ファイルです。
    - **`demo/FEATURES.md`**: デモアプリケーションで利用可能な機能の一覧を説明するファイルです。
    - **`demo/README.md`**: デモアプリケーションの概要と使い方を説明するファイルです。
    - **`demo/index.html`**: デモアプリケーションのメインとなるHTMLファイルです。ユーザーインターフェースの構造を定義します。
    - **`demo/package.json`**: デモアプリケーションのNode.jsプロジェクト設定ファイルです。依存関係やスクリプトが定義されます。
    - **`demo/src/`**: デモアプリケーションのTypeScriptソースコードが格納されています。
        - **`demo/src/audioPlayback.ts`**: 生成されたMIDIデータをブラウザ上で再生するロジック（Tone.jsを使用）を担当します。
        - **`demo/src/audioRenderer.ts`**: MIDIデータからオーディオ波形をレンダリングし、可視化するロジック（Web-YM2151を使用）を実装します。
        - **`demo/src/main.ts`**: デモアプリケーションのメインエントリーポイントです。初期化処理、WASMモジュールのロード、イベントハンドリングなどを担当します。
        - **`demo/src/midiReader.ts`**: MIDIバイナリデータを解析し、構造化されたデータとして読み込むためのユーティリティを提供します。
        - **`demo/src/mmlConverter.ts`**: ユーザーが入力したMML文字列をWASMモジュール経由でSMFに変換する処理を制御します。
        - **`demo/src/parseMidiNotes.ts`**: SMFバイナリから個々のMIDIノートイベントを抽出し、時間情報などと共に解析します。
        - **`demo/src/smfToYm2151.ts`**: SMFデータをYAMAHA YM2151チップのログ形式に変換するWASMラッパーです。
        - **`demo/src/state.ts`**: デモアプリケーションのグローバルな状態を管理するための定義が含まれます。
        - **`demo/src/treeToJSON.ts`**: tree-sitterが生成した抽象構文木（AST）をJSON形式に変換し、デバッグや視覚化に利用可能にします。
        - **`demo/src/ui.ts`**: デモのユーザーインターフェース（ステータス表示、MML例のロードなど）を操作する関数を提供します。
        - **`demo/src/visualization.ts`**: 生成されたオーディオの波形データをHTML Canvas上に描画する視覚化ロジックです。
        - **`demo/src/wavExport.ts`**: 生成されたオーディオデータをWAVファイル形式でエクスポートする機能を提供します。
    - **`demo/test-loader.mjs`**: Node.js環境でのテストファイルロードに関する設定です。
    - **`demo/test-register.mjs`**: Node.js環境でのテストフレームワーク登録に関する設定です。
- **`demo-library/`**: デモライブラリに関連するファイルが格納されています。
    - **`demo-library/index.html`**: デモライブラリのHTMLファイルです。
    - **`demo-library/package.json`**: デモライブラリのNode.jsパッケージ定義です。
- **`generated-docs/development-status-generated-prompt.md`**: 自動生成された開発ステータスに関するプロンプトの記録ファイルです。
- **`googled947dc864c270e07.html`**: Google Search Consoleなどのサイト所有権確認に使用されるHTMLファイルです。
- **`issue-notes/`**: 開発中の課題やバグに関するメモが個別のMarkdownファイルとして整理されています。
    - **`issue-notes/103.md`**, **`issue-notes/123.md`**, **`issue-notes/131.md`**, **`issue-notes/39.md`**, **`issue-notes/44.md`**: 各課題の詳細や対応状況が記述されています。
- **`mmlabc-to-smf-rust.toml.example`**: 外部のStandard MIDI File (SMF) プレイヤーを設定するための設定ファイルの例です。
- **`mmlabc-to-smf-wasm/`**: Rustで実装されたMML-to-SMF変換ロジックをWebAssemblyとして公開するためのクレートです。
    - **`mmlabc-to-smf-wasm/Cargo.lock`**: WASMクレート固有の依存関係ロックファイルです。
    - **`mmlabc-to-smf-wasm/Cargo.toml`**: WASMクレートのプロジェクト設定ファイルです。
    - **`mmlabc-to-smf-wasm/src/lib.rs`**: WASMの外部インターフェース（JS側から呼び出される関数）を定義するライブラリのルートファイルです。
    - **`mmlabc-to-smf-wasm/src/token_extractor.rs`**: WASM環境において、MMLのトークンを抽出するロジックを担当するファイルです。
- **`package.json`**: プロジェクト全体のNode.jsパッケージ定義ファイルです。スクリプトや開発ツールへの依存関係が記述されます。
- **`scripts/`**: ビルドやデプロイ、その他の開発タスクを自動化するためのシェルスクリプトが格納されています。
    - **`scripts/README.md`**: スクリプトの概要と使い方を説明するファイルです。
    - **`scripts/build-demo.sh`**: Webデモアプリケーションをビルドするためのシェルスクリプトです。
    - **`scripts/transform-demo-paths.sh`**: デモ内のファイルパスを調整するスクリプトです。
- **`src/`**: Rustコアライブラリのソースコードが格納されています。MML-to-SMF変換の主要なロジックがここに集約されています。
    - **`src/attachment_json.rs`**: MML変換の各パスで生成される中間データをJSON形式で出力し、デバッグや解析に役立てる機能を提供します。
    - **`src/config.rs`**: アプリケーションの動作に必要な設定（例: 外部MIDIプレイヤーの指定）をファイルから読み込み、管理します。
    - **`src/lib.rs`**: `mmlabc-to-smf-rust`クレートのライブラリとしての公開インターフェースを定義します。MML-to-SMF変換のメイン関数が含まれます。
    - **`src/main.rs`**: CLI (Command Line Interface) アプリケーションのメインエントリポイントです。コマンドライン引数をパースし、ライブラリの機能を利用してMML変換を実行します。
    - **`src/mml_preprocessor.rs`**: MML文字列を実際の解析パスに渡す前に、コメント除去やマクロ展開などの前処理を行うロジックを実装します。
    - **`src/pass1_parser.rs`**: 4パスアーキテクチャの最初のパスです。MML文字列を`tree-sitter`パーサーを使用して構文解析し、トークンストリームを生成します。
    - **`src/pass2_ast.rs`**: 4パスアーキテクチャの第2パスです。パス1で生成されたトークンから、MMLの論理構造を表現する抽象構文木（AST）を構築します。
    - **`src/pass3_events.rs`**: 4パスアーキテクチャの第3パスです。ASTをトラバースし、MIDIの音符オン/オフ、テンポ変更などの低レベルなMIDIイベントシーケンスを生成します。
    - **`src/pass4_midi.rs`**: 4パスアーキテクチャの最終パスです。パス3で生成されたMIDIイベントシーケンスから、Standard MIDI Fileのバイナリフォーマットに準拠した最終的なMIDIファイルを構築します。
    - **`src/tree_sitter_mml.rs`**: RustアプリケーションがMML用の`tree-sitter`パーサーと連携するためのインターフェースやヘルパー関数を提供します。
    - **`src/types.rs`**: プロジェクト全体で利用される共有のデータ構造、列挙型、トレイトなどの型定義を集中管理します。
- **`tests/`**: Rustコードのテストファイルが格納されています。
    - **`tests/integration_test.rs`**: ライブラリ全体または複数のモジュールを連携させた統合テストを定義します。
    - **`tests/test_attachment_json.rs`**: `attachment_json`モジュールの機能が正しく動作するかを検証します。
    - **`tests/test_c1_vs_c64.rs`**: C1（中央ハ）とC64（MIDIノート番号）のような特定の音符表記の変換ロジックをテストします。
    - **`tests/test_channel.rs`**: MMLの多チャンネル対応（`;`による分離）機能のテストを行います。
    - **`tests/test_chord.rs`**: 和音（コード）のMML表記が正しくMIDIに変換されるかをテストします。
    - **`tests/test_cli.rs`**: コマンドラインインターフェースの引数処理や出力などの機能をテストします。
    - **`tests/test_config.rs`**: アプリケーション設定の読み込みや適用に関するテストです。
    - **`tests/test_dotted_notes.rs`**: 付点音符のMML表記のテストを行います。
    - **`tests/test_drum_channel.rs`**: ドラムチャンネル（通常MIDIチャンネル10）のMML表記のテストを行います。
    - **`tests/test_key_transpose.rs`**: キー転調（移調）機能のMML表記のテストを行います。
    - **`tests/test_length.rs`**: 音符の長さ（例: `c4`, `d8`）指定機能のテストを行います。
    - **`tests/test_modifier.rs`**: 音符修飾子（例: シャープ、フラット）のMML表記のテストを行います。
    - **`tests/test_note_length.rs`**: 特定の音符の長さ変換のテストを行います。
    - **`tests/test_octave.rs`**: オクターブ変更（`>`, `<`）機能のテストを行います。
    - **`tests/test_pass1.rs`**: パス1（MMLトークン解析）の単体テストです。
    - **`tests/test_pass2.rs`**: パス2（AST変換）の単体テストです。
    - **`tests/test_pass3.rs`**: パス3（MIDIイベント生成）の単体テストです。
    - **`tests/test_pass4.rs`**: パス4（SMFファイル生成）の単体テストです。
    - **`tests/test_program_change.rs`**: プログラムチェンジ（音色変更）のMML表記のテストを行います。
    - **`tests/test_rest.rs`**: 休符のMML表記のテストを行います。
    - **`tests/test_tempo.rs`**: テンポ変更のMML表記のテストを行います。
    - **`tests/test_velocity.rs`**: 音量（ベロシティ）変更のMML表記のテストを行います。
- **`tree-sitter-mml/`**: MML言語の`tree-sitter`パーサーの定義と生成されたファイルが格納されています。
    - **`tree-sitter-mml/grammar.js`**: MML言語の構文規則を記述したJavaScriptファイルです。このファイルからC言語のパーサーが生成されます。
    - **`tree-sitter-mml/package.json`**: `tree-sitter-mml`パーサーのNode.jsパッケージ定義です。
    - **`tree-sitter-mml/src/`**: `grammar.js`から自動生成されるC言語のパーサーソースコードおよび関連ファイルが格納されています。
        - **`tree-sitter-mml/src/grammar.json`**: 生成された文法定義のJSON表現です。
        - **`tree-sitter-mml/src/node-types.json`**: 生成されたASTノードタイプのJSON表現です。
        - **`tree-sitter-mml/src/parser.c`**: `tree-sitter`によって自動生成されたMMLパーサーのC言語実装です。
        - **`tree-sitter-mml/src/tree_sitter/`**: `tree-sitter`のランタイムに必要なC言語のヘッダーファイル群です。
            - **`tree-sitter-mml/src/tree_sitter/alloc.h`**: メモリ割り当て関連のヘッダー。
            - **`tree-sitter-mml/src/tree_sitter/array.h`**: 動的配列関連のヘッダー。
            - **`tree-sitter-mml/src/tree_sitter/parser.h`**: パーサーのメインヘッダー。

## 関数詳細説明
- **`playAudio(midiNotes: Array<Object>, tempo: number)`** (`demo/src/audioPlayback.ts`):
    - 役割: 提供されたMIDIノートデータとテンポ情報に基づき、Webブラウザ上で音楽を再生します。
    - 引数:
        - `midiNotes`: 再生するMIDIノートイベントの配列。各オブジェクトは音符の開始時刻、持続時間、ピッチなどを含む。
        - `tempo`: 音楽のテンポ（BPM）。
    - 戻り値: なし。
    - 機能: Tone.jsライブラリを使用して仮想シンセサイザーを構築し、指定されたMIDIノートをシーケンス再生します。
- **`stopAudio()`** (`demo/src/audioPlayback.ts`):
    - 役割: 現在再生中のオーディオを即座に停止します。
    - 引数: なし。
    - 戻り値: なし。
    - 機能: Tone.jsのトランスポートを停止し、関連するUI要素（例: 再生ボタンの状態）を更新します。
- **`waitForWebYm2151(check: Function, interval: number)`** (`demo/src/audioRenderer.ts`):
    - 役割: Web-YM2151モジュールの初期化など、特定の非同期処理の完了をポーリングして待ちます。
    - 引数:
        - `check`: 完了条件をチェックするための関数。真を返すと待機を終了します。
        - `interval`: チェックを行う間隔（ミリ秒）。
    - 戻り値: `Promise<void>`。
    - 機能: 定期的に`check`関数を呼び出し、条件が満たされるかタイムアウトするまで待機します。
- **`calculateDuration(midiData: ArrayBuffer)`** (`demo/src/audioRenderer.ts`):
    - 役割: MIDIバイナリデータの演奏時間を計算します。
    - 引数:
        - `midiData`: Standard MIDI Fileのバイナリデータを含む`ArrayBuffer`。
    - 戻り値: 計算された演奏時間（秒）。
    - 機能: `MidiReader`を使用してMIDIデータを解析し、すべてのイベントを走査して曲の総演奏時間を決定します。
- **`renderWaveformAndAudio(mmlText: string)`** (`demo/src/audioRenderer.ts`):
    - 役割: MMLテキストからSMFを生成し、それをYM2151ログに変換、オーディオ波形をレンダリングし、オーディオデータを準備します。
    - 引数:
        - `mmlText`: 変換対象のMML文字列。
    - 戻り値: `Promise<void>`。
    - 機能: `mmlConverter.convertMML`、`smfToYm2151.smfToYM2151Json`を呼び出し、Web-YM2151でオーディオを合成後、`visualization.drawWaveform`で波形を表示します。
- **`initialize()`** (`demo/src/main.ts`):
    - 役割: Webデモアプリケーションの初期設定を行います。
    - 引数: なし。
    - 戻り値: `Promise<void>`。
    - 機能: `tree-sitter`パーサーとWASMモジュールをロードし、UI要素のイベントリスナーを設定して、MML入力フィールドやボタンの動作を初期化します。
- **`constructor(midiData: ArrayBuffer)`** (`demo/src/midiReader.ts`):
    - 役割: MIDIバイナリデータを内部で保持し、読み込みのための準備をします。
    - 引数:
        - `midiData`: 解析するMIDIバイナリデータ。
    - 戻り値: なし（コンストラクタ）。
    - 機能: 渡されたMIDIデータから`DataView`を作成し、ファイルポインタを初期化します。
- **`convertMML(mml: string, debug: boolean, options?: MMLConverterOptions)`** (`demo/src/mmlConverter.ts`):
    - 役割: MML文字列をRust WASMモジュールを介してStandard MIDI File形式のバイナリデータに変換します。
    - 引数:
        - `mml`: 変換するMML文字列。
        - `debug`: デバッグ情報を出力するかどうかのフラグ。
        - `options`: 変換に関する追加オプション。
    - 戻り値: 変換されたSMFバイナリデータとデバッグJSONを含むオブジェクト。
    - 機能: 内部で`mmlabc_to_smf_wasm`の`convert_mml`関数を呼び出し、MML解析・SMF生成を実行します。
- **`parseMidiNotes(smfData: ArrayBuffer)`** (`demo/src/parseMidiNotes.ts`):
    - 役割: Standard MIDI File (SMF) のバイナリデータを解析し、時間情報を持つ一連のノートイベントに変換します。
    - 引数:
        - `smfData`: 解析するSMFバイナリデータを含む`ArrayBuffer`。
    - 戻り値: 各チャンネルのノートイベントを含む構造化されたデータ。
    - 機能: `MidiReader`を利用してMIDIファイルを読み込み、トラックやイベントを処理して、開始時刻と終了時刻を含むノートイベントの配列を生成します。
- **`deltaTicksToSeconds(deltaTime: number, ticksPerBeat: number, tempo: number)`** (`demo/src/parseMidiNotes.ts`):
    - 役割: MIDIのデルタティック（イベント間の時間単位）を実際の秒数に変換します。
    - 引数:
        - `deltaTime`: 変換するデルタティック数。
        - `ticksPerBeat`: 1拍あたりのティック数。
        - `tempo`: 現在のテンポ（BPM）。
    - 戻り値: 変換された秒数。
    - 機能: MIDIのタイムディビジョンとテンポ情報を用いて、相対的なティック数を絶対的な時間（秒）に換算します。
- **`ensureInitialized()`** (`demo/src/smfToYm2151.ts`):
    - 役割: `smf-to-ym2151log-wasm`モジュールがロードされ、初期化されていることを確認します。
    - 引数: なし。
    - 戻り値: `Promise<void>`。
    - 機能: WebAssemblyモジュールのロード状態をチェックし、必要であればロード処理をトリガーします。
- **`smfToYM2151Json(smfBinary: Uint8Array)`** (`demo/src/smfToYm2151.ts`):
    - 役割: Standard MIDI File (SMF) のバイナリデータを、YM2151チップエミュレーション用のJSONログ形式に変換します。
    - 引数:
        - `smfBinary`: SMFのバイナリデータ。
    - 戻り値: YM2151ログ形式のJSON文字列。
    - 機能: `smf-to-ym2151log-wasm`モジュールを呼び出し、SMFを特定のハードウェアシンセサイザーエミュレータで利用できる形式に変換します。
- **`treeToJSON(node: Node)`** (`demo/src/treeToJSON.ts`):
    - 役割: `tree-sitter`の構文木（AST）ノードを再帰的に走査し、JSON形式のオブジェクトとして表現します。
    - 引数:
        - `node`: 変換対象の`tree-sitter`ノード。
    - 戻り値: JSON形式のオブジェクト。
    - 機能: ノードの型、名前付き状態、開始/終了位置、子ノードの情報をJSON構造にまとめ、デバッグ用途に供します。
- **`showStatus(message: string, isError: boolean = false)`** (`demo/src/ui.ts`):
    - 役割: Webデモのユーザーインターフェース上にステータスメッセージを表示します。
    - 引数:
        - `message`: 表示するテキストメッセージ。
        - `isError`: メッセージがエラーであるかどうかのフラグ。`true`の場合、エラーとして強調表示されます。
    - 戻り値: なし。
    - 機能: 指定されたメッセージをUI上のステータス表示領域に表示し、エラーの場合はスタイルを変更します。
- **`loadExample(exampleMML: string)`** (`demo/src/ui.ts`):
    - 役割: 指定されたMMLのサンプル文字列をMML入力フィールドにロードします。
    - 引数:
        - `exampleMML`: ロードするMMLのサンプル文字列。
    - 戻り値: なし。
    - 機能: UIのMML入力エリアに事前定義されたMMLテキストを挿入し、ユーザーが簡単に試せるようにします。
- **`drawWaveform(audioBuffer: AudioBuffer, canvas: HTMLCanvasElement)`** (`demo/src/visualization.ts`):
    - 役割: `AudioBuffer`内のオーディオデータを抽出し、HTML `Canvas`要素上に波形として描画します。
    - 引数:
        - `audioBuffer`: 波形を描画するオーディオデータを含む`AudioBuffer`オブジェクト。
        - `canvas`: 波形を描画する対象のHTML `Canvas`要素。
    - 戻り値: なし。
    - 機能: `AudioBuffer`の各チャンネルのデータを取得し、Canvas 2D APIを使用して音量の変化を視覚化します。
- **`writeString(view: DataView, offset: number, string: string)`** (`demo/src/wavExport.ts`):
    - 役割: `DataView`の指定されたオフセット位置に文字列を書き込みます。WAVヘッダーのチャンクIDなどに使用。
    - 引数:
        - `view`: 書き込み先の`DataView`オブジェクト。
        - `offset`: 書き込みを開始するオフセットバイト位置。
        - `string`: 書き込む文字列。
    - 戻り値: なし。
    - 機能: 文字列をバイト配列に変換し、`DataView`に1バイトずつ書き込みます。
- **`audioBufferToWav(audioBuffer: AudioBuffer)`** (`demo/src/wavExport.ts`):
    - 役割: `AudioBuffer`オブジェクトから生のオーディオデータを抽出し、WAVファイル形式のバイナリデータ（`Blob`）に変換します。
    - 引数:
        - `audioBuffer`: WAVに変換するオーディオデータを含む`AudioBuffer`オブジェクト。
    - 戻り値: WAV形式のバイナリデータを含む`Blob`オブジェクト。
    - 機能: WAVファイルのヘッダー情報（RIFFチャンク、FMTチャンク、DATAチャンク）を構築し、`AudioBuffer`のオーディオデータをそれに合わせて書き込みます。
- **`exportWav(audioBuffer: AudioBuffer, filename: string)`** (`demo/src/wavExport.ts`):
    - 役割: `AudioBuffer`をWAVファイルとしてユーザーのブラウザにダウンロードさせます。
    - 引数:
        - `audioBuffer`: エクスポートするオーディオデータを含む`AudioBuffer`オブジェクト。
        - `filename`: ダウンロードされるファイル名。
    - 戻り値: なし。
    - 機能: `audioBufferToWav`を呼び出してWAVデータを生成し、`URL.createObjectURL`とアンカータグのクリックイベントを利用してダウンロードをトリガーします。
- **`mockAudioBuffer(channels: number, length: number, sampleRate: number)`** (`demo/tests/audioBufferToWav.test.ts`):
    - 役割: テストのためにダミーの`AudioBuffer`オブジェクトを生成します。
    - 引数:
        - `channels`: オーディオチャンネル数。
        - `length`: バッファのサンプルフレーム数。
        - `sampleRate`: サンプルレート。
    - 戻り値: ダミーの`AudioBuffer`オブジェクト。
    - 機能: `AudioBuffer`インターフェースを模倣したオブジェクトを返し、テストにおいて実際のオーディオデータを必要とせずに`audioBufferToWav`の動作を検証できるようにします。
- **`buildSmf(trackEvents: Array<Object>, ticksPerBeat: number)`** (`demo/tests/parseMidiNotes.test.ts`):
    - 役割: テストのために簡易的なStandard MIDI File (SMF) バイナリデータを構築します。
    - 引数:
        - `trackEvents`: MIDIトラックのイベントデータ。
        - `ticksPerBeat`: 1拍あたりのティック数。
    - 戻り値: 構築されたSMFバイナリデータを含む`ArrayBuffer`。
    - 機能: MIDIヘッダとトラックイベントを手動で組み立てて、`parseMidiNotes`テスト用の入力データを提供します。
- **`mockNode(type: string, named: boolean, children: Array<Object>)`** (`demo/tests/treeToJSON.test.ts`):
    - 役割: テストのためにダミーの`tree-sitter`ノードオブジェクトを生成します。
    - 引数:
        - `type`: ノードの型名。
        - `named`: 名前付きノードであるか。
        - `children`: 子ノードの配列。
    - 戻り値: ダミーの`tree-sitter`ノードオブジェクト。
    - 機能: `treeToJSON`のテストにおいて、実際の`tree-sitter`パーサーを使用せずに、さまざまなノード構造をシミュレートできるようにします。

## 関数呼び出し階層ツリー
```
- initialize (demo/src/main.ts)
  - convertMML (demo/src/mmlConverter.ts)
    - (Rust WASM library's MML to SMF conversion is implicitly called here)
    - smfToYM2151Json (demo/src/smfToYm2151.ts)
      - ensureInitialized (demo/src/smfToYm2151.ts)
    - treeToJSON (demo/src/treeToJSON.ts)
  - loadExample (demo/src/ui.ts)
  - playAudio (demo/src/audioPlayback.ts)
    - stopAudio (demo/src/audioPlayback.ts)
      - showStatus (demo/src/ui.ts)
  - renderWaveformAndAudio (demo/src/audioRenderer.ts)
    - calculateDuration (demo/src/audioRenderer.ts)
    - drawWaveform (demo/src/visualization.ts)
    - smfToYM2151Json (demo/src/smfToYm2151.ts)
      - ensureInitialized (demo/src/smfToYm2151.ts)
  - exportWav (demo/src/wavExport.ts)
    - audioBufferToWav (demo/src/wavExport.ts)
      - writeString (demo/src/wavExport.ts)

- parseMML_and_generate_smf (src/lib.rs - main library function, conceptual)
  - mml_preprocessor::preprocess_mml (src/mml_preprocessor.rs)
  - pass1_parser::parse_mml (src/pass1_parser.rs)
    - tree_sitter_mml::parse (src/tree_sitter_mml.rs)
  - pass2_ast::build_ast (src/pass2_ast.rs)
  - pass3_events::generate_midi_events (src/pass3_events.rs)
  - pass4_midi::create_smf (src/pass4_midi.rs)
  - attachment_json::write_debug_json (src/attachment_json.rs) (optional)

- main (src/main.rs - CLI entry point)
  - config::load_config (src/config.rs)
  - parseMML_and_generate_smf (src/lib.rs)
  - (External MIDI player invocation based on config)

- mmlabc_to_smf_wasm::convert_mml (mmlabc-to-smf-wasm/src/lib.rs - WASM public function)
  - parseMML_and_generate_smf (src/lib.rs)
  - token_extractor::extract_tokens (mmlabc-to-smf-wasm/src/token_extractor.rs)

- parseMidiNotes (demo/src/parseMidiNotes.ts)
  - deltaTicksToSeconds (demo/src/parseMidiNotes.ts)

- mockAudioBuffer (demo/tests/audioBufferToWav.test.ts)
- buildSmf (demo/tests/parseMidiNotes.test.ts)
- mockNode (demo/tests/treeToJSON.test.ts)

---
Generated at: 2026-03-29 07:09:21 JST
