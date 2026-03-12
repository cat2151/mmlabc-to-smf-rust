Last updated: 2026-03-13

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) 形式の音楽データをStandard MIDI File (SMF) へ変換するRust製ライブラリです。
- tree-sitterを用いた構文解析を含む4パスアーキテクチャにより、MMLの正確かつ効率的な変換を実現します。
- ネイティブアプリケーション（CLI）としてMMLの再生・SMF出力が可能であり、WebAssembly (WASM) を介してブラウザデモでの利用も想定されています。

## 技術スタック
- フロントエンド: TypeScript (デモUIロジック), Tone.js (デモでのオーディオ再生), web-tree-sitter (ブラウザでのMML構文解析), mmlabc_to_smf_wasm (MML->SMF変換のWASM版), smf_to_ym2151log (SMF->YM2151ログ変換のWASM版)
- 音楽・オーディオ: Standard MIDI File (SMF), Music Macro Language (MML), cat-play-mml (デフォルトMIDIプレイヤー), TiMidity++, FluidSynth, VLC (カスタムMIDIプレイヤー)
- 開発ツール: Cargo (Rustパッケージマネージャ), git (バージョン管理), npx, npm (Node.jsパッケージ管理), Visual Studio Code (エディタ設定), DeepWiki (ドキュメントプラットフォーム)
- テスト: `cargo test` (Rustユニット/統合テスト), Node.js `node:test`, `node:assert/strict` (TypeScriptテスト)
- ビルドツール: Cargo (Rustビルドシステム), tree-sitter-cli (`npx tree-sitter generate`によるパーサー生成), シェルスクリプト (`build-demo.sh`, `transform-demo-paths.sh` によるデモビルド)
- プログラミング言語: Rust (MML→SMF変換ライブラリ本体), C (tree-sitterパーサー生成物)
- 自動化・継続的統合: 特徴としてCI/CDとクロスプラットフォームビルドの簡素化が言及されています。
- 開発標準: `cargo clippy` (Linter), `cargo fmt` (Formatter), .editorconfig (コードスタイル定義)

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
│   ├── 117.md
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
- **Cargo.toml**: Rustプロジェクトの設定ファイル。依存関係やパッケージ情報、ビルド設定を定義します。
- **build.rs**: Rustプロジェクトのビルドスクリプト。tree-sitterパーサーのC言語ファイルを必要に応じて再生成するロジックを含みます。
- **src/main.rs**: コマンドラインインターフェース (CLI) のエントリーポイント。MML文字列の処理、SMF生成、外部プレイヤーでの再生などを制御します。
- **src/lib.rs**: MMLからSMFへの変換ロジックを公開するライブラリクレートのルートです。
- **src/pass1_parser.rs**: MML文字列をトークン列に解析する最初のステップを担います。主にtree-sitterパーサーとの連携を行います。
- **src/pass2_ast.rs**: トークン列から抽象構文木（AST）を構築する処理を担当します。
- **src/pass3_events.rs**: ASTをMIDIイベントのシーケンスに変換するロジックを含みます。
- **src/pass4_midi.rs**: MIDIイベントのシーケンスから最終的なStandard MIDI Fileのバイナリデータを構築します。
- **src/tree_sitter_mml.rs**: Rustコードからtree-sitter MMLパーサーを利用するためのブリッジを提供します。
- **src/types.rs**: プロジェクト内で使用されるMML要素、MIDIイベント、ASTノードなどの共通データ構造を定義します。
- **src/config.rs**: CLIアプリケーションの設定（例: 外部MIDIプレイヤーの指定）を管理します。
- **src/mml_preprocessor.rs**: MML文字列の解析前に必要な前処理（例: コメント除去、マクロ展開）を行うモジュールです。
- **src/attachment_json.rs**: デバッグ目的で、変換プロセスの各パスで生成される中間結果（トークン、AST、MIDIイベント）をJSON形式で出力する機能を提供します。
- **tests/** (ディレクトリ): Rustクレートのテストコードが格納されています。各パスの機能テストや統合テストが含まれます。
- **tree-sitter-mml/grammar.js**: MMLの文法定義をJavaScriptで記述したファイルです。これを用いてtree-sitterパーサーのC言語ソースが生成されます。
- **tree-sitter-mml/src/parser.c**: `grammar.js`から生成されたMMLパーサーのC言語ソースファイルです。
- **mmlabc-to-smf-wasm/src/lib.rs**: WebAssemblyターゲット向けにMMLからSMFへの変換機能を提供するRustライブラリのエントリーポイントです。
- **mmlabc-to-smf-wasm/src/token_extractor.rs**: WASM環境でMMLトークンを抽出するための機能を提供します。
- **demo/index.html**: ブラウザ上で動作するMML-to-SMF変換デモアプリケーションのメインHTMLファイルです。
- **demo/src/main.ts**: デモアプリケーションの起動スクリプト。WebAssemblyモジュールのロードや初期設定を行います。
- **demo/src/mmlConverter.ts**: デモUIからMML文字列を受け取り、WebAssemblyモジュールを介してSMFデータに変換するロジックです。
- **demo/src/audioPlayback.ts**: デモで生成されたオーディオデータ（WAVなど）をブラウザで再生する機能を提供します。Tone.jsを利用します。
- **demo/src/audioRenderer.ts**: 変換されたSMFデータに基づき、ブラウザ上で波形をレンダリングしたり、オーディオバッファを生成する役割を担います。
- **demo/src/midiReader.ts**: デモ内でMIDIファイルのバイナリデータを解析するためのユーティリティクラスです。
- **demo/src/parseMidiNotes.ts**: MIDIデータから個々の音符イベントを抽出し、時間軸に沿ったリストとして解析する機能を提供します。
- **demo/src/smfToYm2151.ts**: SMFデータをYM2151ログ形式に変換するWebAssemblyモジュールとのインターフェースを提供します。
- **demo/src/state.ts**: デモアプリケーションの現在のMML入力、SMF出力、再生状態などのグローバルな状態を管理します。
- **demo/src/treeToJSON.ts**: tree-sitterが生成した抽象構文木（AST）オブジェクトをJSON形式でシリアライズするユーティリティです。
- **demo/src/ui.ts**: デモのユーザーインターフェース要素の操作、状態表示、イベントハンドリングを管理します。
- **demo/src/visualization.ts**: 生成されたオーディオの波形やMIDIイベントの視覚化機能を提供します。
- **demo/src/wavExport.ts**: 変換・生成されたオーディオデータをWAVファイルとしてブラウザからダウンロード可能にする機能です。

## 関数詳細説明
- `audioPlayback.ts`:
    - `playAudio()`: (引数なし) -> `Promise<void>`: 生成されたオーディオの再生を開始します。
    - `stopAudio()`: (引数なし) -> `void`: 現在再生中のオーディオを停止します。
- `audioRenderer.ts`:
    - `waitForWebYm2151()`: (引数なし) -> `Promise<void>`: WebAssemblyのYM2151レンダラーが初期化されるのを待機します。
    - `calculateDuration()`: (引数なし) -> `number`: 現在のMIDIデータの再生時間を秒単位で計算します。
    - `renderWaveformAndAudio()`: (引数なし) -> `Promise<void>`: MIDIデータに基づいて波形をレンダリングし、再生可能なオーディオバッファを生成します。
- `main.ts`:
    - `initialize()`: (引数なし) -> `Promise<void>`: デモアプリケーションの主要なUI要素とロジックを初期化し、イベントリスナーを設定します。
- `midiReader.ts`:
    - `constructor(data: Uint8Array)`: (引数: MIDIバイナリデータ) -> `MidiReader`: MIDIバイナリデータを読み込み、解析するためのインスタンスを生成します。
- `mmlConverter.ts`:
    - `convertMML()`: (引数なし) -> `Promise<void>`: UIのMML入力フィールドからテキストを取得し、WebAssemblyモジュールを呼び出してSMFに変換します。
- `parseMidiNotes.ts`:
    - `parseMidiNotes(midiData: Uint8Array, ticksPerBeat: number, tempo: number)`: (引数: MIDIバイナリデータ, 1拍あたりのティック数, テンポ) -> `Array<ParsedNote>`: MIDIデータから個々の音符イベントを抽出し、開始時間や長さを計算して返します。
    - `deltaTicksToSeconds(deltaTicks: number, ticksPerBeat: number, tempo: number)`: (引数: デルタティック値, 1拍あたりのティック数, テンポ) -> `number`: MIDIのデルタティック値を実時間（秒）に変換します。
- `smfToYm2151.ts`:
    - `ensureInitialized()`: (引数なし) -> `Promise<void>`: YM2151変換WebAssemblyモジュールがロードされ、初期化されていることを保証します。
    - `smfToYM2151Json(smfBuffer: Uint8Array)`: (引数: SMFバイナリデータ) -> `string`: Standard MIDI FileのバイナリデータをYM2151ログ形式のJSON文字列に変換します。
- `treeToJSON.ts`:
    - `treeToJSON(node: Node)`: (引数: tree-sitterノード) -> `any`: tree-sitterの構文木ノードを再帰的にJSONオブジェクトに変換します。
- `ui.ts`:
    - `showStatus(message: string, isError: boolean)`: (引数: 表示するメッセージ, エラーかどうか) -> `void`: デモUIのステータス表示エリアにメッセージを表示します。
    - `loadExample(exampleName: string)`: (引数: 読み込むMML例の名前) -> `void`: 指定されたMMLのサンプルコードをエディタにロードします。
- `visualization.ts`:
    - `drawWaveform(audioBuffer: AudioBuffer, canvas: HTMLCanvasElement)`: (引数: オーディオバッファ, 描画対象Canvas) -> `void`: 提供されたオーディオバッファの波形をCanvasに描画します。
    - `visualizeRealtime(analyser: AnalyserNode, canvas: HTMLCanvasElement)`: (引数: AnalyserNode, 描画対象Canvas) -> `void`: オーディオのリアルタイム波形を視覚化します。
    - `draw(analyser: AnalyserNode, canvasCtx: CanvasRenderingContext2D, bufferLength: number, dataArray: Uint8Array)`: (引数: AnalyserNode, Canvas 2Dコンテキスト, バッファ長, データ配列) -> `void`: リアルタイム視覚化の描画ループの一部です。
- `wavExport.ts`:
    - `writeString(view: DataView, offset: number, s: string)`: (引数: DataView, オフセット, 文字列) -> `number`: DataViewに指定されたオフセットから文字列を書き込みます。
    - `audioBufferToWav(audioBuffer: AudioBuffer)`: (引数: AudioBuffer) -> `Blob`: AudioBufferオブジェクトをWAV形式のBlobデータに変換します。
    - `exportWav()`: (引数なし) -> `Promise<void>`: 生成されたオーディオをWAVファイルとしてダウンロードする処理を開始します。
- `audioBufferToWav.test.ts`:
    - `mockAudioBuffer(channels: number, length: number, sampleRate: number)`: (引数: チャンネル数, サンプル長, サンプルレート) -> `AudioBuffer`: テストのためにAudioBufferのモックオブジェクトを作成します。
- `parseMidiNotes.test.ts`:
    - `buildSmf(trackChunks: Array<Array<MidiEvent>>)`: (引数: MIDIイベントのトラックチャンク配列) -> `Uint8Array`: テストのためにStandard MIDI Fileのバイナリデータを構築します。
- `treeToJSON.test.ts`:
    - `mockNode(type: string, children: Array<any> = [])`: (引数: ノードタイプ, 子ノード配列) -> `object`: テストのためにtree-sitterのASTノードをモックします。

## 関数呼び出し階層ツリー
```
- initialize (demo/src/main.ts)
  - convertMML (demo/src/mmlConverter.ts)
    - smfToYM2151Json (demo/src/smfToYm2151.ts)
    - treeToJSON (demo/src/treeToJSON.ts)
- playAudio (demo/src/audioPlayback.ts)
  - stopAudio (demo/src/audioPlayback.ts)
    - showStatus (demo/src/ui.ts)
    - visualizeRealtime (demo/src/visualization.ts)
- renderWaveformAndAudio (demo/src/audioRenderer.ts)
  - calculateDuration (demo/src/audioRenderer.ts)
  - drawWaveform (demo/src/visualization.ts)
- parseMidiNotes (demo/src/parseMidiNotes.ts)
  - deltaTicksToSeconds (demo/src/parseMidiNotes.ts)
- exportWav (demo/src/wavExport.ts)
  - audioBufferToWav (demo/src/wavExport.ts)
    - writeString (demo/src/wavExport.ts)
- waitForWebYm2151 (demo/src/audioRenderer.ts)
- ensureInitialized (demo/src/smfToYm2151.ts)
- mockAudioBuffer (demo/tests/audioBufferToWav.test.ts)
- buildSmf (demo/tests/parseMidiNotes.test.ts)
- mockNode (demo/tests/treeToJSON.test.ts)

---
Generated at: 2026-03-13 07:07:23 JST
