Last updated: 2026-03-19

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) からStandard MIDI File (SMF) へ変換するRust製のライブラリです。
- MMLの解析からMIDIファイル生成までを4つの明確なパスで処理するアーキテクチャを採用しています。
- ネイティブアプリおよびWebAssembly (WASM) を介したブラウザアプリとして利用可能で、多チャンネル対応やデバッグ機能も備えています。

## 技術スタック
- フロントエンド: `TypeScript`, `JavaScript` (デモUI実装), `WebAssembly (WASM)` (ブラウザでのMML変換ライブラリ利用)。
- 音楽・オーディオ: `Music Macro Language (MML)` (入力フォーマット), `Standard MIDI File (SMF)` (出力フォーマット), `Tone.js` (デモでのオーディオ再生)。
- 開発ツール: `Rust 1.70.0+`, `Cargo` (Rustパッケージマネージャ), `tree-sitter-cli` (MML構文パーサー生成), `Node.js`, `npm` / `npx` (デモのビルドおよびパーサー生成)。
- テスト: `cargo test` (Rustユニット/統合テスト), `node:test`, `node:assert/strict` (TypeScript/JavaScriptテスト)。
- ビルドツール: `Cargo` (Rustプロジェクトのビルドと依存管理), `npm` (デモプロジェクトの依存管理)。
- 言語機能: `Rust` (主要開発言語), `C` (tree-sitterパーサーの生成元言語)。
- 自動化・CI/CD: `cargo clippy` (Linter), `cargo fmt` (Formatter)。
- 開発標準: `.editorconfig` (IDE設定統一), `.gitignore` (Git管理除外設定), `.vscode/settings.json` (VS Code設定)。

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
│   ├── test-register.mjs
│   └── tests/
│       ├── audioBufferToWav.test.ts
│       ├── midiReader.test.ts
│       ├── parseMidiNotes.test.ts
│       └── treeToJSON.test.ts
├── demo-library/
│   ├── index.html
│   └── package.json
├── generated-docs/
│   └── development-status-generated-prompt.md
├── googled947dc864c270e07.html
├── issue-notes/
│   ├── 103.md
│   ├── 123.md
│   ├── 129.md
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
-   `.editorconfig`: 異なるエディタやIDE間で一貫したコーディングスタイルを維持するための設定ファイル。
-   `.gitignore`: Gitによってバージョン管理から除外されるファイルやディレクトリを指定するファイル。
-   `.vscode/settings.json`: Visual Studio Codeエディタのワークスペース固有の設定を定義するファイル。
-   `Cargo.lock`: Rustプロジェクトの依存関係の正確なバージョンを記録し、再現可能なビルドを保証するファイル。
-   `Cargo.toml`: Rustプロジェクトの主要なマニフェストファイル。プロジェクト名、バージョン、依存関係、ビルド設定などを定義する。
-   `IMPLEMENTATION_REPORT.md`: プロジェクトの実装に関する詳細な報告やメモが記述されたMarkdownファイル。
-   `LICENSE`: プロジェクトの配布および使用に関するライセンス情報 (MIT License)。
-   `OPTION_A_IMPLEMENTATION.md`: 特定の実装オプションAに関する設計や詳細が記述されたMarkdownファイル。
-   `README.ja.md`: プロジェクトの日本語版の概要、インストール方法、使用方法、開発状況などが記載された説明ファイル。
-   `README.md`: プロジェクトの英語版の概要、インストール方法、使用方法、開発状況などが記載された説明ファイル。
-   `_config.yml`: (もし存在すれば) Jekyllなどの静的サイトジェネレータの設定ファイルである可能性が高い。
-   `build.rs`: Rustのカスタムビルドスクリプト。主にtree-sitterパーサー関連のC言語ファイルの再生成ロジックが含まれる。
-   `mmlabc-to-smf-rust.toml.example`: カスタムMIDIプレイヤーの選択など、プロジェクトの動作設定の例を示すTOML形式の設定ファイル。
-   `mmlabc-to-smf-wasm/Cargo.lock`: WASMラッパーの依存関係の正確なバージョンを記録するファイル。
-   `mmlabc-to-smf-wasm/Cargo.toml`: WASMラッパーのRustプロジェクトマニフェスト。
-   `mmlabc-to-smf-wasm/src/lib.rs`: Rustで書かれたMMLからSMFへの変換ロジックをWebAssemblyとして公開するためのライブラリエントリポイント。
-   `mmlabc-to-smf-wasm/src/token_extractor.rs`: WASM環境でMMLからトークンを抽出する機能に関連するコード。
-   `package.json`: JavaScript/TypeScriptプロジェクトのメタデータと依存関係を定義するファイル。主に`demo`ディレクトリで使用される。
-   `scripts/README.md`: `scripts`ディレクトリ内のスクリプトに関する説明ファイル。
-   `scripts/build-demo.sh`: デモアプリケーションをビルドするためのシェルスクリプト。
-   `scripts/transform-demo-paths.sh`: デモ内のファイルパスなどを変換・調整するためのシェルスクリプト。
-   `src/attachment_json.rs`: 中間パスのデバッグ情報などをJSON形式で出力するための機能を提供するRustモジュール。
-   `src/config.rs`: プロジェクトの設定（例: 外部MIDIプレイヤーの指定）を管理するRustモジュール。
-   `src/lib.rs`: `mmlabc-to-smf-rust` クレートの主要なライブラリエントリポイント。MMLからSMFへの変換機能を提供する。
-   `src/main.rs`: コマンドラインインターフェース (CLI) のエントリポイント。MML文字列を引数として受け取り、変換処理を実行する。
-   `src/mml_preprocessor.rs`: MML文字列の解析前に、特定の変換や整形を行う前処理ロジックを含むRustモジュール。
-   `src/pass1_parser.rs`: MML文字列をtree-sitterを用いて構文解析し、トークンストリームを生成する「パス1」のRust実装。
-   `src/pass2_ast.rs`: パス1で生成されたトークンストリームから抽象構文木 (AST) を構築する「パス2」のRust実装。
-   `src/pass3_events.rs`: パス2で生成されたASTを基にMIDIイベント（ノートオン/オフなど）を生成する「パス3」のRust実装。
-   `src/pass4_midi.rs`: パス3で生成されたMIDIイベントリストから最終的なStandard MIDI Fileのバイナリデータを作成する「パス4」のRust実装。
-   `src/tree_sitter_mml.rs`: Rustコードとtree-sitterで生成されたMMLパーサーとの連携を担うモジュール。
-   `src/types.rs`: プロジェクト全体で共有されるデータ構造や型定義を格納するRustモジュール。
-   `tests/integration_test.rs`: プロジェクト全体のMMLからSMFへの変換フローを検証する統合テストスイート。
-   `tests/test_attachment_json.rs`: `attachment_json`モジュールのテストケース。
-   `tests/test_c1_vs_c64.rs`: 特定のMIDIノート番号の範囲や挙動に関するテストケース。
-   `tests/test_channel.rs`: MMLの多チャンネル分離機能のテストケース。
-   `tests/test_chord.rs`: 和音のMML記法とMIDI変換に関するテストケース。
-   `tests/test_cli.rs`: コマンドラインインターフェースの引数処理や実行に関するテストケース。
-   `tests/test_config.rs`: 設定ファイル (`mmlabc-to-smf-rust.toml`) の読み込みと反映に関するテストケース。
-   `tests/test_dotted_notes.rs`: 付点音符のMML記法と音長計算に関するテストケース。
-   `tests/test_drum_channel.rs`: ドラムチャンネルのMML記法とMIDI変換に関するテストケース。
-   `tests/test_key_transpose.rs`: キーの移調コマンドの機能テストケース。
-   `tests/test_length.rs`: 音長指定コマンド (`l`) の機能テストケース。
-   `tests/test_modifier.rs`: 音符の修飾子（例:シャープ、フラット）に関するテストケース。
-   `tests/test_note_length.rs`: 音符の基本的な長さ表現に関するテストケース。
-   `tests/test_octave.rs`: オクターブ変更コマンド (`o`, `>`, `<`) の機能テストケース。
-   `tests/test_pass1.rs`: `pass1_parser`モジュールのユニットテスト。
-   `tests/test_pass2.rs`: `pass2_ast`モジュールのユニットテスト。
-   `tests/test_pass3.rs`: `pass3_events`モジュールのユニットテスト。
-   `tests/test_pass4.rs`: `pass4_midi`モジュールのユニットテスト。
-   `tests/test_program_change.rs`: プログラムチェンジ（音色変更）コマンドに関するテストケース。
-   `tests/test_rest.rs`: 休符 (`r`) のMML記法とMIDI変換に関するテストケース。
-   `tests/test_tempo.rs`: テンポ変更コマンド (`t`) の機能テストケース。
-   `tests/test_velocity.rs`: 音量 (ベロシティ) 制御コマンド (`v`) の機能テストケース。
-   `tree-sitter-mml/grammar.js`: MML言語のtree-sitterパーサーを定義するJavaScriptファイル。
-   `tree-sitter-mml/package.json`: `tree-sitter-mml`パーサーのNode.jsパッケージ設定ファイル。
-   `tree-sitter-mml/src/grammar.json`: `grammar.js`から生成されたMML文法のJSON表現。
-   `tree-sitter-mml/src/node-types.json`: `grammar.js`から生成されたMMLのASTノード型定義。
-   `tree-sitter-mml/src/parser.c`: `grammar.js`から生成されたC言語のMMLパーサーソースコード。
-   `tree-sitter-mml/src/tree_sitter/alloc.h`: tree-sitterパーサーが使用するメモリ割り当てに関するCヘッダーファイル。
-   `tree-sitter-mml/src/tree_sitter/array.h`: tree-sitterパーサーが使用する動的配列に関するCヘッダーファイル。
-   `tree-sitter-mml/src/tree_sitter/parser.h`: tree-sitterパーサーの主要なCヘッダーファイル。
-   `tree-sitter-mml/tree-sitter-mml.wasm`: MMLパーサーのWebAssemblyバイナリファイル。
-   `demo/.gitignore`: デモアプリケーション用のGit追跡除外設定。
-   `demo/FEATURES.md`: デモアプリケーションの機能に関するドキュメント。
-   `demo/README.md`: デモアプリケーションのREADMEファイル。
-   `demo/index.html`: デモアプリケーションのメインHTMLファイル。
-   `demo/package.json`: デモアプリケーションのNode.jsパッケージ設定ファイル。
-   `demo/src/audioPlayback.ts`: デモのオーディオ再生ロジックをTypeScriptで実装。
-   `demo/src/audioRenderer.ts`: デモのオーディオ波形レンダリングと可視化ロジックをTypeScriptで実装。
-   `demo/src/main.ts`: デモアプリケーションの主要な初期化およびイベントハンドリングロジック。
-   `demo/src/midiReader.ts`: MIDIファイルの内容を読み込み、解析するためのTypeScriptクラス。
-   `demo/src/mmlConverter.ts`: WASMモジュールを呼び出してMMLからSMFへの変換を orchestrate するTypeScriptファイル。
-   `demo/src/parseMidiNotes.ts`: MIDIデータからノートイベントを抽出し、時間情報と共に解析するTypeScript関数。
-   `demo/src/smfToYm2151.ts`: Standard MIDI FileデータからYM2151ログ形式への変換をWASMを介して行うTypeScriptラッパー。
-   `demo/src/state.ts`: デモアプリケーションのグローバルな状態を管理するTypeScriptファイル。
-   `demo/src/treeToJSON.ts`: tree-sitterによって生成された構文木 (AST) をJSON形式に変換するTypeScript関数。
-   `demo/src/ui.ts`: デモのユーザーインターフェース (UI) 要素の操作や表示更新を担当するTypeScriptファイル。
-   `demo/src/visualization.ts`: 音源の波形データなどを視覚的に描画する機能を提供するTypeScriptファイル。
-   `demo/src/wavExport.ts`: 生成されたオーディオデータをWAVファイルとしてエクスポートする機能を提供するTypeScriptファイル。
-   `demo/test-loader.mjs`: Node.jsのテストランナーで使用されるESモジュールローダー。
-   `demo/test-register.mjs`: Node.jsのテストランナーでテストを登録するためのESモジュール。
-   `demo/tests/audioBufferToWav.test.ts`: `wavExport.ts`の`audioBufferToWav`関数に対するテストケース。
-   `demo/tests/midiReader.test.ts`: `midiReader.ts`モジュールに対するテストケース。
-   `demo/tests/parseMidiNotes.test.ts`: `parseMidiNotes.ts`関数に対するテストケース。
-   `demo/tests/treeToJSON.test.ts`: `treeToJSON.ts`関数に対するテストケース。
-   `demo-library/index.html`: ライブラリ利用例を示すHTMLデモファイル。
-   `demo-library/package.json`: ライブラリデモのNode.jsパッケージ設定ファイル。
-   `generated-docs/development-status-generated-prompt.md`: 自動生成された開発状況に関するドキュメント。
-   `googled947dc864c270e07.html`: Googleサイト認証ファイル。
-   `issue-notes/103.md`: Issue #103に関する開発メモ。
-   `issue-notes/123.md`: Issue #123に関する開発メモ。
-   `issue-notes/129.md`: Issue #129に関する開発メモ。
-   `issue-notes/39.md`: Issue #39に関する開発メモ。
-   `issue-notes/44.md`: Issue #44に関する開発メモ。

## 関数詳細説明
-   `playAudio()` (demo/src/audioPlayback.ts):
    -   役割: 指定されたMIDIイベント配列を再生開始する。
    -   引数: MIDIイベント配列、オーディオコンテキスト、MIDIプレイヤー設定。
    -   戻り値: なし。
    -   機能: Tone.jsを使用してMIDIイベントをオーディオに変換し、再生を開始します。
-   `stopAudio()` (demo/src/audioPlayback.ts):
    -   役割: 現在再生中のオーディオを停止する。
    -   引数: なし。
    -   戻り値: なし。
    -   機能: Tone.jsのトランスポートを停止し、関連するUI要素をリセットします。
-   `if` (demo/src/audioPlayback.ts, demo/src/audioRenderer.ts, demo/src/main.ts, demo/src/midiReader.ts, demo/src/mmlConverter.ts, demo/src/parseMidiNotes.ts, demo/src/smfToYm2151.ts, demo/src/treeToJSON.ts, demo/src/wavExport.ts):
    -   役割: 条件分岐を制御する言語構文。
    -   引数: 条件式。
    -   戻り値: なし。
    -   機能: 特定の条件が真の場合にコードブロックを実行します。
-   `catch` (demo/src/audioPlayback.ts, demo/src/audioRenderer.ts, demo/src/main.ts, demo/src/mmlConverter.ts, demo/src/parseMidiNotes.ts, demo/src/wavExport.ts):
    -   役割: エラー処理を捕捉する言語構文。
    -   引数: エラーオブジェクト。
    -   戻り値: なし。
    -   機能: `try`ブロック内で発生した例外を捕捉し、指定されたエラーハンドリングコードを実行します。
-   `waitForWebYm2151()` (demo/src/audioRenderer.ts):
    -   役割: Web YM2151が初期化されるのを待機する。
    -   引数: なし。
    -   戻り値: Promise<void>。
    -   機能: WebAssemblyモジュールのロード完了を待機し、初期化が完了したことを確認します。
-   `calculateDuration()` (demo/src/audioRenderer.ts):
    -   役割: MIDIイベントの総再生時間を計算する。
    -   引数: MIDIイベント配列。
    -   戻り値: number (秒)。
    -   機能: MIDIイベントの時間情報から楽曲全体の長さを算出します。
-   `renderWaveformAndAudio()` (demo/src/audioRenderer.ts):
    -   役割: 波形をレンダリングし、オーディオを生成する。
    -   引数: SMFデータ、オーディオコンテキスト、キャンバス要素。
    -   戻り値: Promise<AudioBuffer>。
    -   機能: SMFデータを基にYM2151サウンドを生成し、その波形をキャンバスに描画、オーディオバッファを返します。
-   `check()` (demo/src/audioRenderer.ts):
    -   役割: 内部的に状態や条件をチェックする。
    -   引数: 不明。
    -   戻り値: 不明。
    -   機能: プログラムの進行に必要な内部的な条件を確認します。
-   `for` (demo/src/audioRenderer.ts, demo/src/treeToJSON.ts, demo/src/visualization.ts, demo/src/wavExport.ts, demo/tests/audioBufferToWav.test.ts, demo/tests/parseMidiNotes.test.ts):
    -   役割: 繰り返し処理を制御する言語構文。
    -   引数: ループ条件。
    -   戻り値: なし。
    -   機能: 特定の回数または条件が満たされるまでコードブロックを繰り返し実行します。
-   `initialize()` (demo/src/main.ts):
    -   役割: デモアプリケーション全体の初期化を行う。
    -   引数: なし。
    -   戻り値: なし。
    -   機能: tree-sitterおよびWASMモジュールのロード、UI要素のセットアップ、イベントリスナーの登録を行います。
-   `convertMML()` (demo/src/mmlConverter.ts):
    -   役割: MML文字列をStandard MIDI File (SMF) データに変換する。
    -   引数: MML文字列。
    -   戻り値: Promise<Uint8Array> (SMFバイナリデータ)。
    -   機能: WASMラッパーを介してRustコアライブラリを呼び出し、MMLをSMFに変換します。
-   `smfToYM2151Json()` (demo/src/smfToYm2151.ts):
    -   役割: SMFデータをYM2151ログ形式のJSONに変換する。
    -   引数: SMFバイナリデータ。
    -   戻り値: Promise<any> (YM2151ログJSON)。
    -   機能: `smf-to-ym2151log-wasm` WASMモジュールを利用してSMFをYM2151の内部ログ形式に変換します。
-   `treeToJSON()` (demo/src/treeToJSON.ts):
    -   役割: tree-sitterの構文木 (AST) をJSON形式で表現する。
    -   引数: tree-sitterのNodeオブジェクト。
    -   戻り値: JSONオブジェクト。
    -   機能: ASTをデバッグや表示に適した構造化されたJSONデータに変換します。
-   `loadExample()` (demo/src/ui.ts):
    -   役割: プリセットのMML例をロードし、エディタに表示する。
    -   引数: MML文字列。
    -   戻り値: なし。
    -   機能: 指定されたMML文字列をUIのエディタに設定し、変換処理をトリガーします。
-   `constructor()` (demo/src/midiReader.ts):
    -   役割: MIDIファイルリーダーのインスタンスを初期化する。
    -   引数: Uint8Array (MIDIバイナリデータ)。
    -   戻り値: MidiReaderオブジェクト。
    -   機能: 提供されたMIDIバイナリデータを内部で保持し、解析の準備をします。
-   `parseMidiNotes()` (demo/src/parseMidiNotes.ts):
    -   役割: MIDIファイルからノートイベントを抽出し、時間情報に変換する。
    -   引数: MIDIバイナリデータ。
    -   戻り値: { notes: Array, duration: number }。
    -   機能: MIDIファイルを解析し、演奏可能なノートイベントのリストと全体の長さを返します。
-   `deltaTicksToSeconds()` (demo/src/parseMidiNotes.ts):
    -   役割: MIDIのデルタティックを秒単位の時間に変換する。
    -   引数: デルタティック、テンポ、ティックあたりの分解能。
    -   戻り値: number (秒)。
    -   機能: MIDIファイルの内部時間単位を実際の再生時間（秒）に変換します。
-   `writeString()` (demo/src/wavExport.ts):
    -   役割: DataViewに文字列を書き込む。
    -   引数: DataViewオブジェクト、オフセット、文字列。
    -   戻り値: なし。
    -   機能: 指定されたオフセットからDataViewにASCII文字列を書き込みます。
-   `audioBufferToWav()` (demo/src/wavExport.ts):
    -   役割: AudioBufferをWAV形式のバイナリデータに変換する。
    -   引数: AudioBufferオブジェクト。
    -   戻り値: Blobオブジェクト (WAVファイル)。
    -   機能: Web Audio APIのAudioBufferから標準的なWAVファイル形式を生成します。
-   `exportWav()` (demo/src/wavExport.ts):
    -   役割: 生成されたオーディオバッファをWAVファイルとしてダウンロードする。
    -   引数: AudioBufferオブジェクト。
    -   戻り値: なし。
    -   機能: `audioBufferToWav`を呼び出し、その結果をユーザーのブラウザにダウンロードさせます。
-   `ensureInitialized()` (demo/src/smfToYm2151.ts):
    -   役割: `smf-to-ym2151log-wasm`モジュールがロードされていることを保証する。
    -   引数: なし。
    -   戻り値: Promise<void>。
    -   機能: WASMモジュールの非同期ロード処理を管理し、利用可能な状態であることを確認します。
-   `mockAudioBuffer()` (demo/tests/audioBufferToWav.test.ts):
    -   役割: テスト用のダミーAudioBufferオブジェクトを作成する。
    -   引数: channels、length、sampleRate。
    -   戻り値: MockAudioBufferオブジェクト。
    -   機能: `audioBufferToWav`関数のテストに必要なAudioBufferのモックを生成します。
-   `buildSmf()` (demo/tests/parseMidiNotes.test.ts):
    -   役割: テスト用のSMFバイナリデータを構築する。
    -   引数: MIDIイベントデータ。
    -   戻り値: Uint8Array (SMFバイナリデータ)。
    -   機能: テストケースでMIDIイベントをシミュレートするために、簡易なSMF構造を作成します。
-   `while` (demo/src/midiReader.ts, demo/src/parseMidiNotes.ts, demo/tests/parseMidiNotes.test.ts):
    -   役割: 条件が真である間、繰り返し処理を実行する言語構文。
    -   引数: 条件式。
    -   戻り値: なし。
    -   機能: 特定の条件が満たされている限り、コードブロックを繰り返し実行します。
-   `mockNode()` (demo/tests/treeToJSON.test.ts):
    -   役割: tree-sitterのNodeオブジェクトのモックを作成する。
    -   引数: type、children配列。
    -   戻り値: MockNodeオブジェクト。
    -   機能: `treeToJSON`関数のテストに必要なtree-sitter Nodeのモックを生成します。
-   `showStatus()` (demo/src/ui.ts):
    -   役割: UI上にステータスメッセージを表示する。
    -   引数: statusText (文字列), isError (ブーリアン)。
    -   戻り値: なし。
    -   機能: ユーザーに対して現在の処理状況やエラーメッセージを視覚的に通知します。
-   `drawWaveform()` (demo/src/visualization.ts):
    -   役割: オーディオ波形をキャンバスに描画する。
    -   引数: AudioBufferオブジェクト, CanvasRenderingContext2Dオブジェクト。
    -   戻り値: なし。
    -   機能: 提供されたAudioBufferのデータを視覚的に表現する波形をキャンバス上に描画します。

## 関数呼び出し階層ツリー
```
- if (demo/src/audioPlayback.ts)
  - playAudio (demo/src/audioPlayback.ts)
    - stopAudio (demo/src/audioPlayback.ts)
      - showStatus (demo/src/ui.ts)
- waitForWebYm2151 (demo/src/audioRenderer.ts)
  - calculateDuration (demo/src/audioRenderer.ts)
    - renderWaveformAndAudio (demo/src/audioRenderer.ts)
    - check (demo/src/audioRenderer.ts)
    - drawWaveform (demo/src/visualization.ts)
- initialize (demo/src/main.ts)
  - convertMML (demo/src/mmlConverter.ts)
    - smfToYM2151Json (demo/src/smfToYm2151.ts)
    - treeToJSON (demo/src/treeToJSON.ts)
  - loadExample (demo/src/ui.ts)
- constructor (demo/src/midiReader.ts)
- parseMidiNotes (demo/src/parseMidiNotes.ts)
  - deltaTicksToSeconds (demo/src/parseMidiNotes.ts)
- catch (demo/src/audioPlayback.ts)
  - writeString (demo/src/wavExport.ts)
    - audioBufferToWav (demo/src/wavExport.ts)
    - exportWav (demo/src/wavExport.ts)
- ensureInitialized (demo/src/smfToYm2151.ts)
- for (demo/src/audioRenderer.ts)
- mockAudioBuffer (demo/tests/audioBufferToWav.test.ts)
- buildSmf (demo/tests/parseMidiNotes.test.ts)
- while (demo/src/parseMidiNotes.ts)
- mockNode (demo/tests/treeToJSON.test.ts)

---
Generated at: 2026-03-19 07:11:00 JST
