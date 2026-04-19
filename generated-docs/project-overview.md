Last updated: 2026-04-20

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) を Standard MIDI File (SMF) へ変換するRust製のライブラリ兼CLIツールです。
- MMLからSMFへの変換は4つのパスで構成され、各パスの中間結果はJSON形式で出力可能です。
- WebAssembly (WASM) に対応しており、ブラウザでの利用やライブラリ利用のデモも提供しています。

## 技術スタック
- フロントエンド:
    - `TypeScript`: デモアプリケーションのロジック開発に使用されており、型安全なコード記述を可能にします。
    - `JavaScript`: デモアプリケーションの動作に利用され、ブラウザ環境でMML変換やオーディオ再生を制御します。
    - `HTML`: デモのユーザーインターフェースを構築するために使用されます。
    - `WebAssembly (WASM)`: Rustで書かれたコアロジックをウェブブラウザで高速に実行するために利用されます。
- 音楽・オーディオ:
    - `Standard MIDI File (SMF)`: MMLから変換される標準的な音楽ファイル形式です。
    - `Music Macro Language (MML)`: このプロジェクトの入力となる音楽記述言語です。
    - `WAV`: デモアプリケーションで生成されたオーディオデータを標準的なWAV形式でエクスポートするために使用されます。
- 開発ツール:
    - `Rust (cargo)`: プロジェクトの主要なプログラミング言語であり、依存関係管理、ビルド、テスト、ドキュメント生成を行うツールです。
    - `Node.js (npm, npx)`: フロントエンドデモの依存関係管理、スクリプト実行、および`tree-sitter`文法生成に使用されます。
    - `tree-sitter-cli`: カスタムMML文法を定義し、パーサーを生成するために使用されるツールです。
    - `VS Code`: 開発環境として使用され、`settings.json`でエディタの設定を共有します。
    - `cat-play-mml`: 生成されたMIDIファイルを自動再生するための外部コマンドとして設定可能です。
- テスト:
    - `node:test`, `node:assert/strict`: TypeScript/JavaScriptベースのデモコードの単体テストに使用されるNode.jsの組み込みテストモジュールです。
    - `cargo test`: Rustコードの単体テストおよび統合テストを実行するためのコマンドです。
- ビルドツール:
    - `cargo`: RustライブラリとCLI、WASMモジュールのビルドを管理します。
    - `npm`: デモアプリケーションのビルドスクリプトや依存関係を管理します。
    - `npx tree-sitter generate`: `tree-sitter-mml`の文法定義からパーサーファイルを生成します。
- 言語機能:
    - `Rust`: 高性能なMMLからSMFへの変換ロジックを記述するために使用されます。メモリ安全性と並行性を提供します。
    - `TypeScript`: 静的型付けにより、デモのJavaScriptコードの保守性と堅牢性を向上させます。
- 自動化・CI/CD:
    - `cargo clippy`: Rustコードの一般的な間違いや非イディオムなコードを検出するリンターです。
    - `cargo fmt`: Rustコードのフォーマットを自動的に行うツールで、コードスタイルの一貫性を保ちます。
- 開発標準:
    - `.editorconfig`: 異なる開発者やエディタ間でコードスタイルを統一するための設定ファイルです。
    - `.gitattributes`, `.gitignore`: Gitリポジトリでファイル特性の定義やバージョン管理から除外するファイルを指定します。

## ファイル階層ツリー
```
.
├── .editorconfig
├── .gitattributes
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
├── demo/
│   ├── .gitignore
│   ├── FEATURES.md
│   ├── README.md
│   ├── index.html
│   ├── package.json
│   ├── src/
│   │   ├── audioPlayback.ts
│   │   ├── audioRenderer.ts
│   │   ├── main.ts
│   │   ├── midiReader.ts
│   │   ├── mmlConverter.ts
│   │   ├── parseMidiNotes.ts
│   │   ├── smfToYm2151.ts
│   │   ├── state.ts
│   │   ├── treeToJSON.ts
│   │   ├── ui.ts
│   │   ├── visualization.ts
│   │   └── wavExport.ts
│   ├── test-loader.mjs
│   └── test-register.mjs
├── demo-library/
│   ├── index.html
│   └── package.json
├── generated-docs/
│   └── development-status-generated-prompt.md
├── googled947dc864c270e07.html
├── issue-notes/
│   ├── 103.md
│   ├── 123.md
│   ├── 133.md
│   ├── 39.md
│   └── 44.md
├── mmlabc-to-smf-rust.toml.example
├── mmlabc-to-smf-wasm/
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── token_extractor.rs
├── package.json
├── scripts/
│   ├── README.md
│   ├── build-demo.sh
│   └── transform-demo-paths.sh
├── src/
│   ├── attachment_json.rs
│   ├── config.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── mml_preprocessor.rs
│   ├── parse_tree_tokens.rs
│   ├── pass1_parser.rs
│   ├── pass2_ast.rs
│   ├── pass3_events.rs
│   ├── pass4_midi.rs
│   ├── tree_sitter_mml.rs
│   └── types.rs
├── tests/
│   ├── integration_test.rs
│   ├── test_attachment_json.rs
│   ├── test_c1_vs_c64.rs
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

## ファイル詳細説明
-   `.editorconfig`: 異なる開発環境で一貫したコードスタイルを維持するための設定ファイルです。
-   `.gitattributes`: Gitが特定のファイルをどのように扱うかを定義します。例えば、テキストファイルの改行コードの扱いなどです。
-   `.gitignore`: Gitのバージョン管理システムで追跡対象から除外するファイルやディレクトリを指定します。
-   `.vscode/settings.json`: Visual Studio Codeエディタのワークスペース固有の設定を定義するファイルです。
-   `Cargo.lock`: Rustプロジェクトの依存関係の厳密なバージョンを記録し、ビルドの再現性を保証します。
-   `Cargo.toml`: Rustプロジェクトのマニフェストファイルで、プロジェクト名、バージョン、著者、依存関係、ビルド設定などを定義します。
-   `LICENSE`: プロジェクトのライセンス情報（MIT License）が記載されています。
-   `README.ja.md`: プロジェクトの日本語での概要、機能、使用方法、開発状況などが記述されたドキュメントです。
-   `README.md`: プロジェクトの英語での概要、機能、使用方法、開発状況などが記述されたドキュメントです。
-   `_config.yml`: 主にJekyllなどの静的サイトジェネレータで使用される設定ファイルですが、このプロジェクトでの具体的な用途は提供情報からは不明です。
-   `build.rs`: Rustプロジェクトのビルドプロセス中に実行されるカスタムビルドスクリプトです。`tree-sitter`のパーサー生成などの複雑なビルドステップを処理します。
-   `demo/`: WebブラウザでMML変換機能を試すことができるデモンストレーションアプリケーションのルートディレクトリです。
    -   `demo/.gitignore`: デモアプリケーション固有のGit無視設定ファイルです。
    -   `demo/FEATURES.md`: デモアプリケーションが提供する機能についての説明ドキュメントです。
    -   `demo/README.md`: デモアプリケーションの使用方法や概要に関する説明ファイルです。
    -   `demo/index.html`: ブラウザで開かれるデモアプリケーションのメインページとなるHTMLファイルです。
    -   `demo/package.json`: デモアプリケーションのJavaScript/TypeScriptの依存関係と実行スクリプトを定義するファイルです。
    -   `demo/src/`: デモアプリケーションのTypeScriptソースコードが格納されています。
        -   `audioPlayback.ts`: 生成されたオーディオデータをWeb Audio APIを使って再生するロジックを実装しています。
        -   `audioRenderer.ts`: MIDIデータからオーディオ波形をレンダリングし、視覚化・再生準備を行うロジックを含みます。
        -   `main.ts`: デモアプリケーションのエントリポイントであり、初期化処理、イベントリスナーの設定、主要な処理フローを調整します。
        -   `midiReader.ts`: Standard MIDI File (SMF) のバイナリデータを読み込み、解析するためのクラスを定義しています。
        -   `mmlConverter.ts`: MML文字列をWebAssemblyモジュール（`mmlabc-to-smf-wasm`）を介してSMFに変換する処理を管理します。
        -   `parseMidiNotes.ts`: MIDIデータから個々の音符イベントとその時間情報を抽出・解析する機能を提供します。
        -   `smfToYm2151.ts`: SMFデータを特定の音源チップ（YM2151）のログ形式に変換するためのWebAssemblyラッパーを提供します。
        -   `state.ts`: デモアプリケーションの現在の状態（MML文字列、変換結果など）を保持するグローバルステート管理ファイルです。
        -   `treeToJSON.ts`: `tree-sitter`によって生成された構文ツリーをJSON形式にシリアライズするユーティリティ関数を含みます。
        -   `ui.ts`: ユーザーインターフェース（ボタン、テキストエリアなど）の操作や表示更新に関するヘルパー関数を提供します。
        -   `visualization.ts`: オーディオ波形などの音楽関連情報を視覚的に表示するロジックを実装しています。
        -   `wavExport.ts`: 生成されたオーディオバッファをWAV形式のファイルとしてエクスポート・ダウンロードする機能を提供します。
    -   `demo/test-loader.mjs`: Node.jsのテストランナーで使用されるカスタムモジュールローダーです。
    -   `demo/test-register.mjs`: Node.jsのテストランナーでテストスクリプトを登録するために使用されるファイルです。
    -   `demo/tests/`: デモアプリケーションのTypeScriptテストコードが格納されています。
        -   `audioBufferToWav.test.ts`: `wavExport.ts`で定義された`audioBufferToWav`関数のテストケースです。
        -   `midiReader.test.ts`: `midiReader.ts`のMIDIファイル解析機能のテストケースです。
        -   `parseMidiNotes.test.ts`: `parseMidiNotes.ts`のMIDI音符解析機能のテストケースです。
        -   `treeToJSON.test.ts`: `treeToJSON.ts`の構文ツリーJSON変換機能のテストケースです。
-   `demo-library/`: `mmlabc-to-smf-rust`ライブラリを直接JavaScriptから使用するデモのルートディレクトリです。
    -   `demo-library/index.html`: ライブラリ利用例のデモのメインHTMLファイルです。
    -   `demo-library/package.json`: ライブラリデモのJavaScript/TypeScriptの依存関係と実行スクリプトを定義するファイルです。
-   `generated-docs/`: 自動生成されたドキュメントを格納するためのディレクトリです。
    -   `generated-docs/development-status-generated-prompt.md`: 開発状況に関する自動生成されたプロンプトのドキュメントです。
-   `googled947dc864c270e07.html`: Google Search Consoleなどでのサイト所有権確認に使用されるファイルです。
-   `issue-notes/`: 開発中に発生した課題や検討事項に関するメモが格納されています。
    -   `103.md`, `123.md`, `133.md`, `39.md`, `44.md`: 個別の課題に関するMarkdown形式のメモファイルです。
-   `mmlabc-to-smf-rust.toml.example`: `mmlabc-to-smf-rust`CLIツールの設定例が記述されたTOML形式のファイルです。
-   `mmlabc-to-smf-wasm/`: `mmlabc-to-smf-rust`ライブラリのWebAssembly (WASM) 版をビルドするためのRustクレートです。
    -   `mmlabc-to-smf-wasm/Cargo.lock`: WASMクレートの依存関係ロックファイルです。
    -   `mmlabc-to-smf-wasm/Cargo.toml`: WASMクレートのマニフェストファイルです。
    -   `mmlabc-to-smf-wasm/src/`: WASMクレートのRustソースコードが格納されています。
        -   `lib.rs`: WebAssemblyモジュールとして公開される主要な変換関数やAPIを定義します。
        -   `token_extractor.rs`: MML文字列からトークンを抽出し、WASM環境で利用できるようにするロジックを実装しています。
    -   `mmlabc-to-smf-wasm/tests/parity.rs`: WASM版ライブラリがネイティブRust版と同じ結果を生成するかどうかを確認するためのテストです。
-   `package.json`: プロジェクト全体のNode.jsの依存関係とスクリプト（主にデモ関連）を定義するファイルです。
-   `scripts/`: プロジェクトのビルド、テスト、デモの実行などを自動化するためのシェルスクリプトが格納されています。
    -   `scripts/README.md`: スクリプトディレクトリ内のファイルに関する説明です。
    -   `scripts/build-demo.sh`: デモアプリケーションをビルドするための一連のコマンドを実行するシェルスクリプトです。
    -   `scripts/transform-demo-paths.sh`: デモ関連のファイルパスを変換・調整するためのシェルスクリプトです。
-   `src/`: `mmlabc-to-smf-rust`ライブラリとCLIアプリケーションのメインのRustソースコードが格納されています。
    -   `attachment_json.rs`: MMLに埋め込まれた、または外部から提供される添付JSONデータを処理するためのロジックを定義します。
    -   `config.rs`: CLIアプリケーションやライブラリの動作設定を読み込み・管理するモジュールです。
    -   `lib.rs`: `mmlabc_to_smf`ライブラリの公開インターフェースと、MMLからSMFへの変換の中核となる4パスの処理フローを実装しています。
    -   `main.rs`: CLIアプリケーションのエントリポイントです。コマンドライン引数の解析、設定の読み込み、MML変換処理の呼び出しを行います。
    -   `mml_preprocessor.rs`: MML文字列を構文解析に適した形式に前処理するロジックを含みます。
    -   `parse_tree_tokens.rs`: `tree-sitter`によって生成された構文ツリーを走査し、MMLの論理的なトークンに変換する処理を実装しています。
    -   `pass1_parser.rs`: MMLの構文を解析し、初期の構文ツリーを生成する「第1パス」のロジックです（`cli` featureが有効な場合、`tree_sitter_mml`を使用）。
    -   `pass2_ast.rs`: 第1パスで生成された構文ツリーから、より抽象的なAST（抽象構文木）を構築する「第2パス」のロジックです。
    -   `pass3_events.rs`: ASTをMIDIイベントのシーケンスに変換する「第3パス」のロジックです。
    -   `pass4_midi.rs`: 生成されたMIDIイベントシーケンスをStandard MIDI File (SMF) 形式のバイナリデータに最終的に変換する「第4パス」のロジックです。
    -   `tree_sitter_mml.rs`: `tree-sitter-mml`パーサーをRustコードから利用するためのバインディングを提供します。
    -   `types.rs`: プロジェクト全体で共有されるカスタムデータ構造や列挙型（例：MML要素の表現）を定義します。
-   `tests/`: `mmlabc-to-smf-rust`のネイティブRust版のテストコードが格納されています。
    -   `integration_test.rs`: 複数のモジュールを連携させた統合的な動作を検証するテストです。
    -   `test_attachment_json.rs`: 添付JSONのパースと処理に関するテストケースです。
    -   `test_c1_vs_c64.rs`: 特定のMML記法（例: `c1`と`c64`）が正しく処理されるかを検証するテストです。
    -   `test_channel.rs`: MMLのチャンネル分割機能に関するテストケースです。
    -   `test_chord.rs`: MMLの和音（コード）記法に関するテストケースです。
    -   `test_cli.rs`: コマンドラインインターフェース（CLI）の引数解析や出力に関するテストケースです。
    -   `test_config.rs`: `mmlabc-to-smf-rust.toml`などの設定ファイルの読み込みと適用に関するテストケースです。
    -   `test_dotted_notes.rs`: 付点音符（例: `c4.`）の正しい解釈とMIDI変換を検証するテストです。
    -   `test_drum_channel.rs`: `@128`チャンネルのドラムチャンネル割り当て機能に関するテストです。
    -   `test_key_transpose.rs`: キー移調（例: `kt1`）が正しく適用されるかを検証するテストです。
    -   `test_length.rs`: 音長指定（例: `l4`, `l8`）に関するテストケースです。
    -   `test_modifier.rs`: シャープやフラットなどの音符修飾子（例: `c+`, `d-`）に関するテストケースです。
    -   `test_note_length.rs`: 音符の長さに関する詳細なテストケースです。
    -   `test_octave.rs`: オクターブ変更記法（例: `<`, `>`, `o4`）に関するテストケースです。
    -   `test_pass1.rs`: MML変換プロセスの第1パス（構文解析）のテストです。
    -   `test_pass2.rs`: MML変換プロセスの第2パス（AST構築）のテストです。
    -   `test_pass3.rs`: MML変換プロセスの第3パス（イベント変換）のテストです。
    -   `test_pass4.rs`: MML変換プロセスの第4パス（MIDI生成）のテストです。
    -   `test_program_change.rs`: プログラムチェンジ（音色変更）記法（例: `@0`）に関するテストケースです。
    -   `test_rest.rs`: 休符記法（例: `r`）に関するテストケースです。
    -   `test_tempo.rs`: テンポ設定記法（例: `t120`）に関するテストケースです。
    -   `test_velocity.rs`: ベロシティ（音量）設定記法（例: `v8`）に関するテストケースです。
-   `tree-sitter-mml/`: MML言語用の`tree-sitter`パーサーの定義と生成物を含むクレートディレクトリです。
    -   `tree-sitter-mml/grammar.js`: MML言語の構文規則を記述したJavaScriptファイルで、`tree-sitter`がパーサーを生成するために使用します。
    -   `tree-sitter-mml/package.json`: `tree-sitter-mml`パーサーのNode.jsパッケージ設定ファイルです。
    -   `tree-sitter-mml/src/`: 生成されたパーサー関連のソースファイルが格納されています。
        -   `tree-sitter-mml/src/grammar.json`: `grammar.js`から生成されるJSON形式の文法定義です。
        -   `tree-sitter-mml/src/node-types.json`: 構文ツリーのノードタイプ定義です。
        -   `tree-sitter-mml/src/parser.c`: `grammar.js`から生成されるC言語のパーサー実装ファイルです。
        -   `tree-sitter-mml/src/tree_sitter/`: `tree-sitter`のランタイムに必要なCヘッダーファイルが含まれます。
            -   `alloc.h`, `array.h`, `parser.h`: `tree-sitter`パーサーの内部で利用される標準ヘッダーファイルです。
    -   `tree-sitter-mml/tree-sitter-mml.wasm`: `tree-sitter-mml`パーサーのWebAssemblyバイナリで、ブラウザ環境でMMLの構文解析を行うために使用されます。

## 関数詳細説明
-   `playAudio(audioBuffer: AudioBuffer)` (demo/src/audioPlayback.ts):
    -   役割: 提供されたオーディオバッファをWeb Audio APIを使用して再生します。
    -   引数: `audioBuffer`: 再生するオーディオデータを含むAudioBufferオブジェクト。
    -   戻り値: なし。
-   `stopAudio()` (demo/src/audioPlayback.ts):
    -   役割: 現在再生中のオーディオがあれば、それを停止します。
    -   引数: なし。
    -   戻り値: なし。
-   `waitForWebYm2151()` (demo/src/audioRenderer.ts):
    -   役割: `smf-to-ym2151log-wasm`モジュールが初期化されるのを非同期で待ちます。
    -   引数: なし。
    -   戻り値: `Promise<void>`。
-   `calculateDuration(midiNotes: Array<MidiNote>)` (demo/src/audioRenderer.ts):
    -   役割: MIDIノートイベントのリストから、曲の推定総再生時間（秒単位）を計算します。
    -   引数: `midiNotes`: MIDIノートイベントの配列。
    -   戻り値: `number` (秒単位の期間)。
-   `renderWaveformAndAudio(midiData: ArrayBuffer, attachmentJson: object)` (demo/src/audioRenderer.ts):
    -   役割: 提供されたMIDIデータと添付JSONを使用して、オーディオ波形をレンダリングし、再生可能なオーディオを準備します。
    -   引数: `midiData`: SMF形式のMIDIバイナリデータ。 `attachmentJson`: 添付されるJSONデータ。
    -   戻り値: なし。
-   `check(condition: boolean, message: string)` (demo/src/audioRenderer.ts):
    -   役割: 指定された条件が偽の場合に、カスタムメッセージと共にエラーをスローするユーティリティ関数。
    -   引数: `condition`: チェックする真偽値の条件。 `message`: 条件が偽だった場合にスローするエラーメッセージ。
    -   戻り値: なし (条件が偽の場合は例外をスロー)。
-   `initialize()` (demo/src/main.ts):
    -   役割: デモアプリケーション全体の初期化処理を行います。WebAssemblyモジュールのロード、イベントリスナーの設定、初期表示のレンダリングなどが含まれます。
    -   引数: なし。
    -   戻り値: `Promise<void>`。
-   `constructor(midiData: ArrayBuffer)` (demo/src/midiReader.ts):
    -   役割: MIDIファイルのバイナリデータを読み込み、内部的に解析状態を初期化します。
    -   引数: `midiData`: MIDIファイルの内容を含むArrayBuffer。
    -   戻り値: なし (コンストラクタ)。
-   `convertMML(mmlString: string, attachmentOutput: boolean)` (demo/src/mmlConverter.ts):
    -   役割: MML文字列をWebAssemblyモジュール（`mmlabc_to_smf_wasm`）を介してStandard MIDI File (SMF) に変換し、オプションで添付JSONを出力します。
    -   引数: `mmlString`: 変換対象のMML文字列。 `attachmentOutput`: 添付JSONも出力するかどうかの真偽値。
    -   戻り値: `Promise<{ smfBytes: Uint8Array; attachmentJson: object; } | null>` (SMFバイナリと添付JSON、またはエラー時にnull)。
-   `parseMidiNotes(midiFile: MidiFile)` (demo/src/parseMidiNotes.ts):
    -   役割: MIDIファイルオブジェクトから音符のオン/オフイベントを抽出し、時間情報（秒単位）を含むリストに変換します。
    -   引数: `midiFile`: MIDIファイルを表すオブジェクト。
    -   戻り値: `Array<MidiNote>` (解析されたMIDIノートイベントの配列)。
-   `deltaTicksToSeconds(deltaTicks: number, tempo: number, ticksPerBeat: number)` (demo/src/parseMidiNotes.ts):
    -   役割: MIDIのデルタティック値を、現在のテンポとticks per beatに基づいて秒単位の時間に変換します。
    -   引数: `deltaTicks`: 変換するデルタティック値。 `tempo`: 現在のテンポ（マイクロ秒/拍）。 `ticksPerBeat`: 1拍あたりのティック数。
    -   戻り値: `number` (秒単位の時間)。
-   `ensureInitialized()` (demo/src/smfToYm2151.ts):
    -   役割: `smf-to-ym2151log-wasm`というWASMモジュールがロードされ、初期化済みであることを保証します。
    -   引数: なし。
    -   戻り値: `Promise<void>`。
-   `smfToYM2151Json(smfBytes: Uint8Array, attachmentJson: object)` (demo/src/smfToYm2151.ts):
    -   役割: SMFバイナリデータと添付JSONを、YM2151エミュレータが使用できるJSON形式のログデータに変換します。
    -   引数: `smfBytes`: SMF形式のMIDIバイナリデータ。 `attachmentJson`: 添付されるJSONデータ。
    -   戻り値: `object` (YM2151ログ形式のJSONオブジェクト)。
-   `treeToJSON(node: Node)` (demo/src/treeToJSON.ts):
    -   役割: `tree-sitter`パーサーによって生成された構文ツリーのノードを、再帰的にJSONオブジェクトに変換します。
    -   引数: `node`: 変換対象の`tree-sitter`ノード。
    -   戻り値: `object` (ノードのJSON表現)。
-   `showStatus(message: string)` (demo/src/ui.ts):
    -   役割: デモアプリケーションのユーザーインターフェース上に、指定されたステータスメッセージを表示します。
    -   引数: `message`: 表示するステータス文字列。
    -   戻り値: なし。
-   `loadExample(mml: string)` (demo/src/ui.ts):
    -   役割: 指定されたMML文字列をデモのエディタにロードし、自動的に変換・再生プロセスを開始します。
    -   引数: `mml`: ロードするMML文字列。
    -   戻り値: `Promise<void>`。
-   `drawWaveform(canvas: HTMLCanvasElement, audioBuffer: AudioBuffer)` (demo/src/visualization.ts):
    -   役割: 提供された`AudioBuffer`から取得したオーディオデータに基づいて、指定されたHTML `Canvas`要素に波形を描画します。
    -   引数: `canvas`: 波形を描画するHTMLCanvasElement。 `audioBuffer`: 波形描画の元となるオーディオデータを含むAudioBuffer。
    -   戻り値: なし。
-   `writeString(view: DataView, offset: number, string: string)` (demo/src/wavExport.ts):
    -   役割: `DataView`の指定されたオフセット位置に文字列をバイトとして書き込みます。
    -   引数: `view`: 書き込み先のDataView。 `offset`: 書き込み開始オフセット。 `string`: 書き込む文字列。
    -   戻り値: なし。
-   `audioBufferToWav(audioBuffer: AudioBuffer)` (demo/src/wavExport.ts):
    -   役割: Web Audio APIの`AudioBuffer`オブジェクトから、標準的なWAVファイル形式のバイナリデータ（`ArrayBuffer`）を生成します。
    -   引数: `audioBuffer`: 変換対象のAudioBuffer。
    -   戻り値: `ArrayBuffer` (WAV形式のバイナリデータ)。
-   `exportWav(audioBuffer: AudioBuffer)` (demo/src/wavExport.ts):
    -   役割: `AudioBuffer`をWAVファイルに変換し、ユーザーにダウンロードさせるための処理を開始します。
    -   引数: `audioBuffer`: エクスポートするオーディオデータを含むAudioBuffer。
    -   戻り値: `Promise<void>`。
-   `mockAudioBuffer(options: { numberOfChannels: number; length: number; sampleRate: number; })` (demo/tests/audioBufferToWav.test.ts):
    -   役割: テストの目的で、`AudioBuffer`オブジェクトのモック（ダミー）インスタンスを作成します。
    -   引数: `options`: チャンネル数、長さ、サンプルレートなどのAudioBufferのプロパティを定義するオブジェクト。
    -   戻り値: `object` (モックされたAudioBufferオブジェクト)。
-   `buildSmf(events: Array<MidiEvent>)` (demo/tests/parseMidiNotes.test.ts):
    -   役割: テストのために、MIDIイベントの配列から単純なStandard MIDI File (SMF) 構造を構築します。
    -   引数: `events`: MIDIイベントの配列。
    -   戻り値: `Uint8Array` (構築されたSMFバイナリデータ)。
-   `mockNode(type: string, children: Array<Node>)` (demo/tests/treeToJSON.test.ts):
    -   役割: `treeToJSON`関数のテスト用に、`tree-sitter`ノードのモックオブジェクトを再帰的に作成します。
    -   引数: `type`: ノードのタイプ文字列。 `children`: 子ノードの配列。
    -   戻り値: `object` (モックされた`tree-sitter`ノード)。

## 関数呼び出し階層ツリー
```
- main (src/main.rs)
    - lib::convert_mml
- lib::convert_mml (src/lib.rs)
    - mml_preprocessor::preprocess_mml (src/mml_preprocessor.rs)
    - pass1_parser::parse_mml (src/pass1_parser.rs) (if "cli" feature is enabled)
        - tree_sitter_mml::new_parser (src/tree_sitter_mml.rs)
        - parse_tree_tokens::extract_tokens (src/parse_tree_tokens.rs)
    - pass2_ast::build_ast (src/pass2_ast.rs)
    - pass3_events::generate_events (src/pass3_events.rs)
    - pass4_midi::create_midi_file (src/pass4_midi.rs)
    - attachment_json::parse_attachment (src/attachment_json.rs)
    - config::load_config (src/config.rs)
- initialize (demo/src/main.ts)
    - mmlConverter.convertMML (demo/src/mmlConverter.ts)
        - mmlabc_to_smf_wasm.js (WASMモジュールの変換関数)
        - ui.showStatus (demo/src/ui.ts)
    - audioRenderer.renderWaveformAndAudio (demo/src/audioRenderer.ts)
        - smfToYm2151.smfToYM2151Json (demo/src/smfToYm2151.ts)
            - smfToYm2151.ensureInitialized (demo/src/smfToYm2151.ts)
            - smf-to-ym2151log-wasm/smf_to_ym2151log.js (WASMモジュールのYM2151変換関数)
        - parseMidiNotes (demo/src/parseMidiNotes.ts)
            - midiReader.constructor (demo/src/midiReader.ts)
            - deltaTicksToSeconds (demo/src/parseMidiNotes.ts)
        - visualization.drawWaveform (demo/src/visualization.ts)
    - audioPlayback.playAudio (demo/src/audioPlayback.ts)
        - audioPlayback.stopAudio (demo/src/audioPlayback.ts)
            - ui.showStatus (demo/src/ui.ts)
    - ui.loadExample (demo/src/ui.ts)
    - wavExport.exportWav (demo/src/wavExport.ts)
        - wavExport.audioBufferToWav (demo/src/wavExport.ts)
            - wavExport.writeString (demo/src/wavExport.ts)
- mockAudioBuffer (demo/tests/audioBufferToWav.test.ts)
- buildSmf (demo/tests/parseMidiNotes.test.ts)
- mockNode (demo/tests/treeToJSON.test.ts)

---
Generated at: 2026-04-20 07:11:52 JST
