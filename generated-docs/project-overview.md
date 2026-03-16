Last updated: 2026-03-17

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) 形式の文字列をStandard MIDI File (SMF) に変換するRust製ライブラリです。
- ネイティブアプリケーション向けと、WebAssembly (WASM) を利用したブラウザアプリケーション向けの両方で利用可能です。
- MMLのトークン化、AST変換、MIDIイベント生成、SMF作成の4パスアーキテクチャにより、堅牢な変換処理を実現しています。

## 技術スタック
- フロントエンド: HTML (デモUI), TypeScript (デモアプリケーションロジック), JavaScript (Tone.js, web-tree-sitter.js), WebAssembly (mmlabc-to-smf-wasmクレートのブラウザ連携)
- 音楽・オーディオ: Music Macro Language (MML) (入力形式), Standard MIDI File (SMF) (出力形式), MIDIイベント, Tone.js (Web Audio APIベースの音楽フレームワーク), TiMidity++, FluidSynth, VLC, cat-play-mml (外部MIDIプレイヤーとして利用可能)
- 開発ツール: Rust (主要言語), Cargo (Rustのビルドシステム・パッケージマネージャ), Node.js, npm, npx (tree-sitterパーサー生成に使用), Git (バージョン管理), Visual Studio Code (エディタ設定ファイル)
- テスト: `cargo test` (Rustユニット/統合テスト), `node:test`, `node:assert/strict` (TypeScriptテストフレームワーク/アサーションライブラリ)
- ビルドツール: Cargo (Rustクレートビルド), tree-sitter (MMLパーサー生成), WebAssembly (WASM) (ブラウザ向けライブラリのターゲット)
- 言語機能: Rustの型システムと所有権モデル (安全な設計を特徴とする)
- 開発標準: .editorconfig (コーディングスタイルの統一), `cargo fmt` (コードフォーマッタ), `cargo clippy` (Linter)

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
│   ├── 121.md
│   ├── 123.md
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
- **`.editorconfig`**: プロジェクト全体で一貫したコーディングスタイル（インデント、改行コードなど）を強制するための設定ファイルです。
- **`.gitignore`**: Gitがバージョン管理から除外すべきファイルやディレクトリを指定する設定ファイルです。
- **`.vscode/settings.json`**: Visual Studio Codeのワークスペース固有の設定ファイルで、開発環境の統一に貢献します。
- **`Cargo.lock`**: Rustプロジェクトの依存関係ツリーの正確なバージョンを記録し、再現性のあるビルドを保証します。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイルで、プロジェクト名、バージョン、ライブラリの種類、依存関係などが定義されています。
- **`IMPLEMENTATION_REPORT.md`**: プロジェクトの実装に関する詳細なレポートや文書です。
- **`LICENSE`**: プロジェクトのライセンス情報が記載されたファイルで、通常MITライセンスが適用されています。
- **`OPTION_A_IMPLEMENTATION.md`**: 実装オプションAに関する詳細なドキュメントです。
- **`README.ja.md`**: プロジェクトの概要、機能、使い方、開発状況などを日本語で説明するメインのドキュメントです。
- **`README.md`**: プロジェクトの概要、機能、使い方、開発状況などを英語で説明するメインのドキュメントです。
- **`_config.yml`**: おそらくJekyllなどの静的サイトジェネレータで使用される設定ファイルです。
- **`build.rs`**: Rustのビルドスクリプトで、主に`tree-sitter-mml`パーサーのC言語ソースファイルの自動生成ロジックが含まれています。
- **`demo/index.html`**: Webブラウザ上でMML to SMF変換を試せるデモアプリケーションのHTMLファイルです。
- **`demo/src/audioPlayback.ts`**: デモアプリケーションにおける音声再生（Tone.jsを使用）の開始・停止ロジックを担当します。
- **`demo/src/audioRenderer.ts`**: MIDIデータからYM2151ログを生成し、そのオーディオ波形をレンダリングする処理を担います。
- **`demo/src/main.ts`**: デモアプリケーションのメインエントリーポイントで、初期化、WebAssemblyモジュールのロード、イベントハンドラの登録を行います。
- **`demo/src/midiReader.ts`**: MIDIファイルのバイナリデータを読み込み、解析してMIDIイベントを抽出するクラスです。
- **`demo/src/mmlConverter.ts`**: MML文字列をWebAssemblyモジュール（`mmlabc-to-smf-wasm`）を介してSMFに変換するロジックを含みます。
- **`demo/src/parseMidiNotes.ts`**: 解析されたMIDIイベントから具体的な音符（ノートオン/オフ、ピッチ、ベロシティなど）の情報を抽出する処理を行います。
- **`demo/src/smfToYm2151.ts`**: Standard MIDI File (SMF) データをYM2151ログ形式に変換するためのインターフェースを提供します。
- **`demo/src/state.ts`**: デモアプリケーションの現在の状態（MMLコード、MIDIデータ、再生状況など）を管理するグローバルステート定義です。
- **`demo/src/treeToJSON.ts`**: `tree-sitter`によって生成された抽象構文木（AST）をデバッグや表示のためにJSON形式にシリアライズするユーティリティ関数です。
- **`demo/src/ui.ts`**: デモアプリケーションのユーザーインターフェース要素（ステータス表示、サンプルコードのロードなど）に関する操作ロジックを扱います。
- **`demo/src/visualization.ts`**: 生成されたオーディオ波形をCanvas要素に描画し、視覚的に表示する処理を担います。
- **`demo/src/wavExport.ts`**: 生成されたAudioBufferを標準的なWAVファイル形式に変換し、エクスポートする機能を提供します。
- **`demo/tests/audioBufferToWav.test.ts`**: `wavExport.ts`の`audioBufferToWav`関数のテストケースを含みます。
- **`demo/tests/midiReader.test.ts`**: `midiReader.ts`に実装されたMIDIデータ読み取り機能のテストケースを含みます。
- **`demo/tests/parseMidiNotes.test.ts`**: `parseMidiNotes.ts`に実装されたMIDIノート解析機能のテストケースを含みます。
- **`demo/tests/treeToJSON.test.ts`**: `treeToJSON.ts`に実装されたTree-sitter AST変換機能のテストケースを含みます。
- **`demo-library/index.html`**: デモライブラリのインデックスHTMLファイルです。
- **`googled947dc864c270e07.html`**: Googleサイト認証のために使用されるHTMLファイルです。
- **`issue-notes/`**: 開発イシューに関するメモや記録が格納されています。
- **`mmlabc-to-smf-rust.toml.example`**: カスタムMIDIプレイヤーのパスを設定するための設定ファイル例です。
- **`mmlabc-to-smf-wasm/src/lib.rs`**: MMLをSMFに変換するロジックのWebAssembly (WASM) バインディングを提供するRustライブラリのエントリポイントです。
- **`mmlabc-to-smf-wasm/src/token_extractor.rs`**: WASMモジュール内でMML文字列からトークンを抽出する特定のロジックを実装しています。
- **`package.json`**: プロジェクト全体のJavaScriptエコシステムにおける依存関係（例: `tree-sitter-cli`）とスクリプトを定義します。
- **`scripts/build-demo.sh`**: デモアプリケーションをビルドするためのシェルスクリプトです。
- **`scripts/transform-demo-paths.sh`**: デモアプリケーション内のパスを変換するためのシェルスクリプトです。
- **`src/attachment_json.rs`**: デバッグ目的で中間データをJSON形式で出力する機能に関するロジックを扱います。
- **`src/config.rs`**: コマンドライン引数や設定ファイルからの構成情報をパースし、管理するロジックが含まれています。
- **`src/lib.rs`**: Rustライブラリのメインエントリーポイントで、MMLからSMFへの変換を行う主要な公開APIを定義しています。
- **`src/main.rs`**: プロジェクトのコマンドラインインターフェース (CLI) のエントリポイントです。コマンドライン引数を解析し、変換処理を呼び出します。
- **`src/mml_preprocessor.rs`**: MML文字列の解析前に、特定の変換や正規化を行う前処理ロジックを実装しています。
- **`src/pass1_parser.rs`**: MML文字列を`tree-sitter`パーサーを使用してトークンストリームに変換する4パスアーキテクチャの最初のステージです。
- **`src/pass2_ast.rs`**: トークンストリームから抽象構文木（AST）を構築する4パスアーキテクチャの第2ステージです。
- **`src/pass3_events.rs`**: ASTから具体的なMIDIイベント（ノートオン/オフ、テンポチェンジなど）を生成する4パスアーキテクチャの第3ステージです。
- **`src/pass4_midi.rs`**: 生成されたMIDIイベントのリストから、最終的なStandard MIDI Fileのバイナリデータを作成する4パスアーキテクチャの最終ステージです。
- **`src/tree_sitter_mml.rs`**: `tree-sitter` MMLパーサーとの連携およびそのRustラッパーに関するロジックが含まれています。
- **`src/types.rs`**: プロジェクト全体で使用される共通のデータ構造や列挙型などの型定義をまとめています。
- **`tests/integration_test.rs`**: プロジェクト全体のMML to SMF変換フローをテストする統合テストファイルです。
- **`tests/test_attachment_json.rs`**: `attachment_json`モジュールの機能（JSON出力など）をテストします。
- **`tests/test_c1_vs_c64.rs`**: C1とC64など、特定のMML音符記法に関する挙動をテストします。
- **`tests/test_channel.rs`**: MMLにおける多チャンネル（`;`で区切られる）処理の正確性をテストします。
- **`tests/test_chord.rs`**: MMLでの和音指定機能のテストケースを含みます。
- **`tests/test_cli.rs`**: コマンドラインインターフェース（CLI）の引数解析や動作をテストします。
- **`tests/test_config.rs`**: 設定ファイル（例: `mmlabc-to-smf-rust.toml`）の読み込みと適用をテストします。
- **`tests/test_dotted_notes.rs`**: 付点音符（例: `c8.`）の解釈とMIDI変換のテストです。
- **`tests/test_drum_channel.rs`**: ドラムチャンネル（MMLでの表現）のMIDI変換をテストします。
- **`tests/test_key_transpose.rs`**: キー転調（移調）コマンドのテストです。
- **`tests/test_length.rs`**: MMLでの音長指定（例: `l4`）の解釈をテストします。
- **`tests/test_modifier.rs`**: MMLの音符修飾子（例: `^`, `#`, `v`）のテストです。
- **`tests/test_note_length.rs`**: 音符の長さ（例: `c1`, `d2`）の正しい解釈をテストします。
- **`tests/test_octave.rs`**: オクターブ指定（`o`, `<`, `>`）コマンドのテストです。
- **`tests/test_pass1.rs`**: 4パスアーキテクチャの第一パス（MML文字列のトークン化）のユニットテストです。
- **`tests/test_pass2.rs`**: 4パスアーキテクチャの第二パス（トークンからASTへの変換）のユニットテストです。
- **`tests/test_pass3.rs`**: 4パスアーキテクチャの第三パス（ASTからMIDIイベント生成）のユニットテストです。
- **`tests/test_pass4.rs`**: 4パスアーキテクチャの第四パス（MIDIイベントからSMF作成）のユニットテストです。
- **`tests/test_program_change.rs`**: プログラムチェンジ（音色変更）コマンドのテストです。
- **`tests/test_rest.rs`**: 休符（`r`）の解釈とMIDI変換のテストです。
- **`tests/test_tempo.rs`**: テンポ指定（`t`）コマンドのテストです。
- **`tests/test_velocity.rs`**: ベロシティ（音の強さ、`v`）指定コマンドのテストです。
- **`tree-sitter-mml/grammar.js`**: `tree-sitter`パーサーのMML文法定義ファイルです。このファイルからC言語のパーサーが生成されます。
- **`tree-sitter-mml/package.json`**: `tree-sitter-mml`パーサーのNode.js依存関係を定義します。
- **`tree-sitter-mml/src/grammar.json`**: `grammar.js`から生成されるMML文法のJSON表現です。
- **`tree-sitter-mml/src/node-types.json`**: `grammar.js`から生成されるASTノードタイプのJSON表現です。
- **`tree-sitter-mml/src/parser.c`**: `grammar.js`から自動生成されたMMLパーサーのC言語ソースコードです。
- **`tree-sitter-mml/src/tree_sitter/`**: `tree-sitter`のC言語ランタイムに必要なヘッダーファイルが含まれています。
- **`tree-sitter-mml/tree-sitter-mml.wasm`**: `tree-sitter-mml`パーサーのWebAssemblyバイナリファイルで、ブラウザ環境での利用を可能にします。

## 関数詳細説明
- `playAudio()`: デモアプリケーションで、生成されたMIDIデータに基づいて音源を再生開始します。引数はありません。
- `stopAudio()`: 現在再生中の音源を停止します。引数はありません。
- `waitForWebYm2151()`: WebMidi APIを利用してYM2151エミュレータの初期化を待ち、オーディオレンダリングの準備をします。引数はありません。
- `calculateDuration()`: MIDIデータから曲の総演奏時間を計算します。引数はありません。
- `renderWaveformAndAudio()`: 生成されたYM2151ログデータから波形をレンダリングし、最終的なオーディオデータを生成します。引数はありません。
- `check()`: （デモアプリケーションの内部的な状態や条件を）チェックする関数です。具体的な引数と戻り値は提供されていません。
- `initialize()`: デモアプリケーションの起動時に必要な初期設定（Tree-sitterパーサーのロード、WASMモジュールの初期化、UI要素のイベントリスナー設定など）を実行します。引数はありません。
- `convertMML()`: MML文字列を受け取り、WebAssemblyモジュールを介してStandard MIDI File (SMF) 形式に変換します。引数: `mml_string: string` (MML文字列)。戻り値: `Promise<Uint8Array>` (SMFバイナリデータ) またはエラー。
- `parseMidiNotes()`: MIDIファイルバイナリデータから、各音符の開始時刻、終了時刻、ピッチ、ベロシティなどの詳細情報を解析し、構造化されたデータとして返します。引数: `midi_data: Uint8Array` (MIDIファイルバイナリデータ)。戻り値: `NoteEvent[]` (音符イベントの配列)。
- `deltaTicksToSeconds()`: MIDIファイル内で使用されるデルタティック値（相対時間単位）を実際の秒数に変換します。引数: `delta_ticks: number`, `ticks_per_beat: number`, `tempo: number`。戻り値: `number` (秒数)。
- `writeString()`: WAVファイルヘッダの特定の位置に文字列（例: "RIFF", "WAVE"）を書き込みます。引数: `view: DataView`, `offset: number`, `s: string`。戻り値: なし。
- `audioBufferToWav()`: Web Audio APIの`AudioBuffer`オブジェクトを標準的なWAVファイル形式のバイナリデータに変換します。引数: `audio_buffer: AudioBuffer`。戻り値: `ArrayBuffer` (WAVファイルバイナリ)。
- `exportWav()`: 指定されたAudioBufferをWAVファイルとしてブラウザからダウンロード可能にします。引数: `audio_buffer: AudioBuffer`。戻り値: なし。
- `ensureInitialized()`: YM2151ログ変換のWebAssemblyモジュールがロードされ、初期化されていることを確認します。未初期化の場合はロードをトリガーします。引数はありません。
- `smfToYM2151Json()`: Standard MIDI File (SMF) データを受け取り、それをYM2151という音源チップ用の操作ログを表すJSON形式に変換します。引数: `smf_data: Uint8Array` (SMFバイナリデータ)。戻り値: `string` (YM2151ログJSON文字列)。
- `drawWaveform()`: 指定されたオーディオデータに基づいて、Canvas上に波形を視覚的に描画します。引数: `context: CanvasRenderingContext2D`, `data: Float32Array`。戻り値: なし。
- `visualizeRealtime()`: リアルタイムで再生されるオーディオの波形を視覚化します。引数: `analyser_node: AnalyserNode`, `canvas_context: CanvasRenderingContext2D`, `buffer_length: number`, `data_array: Uint8Array`。戻り値: なし。
- `draw()`: （視覚化関連の）描画処理を行います。具体的な引数と戻り値は提供されていません。
- `treeToJSON()`: Tree-sitterで生成された構文木（Nodeオブジェクト）を再帰的に走査し、JSON形式のオブジェクトとして表現します。引数: `node: any` (Tree-sitterのNodeオブジェクト)。戻り値: `object` (JSON表現のAST)。
- `showStatus()`: デモアプリケーションのUI上にユーザー向けの状態メッセージを表示します。引数: `message: string`。戻り値: なし。
- `loadExample()`: デモ用のMMLサンプルコードをテキストエリアにロードします。引数はありません。
- `mockAudioBuffer()`: テストの目的で、Web Audio APIのAudioBufferオブジェクトを模擬したオブジェクトを作成します。引数: `channels: number`, `length: number`, `sample_rate: number`, `data: number[][]`。戻り値: `AudioBuffer`のモックオブジェクト。
- `buildSmf()`: テストの目的で、簡素なStandard MIDI File (SMF) バイナリデータを構築します。引数: `notes: {note: number, start: number, duration: number}[]` (音符イベントの配列)。戻り値: `Uint8Array` (SMFバイナリデータ)。
- `mockNode()`: テストの目的で、Tree-sitterのNodeオブジェクトを模擬したオブジェクトを作成します。引数: `type: string`, `children: any[]`。戻り値: `Node`のモックオブジェクト。

## 関数呼び出し階層ツリー
```
- initialize()
    - convertMML()
        - smfToYM2151Json()
        - treeToJSON()
    - loadExample()
- playAudio()
    - stopAudio()
        - showStatus()
        - visualizeRealtime()
- waitForWebYm2151()
    - calculateDuration()
        - renderWaveformAndAudio()
        - drawWaveform()
- parseMidiNotes()
    - deltaTicksToSeconds()
- exportWav()
    - writeString()
        - audioBufferToWav()
- mockAudioBuffer()
- buildSmf()
- mockNode()

---
Generated at: 2026-03-17 07:13:12 JST
