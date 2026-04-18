Last updated: 2026-04-19

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) を Standard MIDI File (SMF) に変換するRust製のライブラリおよびCLIツールです。
- 変換処理は4つのパスで構成され、各パスで中間結果をJSON形式で出力します。
- ブラウザでの動作検証のためのWebAssemblyモジュールとデモアプリケーションも含まれます。

## 技術スタック
- フロントエンド: HTML (Webデモ構造), TypeScript (デモのロジック), WebAssembly (WASM: RustをWebで実行), Node.js/npm (デモのビルド・開発環境)
- 音楽・オーディオ: Music Macro Language (MML: 入力フォーマット), Standard MIDI File (SMF: 出力フォーマット), MIDI (イベント処理), cat-play-mml (外部MIDIプレイヤー), Web Audio API (デモでの音声再生), Ym2151エミュレータ (SMFからログ変換)
- 開発ツール: Rust (メイン開発言語), Cargo (Rustパッケージマネージャー), Tree-sitter (MMLパーサー生成), VS Code (エディタ設定), Git (バージョン管理)
- テスト: Rust組み込みテストフレームワーク (cargo test), Node.jsのテストモジュール (`node:test`, `node:assert/strict`)
- ビルドツール: Cargo (Rustプロジェクトビルド), npm (JavaScriptパッケージ管理), npx (Node.jsパッケージ実行), `tree-sitter generate` (パーサーコード生成), シェルスクリプト (`build-demo.sh`, `transform-demo-paths.sh`)
- 言語機能: Rust, TypeScript, JavaScript, JSON (中間データ/添付情報), TOML (設定ファイル), C (Tree-sitter生成コード)
- 自動化・CI/CD: カスタムシェルスクリプト (`scripts/`配下)
- 開発標準: cargo clippy (Lint), cargo fmt (フォーマッター), .editorconfig (コーディングスタイル)

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
- **.editorconfig**: 異なるエディタやIDEを使用する開発者間で、インデントスタイル、文字コード、改行コードなどのコーディングスタイルを統一するための設定ファイルです。
- **.gitattributes**: Gitリポジトリでファイルの種類に応じた特別な属性（例: テキストファイルの改行コード自動変換、バイナリファイルの差分表示無効化など）を定義するためのファイルです。
- **.gitignore**: Gitがバージョン管理の対象としないファイルやディレクトリ（例: ビルド生成物、ログファイル、一時ファイルなど）を指定するファイルです。
- **.vscode/settings.json**: Visual Studio Codeエディタにおけるプロジェクト固有の設定を記述するファイルです。
- **Cargo.lock**: Rustプロジェクトの依存関係ツリーにおける各クレートの正確なバージョンとハッシュを記録し、再現可能なビルドを保証するファイルです。
- **Cargo.toml**: Rustプロジェクトのマニフェストファイルです。プロジェクト名、バージョン、作者、依存クレート、ビルド設定などを定義します。
- **LICENSE**: プロジェクトの配布および使用に関するライセンス情報（MIT License）を記載したファイルです。
- **README.ja.md**: プロジェクトの日本語による概要、使い方、機能、開発状況などを説明するファイルです。
- **README.md**: プロジェクトの英語による概要、使い方、機能、開発状況などを説明するファイルです。
- **_config.yml**: Webサイトジェネレーター（例: Jekyll）の設定ファイルとして使われることが多く、サイト全体のメタデータやビルドオプションを定義します。
- **build.rs**: Rustプロジェクトのビルドプロセス中に実行されるカスタムスクリプトです。通常、C/C++ライブラリのビルドやコード生成などに利用されます。
- **demo/.gitignore**: `demo`ディレクトリ内のGit追跡対象外ファイルを指定します。
- **demo/FEATURES.md**: デモアプリケーションが提供する機能や特徴について記述されたMarkdownファイルです。
- **demo/README.md**: デモアプリケーションの概要、実行方法、開発ガイドラインなどを説明するファイルです。
- **demo/index.html**: デモアプリケーションのメインとなるHTMLファイルで、ユーザーインターフェースの構造を定義します。
- **demo/package.json**: デモアプリケーションのJavaScript/TypeScriptの依存関係とスクリプトを定義するファイルです。
- **demo/src/audioPlayback.ts**: Web Audio APIを使用して生成されたオーディオデータの再生を制御するTypeScriptファイルです。
- **demo/src/audioRenderer.ts**: MMLから変換されたMIDIデータを波形としてレンダリングし、オーディオデータを生成するロジックを含むTypeScriptファイルです。
- **demo/src/main.ts**: デモアプリケーションのエントリポイントとなるTypeScriptファイルで、初期化処理や主要なイベントハンドラを設定します。
- **demo/src/midiReader.ts**: バイナリ形式のMIDIファイルを読み込み、解析するためのロジックを提供するTypeScriptファイルです。
- **demo/src/mmlConverter.ts**: MML文字列をSMFデータに変換するWebAssemblyモジュールを呼び出し、その結果を処理するTypeScriptファイルです。
- **demo/src/parseMidiNotes.ts**: MIDIデータから個々の音符イベント（開始時刻、長さなど）を詳細に解析するTypeScriptファイルです。
- **demo/src/smfToYm2151.ts**: Standard MIDI File (SMF) データをYm2151音源エミュレータで利用可能なログ形式のJSONに変換するTypeScriptファイルです。
- **demo/src/state.ts**: デモアプリケーション全体の状態（MML入力、MIDIデータ、オーディオ再生状態など）を管理するTypeScriptファイルです。
- **demo/src/treeToJSON.ts**: Tree-sitterによって生成された構文木を、デバッグや表示に適したJSON形式に変換するTypeScriptユーティリティです。
- **demo/src/ui.ts**: デモアプリケーションのユーザーインターフェースの要素を操作し、ステータス表示や例のロードなどの機能を提供するTypeScriptファイルです。
- **demo/src/visualization.ts**: オーディオデータの波形をCanvas要素に描画し、視覚的なフィードバックを提供するTypeScriptファイルです。
- **demo/src/wavExport.ts**: 生成されたオーディオデータをWAVファイル形式にエンコードし、ダウンロード可能にするTypeScriptファイルです。
- **demo/test-loader.mjs**: Node.jsのESM (ECMAScript Modules) 環境でテストファイルをロードするためのJavaScriptモジュールです。
- **demo/test-register.mjs**: Node.jsのESM環境でテストランナーを登録するためのJavaScriptモジュールです。
- **demo/tests/audioBufferToWav.test.ts**: `wavExport.ts`の`audioBufferToWav`関数の単体テストを記述したTypeScriptファイルです。
- **demo/tests/midiReader.test.ts**: `midiReader.ts`モジュールのMIDIファイル読み込みおよび解析機能を検証するテストファイルです。
- **demo/tests/parseMidiNotes.test.ts**: `parseMidiNotes.ts`モジュールのMIDIノート解析ロジックをテストするファイルです。
- **demo/tests/treeToJSON.test.ts**: `treeToJSON.ts`ユーティリティ関数の、Tree-sitter構文木からJSONへの変換機能をテストするファイルです。
- **demo-library/index.html**: `mmlabc-to-smf-wasm`ライブラリの基本的な利用方法を示すHTMLデモファイルです。
- **demo-library/package.json**: `demo-library`アプリケーションのJavaScript/TypeScriptの依存関係とスクリプトを定義するファイルです。
- **generated-docs/development-status-generated-prompt.md**: プロジェクトの開発ステータスに関する情報が自動生成されたドキュメントです。
- **googled947dc864c270e07.html**: Google Search Consoleなどのサービスでサイトの所有権を証明するために使用されるHTMLファイルです。
- **issue-notes/103.md**: Issue #103に関する開発メモや議論が記述されたMarkdownファイルです。
- **issue-notes/123.md**: Issue #123に関する開発メモや議論が記述されたMarkdownファイルです。
- **issue-notes/133.md**: Issue #133に関する開発メモや議論が記述されたMarkdownファイルです。
- **issue-notes/39.md**: Issue #39に関する開発メモや議論が記述されたMarkdownファイルです。
- **issue-notes/44.md**: Issue #44に関する開発メモや議論が記述されたMarkdownファイルです。
- **mmlabc-to-smf-rust.toml.example**: プロジェクトの実行時設定をカスタマイズするための設定ファイル（TOML形式）の例です。
- **mmlabc-to-smf-wasm/Cargo.lock**: `mmlabc-to-smf-wasm`クレートの依存関係の正確なバージョンを記録するファイルです。
- **mmlabc-to-smf-wasm/Cargo.toml**: `mmlabc-to-smf-wasm`クレートのマニフェストファイルで、WebAssemblyビルドに必要な設定が含まれます。
- **mmlabc-to-smf-wasm/src/lib.rs**: `mmlabc-to-smf-wasm`クレートのライブラリのエントリポイントで、WebAssemblyに公開されるRust関数が定義されています。
- **mmlabc-to-smf-wasm/src/token_extractor.rs**: WebAssembly環境でMMLからトークンを抽出するロジックに関連するRustモジュールです。
- **package.json**: プロジェクト全体のNode.js/JavaScriptの依存関係とスクリプトを定義するファイルです。
- **scripts/README.md**: スクリプトディレクトリの内容と各スクリプトの使用方法を説明するファイルです。
- **scripts/build-demo.sh**: デモアプリケーションをビルドするためのシェルスクリプトです。
- **scripts/transform-demo-paths.sh**: デモ関連のファイルパスをビルド環境に合わせて変換するシェルスクリプトです。
- **src/attachment_json.rs**: 添付JSONデータのパース、生成、管理に関連するRustモジュールです。
- **src/config.rs**: CLIツールやライブラリの動作設定を読み込み、管理するためのRustモジュールです。
- **src/lib.rs**: `mmlabc_to_smf`ライブラリのメインエントリポイントで、MMLからSMFへの変換ロジックが定義されています。
- **src/main.rs**: `mmlabc-to-smf` CLIアプリケーションのメインエントリポイントで、コマンドライン引数の解析と`lib.rs`の呼び出しを行います。
- **src/mml_preprocessor.rs**: MML文字列の解析前に、特定の記法の前処理（例: マクロ展開や正規化）を行うRustモジュールです。
- **src/parse_tree_tokens.rs**: Tree-sitterが生成した具象構文木 (CST) から、意味のあるトークンや要素を抽出するRustモジュールです。
- **src/pass1_parser.rs**: MMLの構文を解析し、初期のパースツリーを生成する「第1パス」のロジックを実装したRustモジュールです。
- **src/pass2_ast.rs**: 第1パスで生成されたパースツリーから、より抽象的な抽象構文木 (AST) を構築する「第2パス」のロジックを実装したRustモジュールです。
- **src/pass3_events.rs**: 第2パスで生成されたASTを基に、SMFで表現されるMIDIイベント列を生成する「第3パス」のロジックを実装したRustモジュールです。
- **src/pass4_midi.rs**: 第3パスで生成されたMIDIイベント列を、最終的なStandard MIDI File (SMF) 形式のバイナリデータに変換する「第4パス」のロジックを実装したRustモジュールです。
- **src/tree_sitter_mml.rs**: Tree-sitter MMLパーサのRustバインディングを提供し、RustコードからMMLの構文解析機能を利用できるようにします。
- **src/types.rs**: プロジェクト全体で使用されるカスタムデータ構造、列挙型、トレイトなどの型定義を集めたRustモジュールです。
- **tests/integration_test.rs**: プロジェクトの複数のコンポーネントが連携して正しく機能するかを検証する統合テストを記述したファイルです。
- **tests/test_attachment_json.rs**: `attachment_json`モジュールが添付JSONデータを正しく処理するかを検証するテストファイルです。
- **tests/test_c1_vs_c64.rs**: C1（Central C）やC64などの特定の音高表記がMML変換で正しく扱われるかを検証するテストファイルです。
- **tests/test_channel.rs**: 複数チャンネルのMML記法がMIDIチャンネルに正しく変換されるかを検証するテストファイルです。
- **tests/test_chord.rs**: 和音（コード）のMML記法がMIDIデータで正しく表現されるかを検証するテストファイルです。
- **tests/test_cli.rs**: CLIアプリケーションが正しい引数解析とMML変換結果を生成するかを検証するテストファイルです。
- **tests/test_config.rs**: `config`モジュールが設定ファイルを正しく読み込み、適用するかを検証するテストファイルです。
- **tests/test_dotted_notes.rs**: 付点音符（例: `c4.`）のMML記法が正しい音長に変換されるかを検証するテストファイルです。
- **tests/test_drum_channel.rs**: `@128`記法によるドラムチャンネルの割り当て機能が正しく動作するかを検証するテストファイルです。
- **tests/test_key_transpose.rs**: キー移調（例: `kt1`）のMML記法が音符のピッチに正しく反映されるかを検証するテストファイルです。
- **tests/test_length.rs**: デフォルト音長 (`lN`) や明示的な音長指定がMML変換で正しく機能するかを検証するテストファイルです。
- **tests/test_modifier.rs**: シャープ (`+`) やフラット (`-`) などの音符修飾記号が正しく処理されるかを検証するテストファイルです。
- **tests/test_note_length.rs**: 音符の長さ計算ロジックが正確であるかを検証するテストファイルです。
- **tests/test_octave.rs**: オクターブ変更記号 (`<`, `>`, `oN`) が音符のオクターブに正しく影響するかを検証するテストファイルです。
- **tests/test_pass1.rs**: MMLtoSMF変換の「第1パス」（パーシング）のテストです。
- **tests/test_pass2.rs**: MMLtoSMF変換の「第2パス」（AST構築）のテストです。
- **tests/test_pass3.rs**: MMLtoSMF変換の「第3パス」（MIDIイベント生成）のテストです。
- **tests/test_pass4.rs**: MMLtoSMF変換の「第4パス」（SMF最終出力）のテストです。
- **tests/test_program_change.rs**: プログラムチェンジ記号 (`@N`) がMIDI楽器の変更に正しく変換されるかを検証するテストファイルです。
- **tests/test_rest.rs**: 休符 (`r`) のMML記法が正しく処理され、音が出ないことを検証するテストファイルです。
- **tests/test_tempo.rs**: テンポ指定 (`tN`) がMIDIファイル内のテンポイベントに正しく反映されるかを検証するテストファイルです。
- **tests/test_velocity.rs**: ベロシティ指定 (`vN`) がMIDIノートの音量に正しく変換されるかを検証するテストファイルです。
- **tree-sitter-mml/grammar.js**: Tree-sitterパーサージェネレーターがMMLの構文解析器を生成するために使用するJavaScript形式の文法定義ファイルです。
- **tree-sitter-mml/package.json**: `tree-sitter-mml`パーサのNode.jsパッケージ設定ファイルです。
- **tree-sitter-mml/src/grammar.json**: `grammar.js`から生成されるMMLのJSON形式文法定義ファイルです。
- **tree-sitter-mml/src/node-types.json**: Tree-sitterパーサーが認識するMMLのノードタイプ定義を含むJSONファイルです。
- **tree-sitter-mml/src/parser.c**: `grammar.js`からTree-sitterによって自動生成された、MMLを解析するためのC言語のソースファイルです。
- **tree-sitter-mml/src/tree_sitter/**: Tree-sitterライブラリのC言語ヘッダーファイルが含まれるディレクトリです。カスタムパーサーがTree-sitterランタイムと連携するために必要です。
- **tree-sitter-mml/tree-sitter-mml.wasm**: `parser.c`からビルドされたTree-sitter MMLパーサのWebAssemblyバイナリファイルで、ブラウザ環境でMML解析を可能にします。

## 関数詳細説明
- **playAudio (demo/src/audioPlayback.ts)**
  - **役割**: 生成されたオーディオデータをWeb Audio APIで再生します。
  - **引数**: なし
  - **戻り値**: `Promise<void>` (再生完了時に解決)
  - **機能**: `state`からYm2151ログのオーディオバッファを取得し、Web Audio APIを通じて音声として出力します。再生中はUIの再生ボタンの状態を更新し、エラー発生時は`showStatus`関数で表示します。
- **stopAudio (demo/src/audioPlayback.ts)**
  - **役割**: 現在再生中のオーディオを即座に停止します。
  - **引数**: なし
  - **戻り値**: `void`
  - **機能**: オーディオコンテキストを停止し、UIの再生ボタンを初期状態に戻します。
- **if (demo/src/audioPlayback.ts)**
  - **役割**: 特定の条件に基づいてコードの実行フローを制御します。
  - **引数**: 条件式 (`boolean`)
  - **戻り値**: なし
  - **機能**: `playAudio`および`stopAudio`のロジック内で、再生状態のチェックやオーディオコンテキストの存在確認などに利用されます。
- **catch (demo/src/audioPlayback.ts)**
  - **役割**: `try`ブロック内で発生した例外を捕捉し、エラーハンドリングを行います。
  - **引数**: エラーオブジェクト (`e: any`)
  - **戻り値**: なし
  - **機能**: オーディオ再生中にエラーが発生した場合、`showStatus`関数を通じてユーザーにエラーメッセージを表示し、UIの状態をリセットします。
- **waitForWebYm2151 (demo/src/audioRenderer.ts)**
  - **役割**: WebAssembly版のYM2151エミュレータモジュールのロードと初期化が完了するまで待機します。
  - **引数**: なし
  - **戻り値**: `Promise<void>` (モジュール初期化完了時に解決)
  - **機能**: 非同期的にWASMモジュールをロードし、準備が整うまで後続処理をブロックします。
- **calculateDuration (demo/src/audioRenderer.ts)**
  - **役割**: MIDIイベントのシーケンスから楽曲の総再生時間を計算します。
  - **引数**: `midiData: { track: { events: { deltaTime: number, type: string, value: number }[] }[] }` (MIDIイベント構造)
  - **戻り値**: `number` (秒単位の楽曲の長さ)
  - **機能**: 各MIDIイベントのデルタタイムと現在のテンポに基づいて、イベント間の経過時間を積算し、最終的な楽曲の長さを秒で返します。
- **renderWaveformAndAudio (demo/src/audioRenderer.ts)**
  - **役割**: MML入力からSMF、YM2151ログを介してオーディオデータを生成し、波形をCanvasに描画します。
  - **引数**: なし
  - **戻り値**: `Promise<void>`
  - **機能**: `mmlConverter`でMMLをSMFに、`smfToYM2151Json`でSMFをYM2151ログに変換後、オーディオを生成し、`visualization.ts`の`drawWaveform`関数を呼び出して波形を視覚化します。
- **check (demo/src/audioRenderer.ts)**
  - **役割**: 特定の条件が満たされているかを確認し、満たされていない場合はUIにエラーメッセージを表示します。
  - **引数**: `condition: boolean` (確認する条件), `message: string` (条件が偽の場合に表示するメッセージ)
  - **戻り値**: `void`
  - **機能**: 条件の真偽を評価し、偽であれば`ui.showStatus`でエラーを通知します。
- **if (demo/src/audioRenderer.ts)**
  - **役割**: 特定の条件に基づいてコードの実行フローを制御します。
  - **引数**: 条件式 (`boolean`)
  - **戻り値**: なし
  - **機能**: `renderWaveformAndAudio`内で、オーディオコンテキストの存在チェック、MIDIデータの検証などに利用されます。
- **for (demo/src/audioRenderer.ts)**
  - **役割**: 配列やイテラブルなオブジェクトの要素を反復処理します。
  - **引数**: 反復処理の対象
  - **戻り値**: なし
  - **機能**: YM2151ログデータの各フレームを処理し、オーディオバッファに書き込む際に使用されます。
- **catch (demo/src/audioRenderer.ts)**
  - **役割**: `try`ブロック内で発生した例外を捕捉し、エラーハンドリングを行います。
  - **引数**: エラーオブジェクト (`e: any`)
  - **戻り値**: なし
  - **機能**: オーディオレンダリング中にエラーが発生した場合、`ui.showStatus`を通じてエラーメッセージを表示し、デモの状態をリセットします。
- **initialize (demo/src/main.ts)**
  - **役割**: デモアプリケーション全体の初期化処理を実行します。
  - **引数**: なし
  - **戻り値**: `Promise<void>`
  - **機能**: Tree-sitterパーサのロード、WebAssemblyモジュールの初期化、UIイベントリスナーの設定、MML入力の初期変換とレンダリングを行います。
- **if (demo/src/main.ts)**
  - **役割**: 特定の条件に基づいてコードの実行フローを制御します。
  - **引数**: 条件式 (`boolean`)
  - **戻り値**: なし
  - **機能**: `initialize`関数内で、パーサやWASMモジュールのロード成功確認などに利用されます。
- **catch (demo/src/main.ts)**
  - **役割**: `try`ブロック内で発生した例外を捕捉し、エラーハンドリングを行います。
  - **引数**: エラーオブジェクト (`e: any`)
  - **戻り値**: なし
  - **機能**: 初期化プロセス中にエラーが発生した場合、`ui.showStatus`でエラーを通知します。
- **constructor (demo/src/midiReader.ts)**
  - **役割**: `MidiReader`クラスの新しいインスタンスを初期化します。
  - **引数**: `data: Uint8Array` (MIDIバイナリデータ)
  - **戻り値**: なし (コンストラクタのため)
  - **機能**: 提供されたMIDIバイナリデータを内部バッファに格納し、データの読み取り位置を初期化します。
- **if (demo/src/midiReader.ts)**
  - **役割**: 特定の条件に基づいてコードの実行フローを制御します。
  - **引数**: 条件式 (`boolean`)
  - **戻り値**: なし
  - **機能**: MIDIファイルヘッダの検証、チャンクタイプの一致チェックなどに利用されます。
- **while (demo/src/midiReader.ts)**
  - **役割**: 指定された条件が真である限り、コードブロックを繰り返し実行します。
  - **引数**: 条件式 (`boolean`)
  - **戻り値**: なし
  - **機能**: MIDIファイルのチャンクやイベントデータが続く限り、それを読み取るために使用されます。
- **convertMML (demo/src/mmlConverter.ts)**
  - **役割**: MML文字列をStandard MIDI File (SMF) のバイナリデータに変換します。
  - **引数**: `mml: string` (MML文字列), `options: MmlToSmfOption` (変換オプション)
  - **戻り値**: `Promise<Uint8Array>` (変換されたSMFバイナリデータ)
  - **機能**: `mmlabc_to_smf_wasm`モジュールの`convert`関数を呼び出し、MMLのパースからSMF生成までの全パスを実行します。
- **if (demo/src/mmlConverter.ts)**
  - **役割**: 特定の条件に基づいてコードの実行フローを制御します。
  - **引数**: 条件式 (`boolean`)
  - **戻り値**: なし
  - **機能**: MML入力の有効性チェック、WASMモジュールのロード状態確認などに利用されます。
- **catch (demo/src/mmlConverter.ts)**
  - **役割**: `try`ブロック内で発生した例外を捕捉し、エラーハンドリングを行います。
  - **引数**: エラーオブジェクト (`e: any`)
  - **戻り値**: なし
  - **機能**: MML変換中にエラーが発生した場合、`ui.showStatus`でエラーメッセージを表示します。
- **parseMidiNotes (demo/src/parseMidiNotes.ts)**
  - **役割**: MIDIバイナリデータから、再生可能な音符イベント（ノートオン/オフ）のリストを抽出します。
  - **引数**: `midiData: Uint8Array` (SMFバイナリデータ)
  - **戻り値**: `Promise<Array<MidiNoteEvent>>` (解析された音符イベントの配列)
  - **機能**: `MidiReader`を使用してMIDIデータを解析し、ノートオン/オフイベントをタイムスタンプ付きの音符イベントに変換します。テンポトラック情報も考慮して正確な時間を計算します。
- **deltaTicksToSeconds (demo/src/parseMidiNotes.ts)**
  - **役割**: MIDIのデルタティック単位を実時間（秒）に変換します。
  - **引数**: `deltaTicks: number` (デルタティック値), `currentTempo: number` (現在のテンポ, µs/四分音符), `ticksPerBeat: number` (四分音符あたりのティック数)
  - **戻り値**: `number` (秒)
  - **機能**: MIDI内部の相対時間表現を絶対的な秒数に変換するためのヘルパー関数です。
- **if (demo/src/parseMidiNotes.ts)**
  - **役割**: 特定の条件に基づいてコードの実行フローを制御します。
  - **引数**: 条件式 (`boolean`)
  - **戻り値**: なし
  - **機能**: MIDIイベントタイプ（ノートオン、ノートオフ、テンポチェンジなど）の識別やデータ有効性の確認に利用されます。
- **for (demo/src/parseMidiNotes.ts)**
  - **役割**: 配列やイテラブルなオブジェクトの要素を反復処理します。
  - **引数**: 反復処理の対象
  - **戻り値**: なし
  - **機能**: MIDIファイル内の各トラックや各イベントを順に処理するために使用されます。
- **while (demo/src/parseMidiNotes.ts)**
  - **役割**: 指定された条件が真である限り、コードブロックを繰り返し実行します。
  - **引数**: 条件式 (`boolean`)
  - **戻り値**: なし
  - **機能**: MIDIの可変長整数（Variable Length Quantity, VLQ）を読み取る際などに使用されます。
- **catch (demo/src/parseMidiNotes.ts)**
  - **役割**: `try`ブロック内で発生した例外を捕捉し、エラーハンドリングを行います。
  - **引数**: エラーオブジェクト (`e: any`)
  - **戻り値**: なし
  - **機能**: MIDI解析中にエラーが発生した場合、コンソールにエラーを出力します。
- **ensureInitialized (demo/src/smfToYm2151.ts)**
  - **役割**: `smf-to-ym2151log-wasm`モジュールがロードされ、初期化されていることを保証します。
  - **引数**: なし
  - **戻り値**: `Promise<void>`
  - **機能**: `smf_to_ym2151log`モジュールを非同期でインポートし、その初期化関数を呼び出します。
- **smfToYM2151Json (demo/src/smfToYm2151.ts)**
  - **役割**: Standard MIDI File (SMF) のバイナリデータをYM2151ログ形式のJSON文字列に変換します。
  - **引数**: `smfData: Uint8Array` (SMFバイナリデータ)
  - **戻り値**: `string` (YM2151ログのJSON文字列)
  - **機能**: ロードされた`smf_to_ym2151log_wasm`モジュールの`smfToYM2151`関数を呼び出し、SMFデータをYM2151のエミュレータが解釈できる形式に変換します。
- **if (demo/src/smfToYm2151.ts)**
  - **役割**: 特定の条件に基づいてコードの実行フローを制御します。
  - **引数**: 条件式 (`boolean`)
  - **戻り値**: なし
  - **機能**: WASMモジュールが正常にロードされているかを確認するのに利用されます。
- **treeToJSON (demo/src/treeToJSON.ts)**
  - **役割**: Tree-sitterの構文木ノードを再帰的に走査し、よりシンプルなJSONオブジェクトに変換します。
  - **引数**: `node: any` (Tree-sitterの構文木ノード)
  - **戻り値**: `object` (JSON形式で表現された構文木)
  - **機能**: ノードのタイプ、テキスト、子ノードなどの情報を抽出し、デバッグ表示などに適した形式で返します。
- **if (demo/src/treeToJSON.ts)**
  - **役割**: 特定の条件に基づいてコードの実行フローを制御します。
  - **引数**: 条件式 (`boolean`)
  - **戻り値**: なし
  - **機能**: 構文木ノードが匿名であるかどうかを判断し、その処理を調整するのに利用されます。
- **for (demo/src/treeToJSON.ts)**
  - **役割**: 配列やイテラブルなオブジェクトの要素を反復処理します。
  - **引数**: 反復処理の対象
  - **戻り値**: なし
  - **機能**: 構文木の各子ノードを再帰的に処理するために使用されます。
- **showStatus (demo/src/ui.ts)**
  - **役割**: ユーザーインターフェース上のステータス表示エリアにメッセージを表示します。
  - **引数**: `message: string` (表示するメッセージ), `type?: string` (メッセージのタイプ、例: "error", "success")
  - **戻り値**: `void`
  - **機能**: ユーザーに現在の状態やエラー、成功などのフィードバックを視覚的に提供します。
- **loadExample (demo/src/ui.ts)**
  - **役割**: 定義済みのMMLのサンプルコードをエディタにロードします。
  - **引数**: なし
  - **戻り値**: `void`
  - **機能**: ユーザーがドロップダウンから選択したMMLのサンプルをテキストエリアに挿入し、`initialize`関数をトリガーして変換・再生を開始します。
- **drawWaveform (demo/src/visualization.ts)**
  - **役割**: オーディオデータから生成された波形をHTML Canvas要素に描画します。
  - **引数**: `audioData: Float32Array` (オーディオサンプルの配列), `canvas: HTMLCanvasElement` (描画対象のCanvas), `color: string` (波形の色)
  - **戻り値**: `void`
  - **機能**: オーディオデータの振幅に基づいて、Canvas上に波形の視覚的な表現を描画します。
- **for (demo/src/visualization.ts)**
  - **役割**: 配列やイテラブルなオブジェクトの要素を反復処理します。
  - **引数**: 反復処理の対象
  - **戻り値**: なし
  - **機能**: オーディオデータの各チャンクを処理し、Canvasに描画するラインの座標を計算するために使用されます。
- **writeString (demo/src/wavExport.ts)**
  - **役割**: `DataView`オブジェクトの指定されたオフセットに文字列を書き込みます。
  - **引数**: `view: DataView` (書き込み先のDataView), `offset: number` (開始オフセット), `s: string` (書き込む文字列)
  - **戻り値**: `void`
  - **機能**: WAVファイルヘッダなどの固定長文字列フィールドをバイナリデータとして正しく書き込むためのユーティリティ関数です。
- **audioBufferToWav (demo/src/wavExport.ts)**
  - **役割**: Web Audio APIの`AudioBuffer`オブジェクトを標準的なWAV形式の`Blob`データに変換します。
  - **引数**: `buffer: AudioBuffer` (変換するAudioBuffer)
  - **戻り値**: `Blob` (WAV形式のバイナリデータ)
  - **機能**: `AudioBuffer`からオーディオデータとメタデータを抽出し、WAVファイルヘッダとデータチャンクを構築して、最終的なWAV`Blob`を生成します。
- **exportWav (demo/src/wavExport.ts)**
  - **役割**: 現在のオーディオバッファの内容をWAVファイルとしてダウンロードさせます。
  - **引数**: なし
  - **戻り値**: `Promise<void>`
  - **機能**: `state`からオーディオデータを取得し、`audioBufferToWav`でWAV `Blob`を生成後、ブラウザのダウンロード機能を通じてファイルとして提供します。
- **for (demo/src/wavExport.ts)**
  - **役割**: 配列やイテラブルなオブジェクトの要素を反復処理します。
  - **引数**: 反復処理の対象
  - **戻り値**: なし
  - **機能**: 複数のオーディオチャンネルデータを結合する際や、WAVデータチャンクを構築する際に使用されます。
- **if (demo/src/wavExport.ts)**
  - **役割**: 特定の条件に基づいてコードの実行フローを制御します。
  - **引数**: 条件式 (`boolean`)
  - **戻り値**: なし
  - **機能**: エクスポートするオーディオデータが存在するか、またチャンネル数に応じた処理の分岐に利用されます。
- **catch (demo/src/wavExport.ts)**
  - **役割**: `try`ブロック内で発生した例外を捕捉し、エラーハンドリングを行います。
  - **引数**: エラーオブジェクト (`e: any`)
  - **戻り値**: なし
  - **機能**: WAVエクスポート中にエラーが発生した場合、`ui.showStatus`でエラーを通知します。
- **mockAudioBuffer (demo/tests/audioBufferToWav.test.ts)**
  - **役割**: `AudioBuffer`オブジェクトのモックを作成し、テストに使用します。
  - **引数**: `numberOfChannels: number` (チャネル数), `length: number` (フレーム数), `sampleRate: number` (サンプルレート), `data?: Float32Array` (オプションのオーディオデータ)
  - **戻り値**: モックされた`AudioBuffer`オブジェクト
  - **機能**: `audioBufferToWav`関数のテストにおいて、依存する`AudioBuffer`の動作をシミュレートするために使われます。
- **for (demo/tests/audioBufferToWav.test.ts)**
  - **役割**: 配列やイテラブルなオブジェクトの要素を反復処理します。
  - **引数**: 反復処理の対象
  - **戻り値**: なし
  - **機能**: モックの`AudioBuffer`にデータを設定する際に使用されます。
- **buildSmf (demo/tests/parseMidiNotes.test.ts)**
  - **役割**: テスト用の簡易なStandard MIDI File (SMF) バイナリデータを作成します。
  - **引数**: `trackEvents: number[][]` (各トラックのMIDIイベントの配列)
  - **戻り値**: `Uint8Array` (生成されたSMFバイナリデータ)
  - **機能**: `parseMidiNotes`関数のテストにおいて、入力となるSMFデータをプログラムで生成するために使用されます。
- **for (demo/tests/parseMidiNotes.test.ts)**
  - **役割**: 配列やイテラブルなオブジェクトの要素を反復処理します。
  - **引数**: 反復処理の対象
  - **戻り値**: なし
  - **機能**: SMFのヘッダーやトラックチャンクを構築する際に使用されます。
- **while (demo/tests/parseMidiNotes.test.ts)**
  - **役割**: 指定された条件が真である限り、コードブロックを繰り返し実行します。
  - **引数**: 条件式 (`boolean`)
  - **戻り値**: なし
  - **機能**: 可変長整数（VLQ）のエンコード処理に使用されます。
- **mockNode (demo/tests/treeToJSON.test.ts)**
  - **役割**: Tree-sitterノードのモックオブジェクトを作成し、テストに使用します。
  - **引数**: `type: string` (ノードのタイプ), `text: string` (ノードのテキスト), `children: any[]` (子ノードの配列)
  - **戻り値**: モックされたTree-sitterノードオブジェクト
  - **機能**: `treeToJSON`関数のテストにおいて、入力となるTree-sitterノードの動作をシミュレートするために使われます。

## 関数呼び出し階層ツリー
```
initialize (demo/src/main.ts)
├── convertMML (demo/src/mmlConverter.ts)
│   ├── (WASMモジュールの内部変換ロジックを呼び出し)
│   └── if (demo/src/mmlConverter.ts)
│   └── catch (demo/src/mmlConverter.ts)
├── loadExample (demo/src/ui.ts)
├── if (demo/src/main.ts)
└── catch (demo/src/main.ts)

playAudio (demo/src/audioPlayback.ts)
├── stopAudio (demo/src/audioPlayback.ts)
│   └── showStatus (demo/src/ui.ts)
├── if (demo/src/audioPlayback.ts)
└── catch (demo/src/audioPlayback.ts)

waitForWebYm2151 (demo/src/audioRenderer.ts)
├── calculateDuration (demo/src/audioRenderer.ts)
├── renderWaveformAndAudio (demo/src/audioRenderer.ts)
│   ├── drawWaveform (demo/src/visualization.ts)
│   ├── if (demo/src/audioRenderer.ts)
│   ├── for (demo/src/audioRenderer.ts)
│   └── catch (demo/src/audioRenderer.ts)
├── check (demo/src/audioRenderer.ts)
├── if (demo/src/audioRenderer.ts)
├── for (demo/src/audioRenderer.ts)
└── catch (demo/src/audioRenderer.ts)

constructor (demo/src/midiReader.ts)
├── if (demo/src/midiReader.ts)
└── while (demo/src/midiReader.ts)

parseMidiNotes (demo/src/parseMidiNotes.ts)
├── deltaTicksToSeconds (demo/src/parseMidiNotes.ts)
├── if (demo/src/parseMidiNotes.ts)
├── for (demo/src/parseMidiNotes.ts)
├── while (demo/src/parseMidiNotes.ts)
└── catch (demo/src/parseMidiNotes.ts)

ensureInitialized (demo/src/smfToYm2151.ts)

smfToYM2151Json (demo/src/smfToYm2151.ts)
├── if (demo/src/smfToYm2151.ts)

treeToJSON (demo/src/treeToJSON.ts)
├── if (demo/src/treeToJSON.ts)
└── for (demo/src/treeToJSON.ts)

showStatus (demo/src/ui.ts)

loadExample (demo/src/ui.ts)

drawWaveform (demo/src/visualization.ts)
└── for (demo/src/visualization.ts)

writeString (demo/src/wavExport.ts)

audioBufferToWav (demo/src/wavExport.ts)
├── for (demo/src/wavExport.ts)
└── if (demo/src/wavExport.ts)

exportWav (demo/src/wavExport.ts)
├── audioBufferToWav (demo/src/wavExport.ts)
│   ├── for (demo/src/wavExport.ts)
│   └── if (demo/src/wavExport.ts)
├── for (demo/src/wavExport.ts)
├── if (demo/src/wavExport.ts)
└── catch (demo/src/wavExport.ts)

mockAudioBuffer (demo/tests/audioBufferToWav.test.ts)
└── for (demo/tests/audioBufferToWav.test.ts)

buildSmf (demo/tests/parseMidiNotes.test.ts)
├── for (demo/tests/parseMidiNotes.test.ts)
└── while (demo/tests/parseMidiNotes.test.ts)

mockNode (demo/tests/treeToJSON.test.ts)

---
Generated at: 2026-04-19 07:11:23 JST
