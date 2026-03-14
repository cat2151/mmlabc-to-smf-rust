Last updated: 2026-03-15

# Project Overview

## プロジェクト概要
- MML形式の楽譜データをStandard MIDI File（SMF）形式へ変換するRust製ライブラリです。
- 独自の4パスアーキテクチャとtree-sitterパーサーを統合し、高精度なMML構文解析を実現します。
- ネイティブアプリケーションやWebAssemblyベースのブラウザアプリでの音楽生成をサポートします。

## 技術スタック
- フロントエンド: **HTML** (ウェブデモの構造定義), **TypeScript** (ウェブデモのスクリプト言語), **Tone.js** (ウェブデモでのオーディオ再生用ライブラリ)
- 音楽・オーディオ: **Music Macro Language (MML)** (入力楽譜形式), **Standard MIDI File (SMF)** (出力音楽ファイル形式), **YM2151** (ウェブデモで利用される音源ログ形式)
- 開発ツール: **Cargo** (Rustプロジェクトのビルド・テスト・依存関係管理), **npm/npx** (Node.jsパッケージ管理およびtree-sitterパーサー生成に使用)
- テスト: **cargo test** (Rustコードのユニット・統合テスト), **node:test** (ウェブデモTypeScriptコードのテストフレームワーク)
- ビルドツール: **Cargo** (Rustコードのビルド), **npx tree-sitter generate** (tree-sitterパーサーのC言語ファイル生成), **WebAssembly (WASM)** (ブラウザ向けライブラリのビルドターゲット)
- 言語機能: **Rust 1.70.0以上** (メイン開発言語), **WebAssembly (WASM)** (ブラウザ環境での利用を可能にするバイナリフォーマット)
- 自動化・CI/CD: **Cargo Clippy** (Rustコードの品質チェック), **Cargo fmt** (Rustコードの自動フォーマット)
- 開発標準: **.editorconfig** (IDE/エディタ間のコーディングスタイル統一)

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

-   `.editorconfig`: 異なるIDEやエディタ間でコードのスタイルを統一するための設定ファイル。
-   `.gitignore`: Gitのバージョン管理から除外するファイルやディレクトリを指定するファイル。
-   `.vscode/settings.json`: Visual Studio Codeのワークスペース設定ファイル。
-   `Cargo.lock`: Cargoが依存関係を解決した結果を記録し、再現可能なビルドを保証するファイル。
-   `Cargo.toml`: Rustプロジェクトのマニフェストファイル。プロジェクトのメタデータと依存関係を定義。
-   `IMPLEMENTATION_REPORT.md`: 実装に関する報告やメモを記述したMarkdownファイル。
-   `LICENSE`: プロジェクトのライセンス情報（MIT License）。
-   `OPTION_A_IMPLEMENTATION.md`: 特定の実装オプションAに関する詳細を記述したMarkdownファイル。
-   `README.ja.md`: プロジェクトの日本語版説明書。
-   `README.md`: プロジェクトの英語版説明書。
-   `_config.yml`: Jekyllなどの静的サイトジェネレーターの設定ファイル（存在する場合）。
-   `build.rs`: Rustプロジェクトのビルドスクリプト。tree-sitterパーサーの生成などを自動化。
-   `demo/`: ウェブブラウザで動作するデモンストレーションアプリケーション関連ファイル群。
    -   `demo/.gitignore`: demoディレクトリ用のGitignore設定。
    -   `demo/FEATURES.md`: デモアプリケーションの機能一覧。
    -   `demo/README.md`: デモアプリケーションのREADMEファイル。
    -   `demo/index.html`: デモのユーザーインターフェースを定義するHTMLファイル。
    -   `demo/package.json`: デモのNode.jsプロジェクト設定ファイル。
    -   `demo/src/audioPlayback.ts`: 生成されたオーディオの再生ロジック（TypeScript）。
    -   `demo/src/audioRenderer.ts`: 音源の波形表示とオーディオレンダリングを制御するロジック（TypeScript）。
    -   `demo/src/main.ts`: デモアプリケーションの主要な初期化およびイベントハンドリングロジック（TypeScript）。
    -   `demo/src/midiReader.ts`: MIDIファイルのバイナリデータを解析するクラス（TypeScript）。
    -   `demo/src/mmlConverter.ts`: MML文字列をWebAssembly経由でSMFに変換するデモ側のラッパー（TypeScript）。
    -   `demo/src/parseMidiNotes.ts`: MIDIデータから音楽イベント（音符、テンポなど）を抽出するロジック（TypeScript）。
    -   `demo/src/smfToYm2151.ts`: SMFデータをYM2151形式に変換するWASMモジュールとの連携ロジック（TypeScript）。
    -   `demo/src/state.ts`: デモアプリケーションのグローバルな状態を管理するモジュール（TypeScript）。
    -   `demo/src/treeToJSON.ts`: tree-sitterのASTをJSON形式に変換するユーティリティ（TypeScript）。
    -   `demo/src/ui.ts`: デモのユーザーインターフェース要素（ステータス表示、例のロードなど）を操作するロジック（TypeScript）。
    -   `demo/src/visualization.ts`: 音符イベントや波形を視覚的に表示するロジック（TypeScript）。
    -   `demo/src/wavExport.ts`: 生成されたオーディオデータをWAVファイルとしてエクスポートする機能（TypeScript）。
    -   `demo/test-loader.mjs`: Node.jsのテストローダー設定ファイル。
    -   `demo/test-register.mjs`: Node.jsのテスト登録設定ファイル。
    -   `demo/tests/audioBufferToWav.test.ts`: `wavExport.ts`の`audioBufferToWav`関数のテスト（TypeScript）。
    -   `demo/tests/midiReader.test.ts`: `midiReader.ts`のMIDIパース機能のテスト（TypeScript）。
    -   `demo/tests/parseMidiNotes.test.ts`: `parseMidiNotes.ts`のMIDI音符解析機能のテスト（TypeScript）。
    -   `demo/tests/treeToJSON.test.ts`: `treeToJSON.ts`のAST変換機能のテスト（TypeScript）。
-   `demo-library/`: ライブラリとしてデモを組み込む際のサンプルまたはビルド成果物。
    -   `demo-library/index.html`: デモライブラリのサンプルHTML。
    -   `demo-library/package.json`: デモライブラリのNode.jsプロジェクト設定ファイル。
-   `generated-docs/development-status-generated-prompt.md`: 生成された開発状況ドキュメント。
-   `googled947dc864c270e07.html`: Googleサイト所有権確認ファイル。
-   `issue-notes/`: 開発中の課題や検討事項に関するメモ。
    -   `issue-notes/103.md`, `121.md`, `123.md`, `39.md`, `44.md`: 特定の課題に関連するメモファイル。
-   `mmlabc-to-smf-rust.toml.example`: カスタムMIDIプレイヤー設定ファイルの例。
-   `mmlabc-to-smf-wasm/`: WebAssembly (WASM) ターゲット向けにRustライブラリをビルドするためのクレート。
    -   `mmlabc-to-smf-wasm/Cargo.lock`: WASMクレートの依存関係ロックファイル。
    -   `mmlabc-to-smf-wasm/Cargo.toml`: WASMクレートのマニフェストファイル。
    -   `mmlabc-to-smf-wasm/src/lib.rs`: WASMクレートのライブラリルート。JavaScriptからのFFI定義を含む。
    -   `mmlabc-to-smf-wasm/src/token_extractor.rs`: WASM向けにMMLトークンを抽出するロジック。
-   `package.json`: プロジェクト全体のNode.jsパッケージ設定ファイル。
-   `scripts/`: ビルドやデモ関連のスクリプト。
    -   `scripts/README.md`: スクリプトに関する説明。
    -   `scripts/build-demo.sh`: デモアプリケーションをビルドするためのシェルスクリプト。
    -   `scripts/transform-demo-paths.sh`: デモのファイルパスを変換するためのシェルスクリプト。
-   `src/`: Rustライブラリの主要なソースコード。
    -   `src/attachment_json.rs`: デバッグ用のJSON出力機能を実装。
    -   `src/config.rs`: 実行時設定（例: 外部MIDIプレイヤー）を処理。
    -   `src/lib.rs`: Rustライブラリのメインエントリーポイント。MMLからSMFへの変換APIを公開。
    -   `src/main.rs`: コマンドラインインターフェース（CLI）のエントリーポイント。
    -   `src/mml_preprocessor.rs`: MML文字列の整形や基本的な変換前処理を行う。
    -   `src/pass1_parser.rs`: MML文字列をトークンに分割する最初の解析パス（tree-sitterを使用）。
    -   `src/pass2_ast.rs`: トークンから抽象構文木（AST）を構築する第二の解析パス。
    -   `src/pass3_events.rs`: ASTをMIDIイベントのシーケンスに変換する第三の解析パス。
    -   `src/pass4_midi.rs`: MIDIイベントから最終的なStandard MIDI Fileを生成する第四の解析パス。
    -   `src/tree_sitter_mml.rs`: tree-sitter MMLパーサーをRustコードに統合するためのモジュール。
    -   `src/types.rs`: プロジェクト全体で利用されるカスタムデータ構造や列挙型などの定義。
-   `tests/`: Rustライブラリのテストコード。
    -   `tests/integration_test.rs`: ライブラリ全体の統合テスト。
    -   `tests/test_attachment_json.rs`: JSONアタッチメント機能のテスト。
    -   `tests/test_c1_vs_c64.rs`: 特定のMML記法（c1 vs c64）に関するテスト。
    -   `tests/test_channel.rs`: 多チャンネルMMLの変換テスト。
    -   `tests/test_chord.rs`: 和音機能のテスト。
    -   `tests/test_cli.rs`: コマンドラインインターフェースのテスト。
    -   `tests/test_config.rs`: 設定ファイル読み込みのテスト。
    -   `tests/test_dotted_notes.rs`: 付点音符のテスト。
    -   `tests/test_drum_channel.rs`: ドラムチャンネルのテスト。
    -   `tests/test_key_transpose.rs`: キー（調）の移調機能のテスト。
    -   `tests/test_length.rs`: 音長指定のテスト。
    -   `tests/test_modifier.rs`: 音符修飾子（例: シャープ、フラット）のテスト。
    -   `tests/test_note_length.rs`: 音符の長さに関するテスト。
    -   `tests/test_octave.rs`: オクターブ指定のテスト。
    -   `tests/test_pass1.rs`: パス1（トークン解析）のテスト。
    -   `tests/test_pass2.rs`: パス2（AST変換）のテスト。
    -   `tests/test_pass3.rs`: パス3（MIDIイベント生成）のテスト。
    -   `tests/test_pass4.rs`: パス4（SMFファイル作成）のテスト。
    -   `tests/test_program_change.rs`: プログラムチェンジ（音色変更）のテスト。
    -   `tests/test_rest.rs`: 休符のテスト。
    -   `tests/test_tempo.rs`: テンポ変更のテスト。
    -   `tests/test_velocity.rs`: ベロシティ（音量）変更のテスト。
-   `tree-sitter-mml/`: MML用のtree-sitterパーサー定義。
    -   `tree-sitter-mml/grammar.js`: MMLの文法を定義するJavaScriptファイル。
    -   `tree-sitter-mml/package.json`: tree-sitterパーサーのNode.jsパッケージ設定ファイル。
    -   `tree-sitter-mml/src/grammar.json`: `grammar.js`から生成されるJSON形式の文法定義。
    -   `tree-sitter-mml/src/node-types.json`: `grammar.js`から生成されるノードタイプ定義。
    -   `tree-sitter-mml/src/parser.c`: `grammar.js`から生成されるC言語のパーサーソースコード。
    -   `tree-sitter-mml/src/tree_sitter/`: tree-sitterパーサーに必要なC言語ヘッダーファイル。
        -   `tree-sitter-mml/src/tree_sitter/alloc.h`: メモリ割り当て関連ヘッダー。
        -   `tree-sitter-mml/src/tree_sitter/array.h`: 配列関連ヘッダー。
        -   `tree-sitter-mml/src/tree_sitter/parser.h`: パーサーコア関連ヘッダー。
    -   `tree-sitter-mml/tree-sitter-mml.wasm`: MMLパーサーのWebAssemblyバイナリ。

## 関数詳細説明

-   **`playAudio`** (demo/src/audioPlayback.ts)
    -   役割: デモアプリケーションで生成されたMIDIデータ（または変換されたYM2151ログ）をWeb Audio API (`Tone.js`) を使用して再生します。
    -   引数: なし。
    -   戻り値: Promise<void>。再生処理が完了またはエラーが発生したときに解決/拒否されるPromise。
-   **`stopAudio`** (demo/src/audioPlayback.ts)
    -   役割: 現在再生中のオーディオを停止し、関連するオーディオリソースを解放します。
    -   引数: なし。
    -   戻り値: void。
-   **`waitForWebYm2151`** (demo/src/audioRenderer.ts)
    -   役割: 外部の`web-ym2151` WebAssemblyモジュールが完全にロードされ、初期化されるのを待ちます。オーディオレンダリングの前提条件。
    -   引数: なし。
    -   戻り値: Promise<void>。モジュールが利用可能になったときに解決されるPromise。
-   **`calculateDuration`** (demo/src/audioRenderer.ts)
    -   役割: MIDIファイルデータから、曲の総再生時間を計算します。
    -   引数: `smfBuffer: Uint8Array` (Standard MIDI Fileのバイナリデータ)。
    -   戻り値: `number`。計算された再生時間（秒）。
-   **`renderWaveformAndAudio`** (demo/src/audioRenderer.ts)
    -   役割: MIDIデータまたはYM2151ログを基にオーディオ波形をレンダリングし、その波形をCanvasに描画するとともに、再生可能なAudioBufferを生成します。
    -   引数: `smfBuffer: Uint8Array` (Standard MIDI Fileのバイナリデータ)。
    -   戻り値: Promise<void>。レンダリングと描画処理が完了したときに解決されるPromise。
-   **`initialize`** (demo/src/main.ts)
    -   役割: デモアプリケーションの起動時に、WebAssemblyモジュールのロード、tree-sitterパーサーの初期化、UI要素のイベントリスナー設定など、全ての初期設定を行います。
    -   引数: なし。
    -   戻り値: Promise<void>。初期化処理が完了したときに解決されるPromise。
-   **`constructor`** (demo/src/midiReader.ts)
    -   役割: MIDIファイルのバイナリデータを入力として受け取り、MIDIメッセージやトラック情報を解析するための`MidiReader`インスタンスを初期化します。
    -   引数: `midiBytes: Uint8Array` (MIDIファイルの生バイナリデータ)。
    -   戻り値: `MidiReader`オブジェクト。
-   **`convertMML`** (demo/src/mmlConverter.ts)
    -   役割: ユーザーが入力したMML文字列を、`mmlabc-to-smf-wasm` WebAssemblyモジュールを利用してStandard MIDI File形式のバイナリデータに変換します。
    -   引数: `mml: string` (Music Macro Language形式の文字列)。
    -   戻り値: `Uint8Array`。変換されたStandard MIDI Fileのバイナリデータ。
-   **`parseMidiNotes`** (demo/src/parseMidiNotes.ts)
    -   役割: Standard MIDI Fileのバイナリデータから、個々の音符イベント（ノートオン/オフ）とそれらのタイミング情報を抽出し、JavaScriptで扱いやすい構造に変換します。
    -   引数: `smfBuffer: Uint8Array` (Standard MIDI Fileのバイナリデータ)。
    -   戻り値: `Array<{time: number, duration: number, pitch: number, velocity: number}>`。解析された音符イベントの配列。
-   **`deltaTicksToSeconds`** (demo/src/parseMidiNotes.ts)
    -   役割: MIDIイベント間のデルタタイム（ティック単位）を実際の秒単位の時間に変換します。テンポやタイムベース（PPQN）を考慮します。
    -   引数: `delta: number` (デルタティック数), `bpm: number` (現在のテンポ、BPM), `ppqn: number` (四分音符あたりのパルス数、PPQN)。
    -   戻り値: `number`。変換された秒数。
-   **`ensureInitialized`** (demo/src/smfToYm2151.ts)
    -   役割: `smf-to-ym2151log-wasm` モジュールが初期化されていることを確認し、必要であれば初期化を実行します。
    -   引数: なし。
    -   戻り値: Promise<void>。モジュールの初期化が完了したときに解決されるPromise。
-   **`smfToYM2151Json`** (demo/src/smfToYm2151.ts)
    -   役割: Standard MIDI Fileを、YM2151シンセサイザーのログ形式であるJSON文字列に変換します。
    -   引数: `smfBuffer: Uint8Array` (Standard MIDI Fileのバイナリデータ)。
    -   戻り値: `string`。YM2151ログのJSON文字列。
-   **`treeToJSON`** (demo/src/treeToJSON.ts)
    -   役割: `tree-sitter`によって生成された抽象構文木（AST）を再帰的に走査し、デバッグ表示に適したJSONオブジェクト構造に変換します。
    -   引数: `node: any` (tree-sitterのNodeオブジェクト)。
    -   戻り値: `object`。ASTのJSON表現。
-   **`showStatus`** (demo/src/ui.ts)
    -   役割: デモのユーザーインターフェース上のステータス表示領域にメッセージを表示します。処理の進捗やエラーをユーザーに伝えます。
    -   引数: `message: string` (表示するメッセージ)。
    -   戻り値: void。
-   **`loadExample`** (demo/src/ui.ts)
    -   役割: プリセットのMMLサンプルコードをMML入力エディタにロードします。
    -   引数: `exampleText: string` (ロードするMMLサンプルコード)。
    -   戻り値: void。
-   **`drawWaveform`** (demo/src/visualization.ts)
    -   役割: `AudioBuffer`から取得したオーディオデータに基づいて、Canvas上に音の波形を描画します。
    -   引数: `buffer: AudioBuffer` (描画するオーディオデータ), `canvasCtx: CanvasRenderingContext2D` (描画対象のCanvasコンテキスト), `width: number` (描画領域の幅), `height: number` (描画領域の高さ)。
    -   戻り値: void。
-   **`visualizeRealtime`** (demo/src/visualization.ts)
    -   役割: 現在再生中のオーディオデータをリアルタイムで解析し、その視覚化を更新します。
    -   引数: `analyser: AnalyserNode` (Web Audio APIのAnalyserNode)。
    -   戻り値: void。
-   **`draw`** (demo/src/visualization.ts)
    -   役割: 視覚化コンポーネントのメイン描画ループ。フレームごとに波形やその他の視覚要素を更新します。
    -   引数: なし（内部状態を参照）。
    -   戻り値: void。
-   **`writeString`** (demo/src/wavExport.ts)
    -   役割: WAVファイルヘッダーの特定のオフセットにASCII文字列を書き込みます。
    -   引数: `view: DataView`, `offset: number`, `s: string` (書き込む文字列)。
    -   戻り値: void。
-   **`audioBufferToWav`** (demo/src/wavExport.ts)
    -   役割: Web Audio APIの`AudioBuffer`オブジェクトを標準的なWAVファイル形式のバイナリデータに変換します。
    -   引数: `buffer: AudioBuffer` (変換するAudioBuffer)。
    -   戻り値: `Blob`。WAV形式のバイナリデータを格納したBlobオブジェクト。
-   **`exportWav`** (demo/src/wavExport.ts)
    -   役割: 現在のオーディオバッファをWAVファイルとして生成し、ブラウザのダウンロード機能を通じてユーザーに提供します。
    -   引数: `audioBuffer: AudioBuffer` (エクスポートするAudioBuffer)。
    -   戻り値: void。
-   **`mockAudioBuffer`** (demo/tests/audioBufferToWav.test.ts)
    -   役割: `audioBufferToWav`関数のテスト用に、擬似的な`AudioBuffer`オブジェクトを作成します。
    -   引数: `numberOfChannels: number`, `length: number`, `sampleRate: number`。
    -   戻り値: `AudioBuffer`に似たモックオブジェクト。
-   **`buildSmf`** (demo/tests/parseMidiNotes.test.ts)
    -   役割: `parseMidiNotes`関数のテスト用に、簡易なStandard MIDI Fileのバイナリデータ（Uint8Array）を構築します。
    -   引数: `trackEvents: Array<Array<{deltaTime: number, event: Array<number>}>>` (MIDIイベントの配列)。
    -   戻り値: `Uint8Array`。構築されたSMFバイナリデータ。
-   **`mockNode`** (demo/tests/treeToJSON.test.ts)
    -   役割: `treeToJSON`関数のテスト用に、`tree-sitter`のASTノードを模倣したオブジェクトを生成します。
    -   引数: `type: string` (ノードのタイプ), `children: Array<any>` (子ノードの配列)。
    -   戻り値: ASTノードに似たモックオブジェクト。

## 関数呼び出し階層ツリー
```
- if (demo/src/audioPlayback.ts)
  - playAudio (demo/src/audioPlayback.ts)
    - stopAudio (demo/src/audioPlayback.ts)
      - showStatus (demo/src/ui.ts)
    - visualizeRealtime (demo/src/visualization.ts)
  - waitForWebYm2151 (demo/src/audioRenderer.ts)
    - calculateDuration (demo/src/audioRenderer.ts)
    - renderWaveformAndAudio (demo/src/audioRenderer.ts)
      - drawWaveform (demo/src/visualization.ts)
  - initialize (demo/src/main.ts)
    - convertMML (demo/src/mmlConverter.ts)
      - smfToYM2151Json (demo/src/smfToYm2151.ts)
      - treeToJSON (demo/src/treeToJSON.ts)
  - constructor (demo/src/midiReader.ts)
  - parseMidiNotes (demo/src/parseMidiNotes.ts)
    - deltaTicksToSeconds (demo/src/parseMidiNotes.ts)
  - catch (demo/src/audioPlayback.ts)
    - writeString (demo/src/wavExport.ts)
    - audioBufferToWav (demo/src/wavExport.ts)
    - exportWav (demo/src/wavExport.ts)
  - ensureInitialized (demo/src/smfToYm2151.ts)
  - draw (demo/src/visualization.ts)
- for (demo/src/audioRenderer.ts)
  - mockAudioBuffer (demo/tests/audioBufferToWav.test.ts)
  - buildSmf (demo/tests/parseMidiNotes.test.ts)
- while (demo/src/parseMidiNotes.ts)
- mockNode (demo/tests/treeToJSON.test.ts)

---
Generated at: 2026-03-15 07:08:12 JST
