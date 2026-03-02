Last updated: 2026-03-03

# Project Overview

## プロジェクト概要
- 本プロジェクトは、Music Macro Language (MML) 形式の文字列をStandard MIDI File (SMF) へ変換するRust製ライブラリです。
- MML構文解析にtree-sitterを活用した4パスアーキテクチャを採用し、堅牢かつ拡張性の高い設計を実現しています。
- ネイティブアプリケーション向けRustクレートとして利用されるほか、WebAssemblyを通じてブラウザ上でのMML音楽生成デモも提供します。

## 技術スタック
- フロントエンド: HTML (デモUI)、TypeScript (デモロジック)、WebAssembly (WASM) (Rustコードのブラウザ実行)
- 音楽・オーディオ: Standard MIDI File (SMF) (出力フォーマット)、Music Macro Language (MML) (入力フォーマット)、Tone.js (デモでのオーディオ再生)、web-ym2151 (デモでのYM2151音源エミュレーション)、TiMidity++ / FluidSynth / VLC / cat-play-mml (外部MIDIプレイヤー)
- 開発ツール: Rust (主要開発言語)、Cargo (Rustパッケージマネージャー・ビルドシステム)、Node.js / npx (tree-sitterパーサー生成用)
- テスト: Rust標準テストフレームワーク (ユニット・統合テスト、合計35ケース)
- ビルドツール: Cargo (Rustプロジェクトビルド)、tree-sitter-cli (MMLパーサー生成)
- 言語機能: Rust (メモリ安全性、並行処理)、WebAssembly (ブラウザ環境での高性能なコード実行)
- 自動化・CI/CD: (未定だが短期目標としてフォーマッター・リンター設定の整備が挙げられている)
- 開発標準: cargo clippy (Lintツール)、cargo fmt (フォーマッター)、.editorconfig (コードスタイル定義)

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
│   └── src/
│       ├── audioPlayback.ts
│       ├── audioRenderer.ts
│       ├── main.ts
│       ├── midiReader.ts
│       ├── mmlConverter.ts
│       ├── parseMidiNotes.ts
│       ├── smfToYm2151.ts
│       ├── state.ts
│       ├── ui.ts
│       ├── visualization.ts
│       └── wavExport.ts
├── demo-library/
│   ├── index.html
│   └── package.json
├── generated-docs/
│   └── development-status-generated-prompt.md
├── googled947dc864c270e07.html
├── issue-notes/
│   ├── 103.md
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

*   `.editorconfig`: 異なるエディタやIDE間で一貫したコーディングスタイルを維持するための設定ファイル。
*   `.gitignore`: Gitがバージョン管理の対象外とするファイルやディレクトリを指定するファイル。
*   `.vscode/settings.json`: VS Codeエディタのワークスペース固有の設定を定義するファイル。
*   `Cargo.lock`: Cargoが依存関係を解決した結果を記録し、ビルドの再現性を保証するファイル。
*   `Cargo.toml`: Rustプロジェクトのメタデータ、依存関係、ビルド設定などを定義するファイル。
*   `IMPLEMENTATION_REPORT.md`: 実装に関する詳細な報告書。
*   `LICENSE`: プロジェクトのライセンス情報（MIT License）。
*   `OPTION_A_IMPLEMENTATION.md`: 開発中の実装オプションAに関するドキュメント。
*   `README.ja.md`: プロジェクトの概要、機能、使い方などを日本語で説明するファイル。
*   `README.md`: プロジェクトの概要、機能、使い方などを英語で説明するファイル。
*   `_config.yml`: Jekyllなどの静的サイトジェネレータの設定ファイル（もしあれば）。
*   `build.rs`: Rustプロジェクトのビルドスクリプト。tree-sitterパーサーのC言語ソースコードの再生成などを担当。
*   `demo/`: WebブラウザでMML変換デモを実行するための関連ファイル群。
    *   `demo/.gitignore`: デモディレクトリのGit無視設定。
    *   `demo/FEATURES.md`: デモの機能に関するドキュメント。
    *   `demo/README.md`: デモに関する説明。
    *   `demo/index.html`: ブラウザデモのメインページ。MML入力、MIDI出力、波形可視化、オーディオ再生などのUIを構築。
    *   `demo/package.json`: デモのフロントエンド（TypeScriptなど）の依存関係やスクリプトを定義。
    *   `demo/src/audioPlayback.ts`: デモのオーディオ再生ロジックを管理。Tone.jsを利用してMIDIデータを音声として再生。
    *   `demo/src/audioRenderer.ts`: 音源合成や波形レンダリングを制御。SMFデータをYM2151ログに変換し、それを音声化。
    *   `demo/src/main.ts`: ブラウザデモのメインスクリプト。WASMモジュールの初期化、イベントリスナーの設定、UIとMML変換ロジックの連携。
    *   `demo/src/midiReader.ts`: MIDIファイル（SMF）データを読み込み、解析するユーティリティ。
    *   `demo/src/mmlConverter.ts`: ブラウザデモにおけるMMLからSMFへの変換処理を管理。WASMモジュールを呼び出してMML文字列を処理。
    *   `demo/src/parseMidiNotes.ts`: MIDIデータから音符イベントを抽出し、時間情報と共に解析。波形可視化やリアルタイム演奏に利用。
    *   `demo/src/smfToYm2151.ts`: SMFデータをYM2151形式のログデータに変換するモジュール。WebAssembly版の`smf-to-ym2151log-rust`を利用。
    *   `demo/src/state.ts`: ブラウザデモアプリケーションのグローバルな状態を管理。MML文字列、生成されたMIDIデータ、UIの状態などを保持。
    *   `demo/src/ui.ts`: ブラウザデモのユーザーインターフェース操作に関連する関数を提供。ステータス表示、サンプルMMLの読み込みなどを担当。
    *   `demo/src/visualization.ts`: 生成されたMIDIデータやオーディオ波形を視覚的に表示する機能を提供。
    *   `demo/src/wavExport.ts`: 生成されたオーディオデータをWAVファイルとしてエクスポートする機能。
*   `demo-library/`: デモのライブラリ版に関連するファイル群。
    *   `demo-library/index.html`: デモライブラリのインデックスHTML。
    *   `demo-library/package.json`: デモライブラリの依存関係とスクリプト定義。
*   `generated-docs/development-status-generated-prompt.md`: 生成された開発ステータスに関するドキュメント。
*   `googled947dc864c270e07.html`: Googleサイト認証ファイル。
*   `issue-notes/`: 課題やイシューに関するメモをまとめたディレクトリ。
    *   `issue-notes/103.md`, `issue-notes/39.md`, `issue-notes/44.md`: 特定のイシューに関する詳細メモ。
*   `mmlabc-to-smf-rust.toml.example`: 外部MIDIプレイヤーの設定例を示すTOML形式の設定ファイル。
*   `mmlabc-to-smf-wasm/`: WebAssembly (WASM) バイナリを生成するためのRustクレート。
    *   `mmlabc-to-smf-wasm/Cargo.lock`: WASMクレートの依存関係ロックファイル。
    *   `mmlabc-to-smf-wasm/Cargo.toml`: WASMクレートのメタデータと依存関係。
    *   `mmlabc-to-smf-wasm/src/lib.rs`: `mmlabc-to-smf-rust`ライブラリの機能をWebAssembly向けに公開するエントリポイント。
    *   `mmlabc-to-smf-wasm/src/token_extractor.rs`: WASMモジュール内でMMLトークン抽出に関連する機能を提供。
*   `package.json`: プロジェクト全体のJavaScript/Node.js依存関係を管理するファイル。主に`tree-sitter-cli`用。
*   `scripts/`: ビルドやデモ関連のヘルパースクリプトを格納。
    *   `scripts/README.md`: スクリプトに関する説明。
    *   `scripts/build-demo.sh`: デモのビルドプロセスを実行するシェルスクリプト。
    *   `scripts/transform-demo-paths.sh`: デモのパスを変換するシェルスクリプト。
*   `src/`: `mmlabc-to-smf-rust` Rustライブラリの主要ソースコード。
    *   `src/config.rs`: 実行時設定やカスタマイズオプション（外部MIDIプレイヤーのパスなど）を処理するモジュール。
    *   `src/lib.rs`: `mmlabc-to-smf-rust`クレートのライブラリとしての公開インターフェース。主要な変換ロジックをカプセル化。
    *   `src/main.rs`: CLIアプリケーションのエントリーポイント。MML文字列の受け取り、処理のオーケストレーション、結果の出力、外部MIDIプレイヤーでの再生制御。
    *   `src/pass1_parser.rs`: MML文字列をトークンに分解する役割。tree-sitterパーサーを利用して構文解析を実行し、初期のトークン列を生成。
    *   `src/pass2_ast.rs`: パス1で生成されたトークン列から抽象構文木（AST）を構築。MMLの論理的な構造を表現するデータ構造を作成。
    *   `src/pass3_events.rs`: ASTをトラバースし、MIDIイベント（NoteOn, NoteOff, TempoChangeなど）のシーケンスを生成。MMLの各要素をMIDIプロトコルにマッピング。
    *   `src/pass4_midi.rs`: パス3で生成されたMIDIイベントから最終的なStandard MIDI File（SMF）データを構築し、ファイルとして出力。SMFフォーマットへの変換を担う。
    *   `src/tree_sitter_mml.rs`: Rustコードとtree-sitter MMLパーサーとの統合層。MML構文解析のための外部CライブラリとのFFI（Foreign Function Interface）を提供。
    *   `src/types.rs`: プロジェクト全体で共有されるデータ構造や列挙型などの型定義を集約。MML要素、ASTノード、MIDIイベントなどの内部表現を定義。
*   `tests/`: ユニットテストおよび統合テストファイル群。
    *   `tests/integration_test.rs`: ライブラリ全体の統合テスト。
    *   `tests/test_channel.rs`: MMLのチャンネル機能に関するテスト。
    *   `tests/test_chord.rs`: MMLの和音機能に関するテスト。
    *   `tests/test_cli.rs`: コマンドラインインターフェースの動作に関するテスト。
    *   `tests/test_config.rs`: 設定ファイルの読み込みと適用に関するテスト。
    *   `tests/test_dotted_notes.rs`: 付点音符の処理に関するテスト。
    *   `tests/test_drum_channel.rs`: ドラムチャンネルのMML処理に関するテスト。
    *   `tests/test_key_transpose.rs`: キー移調機能に関するテスト。
    *   `tests/test_length.rs`: 音長指定のMML処理に関するテスト。
    *   `tests/test_modifier.rs`: MMLの修飾子に関するテスト。
    *   `tests/test_note_length.rs`: 音符の長さの正確な処理に関するテスト。
    *   `tests/test_octave.rs`: オクターブ指定のMML処理に関するテスト。
    *   `tests/test_pass1.rs`: 変換パイプラインの第1パス（トークン解析）のテスト。
    *   `tests/test_pass2.rs`: 変換パイプラインの第2パス（AST変換）のテスト。
    *   `tests/test_pass3.rs`: 変換パイプラインの第3パス（MIDIイベント生成）のテスト。
    *   `tests/test_pass4.rs`: 変換パイプラインの第4パス（MIDIファイル作成）のテスト。
    *   `tests/test_program_change.rs`: プログラムチェンジ（音色変更）のMML処理に関するテスト。
    *   `tests/test_rest.rs`: 休符のMML処理に関するテスト。
    *   `tests/test_tempo.rs`: テンポ変更のMML処理に関するテスト。
    *   `tests/test_velocity.rs`: ベロシティ（音量）のMML処理に関するテスト。
*   `tree-sitter-mml/`: MML用のtree-sitterパーサー定義とその生成物。
    *   `tree-sitter-mml/grammar.js`: MMLの構文ルールを定義するJavaScriptファイル。tree-sitterパーサーのC言語ソースコード生成に使用。
    *   `tree-sitter-mml/package.json`: tree-sitter MMLパーサーの依存関係（tree-sitter-cliなど）を定義。
    *   `tree-sitter-mml/src/`: 生成されたtree-sitterパーサーのソースファイル。
        *   `tree-sitter-mml/src/grammar.json`: `grammar.js`から生成されたJSON形式の文法定義。
        *   `tree-sitter-mml/src/node-types.json`: ASTノードの型定義。
        *   `tree-sitter-mml/src/parser.c`: `grammar.js`から生成されたC言語のパーサー実装。
        *   `tree-sitter-mml/src/tree_sitter/`: tree-sitterパーサーを構成するC言語ヘッダファイル群。
    *   `tree-sitter-mml/tree-sitter-mml.wasm`: MMLパーサーのWebAssemblyバイナリ。ブラウザデモで使用。

## 関数詳細説明
*   `playAudio` (demo/src/audioPlayback.ts)
    *   役割: 生成されたMIDIデータをブラウザ上で再生します。
    *   引数: なし
    *   戻り値: Promise<void>
    *   機能: Tone.jsを使用してWeb Audio API経由でMIDIイベントを音楽として再生します。
*   `stopAudio` (demo/src/audioPlayback.ts)
    *   役割: 現在再生中のオーディオを停止します。
    *   引数: なし
    *   戻り値: void
    *   機能: Tone.jsの再生を停止し、関連するオーディオリソースを解放します。
*   `waitForWebYm2151` (demo/src/audioRenderer.ts)
    *   役割: `web-ym2151` WebAssemblyモジュールの初期化が完了するのを待機します。
    *   引数: なし
    *   戻り値: Promise<void>
    *   機能: WebAssemblyモジュールのロード状態を確認し、準備が整うまで待機します。
*   `calculateDuration` (demo/src/audioRenderer.ts)
    *   役割: MIDIイベントデータから曲の総再生時間を計算します。
    *   引数: MIDIイベントの配列
    *   戻り値: number (秒単位の再生時間)
    *   機能: MIDIイベントのタイムスタンプを解析し、最も遅いイベントの時間を基準に曲の長さを算出します。
*   `renderWaveformAndAudio` (demo/src/audioRenderer.ts)
    *   役割: MIDIデータから波形をレンダリングし、オーディオバッファを生成します。
    *   引数: MIDIイベントのデータ
    *   戻り値: Promise<AudioBuffer>
    *   機能: `smfToYM2151Json`を呼び出してYM2151ログを生成し、そのログから波形データをレンダリングします。
*   `check` (demo/src/audioRenderer.ts)
    *   役割: レンダリング処理の特定の状態や条件を確認します。
    *   引数: なし (文脈依存)
    *   戻り値: boolean または特定の状態情報
    *   機能: レンダリングが適切に実行できるかを判断するための内部チェックを行います。
*   `initialize` (demo/src/main.ts)
    *   役割: ブラウザデモアプリケーションの初期設定とイベントハンドラの登録を行います。
    *   引数: なし
    *   戻り値: Promise<void>
    *   機能: WebAssemblyモジュールのロード、UI要素へのイベントリスナーの追加、初期MMLの変換と表示を行います。
*   `treeToJSON` (demo/src/mmlConverter.ts)
    *   役割: tree-sitterによって生成された抽象構文木 (AST) をJSON形式に変換します。
    *   引数: tree-sitterのASTオブジェクト
    *   戻り値: string (JSON形式のAST表現)
    *   機能: ASTの構造をデバッグや可視化のために人間が読みやすいJSON文字列にシリアライズします。
*   `convertMML` (demo/src/mmlConverter.ts)
    *   役割: MML文字列をStandard MIDI File (SMF) データに変換します。
    *   引数: mmlString (string)
    *   戻り値: Promise<Uint8Array> (SMFデータ)
    *   機能: Rustで書かれたWASMモジュール (`mmlabc_to_smf_wasm`) を呼び出し、MMLを解析してMIDIファイルデータを生成します。
*   `parseMidiNotes` (demo/src/parseMidiNotes.ts)
    *   役割: MIDIファイルデータからノートオン/オフイベントを抽出し、演奏情報として解析します。
    *   引数: midiData (Uint8Array)
    *   戻り値: Array<NoteEvent> (音符イベントの配列)
    *   機能: MIDIデータのバイナリ構造を読み解き、各チャンネルの音符の開始時刻、持続時間、ベロシティなどを計算します。
*   `deltaTicksToSeconds` (demo/src/parseMidiNotes.ts)
    *   役割: MIDIのデルタタイム（ティック単位）を実際の時間（秒単位）に変換します。
    *   引数: deltaTicks (number), ticksPerBeat (number), tempo (number)
    *   戻り値: number (秒単位の時間)
    *   機能: MIDIファイルの時間分解能とテンポ情報に基づき、相対的なティック数を絶対的な秒数に変換します。
*   `ensureInitialized` (demo/src/smfToYm2151.ts)
    *   役割: `smf-to-ym2151log-wasm` モジュールが確実に初期化されていることを保証します。
    *   引数: なし
    *   戻り値: Promise<void>
    *   機能: 必要に応じてWebAssemblyモジュールをロードし、初期化が完了するまで待機します。
*   `smfToYM2151Json` (demo/src/smfToYm2151.ts)
    *   役割: Standard MIDI File (SMF) データをYM2151ログのJSON形式に変換します。
    *   引数: smfData (Uint8Array)
    *   戻り値: Promise<string> (YM2151ログのJSON文字列)
    *   機能: WASMモジュールを呼び出してSMFデータを処理し、特定の音源（YM2151）で再生可能なフォーマットに変換します。
*   `showStatus` (demo/src/ui.ts)
    *   役割: ユーザーインターフェースに処理状況やエラーメッセージを表示します。
    *   引数: message (string), isError (boolean, optional)
    *   戻り値: void
    *   機能: 指定されたメッセージをUI上のステータス表示領域に更新します。エラーの場合は色を変更するなどして強調表示します。
*   `loadExample` (demo/src/ui.ts)
    *   役割: 事前定義されたMMLの例をMML入力フィールドにロードします。
    *   引数: なし
    *   戻り値: void
    *   機能: ユーザーがMMLを試す際に便利なように、サンプルコードを提供します。
*   `drawWaveform` (demo/src/visualization.ts)
    *   役割: 指定されたオーディオデータから波形をCanvasに描画します。
    *   引数: audioBuffer (AudioBuffer)
    *   戻り値: void
    *   機能: 音の波形を視覚的に表示することで、生成されたオーディオの様子をユーザーに伝えます。
*   `visualizeRealtime` (demo/src/visualization.ts)
    *   役割: リアルタイムでMIDIイベントの演奏状況を視覚化します。
    *   引数: midiNotes (Array<NoteEvent>), audioContext (AudioContext)
    *   戻り値: void
    *   機能: 再生中の音符の位置や長さを時間軸上でグラフィカルに表示し、音楽の流れを追跡できるようにします。
*   `draw` (demo/src/visualization.ts)
    *   役割: 可視化用のCanvasに描画するメインループまたは単一描画フレームのロジックを含みます。
    *   引数: なし (文脈依存)
    *   戻り値: void
    *   機能: アニメーションフレームごとに描画を更新したり、特定のデータをCanvasに描画します。
*   `writeString` (demo/src/wavExport.ts)
    *   役割: WAVヘッダーに文字列データを書き込むユーティリティ関数です。
    *   引数: view (DataView), offset (number), s (string)
    *   戻り値: void
    *   機能: バイナリデータビューに文字列をバイト単位で書き込みます。
*   `audioBufferToWav` (demo/src/wavExport.ts)
    *   役割: AudioBufferオブジェクトをWAVファイル形式のバイナリデータに変換します。
    *   引数: audioBuffer (AudioBuffer)
    *   戻り値: Blob (WAVファイルデータ)
    *   機能: Web Audio APIのAudioBufferから標準的なWAVファイルヘッダーと音声データを構築します。
*   `exportWav` (demo/src/wavExport.ts)
    *   役割: 現在のオーディオデータをWAVファイルとしてダウンロードします。
    *   引数: なし
    *   戻り値: Promise<void>
    *   機能: `renderWaveformAndAudio` で取得したAudioBufferをWAV形式に変換し、ユーザーにダウンロードを促します。

## 関数呼び出し階層ツリー
```
- initialize (demo/src/main.ts)
  - convertMML (demo/src/mmlConverter.ts)
    - treeToJSON (demo/src/mmlConverter.ts)
  - parseMidiNotes (demo/src/parseMidiNotes.ts)
    - deltaTicksToSeconds (demo/src/parseMidiNotes.ts)
  - showStatus (demo/src/ui.ts)
  - loadExample (demo/src/ui.ts)
  - playAudio (demo/src/audioPlayback.ts)
    - stopAudio (demo/src/audioPlayback.ts)
      - showStatus (demo/src/ui.ts)
      - visualizeRealtime (demo/src/visualization.ts)
        - draw (demo/src/visualization.ts)
  - waitForWebYm2151 (demo/src/audioRenderer.ts)
    - ensureInitialized (demo/src/smfToYm2151.ts)
    - calculateDuration (demo/src/audioRenderer.ts)
    - renderWaveformAndAudio (demo/src/audioRenderer.ts)
      - smfToYM2151Json (demo/src/smfToYm2151.ts)
      - drawWaveform (demo/src/visualization.ts)
    - check (demo/src/audioRenderer.ts)
  - exportWav (demo/src/wavExport.ts)
    - audioBufferToWav (demo/src/wavExport.ts)
    - writeString (demo/src/wavExport.ts)
    - showStatus (demo/src/ui.ts)

---
Generated at: 2026-03-03 07:10:05 JST
