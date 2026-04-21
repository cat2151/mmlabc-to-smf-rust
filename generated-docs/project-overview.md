Last updated: 2026-04-22

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) をStandard MIDI File (SMF) へ変換するRust製のライブラリ兼CLIです。
- MMLのパースからSMFバイト列生成までを4パス構成で処理し、ブラウザ向けWASM版も提供しています。
- 音符、オクターブ、和音、テンポ、ベロシティなど多様なMML記法に対応し、CLIおよびライブラリAPIで柔軟な利用が可能です。

## 技術スタック
- フロントエンド: HTML, TypeScript, JavaScript, WebAssembly (WASM), Tone.js (Web Audio APIラッパー), Web-YM2151 (YM2151エミュレータ)
- 音楽・オーディオ: Standard MIDI File (SMF), Music Macro Language (MML), MIDI
- 開発ツール: Rust (cargo), Node.js (npm, npx), tree-sitter-cli, Visual Studio Code (.vscode), Git
- テスト: Node.js (node:test, node:assert/strict), Rust (cargo test)
- ビルドツール: Rust (cargo build), wasm-bindgen, tree-sitter (パーサジェネレータ)
- 言語機能: Rust (強力な型システムと所有権モデル), TypeScript (型安全なJavaScript開発)
- 自動化・CI/CD: Shell Script (build-demo.sh, transform-demo-paths.sh)
- 開発標準: cargo clippy (Lint), cargo fmt (Formatter), EditorConfig (.editorconfig)

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
│   ├── 133.md
│   ├── 39.md
│   └── 44.md
├── mmlabc-to-smf-rust.toml.example
├── mmlabc-to-smf-wasm/
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── pkg/
│   │   └── mmlabc_to_smf_wasm.js (生成物)
│   ├── src/
│   │   ├── lib.rs
│   │   └── token_extractor.rs
│   └── tests/
│       └── parity.rs
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
│   ├── test_chord/
│   │   ├── basic.rs
│   │   ├── length.rs
│   │   └── octave.rs
│   ├── test_chord.rs
│   ├── test_cli.rs
│   ├── test_config.rs
│   ├── test_dotted_notes.rs
│   ├── test_drum_channel.rs
│   ├── test_key_transpose.rs
│   ├── test_length.rs
│   ├── test_library_api.rs
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
- **.editorconfig**: エディタのコードスタイル設定を定義し、プロジェクト全体のコーディング規約を統一します。
- **.gitattributes**: Gitがファイルを扱う際の属性（改行コード、マージ方法など）を定義し、クロスプラットフォームでの一貫性を保ちます。
- **.gitignore**: Gitのバージョン管理から除外するファイルやディレクトリを指定します。
- **.vscode/settings.json**: Visual Studio Codeのワークスペース固有の設定ファイルで、開発環境をカスタマイズします。
- **Cargo.lock**: Rustプロジェクトの依存関係の正確なバージョンを記録し、再現可能なビルドを保証します。
- **Cargo.toml**: Rustプロジェクトのビルド設定、依存クレート、メタデータを定義するマニフェストファイルです。
- **LICENSE**: プロジェクトのライセンス情報（MIT License）を記載したファイルです。
- **README.ja.md**: プロジェクトの日本語での概要、機能、使用方法、開発方法などが記載された説明書です。
- **README.md**: プロジェクトの英語での概要、機能、使用方法、開発方法などが記載された説明書です。
- **_config.yml**: Jekyllなど静的サイトジェネレーターの設定ファイルで、デモサイトのビルド設定に使われる可能性があります。
- **build.rs**: Rustのビルドスクリプトで、特に`tree-sitter-mml`パーサの生成など、カスタムビルドステップを実行します。
- **demo/index.html**: ブラウザデモのメインHTMLファイルで、ユーザーインターフェースとMML変換/再生ロジックを読み込みます。
- **demo/src/audioPlayback.ts**: Web Audio API (Tone.js) を利用して、SMFデータを音声として再生・停止するロジックを提供します。
- **demo/src/audioRenderer.ts**: SMFデータからオーディオをレンダリングし、その波形をCanvasに描画する処理を担います。Web-YM2151との連携も行います。
- **demo/src/main.ts**: デモアプリケーションの初期化処理（WASMモジュールのロード、イベントハンドラ設定）を記述しています。
- **demo/src/midiReader.ts**: MIDIファイルをバイト列として読み込み、その構造を解析するためのヘルパークラスが含まれています。
- **demo/src/mmlConverter.ts**: ブラウザ上で`mmlabc-to-smf-wasm`モジュールを呼び出し、MML文字列をSMFバイト列に変換する役割を持ちます。
- **demo/src/parseMidiNotes.ts**: MIDIデータから個々の音符イベントとその時間情報を抽出し、解析するロジックを提供します。
- **demo/src/smfToYm2151.ts**: SMFデータをYM2151エミュレータが解釈可能なJSONログ形式に変換するためのインターフェースです。
- **demo/src/state.ts**: デモアプリケーション全体で共有される状態変数や設定を定義します。
- **demo/src/treeToJSON.ts**: `tree-sitter`によって生成された構文木構造をJSON形式でシリアライズするユーティリティ関数を提供します。
- **demo/src/ui.ts**: デモのユーザーインターフェース要素（ステータス表示、MML入力フィールド、ボタンなど）を操作する関数を定義します。
- **demo/src/visualization.ts**: 生成されたオーディオの波形をCanvas要素に描画する視覚化ロジックを含みます。
- **demo/src/wavExport.ts**: 変換・生成されたオーディオデータをWAVファイル形式でエクスポート・ダウンロードする機能を提供します。
- **demo/tests/audioBufferToWav.test.ts**: `wavExport.ts`モジュールの`audioBufferToWav`関数のテストケースです。
- **demo/tests/midiReader.test.ts**: `midiReader.ts`モジュールのMIDIデータ読み取り機能に関するテストケースです。
- **demo/tests/parseMidiNotes.test.ts**: `parseMidiNotes.ts`モジュールのMIDIノート解析機能に関するテストケースです。
- **demo/tests/treeToJSON.test.ts**: `treeToJSON.ts`モジュールの構文木JSON変換機能に関するテストケースです。
- **demo-library/index.html**: `mmlabc-to-smf-wasm`ライブラリの基本的な使用例を示すシンプルなHTMLデモです。
- **googled947dc864c270e07.html**: Googleサイト所有権の検証に使用されるファイルです。
- **issue-notes/*.md**: 開発中に特定された課題や検討事項を記録するためのMarkdownファイルです。
- **mmlabc-to-smf-rust.toml.example**: プロジェクトの挙動をカスタマイズするための設定ファイル（TOML形式）のサンプルです。
- **mmlabc-to-smf-wasm/Cargo.lock**: `mmlabc-to-smf-wasm`クレートの依存関係のロックファイルです。
- **mmlabc-to-smf-wasm/Cargo.toml**: WebAssemblyターゲット向け`mmlabc-to-smf`ライブラリのビルド設定ファイルです。
- **mmlabc-to-smf-wasm/src/lib.rs**: `mmlabc-to-smf-wasm`クレートのメインライブラリコード。WASMバインディングを介してMML-to-SMF変換機能を提供します。
- **mmlabc-to-smf-wasm/src/token_extractor.rs**: MML文字列からトークンを抽出するロジックを実装しています。
- **mmlabc-to-smf-wasm/tests/parity.rs**: WASM版ライブラリがネイティブ版と同じ結果を生成することを確認するためのテストです。
- **package.json**: プロジェクト全体のNode.js依存関係（`tree-sitter-cli`など）とスクリプトを定義します。
- **scripts/README.md**: `scripts/`ディレクトリに含まれるシェルスクリプトの目的を説明するファイルです。
- **scripts/build-demo.sh**: デモアプリケーションをビルドするためのシェルスクリプトです。
- **scripts/transform-demo-paths.sh**: デモのファイルパスを変換するためのシェルスクリプトです。
- **src/attachment_json.rs**: MMLの先頭に埋め込まれたJSON形式のメタデータをパースし、利用するロジックを扱います。
- **src/config.rs**: 外部MIDIプレイヤーの指定やドラムチャンネルの割り当てなど、アプリケーションの実行時設定を管理するモジュールです。
- **src/lib.rs**: `mmlabc-to-smf`ライブラリの主要な公開APIを定義し、MML-to-SMF変換プロセスの全体をオーケストレートします。
- **src/main.rs**: CLIツールのエントリポイント。コマンドライン引数を解析し、ライブラリの変換機能を呼び出して結果を出力します。
- **src/mml_preprocessor.rs**: MML文字列の構文解析前に、特定のパターン（埋め込みJSONなど）を前処理するロジックを実装します。
- **src/parse_tree_tokens.rs**: `tree-sitter`が生成したパースツリーから、MMLの意味を表すトークンを抽出し、中間表現に変換します。
- **src/pass1_parser.rs**: MML変換プロセスの最初のパス。`tree-sitter-mml`クレートを使用してMML文字列を構文解析し、ASTを生成します。
- **src/pass2_ast.rs**: MML変換プロセスの2番目のパス。パス1で生成された抽象構文木を、さらに意味的な情報を付加したASTに変換します。
- **src/pass3_events.rs**: MML変換プロセスの3番目のパス。パス2のASTを基に、MIDIイベントのシーケンス（ノートオン/オフ、テンポチェンジなど）を生成します。
- **src/pass4_midi.rs**: MML変換プロセスの4番目のパス。パス3で生成されたMIDIイベントシーケンスを、最終的なStandard MIDI File (SMF) のバイト列に変換します。
- **src/tree_sitter_mml.rs**: `tree-sitter-mml`クレートへのラッパーを提供し、MML言語の構文解析器をRustから利用可能にします。
- **src/types.rs**: プロジェクト全体で利用されるカスタムのデータ構造や型定義を一元的に管理します。
- **tests/*.rs**: RustライブラリとCLIの機能が正しく動作するかを検証する様々な単体テストおよび統合テストファイルです。
- **tree-sitter-mml/grammar.js**: `tree-sitter`パーサジェネレータがMML言語の構文解析器を生成するために使用する文法定義ファイルです。
- **tree-sitter-mml/src/grammar.json**: `grammar.js`から生成されるMMLのJSON形式の文法定義です。
- **tree-sitter-mml/src/node-types.json**: `grammar.js`から生成されるMMLのノードタイプ定義です。
- **tree-sitter-mml/src/parser.c**: `grammar.js`から生成されるC言語で書かれたMMLパーサのソースコードです。
- **tree-sitter-mml/src/tree_sitter/*.h**: `tree-sitter`ライブラリのC言語ヘッダーファイル群です。
- **tree-sitter-mml/tree-sitter-mml.wasm**: `grammar.js`から生成されるMMLパーサのWebAssemblyバイナリで、ブラウザ環境で利用されます。

## 関数詳細説明
- `playAudio(midiData: Uint8Array, audioContext: AudioContext)`: 指定されたMIDIデータをWeb Audio APIで再生します。`midiData`はSMFバイト列、`audioContext`はWeb Audio Context。戻り値なし。
- `stopAudio(audioContext: AudioContext)`: 現在再生中のオーディオを停止します。`audioContext`はWeb Audio Context。戻り値なし。
- `waitForWebYm2151(): Promise<void>`: Web-YM2151モジュールの初期化完了を待機します。引数なし。初期化完了を示すPromiseを返します。
- `calculateDuration(midiData: Uint8Array): number`: MIDIデータの総再生時間（秒）を計算します。`midiData`はSMFバイト列。再生時間（秒）を返します。
- `renderWaveformAndAudio(midiData: Uint8Array, audioContext: AudioContext, tempo: number, visualizationElement: HTMLCanvasElement): Promise<AudioBuffer>`: MIDIデータから音声をレンダリングし、波形をCanvasに描画します。`midiData`はSMFバイト列、`audioContext`はAudioContext、`tempo`はテンポ、`visualizationElement`はCanvas要素。レンダリングされたAudioBufferを返します。
- `check(midiData: Uint8Array)`: MIDIデータの整合性をチェックするデバッグ用関数です。詳細はコード参照。`midiData`はSMFバイト列。戻り値なし。
- `initialize(mmlabc_to_smf_wasm, WebAssembly, midiPlayer)`: デモアプリケーションの初期化を行います。WASMモジュールのロードやイベントリスナーの設定などを行います。戻り値なし。
- `constructor(midiData: Uint8Array)`: MIDIデータ読み取りクラスのコンストラクタです。`midiData`はMIDIデータのUint8Array。クラスインスタンスを構築します。
- `convertMML(mmlString: string, options: SmfConversionOptions)`: MML文字列をSMFバイト配列に変換します。`mmlString`はMML文字列、`options`は変換オプション。変換されたSMFバイト配列を返します。
- `parseMidiNotes(midiData: Uint8Array, options?: { tickDuration: number })`: MIDIデータから音符イベントとタイム情報を解析し、構造化された配列として返します。`midiData`はSMFバイト列、`options`は解析オプション。解析された音符イベントの配列を返します。
- `deltaTicksToSeconds(deltaTicks: number, bpm: number, ticksPerBeat: number): number`: デルタティック（MIDIのタイム単位）を秒数に変換します。`deltaTicks`はデルタティック値、`bpm`はテンポ（BPM）、`ticksPerBeat`は1拍あたりのティック数。変換された秒数を返します。
- `ensureInitialized()`: `smf-to-ym2151log-wasm`モジュールの初期化を保証します。引数なし。初期化が完了したPromiseを返します。
- `smfToYM2151Json(midiData: Uint8Array, options?: { programChangeToYM2151Patch: Record<number, number> })`: SMFデータをWeb-YM2151が解釈できるJSONログ形式に変換します。`midiData`は変換対象のSMFバイト列、`options`は変換オプション。YM2151 JSONログを返します。
- `treeToJSON(node: Node)`: `tree-sitter`の構文木ノードを再帰的にJSON形式に変換します。デバッグや可視化に使用されます。`node`は`tree-sitter`ノード。JSON形式のノード構造を返します。
- `showStatus(message: string, isError: boolean)`: ユーザーインターフェースにステータスメッセージを表示します。`message`は表示する文字列、`isError`はエラーメッセージフラグ。戻り値なし。
- `loadExample(exampleMML: string)`: 指定されたMML文字列をエディタにロードし、変換プロセスを開始します。`exampleMML`はロードするMML文字列。戻り値なし。
- `drawWaveform(canvas: HTMLCanvasElement, audioBuffer: AudioBuffer, color: string)`: AudioBufferの内容をCanvasに波形として描画します。`canvas`は描画対象Canvas、`audioBuffer`はオーディオデータ、`color`は波形の色。戻り値なし。
- `writeString(view: DataView, offset: number, s: string): number`: DataViewに文字列を書き込みます。WAVファイルヘッダの構築に使用されます。`view`はDataView、`offset`はオフセット、`s`は文字列。書き込んだバイト数を返します。
- `audioBufferToWav(buffer: AudioBuffer): Blob`: AudioBufferのオーディオデータをWAV形式のBlobに変換します。`buffer`はAudioBuffer。WAV形式のBlobデータを返します。
- `exportWav(audioBuffer: AudioBuffer, filename: string)`: AudioBufferの内容をWAVファイルとしてダウンロードさせます。`audioBuffer`はエクスポートするAudioBuffer、`filename`はファイル名。戻り値なし。
- `mockAudioBuffer(config: { sampleRate: number; numberOfChannels: number; length: number; data?: Float32Array[] })`: テスト用の`AudioBuffer`オブジェクトを模擬的に作成します。`config`は設定オブジェクト。模擬`AudioBuffer`オブジェクトを返します。
- `buildSmf(trackEvents: { type: string; delta: number; data: number[] }[][], ticksPerBeat: number = 480)`: テスト用に指定されたイベントリストとティック情報から簡略化されたSMF（Standard MIDI File）バイト配列を構築します。`trackEvents`はMIDIイベントリスト、`ticksPerBeat`は1拍あたりのティック数。構築されたSMFバイト配列を返します。
- `mockNode(type: string, children: any[] = [], text: string = '')`: `treeToJSON`関数のテストに使用する`tree-sitter`ノードのモックオブジェクトを作成します。`type`はノードタイプ、`children`は子ノード配列、`text`はノードテキスト。モックされたノードオブジェクトを返します。

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
Generated at: 2026-04-22 07:17:59 JST
