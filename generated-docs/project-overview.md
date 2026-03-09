Last updated: 2026-03-10

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) 形式の楽譜をStandard MIDI File (SMF) へ変換するRust製ライブラリです。
- 独自の4パスアーキテクチャ（トークン化、AST変換、MIDIイベント生成、SMF作成）により、堅牢な変換を実現します。
- ネイティブCLIツールとしてだけでなく、WebAssembly (WASM) を介してブラウザアプリケーションでも利用可能です。

## 技術スタック
- フロントエンド: TypeScript (デモアプリケーション開発), Tone.js (Web Audio APIライブラリ), WebAssembly (mmlabc-to-smf-wasmクレート)
- 音楽・オーディオ: Music Macro Language (MML), Standard MIDI File (SMF), `cat-play-mml` (デフォルトMIDIプレイヤー), TiMidity++, FluidSynth, VLC (カスタムMIDIプレイヤーオプション)
- 開発ツール: Node.js (tree-sitterパーサー生成, デモ開発), npx (tree-sitter-cli実行), `git` (バージョン管理), CodeQL (静的解析用ディレクトリ検出)
- テスト: `node:test` (TypeScriptデモテスト), `cargo test` (Rustライブラリテスト)
- ビルドツール: Cargo (Rustプロジェクト管理), npm (Node.js/TypeScriptプロジェクト管理), `wasm-pack` (RustからWASMへのコンパイル, 推測), `tree-sitter-cli` (パーサー生成)
- 言語機能: Rustの型システムと所有権モデル (メモリ安全性、並行性保証)
- 自動化・CI/CD: (提供情報には明記されていませんが、フォーマットやリンターがCI/CDパイプラインで利用される可能性があります)
- 開発標準: `cargo clippy` (Rustコード品質チェック), `cargo fmt` (Rustコードフォーマッター), .editorconfig (エディタ設定統一)

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
│   └── test-register.mjs
├── demo-library/
│   ├── index.html
│   └── package.json
├── generated-docs/
│   └── development-status-generated-prompt.md
├── googled947dc864c270e07.html
├── issue-notes/
│   ├── 103.md
│   ├── 115.md
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
```

## ファイル詳細説明
-   **`.editorconfig`**: 異なるエディタやIDE間でコードスタイルを統一するための設定ファイル。
-   **`.gitignore`**: Gitがバージョン管理の対象外とするファイルやディレクトリのパターンを定義するファイル。
-   **`.vscode/settings.json`**: Visual Studio Codeのワークスペース固有の設定を定義し、プロジェクトメンバー間で一貫した開発環境を提供します。
-   **`Cargo.lock`**: Rustプロジェクトの依存関係の厳密なバージョンとハッシュを記録し、ビルドの再現性を保証します。
-   **`Cargo.toml`**: Rustプロジェクトのマニフェストファイルで、プロジェクト名、バージョン、依存関係、ビルド設定などを定義します。
-   **`IMPLEMENTATION_REPORT.md`**: プロジェクトの実装に関する詳細な報告やメモを記述したMarkdownファイル。
-   **`LICENSE`**: プロジェクトのライセンス情報（MIT License）が記述されています。
-   **`OPTION_A_IMPLEMENTATION.md`**: 特定の実装オプション「A」に関する詳細な設計や決定事項を記したMarkdownファイル。
-   **`README.ja.md`**: プロジェクトの目的、機能、使用方法、開発状況などを日本語で説明するメインドキュメント。
-   **`README.md`**: プロジェクトの目的、機能、使用方法、開発状況などを英語で説明するメインドキュメント。
-   **`_config.yml`**: Jekyllなどの静的サイトジェネレータで使用される設定ファイル。主にドキュメント生成やウェブサイト設定に関連する。
-   **`build.rs`**: Rustプロジェクトのビルドスクリプト。tree-sitterパーサーのCソースファイルの自動生成など、コンパイル前のカスタム処理を実行します。
-   **`demo/`**: WebブラウザでMMLからSMFへの変換を試すためのデモアプリケーションのソースコードや関連ファイルを格納するディレクトリ。
    -   **`demo/.gitignore`**: デモディレクトリ内のGit管理対象外ファイルを定義。
    -   **`demo/FEATURES.md`**: デモアプリケーションが提供する機能のリストを記述したMarkdownファイル。
    -   **`demo/README.md`**: デモアプリケーションに関する説明ドキュメント。
    -   **`demo/index.html`**: デモアプリケーションのメインとなるHTMLファイル。
    -   **`demo/package.json`**: デモアプリケーションのNode.jsパッケージ定義ファイル。依存関係やスクリプトを記述。
    -   **`demo/src/audioPlayback.ts`**: Web Audio API (Tone.js) を利用してMIDIデータをオーディオとして再生するロジックを実装したTypeScriptファイル。
    -   **`demo/src/audioRenderer.ts`**: MIDIデータからYM2151ログを生成し、その波形を可視化・レンダリングするロジックを実装したTypeScriptファイル。
    -   **`demo/src/main.ts`**: デモアプリケーションの起動処理、Tree-sitterとWASMモジュールの初期化、イベントハンドラの登録などを行うメインのTypeScriptファイル。
    -   **`demo/src/midiReader.ts`**: Standard MIDI File (SMF) のバイナリデータを読み込み、解析するためのクラスを定義したTypeScriptファイル。
    -   **`demo/src/mmlConverter.ts`**: WebAssemblyを介してMML文字列をSMFバイナリに変換する主要なロジックを実装したTypeScriptファイル。
    -   **`demo/src/parseMidiNotes.ts`**: 解析されたMIDIデータから音符イベントを抽出し、時間情報などと共に構造化するTypeScriptファイル。
    -   **`demo/src/smfToYm2151.ts`**: SMFバイナリをYM2151ログ形式に変換するWebAssemblyモジュール `smf-to-ym2151log-wasm` のラッパーを提供。
    -   **`demo/src/state.ts`**: デモアプリケーションのグローバルな状態を管理するための定義を含むTypeScriptファイル。
    -   **`demo/src/treeToJSON.ts`**: Tree-sitterによって生成された抽象構文木 (AST) をJSON形式に変換し、デバッグや表示に利用するTypeScriptファイル。
    -   **`demo/src/ui.ts`**: ユーザーインターフェース (UI) の要素（ステータス表示、例のロードなど）を操作する関数を含むTypeScriptファイル。
    -   **`demo/src/visualization.ts`**: 音声波形などをCanvasに描画する視覚化ロジックを実装したTypeScriptファイル。
    -   **`demo/src/wavExport.ts`**: 生成されたオーディオバッファをWAVファイル形式でエクスポートする機能を提供するTypeScriptファイル。
    -   **`demo/test-loader.mjs`**: Node.jsのテスト実行時のカスタムローダー設定ファイル。
    -   **`demo/test-register.mjs`**: Node.jsのテストフレームワークにテストを登録するためのスクリプト。
-   **`demo-library/`**: デモアプリケーションとは別に、MML変換機能のライブラリ利用例を示すディレクトリ。
    -   **`demo-library/index.html`**: デモライブラリのメインHTMLファイル。
    -   **`demo-library/package.json`**: デモライブラリのNode.jsパッケージ情報と依存関係を定義。
-   **`generated-docs/`**: 自動生成されたドキュメントを格納するためのディレクトリ。
    -   **`generated-docs/development-status-generated-prompt.md`**: 開発状況に関する自動生成されたレポートやプロンプト。
-   **`googled947dc864c270e07.html`**: Googleサイト所有権確認のための認証ファイル。
-   **`issue-notes/`**: 開発中の課題や特定の機能に関するメモ、議論をMarkdown形式でまとめたディレクトリ。
    -   **`issue-notes/103.md`**: Issue #103に関連するメモ。
    -   **`issue-notes/115.md`**: Issue #115に関連するメモ。
    -   **`issue-notes/39.md`**: Issue #39に関連するメモ。
    -   **`issue-notes/44.md`**: Issue #44に関連するメモ。
-   **`mmlabc-to-smf-rust.toml.example`**: カスタムMIDIプレイヤーを指定する際の設定ファイルの例。
-   **`mmlabc-to-smf-wasm/`**: Rustで実装されたMMLからSMFへの変換ロジックをWebAssembly (WASM) にコンパイルするためのクレート。
    -   **`mmlabc-to-smf-wasm/Cargo.lock`**: WASMクレートの依存関係の正確なバージョンを記録。
    -   **`mmlabc-to-smf-wasm/Cargo.toml`**: WASMクレートのビルド設定、依存関係、メタデータを定義。
    -   **`mmlabc-to-smf-wasm/src/lib.rs`**: WASMクレートのライブラリのエントリーポイント。JavaScriptから呼び出されるRust関数をエクスポートする。
    -   **`mmlabc-to-smf-wasm/src/token_extractor.rs`**: WASM向けにMMLトークンを抽出するロジック。
-   **`package.json`**: プロジェクト全体のNode.jsパッケージ定義ファイル。主に開発スクリプトやルートの依存関係を管理。
-   **`scripts/`**: プロジェクトのビルド、デモの準備、その他のユーティリティタスクを実行するためのシェルスクリプトを格納するディレクトリ。
    -   **`scripts/README.md`**: スクリプトの利用方法に関する説明。
    -   **`scripts/build-demo.sh`**: デモアプリケーションをビルドするスクリプト。
    -   **`scripts/transform-demo-paths.sh`**: デモ内のファイルパスを変換するスクリプト。
-   **`src/`**: メインのRustライブラリとコマンドラインインターフェース (CLI) アプリケーションのソースコードが格納されているディレクトリ。
    -   **`src/attachment_json.rs`**: 変換プロセスの中間結果をJSONとして出力するためのアタッチメント（デバッグ情報）に関するロジック。
    -   **`src/config.rs`**: アプリケーションの設定（例: 外部MIDIプレイヤーの指定）を読み込み、管理するモジュール。
    -   **`src/lib.rs`**: プロジェクトのライブラリクレートのルート。主要なMMLからSMFへの変換ロジックを公開するインターフェース。
    -   **`src/main.rs`**: コマンドラインインターフェース (CLI) アプリケーションのエントリーポイント。コマンドライン引数の解析と変換処理のオーケストレーションを行う。
    -   **`src/mml_preprocessor.rs`**: MML文字列を解析前に前処理するロジック（例: マクロ展開、コメント除去）。
    -   **`src/pass1_parser.rs`**: MML文字列をトークンに解析する最初のパス。主に`tree-sitter-mml`パーサーを利用して構文解析を実行。
    -   **`src/pass2_ast.rs`**: パス1で生成されたトークンストリームから抽象構文木 (AST) を構築する第2パス。
    -   **`src/pass3_events.rs`**: 構築されたASTを基に、MIDIイベント（ノートオン、ノートオフ、テンポチェンジなど）のシーケンスを生成する第3パス。
    -   **`src/pass4_midi.rs`**: 生成されたMIDIイベントのシーケンスをStandard MIDI File (SMF) 形式のバイナリデータに変換する最終パス。
    -   **`src/tree_sitter_mml.rs`**: `tree-sitter-mml`パーサーをRustプロジェクト内で利用するためのインターフェースやヘルパー関数。
    -   **`src/types.rs`**: プロジェクト全体で共通して使用されるカスタムデータ型、構造体、列挙型などを定義。
-   **`tests/`**: Rustライブラリの単体テストおよび統合テストを格納するディレクトリ。
    -   **`tests/integration_test.rs`**: ライブラリ全体の統合的な機能テスト。
    -   **`tests/test_attachment_json.rs`**: JSONアタッチメント機能に関するテスト。
    -   **`tests/test_c1_vs_c64.rs`**: 特定の音符表記（C1とC64など）のテスト。
    -   **`tests/test_channel.rs`**: MMLの多チャンネル処理に関するテスト。
    -   **`tests/test_chord.rs`**: 和音（コード）のMML表記とMIDI変換に関するテスト。
    -   **`tests/test_cli.rs`**: コマンドラインインターフェースの引数解析や動作に関するテスト。
    -   **`tests/test_config.rs`**: 設定ファイルの読み込みと適用に関するテスト。
    -   **`tests/test_dotted_notes.rs`**: 付点音符の処理に関するテスト。
    -   **`tests/test_drum_channel.rs`**: ドラムチャンネルのMML処理に関するテスト。
    -   **`tests/test_key_transpose.rs`**: キーの移調機能に関するテスト。
    -   **`tests/test_length.rs`**: 音長指定コマンドに関するテスト。
    -   **`tests/test_modifier.rs`**: 音符修飾子（シャープ、フラットなど）に関するテスト。
    -   **`tests/test_note_length.rs`**: 個々の音符の長さの計算と適用に関するテスト。
    -   **`tests/test_octave.rs`**: オクターブ指定コマンドに関するテスト。
    -   **`tests/test_pass1.rs`**: 変換プロセスにおけるパス1（トークン解析）の単体テスト。
    -   **`tests/test_pass2.rs`**: 変換プロセスにおけるパス2（AST変換）の単体テスト。
    -   **`tests/test_pass3.rs`**: 変換プロセスにおけるパス3（MIDIイベント生成）の単体テスト。
    -   **`tests/test_pass4.rs`**: 変換プロセスにおけるパス4（MIDIファイル作成）の単体テスト。
    -   **`tests/test_program_change.rs`**: プログラムチェンジ（音色変更）コマンドに関するテスト。
    -   **`tests/test_rest.rs`**: 休符の処理に関するテスト。
    -   **`tests/test_tempo.rs`**: テンポ指定コマンドに関するテスト。
    -   **`tests/test_velocity.rs`**: ベロシティ（音量）指定コマンドに関するテスト。
-   **`tree-sitter-mml/`**: Music Macro Language (MML) の構文を解析するためのTree-sitterパーサーの定義と生成されたファイルを格納するディレクトリ。
    -   **`tree-sitter-mml/grammar.js`**: MMLの文法ルールを定義するJavaScriptファイル。
    -   **`tree-sitter-mml/package.json`**: `tree-sitter-mml`パーサーのNode.jsパッケージ情報と依存関係を定義。
    -   **`tree-sitter-mml/src/`**: Tree-sitterパーサーのC言語ソースコードおよび関連定義ファイルが生成されるディレクトリ。
        -   **`tree-sitter-mml/src/grammar.json`**: `grammar.js`から生成されるMML文法のJSON表現。
        -   **`tree-sitter-mml/src/node-types.json`**: ASTノードの型定義のJSON表現。
        -   **`tree-sitter-mml/src/parser.c`**: `grammar.js`から生成されるMMLパーサーのC言語実装。
        -   **`tree-sitter-mml/src/tree_sitter/`**: Tree-sitterライブラリのコアとなるヘッダーファイル群。
            -   **`tree-sitter-mml/src/tree_sitter/alloc.h`**: メモリ割り当て関連のヘッダーファイル。
            -   **`tree-sitter-mml/src/tree_sitter/array.h`**: 配列操作関連のヘッダーファイル。
            -   **`tree-sitter-mml/src/tree_sitter/parser.h`**: Tree-sitterパーサーのコアヘッダーファイル。

## 関数詳細説明
-   **`playAudio(midiNotes, duration)`** (`demo/src/audioPlayback.ts`):
    -   役割: MIDI音符データと総再生時間を受け取り、Web Audio API (Tone.js) を使って音声を再生します。
    -   引数: `midiNotes`: 再生するMIDI音符の配列 (例: `[{time: number, duration: number, midi: number, velocity: number}]`), `duration`: 総再生時間 (秒)。
    -   戻り値: なし。
-   **`stopAudio()`** (`demo/src/audioPlayback.ts`):
    -   役割: 現在再生中の音声を停止します。
    -   引数: なし。
    -   戻り値: なし。
-   **`waitForWebYm2151()`** (`demo/src/audioRenderer.ts`):
    -   役割: WebAssemblyモジュール `smf-to-ym2151log-wasm` の初期化が完了するまで待機します。
    -   引数: なし。
    -   戻り値: Promise<void>。
-   **`calculateDuration(smfBinary)`** (`demo/src/audioRenderer.ts`):
    -   役割: Standard MIDI Fileのバイナリデータから曲の総再生時間を計算します。
    -   引数: `smfBinary`: SMFバイナリデータ (Uint8Array)。
    -   戻り値: `number` (総再生時間(秒))。
-   **`renderWaveformAndAudio(mml)`** (`demo/src/audioRenderer.ts`):
    -   役割: MML文字列からMIDIファイルを生成し、そのMIDIファイルをYM2151ログに変換、波形をレンダリングして音声を生成します。
    -   引数: `mml`: MML形式の文字列。
    -   戻り値: Promise<{ audioBuffer: AudioBuffer; smfBinary: Uint8Array }>。
-   **`check(smfBinary)`** (`demo/src/audioRenderer.ts`):
    -   役割: SMFバイナリデータをチェックし、不正なデータがないか確認します。
    -   引数: `smfBinary`: SMFバイナリデータ (Uint8Array)。
    -   戻り値: `boolean`。
-   **`initialize()`** (`demo/src/main.ts`):
    -   役割: デモアプリケーションの初期化処理（tree-sitterとmmlabc-to-smf-wasmのロード、イベントリスナー設定など）を行います。
    -   引数: なし。
    -   戻り値: Promise<void>。
-   **`constructor(smfBinary)`** (`demo/src/midiReader.ts`):
    -   役割: MIDIファイルバイナリを受け取り、解析するための `MidiReader` オブジェクトを初期化します。
    -   引数: `smfBinary`: MIDIファイルバイナリデータ。
    -   戻り値: なし (コンストラクタ)。
-   **`convertMML(mmlString)`** (`demo/src/mmlConverter.ts`):
    -   役割: MML文字列をWebAssembly (mmlabc-to-smf-wasm) を介してStandard MIDI Fileバイナリに変換します。
    -   引数: `mmlString`: 変換するMML形式の文字列。
    -   戻り値: Promise<Uint8Array> (SMFバイナリデータ)。
-   **`parseMidiNotes(smfBinary)`** (`demo/src/parseMidiNotes.ts`):
    -   役割: Standard MIDI Fileバイナリを解析し、音符イベントのリストを抽出します。
    -   引数: `smfBinary`: 解析するSMFバイナリデータ (Uint8Array)。
    -   戻り値: `{notes: Array<{time: number, duration: number, midi: number, velocity: number}>, duration: number}`。
-   **`deltaTicksToSeconds(deltaTime, timebase, tempo)`** (`demo/src/parseMidiNotes.ts`):
    -   役割: MIDIデルタタイムを秒単位の時間に変換します。
    -   引数: `deltaTime`: MIDIデルタタイム, `timebase`: MIDIタイムベース (ticks per quarter note), `tempo`: テンポ (microseconds per quarter note)。
    -   戻り値: `number` (秒)。
-   **`ensureInitialized()`** (`demo/src/smfToYm2151.ts`):
    -   役割: `smf-to-ym2151log-wasm` が初期化されていることを保証します。必要であれば初期化をトリガーします。
    -   引数: なし。
    -   戻り値: Promise<void>。
-   **`smfToYM2151Json(smfBinary)`** (`demo/src/smfToYm2151.ts`):
    -   役割: SMFバイナリをYM2151ログのJSON形式に変換します。
    -   引数: `smfBinary`: SMFバイナリデータ (Uint8Array)。
    -   戻り値: `string` (YM2151ログのJSON文字列)。
-   **`treeToJSON(tree)`** (`demo/src/treeToJSON.ts`):
    -   役割: tree-sitterで生成された構文木を再帰的に走査し、JSON形式のオブジェクトに変換します。デバッグ出力に利用されます。
    -   引数: `tree`: tree-sitterの構文木オブジェクト。
    -   戻り値: `object` (JSON形式のAST表現)。
-   **`showStatus(message)`** (`demo/src/ui.ts`):
    -   役割: UIにステータスメッセージを表示します。
    -   引数: `message`: 表示する文字列。
    -   戻り値: なし。
-   **`loadExample()`** (`demo/src/ui.ts`):
    -   役割: 例となるMMLコードをUIの入力フィールドに読み込みます。
    -   引数: なし。
    -   戻り値: なし。
-   **`drawWaveform(audioBuffer, canvas)`** (`demo/src/visualization.ts`):
    -   役割: 音声バッファから波形を抽出し、指定されたCanvasに描画します。
    -   引数: `audioBuffer`: AudioBufferオブジェクト, `canvas`: HTMLCanvasElementオブジェクト。
    -   戻り値: なし。
-   **`visualizeRealtime(analyser, canvas)`** (`demo/src/visualization.ts`):
    -   役割: Web Audio APIのAnalyserNodeからリアルタイムで音声データを取得し、波形をCanvasに描画します。
    -   引数: `analyser`: AnalyserNodeオブジェクト, `canvas`: HTMLCanvasElementオブジェクト。
    -   戻り値: なし (ループ処理)。
-   **`draw()`** (`demo/src/visualization.ts`):
    -   役割: `visualizeRealtime` が内部的に再帰的に呼び出す描画ループのヘルパー関数。
    -   引数: なし。
    -   戻り値: なし。
-   **`writeString(view, offset, string)`** (`demo/src/wavExport.ts`):
    -   役割: `DataView` オブジェクトに文字列を書き込みます。WAVヘッダーの構築に使用されます。
    -   引数: `view`: DataViewオブジェクト, `offset`: 書き込み開始オフセット, `string`: 書き込む文字列。
    -   戻り値: なし。
-   **`audioBufferToWav(audioBuffer)`** (`demo/src/wavExport.ts`):
    -   役割: `AudioBuffer` をWAVファイル形式の `Blob` に変換します。
    -   引数: `audioBuffer`: 変換するAudioBufferオブジェクト。
    -   戻り値: `Blob` (WAVデータ)。
-   **`exportWav(audioBuffer, filename)`** (`demo/src/wavExport.ts`):
    -   役割: `AudioBuffer` をWAVファイルとしてダウンロード可能な形式でエクスポートします。
    -   引数: `audioBuffer`: エクスポートするAudioBufferオブジェクト, `filename`: ダウンロード時のファイル名。
    -   戻り値: なし。
-   **`mockAudioBuffer(channels, length, sampleRate)`** (`demo/tests/audioBufferToWav.test.ts`):
    -   役割: テスト用に `AudioBuffer` オブジェクトのモックを作成します。
    -   引数: `channels`: チャンネル数, `length`: サンプルフレーム数, `sampleRate`: サンプリングレート。
    -   戻り値: Mock AudioBufferオブジェクト。
-   **`buildSmf(trackEvents)`** (`demo/tests/parseMidiNotes.test.ts`):
    -   役割: テスト用に簡易的なSMFバイナリデータを構築します。
    -   引数: `trackEvents`: トラックイベントの配列。
    -   戻り値: `Uint8Array` (SMFバイナリデータ)。
-   **`main()`** (`src/main.rs`):
    -   役割: CLIアプリケーションのエントリーポイント。コマンドライン引数を解析し、MML変換とオプションの再生処理をオーケストレートします。
    -   引数: なし (環境引数から取得)。
    -   戻り値: `Result<(), Box<dyn Error>>`。
-   **`run_conversion(mml: &str, output_path: Option<&Path>, no_play: bool)`** (`src/lib.rs`):
    -   役割: MML文字列からStandard MIDI Fileへの変換を実行し、必要に応じてファイルを保存し、外部プレイヤーで再生します。
    -   引数: `mml`: MML文字列, `output_path`: 出力ファイルパス (Optional), `no_play`: 再生を無効にするフラグ。
    -   戻り値: `Result<(), Error>`。
-   **`parse_mml(mml_string: &str)`** (`src/pass1_parser.rs`):
    -   役割: 入力MML文字列をTree-sitterパーサーを使用してトークンストリームに解析します。
    -   引数: `mml_string`: 解析するMML文字列。
    -   戻り値: `Result<MmlTokenStream, ParserError>`。
-   **`build_ast(token_stream: &MmlTokenStream)`** (`src/pass2_ast.rs`):
    -   役割: トークンストリームから抽象構文木 (AST) を構築します。
    -   引数: `token_stream`: パス1で生成されたMMLトークンストリーム。
    -   戻り値: `Result<MmlAst, AstError>`。
-   **`generate_midi_events(ast: &MmlAst)`** (`src/pass3_events.rs`):
    -   役割: 抽象構文木 (AST) を走査し、MIDIイベントのシーケンスを生成します。
    -   引数: `ast`: パス2で構築されたMML AST。
    -   戻り値: `Result<MidiEventSequence, EventError>`。
-   **`create_smf(midi_events: &MidiEventSequence)`** (`src/pass4_midi.rs`):
    -   役割: 生成されたMIDIイベントのシーケンスからStandard MIDI File (SMF) のバイナリデータを作成します。
    -   引数: `midi_events`: パス3で生成されたMIDIイベントシーケンス。
    -   戻り値: `Result<Vec<u8>, MidiError>` (SMFバイナリデータ)。
-   **`convert_mml_to_smf_wasm(mml_input: String)`** (`mmlabc-to-smf-wasm/src/lib.rs`):
    -   役割: WebAssemblyのエクスポート関数として、MML文字列を受け取りSMFバイナリデータに変換します。Rustのコア変換ロジックを呼び出します。
    -   引数: `mml_input`: 変換するMML文字列。
    -   戻り値: `Result<Vec<u8>, JsValue>` (SMFバイナリデータ、またはエラー)。

## 関数呼び出し階層ツリー
```
main (src/main.rs)
└── run_conversion (src/lib.rs)
    ├── parse_mml (src/pass1_parser.rs)
    │   └── (uses tree-sitter-mml parser)
    ├── build_ast (src/pass2_ast.rs)
    ├── generate_midi_events (src/pass3_events.rs)
    ├── create_smf (src/pass4_midi.rs)
    └── (external MIDI player, e.g., `cat-play-mml`)

initialize (demo/src/main.ts)
├── (loads web-tree-sitter)
├── convertMML (demo/src/mmlConverter.ts)
│   └── convert_mml_to_smf_wasm (mmlabc-to-smf-wasm/src/lib.rs)
│       ├── parse_mml (Rust core)
│       ├── build_ast (Rust core)
│       ├── generate_midi_events (Rust core)
│       └── create_smf (Rust core)
├── renderWaveformAndAudio (demo/src/audioRenderer.ts)
│   ├── convertMML (demo/src/mmlConverter.ts)
│   ├── smfToYM2151Json (demo/src/smfToYm2151.ts)
│   │   └── ensureInitialized (demo/src/smfToYm2151.ts)
│   │       └── (loads smf-to-ym2151log-wasm)
│   ├── (new MidiReader) (demo/src/midiReader.ts::constructor)
│   ├── parseMidiNotes (demo/src/parseMidiNotes.ts)
│   │   └── deltaTicksToSeconds (demo/src/parseMidiNotes.ts)
│   └── drawWaveform (demo/src/visualization.ts)
├── playAudio (demo/src/audioPlayback.ts)
│   ├── visualizeRealtime (demo/src/visualization.ts)
│   │   └── draw (demo/src/visualization.ts)
│   └── stopAudio (demo/src/audioPlayback.ts)
│       └── showStatus (demo/src/ui.ts)
├── exportWav (demo/src/wavExport.ts)
│   └── audioBufferToWav (demo/src/wavExport.ts)
│       └── writeString (demo/src/wavExport.ts)
├── loadExample (demo/src/ui.ts)
└── treeToJSON (demo/src/treeToJSON.ts)

// Test functions are generally standalone or call specific modules for verification.
mockAudioBuffer (demo/tests/audioBufferToWav.test.ts)
buildSmf (demo/tests/parseMidiNotes.test.ts)
mockNode (demo/tests/treeToJSON.test.ts)

---
Generated at: 2026-03-10 07:09:35 JST
