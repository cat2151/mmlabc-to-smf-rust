Last updated: 2026-03-09

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) から Standard MIDI File (SMF) への変換をRustで実現するライブラリです。
- ネイティブCLIアプリケーション、およびWebAssembly (WASM) を通じてブラウザアプリケーションで利用可能です。
- tree-sitterを用いた精密なMML構文解析と4パスアーキテクチャにより、効率的にMIDIイベントを生成します。

## 技術スタック
- フロントエンド: HTML (デモのユーザーインターフェース), TypeScript/JavaScript (デモアプリケーションのロジック), WebAssembly (Rustコードのブラウザ実行), Tone.js (Webオーディオ再生), web-tree-sitter.js (ブラウザでの構文解析)
- 音楽・オーディオ: Music Macro Language (MML) (入力形式), Standard MIDI File (SMF) (出力形式), smf-to-ym2151log-wasm (SMFからYM2151ログへの変換、デモ利用)
- 開発ツール: Rust (主要開発言語), Cargo (Rustプロジェクトのビルド・パッケージ管理), Node.js, npm, npx (tree-sitterパーサー生成スクリプト実行), Visual Studio Code (.vscode/settings.jsonによる設定), Git (バージョン管理)
- テスト: Rustのユニット/統合テスト (`cargo test`で実行), Node.jsのテストフレームワーク (`node:test`, `node:assert/strict`を使用したデモのテスト)
- ビルドツール: Cargo (Rustクレートのビルド), WebAssembly (WASMモジュールの生成), tree-sitter-cli (MMLパーサーのC言語ソースファイル生成)
- 言語機能: Rustの強力な型システムと所有権モデル (メモリ安全性と信頼性の高いコード), C言語 (tree-sitterパーサーの基盤言語)
- 自動化・CI/CD: build-demo.sh, transform-demo-paths.sh (デモビルドとパス変換のためのシェルスクリプト)
- 開発標準: `cargo clippy` (Rustコード品質チェック), `cargo fmt` (Rustコードフォーマット), .editorconfig (エディタ設定の統一)

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
│   ├── 111.md
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
- **`.editorconfig`**: 異なるエディタやIDE間で一貫したコーディングスタイルを維持するための設定ファイルです。
- **`.gitignore`**: Gitがバージョン管理の対象から除外するファイルやディレクトリを指定します。
- **`.vscode/settings.json`**: Visual Studio Codeのワークスペース設定ファイルで、開発環境の統一に役立ちます。
- **`Cargo.lock`**: Rustプロジェクトの依存関係の正確なバージョンを記録し、ビルドの再現性を保証します。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイルで、プロジェクトのメタデータ、依存関係、ビルド設定を定義します。
- **`IMPLEMENTATION_REPORT.md`**: 実装に関する詳細なレポートやドキュメントを記述するMarkdownファイルです。
- **`LICENSE`**: プロジェクトのライセンス情報（MIT License）を記載したファイルです。
- **`OPTION_A_IMPLEMENTATION.md`**: 特定の実装オプション「A」に関する詳細を記述したMarkdownファイルです。
- **`README.ja.md`**: プロジェクトの概要、目的、使用方法などを日本語で説明する主要なドキュメントです。
- **`README.md`**: プロジェクトの概要、目的、使用方法などを英語で説明する主要なドキュメントです。
- **`_config.yml`**: GitHub Pagesなどの静的サイトジェネレーターの設定ファイルである可能性があります。
- **`build.rs`**: Rustプロジェクトのカスタムビルドスクリプトで、特に`tree-sitter-mml`パーサーのCソースファイルの再生成を自動化するために使用されます。
- **`demo/`**: Webブラウザ上でMMLからSMFへの変換機能を試せるデモアプリケーションのソースコードを格納するディレクトリです。
  - **`demo/index.html`**: デモアプリケーションのメインHTMLファイルです。
  - **`demo/src/audioPlayback.ts`**: 生成されたMIDIデータをブラウザ上で再生するためのオーディオ再生ロジック（Tone.jsを使用）を管理します。
  - **`demo/src/audioRenderer.ts`**: 生成されたオーディオの波形表示やレンダリングに関連するロジックを提供します。
  - **`demo/src/main.ts`**: デモアプリケーションの主要なJavaScript/TypeScriptコードのエントリーポイント。WebAssemblyモジュールの読み込みや初期化を行います。
  - **`demo/src/midiReader.ts`**: MIDIファイルのバイトデータを読み込み、構造化されたMIDIイベントに解析するためのユーティリティクラスです。
  - **`demo/src/mmlConverter.ts`**: デモ内でWebAssemblyモジュールを呼び出し、MML文字列をSMFに変換するロジックを扱います。
  - **`demo/src/parseMidiNotes.ts`**: MIDIイベントから具体的な音楽ノート情報（音符の開始時刻、終了時刻など）を抽出し、タイムライン上の位置を計算するロジックを提供します。
  - **`demo/src/smfToYm2151.ts`**: SMFデータをYM2151ログ形式に変換するためのWebAssemblyモジュールへのインターフェースを提供します。
  - **`demo/src/state.ts`**: デモアプリケーション全体の状態管理を定義するファイルです。
  - **`demo/src/treeToJSON.ts`**: Tree-sitterが生成した構文木をJSON形式に変換するユーティリティ。デバッグ出力に利用されます。
  - **`demo/src/ui.ts`**: デモのユーザーインターフェース要素の操作や表示更新を行うロジックをカプセル化します。
  - **`demo/src/visualization.ts`**: 音符データや波形データをCanvasに視覚的に表示するためのロジックを含みます。
  - **`demo/src/wavExport.ts`**: 生成されたオーディオデータをWAVファイルとしてエクスポートし、ダウンロード可能にする機能を提供します。
  - **`demo/tests/`**: デモアプリケーションのユニットテストを格納するディレクトリです。
- **`demo-library/`**: デモで使用される可能性のある外部ライブラリまたは別のデモのコードを格納するディレクトリです。
- **`generated-docs/`**: 自動生成されたドキュメントを格納するディレクトリです。
- **`googled947dc864c270e07.html`**: Googleサイト認証用ファイルです。
- **`issue-notes/`**: 開発中の課題やメモを管理するためのMarkdownファイルを格納するディレクトリです。
- **`mmlabc-to-smf-rust.toml.example`**: カスタムMIDIプレイヤーを設定するための設定ファイル例です。
- **`mmlabc-to-smf-wasm/`**: RustコードをWebAssemblyにコンパイルし、ブラウザ環境で利用可能にするためのクレートです。
  - **`mmlabc-to-smf-wasm/src/lib.rs`**: WASMクレートのメインライブラリファイルで、JavaScriptから呼び出されるMML変換関数などをエクスポートします。
  - **`mmlabc-to-smf-wasm/src/token_extractor.rs`**: WASMクレート内で利用されるトークン抽出関連のロジックを定義します。
- **`package.json`**: Node.jsプロジェクト（tree-sitterパーサーやデモ）の依存関係、スクリプト、メタデータを定義するファイルです。
- **`scripts/`**: ビルドやデプロイなどの補助的なスクリプトを格納するディレクトリです。
  - **`scripts/build-demo.sh`**: デモアプリケーションをビルドするためのシェルスクリプトです。
  - **`scripts/transform-demo-paths.sh`**: デモのファイルパスを変換・調整するためのシェルスクリプトです。
- **`src/`**: 主要なRustライブラリクレートのソースコードを格納するディレクトリです。
  - **`src/config.rs`**: アプリケーションの実行時設定や外部プレイヤー設定を定義するモジュールです。
  - **`src/lib.rs`**: `mmlabc-to-smf-rust`ライブラリのルートファイル。MMLからSMFへの変換の主要な公開インターフェースを提供します。
  - **`src/main.rs`**: CLI (コマンドラインインターフェース) アプリケーションのエントリーポイント。コマンドライン引数を解析し、ライブラリの変換機能を利用してMIDIファイルの出力や自動再生を制御します。
  - **`src/pass1_parser.rs`**: MML文字列をTree-sitterパーサーを用いて構文解析し、トークン列に変換する「パス1」のロジックを実装します。
  - **`src/pass2_ast.rs`**: パス1で生成されたトークン列から抽象構文木（AST）を構築する「パス2」のロジックを実装します。
  - **`src/pass3_events.rs`**: ASTから具体的なMIDIイベント（ノートオン、ノートオフ、テンポチェンジなど）を生成する「パス3」のロジックを実装します。
  - **`src/pass4_midi.rs`**: パス3で生成されたMIDIイベントから、Standard MIDI File (SMF) の最終的なバイナリデータを構築する「パス4」のロジックを実装します。
  - **`src/tree_sitter_mml.rs`**: Rustプロジェクト内でTree-sitter MMLパーサーを統合するためのモジュール。パーサーのローディングやMML文字列の解析インターフェースを提供します。
  - **`src/types.rs`**: プロジェクト全体で利用される共通のデータ構造（例: MIDIイベント、MMLトークンなど）や型エイリアスを定義します。
- **`tests/`**: Rustライブラリのテストコードを格納するディレクトリです。
  - **`tests/integration_test.rs`**: ライブラリ全体の統合テストを記述します。
  - **`tests/test_c1_vs_c64.rs`**: C1とC64などの音高表現に関する特定のテストケースです。
  - **`tests/test_channel.rs`**: MMLの多チャンネル機能（セミコロン`;`）のテストケースです。
  - **`tests/test_chord.rs`**: 和音機能に関するテストケースです。
  - **`tests/test_cli.rs`**: コマンドラインインターフェースの動作を検証するテストケースです。
  - **`tests/test_config.rs`**: 外部プレイヤー設定などのコンフィグレーション機能に関するテストケースです。
  - **`tests/test_dotted_notes.rs`**: 付点音符の処理に関するテストケースです。
  - **`tests/test_drum_channel.rs`**: ドラムチャンネルのMML表現とMIDI変換に関するテストケースです。
  - **`tests/test_key_transpose.rs`**: キーの移調（トランスポーズ）機能に関するテストケースです。
  - **`tests/test_length.rs`**: 音長指定に関するテストケースです。
  - **`tests/test_modifier.rs`**: 音符の修飾子（例: シャープ、フラット）に関するテストケースです。
  - **`tests/test_note_length.rs`**: 音符の長さに関する詳細なテストケースです。
  - **`tests/test_octave.rs`**: オクターブ指定（`<`, `>`）に関するテストケースです。
  - **`tests/test_pass1.rs`**: パス1 (トークン解析) の機能を検証するテストケースです。
  - **`tests/test_pass2.rs`**: パス2 (AST変換) の機能を検証するテストケースです。
  - **`tests/test_pass3.rs`**: パス3 (MIDIイベント生成) の機能を検証するテストケースです。
  - **`tests/test_pass4.rs`**: パス4 (MIDIファイル作成) の機能を検証するテストケースです。
  - **`tests/test_program_change.rs`**: プログラムチェンジ（音色変更）機能に関するテストケースです。
  - **`tests/test_rest.rs`**: 休符の処理に関するテストケースです。
  - **`tests/test_tempo.rs`**: テンポ変更機能に関するテストケースです。
  - **`tests/test_velocity.rs`**: ベロシティ（音量）制御に関するテストケースです。
- **`tree-sitter-mml/`**: MML用のTree-sitterパーサー定義を格納するディレクトリです。
  - **`tree-sitter-mml/grammar.js`**: MML言語の文法規則を定義するJavaScriptファイルです。
  - **`tree-sitter-mml/package.json`**: Tree-sitterパーサーのNode.jsパッケージ設定ファイルです。
  - **`tree-sitter-mml/src/grammar.json`**: `grammar.js`から生成される、Tree-sitterが内部で使用するJSON形式の文法定義ファイルです。
  - **`tree-sitter-mml/src/node-types.json`**: Tree-sitterが生成するASTノードの型情報を定義するJSONファイルです。
  - **`tree-sitter-mml/src/parser.c`**: `grammar.js`から自動生成される、MMLパーサーのC言語ソースコードです。
  - **`tree-sitter-mml/src/tree_sitter/`**: Tree-sitterパーサーのC言語実装に必要なヘッダーファイル群です。
  - **`tree-sitter-mml/tree-sitter-mml.wasm`**: Tree-sitter MMLパーサーのWebAssemblyバイナリファイルです。

## 関数詳細説明
- **`playAudio(midiData: Uint8Array, startOffset?: number, bpm?: number)`** (demo/src/audioPlayback.ts)
  - 役割: デモアプリケーションでSMFデータ（MIDIファイルバイト列）をTone.jsライブラリを使用して再生します。
  - 引数: `midiData` (Uint8Array): 再生するMIDIファイルのバイトデータ。 `startOffset` (number, オプション): 再生開始オフセット（秒）。 `bpm` (number, オプション): 再生テンポ（Beats Per Minute）。
  - 戻り値: なし
- **`stopAudio()`** (demo/src/audioPlayback.ts)
  - 役割: 現在再生中のオーディオを停止します。
  - 引数: なし
  - 戻り値: なし
- **`waitForWebYm2151()`** (demo/src/audioRenderer.ts)
  - 役割: WebAssemblyバージョンのYM2151ロガーが初期化されるのを待機します。
  - 引数: なし
  - 戻り値: Promise<void>
- **`calculateDuration(midiNotes: Array<{start: number, end: number}>)`** (demo/src/audioRenderer.ts)
  - 役割: MIDIノートの配列から楽曲の総再生時間を計算します。
  - 引数: `midiNotes` (Array<{start: number, end: number}>): 各MIDIノートの開始・終了時刻を含むオブジェクトの配列。
  - 戻り値: number (楽曲の総再生時間（秒）)
- **`renderWaveformAndAudio(midiNotes: Array<any>, duration: number)`** (demo/src/audioRenderer.ts)
  - 役割: MIDIノートと楽曲の長さを基に波形をレンダリングし、オーディオを生成するデモ用の関数です。
  - 引数: `midiNotes` (Array<any>): MIDIノート情報。 `duration` (number): 楽曲の総再生時間。
  - 戻り値: Promise<void>
- **`check(condition: boolean, message: string)`** (demo/src/audioRenderer.ts)
  - 役割: 条件が偽の場合にエラーメッセージをスローするユーティリティ関数。主にテストやデバッグで使用されます。
  - 引数: `condition` (boolean): チェックする条件。 `message` (string): 条件が偽の場合にスローされるエラーメッセージ。
  - 戻り値: なし
- **`initialize()`** (demo/src/main.ts)
  - 役割: デモアプリケーションの初期化処理を実行します。tree-sitterパーサーやWebAssemblyモジュールの読み込み、イベントリスナーの設定などを行います。
  - 引数: なし
  - 戻り値: Promise<void>
- **`constructor(data: Uint8Array)`** (demo/src/midiReader.ts, MidiReaderクラス)
  - 役割: MIDIファイルのバイトデータを解析するためのMidiReaderクラスのコンストラクタ。
  - 引数: `data` (Uint8Array): MIDIファイルのバイトデータ。
  - 戻り値: MidiReaderインスタンス
- **`convertMML(mmlString: string)`** (demo/src/mmlConverter.ts)
  - 役割: MML文字列をWebAssembly経由でSMFデータに変換します。変換結果のSMFデータとデバッグ用のJSONデータ（パス1, 2, 3）を返します。
  - 引数: `mmlString` (string): 変換対象のMML文字列。
  - 戻り値: Promise<{ smfData: Uint8Array; pass1Json: any; pass2Json: any; pass3Json: any; }>
- **`parseMidiNotes(smfData: Uint8Array)`** (demo/src/parseMidiNotes.ts)
  - 役割: SMFバイトデータを解析し、各MIDIノートの開始・終了時刻、ピッチ、ベロシティなどの情報を抽出します。
  - 引数: `smfData` (Uint8Array): SMFファイルのバイトデータ。
  - 戻り値: Array<{ note: number; velocity: number; channel: number; start: number; end: number; }>
- **`deltaTicksToSeconds(deltaTicks: number, ticksPerBeat: number, tempo: number)`** (demo/src/parseMidiNotes.ts)
  - 役割: MIDIのデルタティック値を、指定されたテンポとティック/拍情報に基づいて秒単位の時間に変換します。
  - 引数: `deltaTicks` (number): MIDIのデルタティック値。 `ticksPerBeat` (number): 1拍あたりのティック数。 `tempo` (number): テンポ（マイクロ秒/拍）。
  - 戻り値: number (秒単位の時間)
- **`writeString(view: DataView, offset: number, string: string)`** (demo/src/wavExport.ts)
  - 役割: DataViewオブジェクトの指定されたオフセットに文字列を書き込みます。WAVファイルのヘッダ書き込みなどに使用されます。
  - 引数: `view` (DataView): 書き込み対象のDataView。 `offset` (number): 書き込み開始オフセット。 `string` (string): 書き込む文字列。
  - 戻り値: number (書き込んだバイト数)
- **`audioBufferToWav(buffer: AudioBuffer)`** (demo/src/wavExport.ts)
  - 役割: AudioBufferオブジェクトのオーディオデータをWAV形式のバイトデータに変換します。
  - 引数: `buffer` (AudioBuffer): 変換対象のAudioBuffer。
  - 戻り値: Uint8Array (WAV形式のバイトデータ)
- **`exportWav(audioBuffer: AudioBuffer, filename: string)`** (demo/src/wavExport.ts)
  - 役割: AudioBufferのオーディオデータをWAVファイルとしてエクスポートし、ユーザーにダウンロードを促します。
  - 引数: `audioBuffer` (AudioBuffer): エクスポートするオーディオデータ。 `filename` (string): ダウンロードされるファイル名。
  - 戻り値: なし
- **`smfToYM2151Json(smfData: Uint8Array)`** (demo/src/smfToYm2151.ts)
  - 役割: SMFバイトデータをWebAssemblyモジュールを介してYM2151のJSONログ形式に変換します。
  - 引数: `smfData` (Uint8Array): 変換対象のSMFバイトデータ。
  - 戻り値: string (YM2151 JSONログ)
- **`treeToJSON(node: any)`** (demo/src/treeToJSON.ts)
  - 役割: Tree-sitterの構文木ノードを再帰的にJSONオブジェクトに変換します。デバッグ出力の可読性を高めるために利用されます。
  - 引数: `node` (any): Tree-sitterの構文木ノード。
  - 戻り値: any (JSON形式の構文木表現)
- **`showStatus(message: string, isError: boolean = false)`** (demo/src/ui.ts)
  - 役割: デモのユーザーインターフェース上に状態メッセージを表示します。エラーの場合には異なるスタイルで表示します。
  - 引数: `message` (string): 表示するメッセージ。 `isError` (boolean, オプション): エラーメッセージかどうかを示すフラグ。
  - 戻り値: なし
- **`loadExample(mml: string)`** (demo/src/ui.ts)
  - 役割: 指定されたMML文字列をデモの入力欄にロードし、自動的に変換処理を開始します。
  - 引数: `mml` (string): ロードするMML文字列。
  - 戻り値: Promise<void>
- **`drawWaveform(audioBuffer: AudioBuffer, canvas: HTMLCanvasElement)`** (demo/src/visualization.ts)
  - 役割: AudioBufferのデータを用いて指定されたCanvas要素に波形を描画します。
  - 引数: `audioBuffer` (AudioBuffer): 描画するオーディオデータ。 `canvas` (HTMLCanvasElement): 描画対象のCanvas要素。
  - 戻り値: なし
- **`visualizeRealtime(audioContext: AudioContext, analyser: AnalyserNode, canvas: HTMLCanvasElement)`** (demo/src/visualization.ts)
  - 役割: Web Audio APIのAnalyserNodeからリアルタイムでオーディオデータを取得し、Canvasに視覚化（スペクトラムアナライザーなど）します。
  - 引数: `audioContext` (AudioContext): 使用するオーディオコンテキスト。 `analyser` (AnalyserNode): オーディオデータを分析するノード。 `canvas` (HTMLCanvasElement): 描画対象のCanvas要素。
  - 戻り値: なし
- **`draw()`** (demo/src/visualization.ts, visualizeRealtime内部関数)
  - 役割: `visualizeRealtime`関数から繰り返し呼び出され、リアルタイムオーディオの視覚化の実際の描画処理を行います。
  - 引数: なし (クロージャにより外部スコープの変数にアクセス)
  - 戻り値: なし
- **`mockAudioBuffer(channels: number, length: number, sampleRate: number)`** (demo/tests/audioBufferToWav.test.ts)
  - 役割: テストのために、指定されたチャンネル数、長さ、サンプルレートを持つ擬似的なAudioBufferオブジェクトを生成します。
  - 引数: `channels` (number): チャンネル数。 `length` (number): サンプルフレーム数。 `sampleRate` (number): サンプルレート。
  - 戻り値: AudioBuffer (擬似AudioBufferオブジェクト)
- **`buildSmf(events: Array<{deltaTime: number, type: number, data: number[]}>)`** (demo/tests/parseMidiNotes.test.ts)
  - 役割: MIDIイベントのリストからSMFバイトデータを構築します。これはテスト目的で使用されるヘルパー関数です。
  - 引数: `events` (Array<{deltaTime: number, type: number, data: number[]}>): MIDIイベントの配列。
  - 戻り値: Uint8Array (構築されたSMFバイトデータ)
- **`mockNode(type: string, children: any[] = [], text: string = '')`** (demo/tests/treeToJSON.test.ts)
  - 役割: テストのために、Tree-sitterのASTノードを模倣したオブジェクトを生成します。
  - 引数: `type` (string): ノードの型。 `children` (any[], オプション): 子ノードの配列。 `text` (string, オプション): ノードのテキスト内容。
  - 戻り値: object (擬似ASTノード)

## 関数呼び出し階層ツリー
```
- initialize() (demo/src/main.ts)
  - convertMML(mmlString) (demo/src/mmlConverter.ts)
    - [mmlabc_to_smf_wasm.js の WebAssembly 関数呼び出し]
  - parseMidiNotes(smfData) (demo/src/parseMidiNotes.ts)
    - deltaTicksToSeconds(deltaTicks, ticksPerBeat, tempo)
  - renderWaveformAndAudio(midiNotes, duration) (demo/src/audioRenderer.ts)
    - check(condition, message)
    - drawWaveform(audioBuffer, canvas) (demo/src/visualization.ts)
    - visualizeRealtime(audioContext, analyser, canvas) (demo/src/visualization.ts)
      - draw() (demo/src/visualization.ts)
  - playAudio(midiData, startOffset, bpm) (demo/src/audioPlayback.ts)
    - stopAudio()
    - showStatus(message, isError) (demo/src/ui.ts)
    - visualizeRealtime(audioContext, analyser, canvas) (demo/src/visualization.ts)
      - draw()
  - exportWav(audioBuffer, filename) (demo/src/wavExport.ts)
    - writeString(view, offset, string)
    - audioBufferToWav(buffer)
    - showStatus(message, isError)
  - treeToJSON(node) (demo/src/treeToJSON.ts)
  - showStatus(message, isError) (demo/src/ui.ts)
  - loadExample(mml) (demo/src/ui.ts)

- waitForWebYm2151() (demo/src/audioRenderer.ts)
  - calculateDuration(midiNotes)
  - renderWaveformAndAudio(midiNotes, duration)

- smfToYM2151Json(smfData) (demo/src/smfToYm2151.ts)
  - ensureInitialized()

- mockAudioBuffer(channels, length, sampleRate) (demo/tests/audioBufferToWav.test.ts)

- buildSmf(events) (demo/tests/parseMidiNotes.test.ts)

- mockNode(type, children, text) (demo/tests/treeToJSON.test.ts)

---
Generated at: 2026-03-09 07:06:59 JST
