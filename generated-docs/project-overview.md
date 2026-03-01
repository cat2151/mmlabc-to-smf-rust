Last updated: 2026-03-02

# Project Overview

## プロジェクト概要
-   このプロジェクトは、Music Macro Language（MML）形式の楽譜データをStandard MIDI File（SMF）に変換するRust製のライブラリです。
-   MML解析からSMF生成までを4段階のアーキテクチャで処理し、ネイティブアプリケーションおよびWebAssembly（WASM）対応のブラウザアプリケーションへの組み込みが可能です。
-   多チャンネル対応、CLI、JSONデバッグ出力、包括的なテストを備え、音楽プログラミングの基盤として利用されます。

## 技術スタック
使用している技術をカテゴリ別に整理して説明します。

-   フロントエンド:
    -   **TypeScript**: Webデモのロジック実装に使用されるプログラミング言語。
    -   **HTML**: Webデモのユーザーインターフェース構造を定義するために使用されるマークアップ言語。
    -   **Tone.js**: 高度なオーディオ処理とWeb MIDI機能を提供するJavaScriptライブラリ。Webデモでのオーディオ再生に利用されます（ただし、現状はデモ内での自前実装が行われています）。
    -   **WebAssembly (WASM)**: Rustで書かれたコアロジックをブラウザで実行可能にするためのバイナリフォーマットおよび技術。
    -   **web-tree-sitter**: tree-sitterパーサーをWebAssemblyとしてブラウザで実行するためのライブラリ。
-   音楽・オーディオ:
    -   **Music Macro Language (MML)**: 楽譜情報をテキスト形式で記述するためのマクロ言語。このプロジェクトの入力形式です。
    -   **Standard MIDI File (SMF)**: 音楽情報をデジタル形式で保存・交換するための標準ファイルフォーマット。このプロジェクトの出力形式です。
    -   **MIDI**: 楽器間のデジタル通信プロトコル。
-   開発ツール:
    -   **Rust**: 安全性とパフォーマンスに優れたシステムプログラミング言語。
    -   **Cargo**: Rustの公式パッケージマネージャー兼ビルドシステム。依存関係管理、ビルド、テスト、ドキュメント生成などを行います。
    -   **tree-sitter**: 構文解析のためのインクリメンタルパーサー生成ツールおよびライブラリ。MMLの構文解析に利用されます。
    -   **Node.js/npm**: JavaScriptのランタイム環境およびパッケージマネージャー。tree-sitterパーサーの生成に使用されます。
    -   **Cコンパイラ (例: GCC/Clang)**: tree-sitterによって生成されたC言語のパーサーファイルをコンパイルするために必要です。
-   テスト:
    -   **Cargo test**: Rustプロジェクトのユニットテストおよび統合テストを実行するためのコマンド。
-   ビルドツール:
    -   **Cargo**: Rustプロジェクトのビルドプロセスを管理します。
    -   **Shell Script**: `scripts/` ディレクトリに、デモのビルドやパス変換のためのシェルスクリプトが含まれています。
-   言語機能:
    -   **Rustの型システムと所有権モデル**: メモリ安全性を確保し、堅牢なコードを記述するためのRustの主要な機能。
-   自動化・CI/CD:
    -   **継続的インテグレーション/デプロイメント**: コードの変更を自動的にテストし、デプロイするプロセス（具体的なツールは明記されていませんが、リポジトリ設定整備の短期目標に示唆されています）。
-   開発標準:
    -   **Cargo fmt**: Rustコードの自動フォーマットツール。
    -   **Cargo clippy**: Rustコードのリンター。潜在的なバグや非効率なコードパターンを検出します。

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
├── googled947dc864c270e07.html
├── issue-notes/
│   ├── 39.md
│   ├── 44.md
│   ├── 89.md
│   └── 97.md
├── mmlabc-to-smf-rust.toml.example
├── mmlabc-to-smf-wasm/
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
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
-   `.editorconfig`: 開発者間でコードのフォーマットスタイル（インデント、改行コードなど）を統一するための設定ファイル。
-   `.gitignore`: Gitのバージョン管理から除外するファイルやディレクトリを指定する設定ファイル。
-   `.vscode/settings.json`: Visual Studio Codeエディタのワークスペース固有の設定ファイル。開発環境の統一に貢献します。
-   `Cargo.lock`: Rustプロジェクトの依存関係の正確なバージョンを記録し、再現可能なビルドを保証するファイル。
-   `Cargo.toml`: Rustプロジェクトのメタデータ、依存関係、およびビルド設定を定義するマニフェストファイル。
-   `IMPLEMENTATION_REPORT.md`: プロジェクトの実装に関する詳細な報告や記録が記述されたMarkdownファイル。
-   `LICENSE`: プロジェクトの配布および使用条件を示すMITライセンスが記述されたファイル。
-   `OPTION_A_IMPLEMENTATION.md`: 開発中に検討された特定の設計選択肢（オプションA）の実装内容について記述されたMarkdownファイル。
-   `README.ja.md`: プロジェクトの日本語版説明書。プロジェクトの目的、機能、使用方法、開発状況などが網羅されています。
-   `README.md`: プロジェクトの英語版説明書。`README.ja.md` と同等の内容が英語で記述されています。
-   `_config.yml`: 主にJekyllなどの静的サイトジェネレータで使用される設定ファイル。ドキュメントサイトの生成などに関連します。
-   `build.rs`: カスタムビルドロジックを定義するRustスクリプト。tree-sitterパーサーのC言語ソースファイルの生成やコンパイルを自動化します。
-   `demo/`: Webブラウザ上でMMLからSMFへの変換機能を試せるデモアプリケーションのソースコードを格納するディレクトリ。
    -   `demo/.gitignore`: デモアプリケーションに特化したGit無視設定。
    -   `demo/FEATURES.md`: デモアプリケーションが提供する機能の詳細を記述するMarkdownファイル。
    -   `demo/README.md`: デモアプリケーション固有のREADMEファイル。
    -   `demo/index.html`: デモアプリケーションのメインとなるHTMLファイル。ユーザーインターフェースを定義します。
    -   `demo/package.json`: デモアプリケーションのJavaScript依存関係とスクリプトを管理するファイル。
    -   `demo/src/audioPlayback.ts`: Webデモにおけるオーディオ再生のロジックを実装するTypeScriptファイル。Tone.jsライブラリを介して音を鳴らします。
    -   `demo/src/audioRenderer.ts`: MIDIデータをオーディオ形式にレンダリングし、波形として視覚化する処理を行うTypeScriptファイル。
    -   `demo/src/main.ts`: デモアプリケーションの主要なエントリーポイント。WebAssemblyモジュールの初期化やUIイベントのハンドリングを設定します。
    -   `demo/src/midiReader.ts`: MIDIファイルのバイトデータを解析し、構造化されたMIDIイベントとして読み込むためのTypeScriptロジック。
    -   `demo/src/mmlConverter.ts`: MML文字列をWebAssembly経由でRustライブラリに渡し、SMFデータに変換するラッパー機能を提供するTypeScriptファイル。
    -   `demo/src/parseMidiNotes.ts`: 解析されたMIDIデータから具体的な音符イベント（ノートオン/オフ、時間など）を抽出するTypeScriptロジック。
    -   `demo/src/smfToYm2151.ts`: Standard MIDI FileのデータをヤマハYM2151音源用のログ形式JSONに変換する（デモ用途の）TypeScriptファイル。
    -   `demo/src/state.ts`: デモアプリケーションのグローバルな状態（MML入力、変換結果など）を管理するTypeScriptファイル。
    -   `demo/src/ui.ts`: デモアプリケーションのユーザーインターフェース要素の操作や表示更新を行うTypeScriptファイル。
    -   `demo/src/visualization.ts`: 生成された音楽の波形やMIDIイベントを視覚的に表示するロジックを実装するTypeScriptファイル。
    -   `demo/src/wavExport.ts`: 生成されたオーディオデータをWAVファイル形式でエクスポートし、ダウンロード可能にするTypeScript機能。
-   `demo-library/`: 別のデモ、またはMML変換ライブラリの利用例を示すためのディレクトリ。
    -   `demo-library/index.html`: デモライブラリのメインHTMLファイル。
    -   `demo-library/package.json`: デモライブラリのJavaScript依存関係とスクリプトを定義するファイル。
-   `generated-docs/`: プロジェクトのドキュメントが自動生成されて格納されるディレクトリ。
-   `googled947dc864c270e07.html`: Google Search Consoleのサイト所有権確認ファイル。
-   `issue-notes/`: 開発中の課題やイシューに関する詳細なメモ、議論、進捗を記録するためのMarkdownファイル群。
    -   `issue-notes/39.md`, `issue-notes/44.md`, `issue-notes/89.md`, `issue-notes/97.md`: 個々のイシューに対応するメモファイル。
-   `mmlabc-to-smf-rust.toml.example`: カスタムMIDIプレイヤーを設定するためのサンプルTOMLファイル。
-   `mmlabc-to-smf-wasm/`: Rust製の `mmlabc-to-smf` コアライブラリをWebAssemblyにコンパイルするためのクレート。
    -   `mmlabc-to-smf-wasm/Cargo.lock`: WASMクレートの依存関係ロックファイル。
    -   `mmlabc-to-smf-wasm/Cargo.toml`: WASMクレートのメタデータと依存関係を定義するマニフェストファイル。
    -   `mmlabc-to-smf-wasm/src/lib.rs`: WASMクレートのルートライブラリファイル。Rust関数をJavaScriptから呼び出せるように公開します。
-   `package.json`: プロジェクト全体のJavaScript/Node.js依存関係を管理するファイル。特にtree-sitter-cliの依存関係を記述します。
-   `scripts/`: ビルドプロセスやデモの管理に役立つシェルスクリプト群を格納するディレクトリ。
    -   `scripts/README.md`: スクリプトディレクトリの内容説明。
    -   `scripts/build-demo.sh`: デモアプリケーションをビルドするためのシェルスクリプト。
    -   `scripts/transform-demo-paths.sh`: デモ内のファイルパスを処理するためのシェルスクリプト。
-   `src/`: メインのRustライブラリのソースコードを格納するディレクトリ。MMLからSMFへの変換ロジックが実装されています。
    -   `src/config.rs`: アプリケーションの構成設定（外部MIDIプレイヤーパスなど）を扱うRustモジュール。
    -   `src/lib.rs`: Rustライブラリのルートモジュール。主要なMML-to-SMF変換関数を外部に公開します。
    -   `src/main.rs`: CLI（コマンドラインインターフェース）アプリケーションのエントリーポイント。ユーザー入力を受け取り、変換処理を調整します。
    -   `src/pass1_parser.rs`: 変換プロセスの最初のパス。MML文字列をtree-sitterを使用してトークンに解析します。
    -   `src/pass2_ast.rs`: 変換プロセスの第2パス。パス1で生成されたトークン列を抽象構文木（AST）に変換します。
    -   `src/pass3_events.rs`: 変換プロセスの第3パス。ASTからMIDIイベント（ノートオン、オフ、テンポなど）のリストを生成します。
    -   `src/pass4_midi.rs`: 変換プロセスの第4パス。パス3で生成されたMIDIイベントリストから最終的なStandard MIDI Fileを構築します。
    -   `src/tree_sitter_mml.rs`: tree-sitter MMLパーサーをRustプロジェクトに統合するためのラッパーやユーティリティ関数を提供します。
    -   `src/types.rs`: プロジェクト全体で共有されるカスタムデータ構造や列挙型（MMLトークン、ASTノード、MIDIイベントなど）を定義するモジュール。
-   `tests/`: Rustライブラリのテストコードを格納するディレクトリ。機能ごとに詳細なテストが書かれています。
    -   `tests/integration_test.rs`: ライブラリ全体の統合テストを実行します。
    -   `tests/test_channel.rs`: MMLの多チャンネル処理に関するテスト。
    -   `tests/test_chord.rs`: MMLの和音記法に関するテスト。
    -   `tests/test_cli.rs`: コマンドラインインターフェースの動作に関するテスト。
    -   `tests/test_config.rs`: 設定ファイルの読み込みと適用に関するテスト。
    -   `tests/test_dotted_notes.rs`: 付点音符のMML記法変換に関するテスト。
    -   `tests/test_drum_channel.rs`: ドラムチャンネル（MIDIチャンネル10）のMML処理に関するテスト。
    -   `tests/test_key_transpose.rs`: キーの移調（音高変更）に関するテスト。
    -   `tests/test_length.rs`: 音符や休符の長さ指定に関するテスト。
    -   `tests/test_modifier.rs`: 音符の修飾子（例: シャープ、フラット）に関するテスト。
    -   `tests/test_note_length.rs`: 音符の実際の長さ（持続時間）計算に関するテスト。
    -   `tests/test_octave.rs`: オクターブ変更コマンドに関するテスト。
    -   `tests/test_pass1.rs`: パス1（MMLトークン解析）のユニットテスト。
    -   `tests/test_pass2.rs`: パス2（AST変換）のユニットテスト。
    -   `tests/test_pass3.rs`: パス3（MIDIイベント生成）のユニットテスト。
    -   `tests/test_pass4.rs`: パス4（Standard MIDI File作成）のユニットテスト。
    -   `tests/test_program_change.rs`: プログラムチェンジ（音色変更）コマンドに関するテスト。
    -   `tests/test_rest.rs`: 休符のMML記法変換に関するテスト。
    -   `tests/test_tempo.rs`: テンポ変更コマンドに関するテスト。
    -   `tests/test_velocity.rs`: ベロシティ（音の強さ）変更コマンドに関するテスト。
-   `tree-sitter-mml/`: Music Macro Language用のtree-sitterパーサーの定義と生成されたファイルを格納するディレクトリ。
    -   `tree-sitter-mml/grammar.js`: MMLの構文ルールを定義するJavaScriptファイル。tree-sitterツールによってC言語パーサーが生成されます。
    -   `tree-sitter-mml/package.json`: `tree-sitter-mml` ディレクトリのJavaScript依存関係を管理するファイル。
    -   `tree-sitter-mml/src/grammar.json`: `grammar.js` から自動生成されるMML文法のJSON表現。
    -   `tree-sitter-mml/src/node-types.json`: `grammar.js` から自動生成される抽象構文木（AST）のノード型定義。
    -   `tree-sitter-mml/src/parser.c`: `grammar.js` から自動生成されるMMLパーサーのC言語ソースコード。
    -   `tree-sitter-mml/src/tree_sitter/`: tree-sitterランタイムのコアヘッダーファイルが含まれるディレクトリ。
        -   `tree-sitter-mml/src/tree_sitter/alloc.h`: メモリ割り当て関連のヘッダー。
        -   `tree-sitter-mml/src/tree_sitter/array.h`: 動的配列の実装関連のヘッダー。
        -   `tree-sitter-mml/src/tree_sitter/parser.h`: tree-sitterパーサーのAPI定義に関するヘッダー。
    -   `tree-sitter-mml/tree-sitter-mml.wasm`: `tree-sitter-mml` パーサーをWebAssembly形式にコンパイルしたバイナリファイル。

## 関数詳細説明
-   `playAudio()`:
    -   役割: Webデモにおいて、生成されたMIDIデータまたはオーディオデータを再生します。
    -   引数: なし (内部のグローバル状態やUI要素から再生対象を取得)。
    -   戻り値: なし (サイドエフェクトとしてオーディオ再生を開始)。
    -   機能: ユーザーがMML変換結果をブラウザ上で試聴できるようにします。
-   `stopAudio()`:
    -   役割: Webデモにおいて、現在再生中のオーディオを停止します。
    -   引数: なし。
    -   戻り値: なし (サイドエフェクトとしてオーディオ再生を停止)。
    -   機能: オーディオ再生を中断し、関連するUI状態をリセットします。
-   `midiToAudio()`:
    -   役割: Webデモにおいて、MIDIデータを入力としてオーディオデータに変換します。
    -   引数: MIDIデータ (具体的な型はデモ内部で定義されたMIDIイベントリストやバイト配列)。
    -   戻り値: オーディオデータ (例: `AudioBuffer` オブジェクト)。
    -   機能: 視覚化やWAVエクスポートのために、MIDIの楽譜情報をブラウザで扱えるオーディオ信号に変換します。
-   `renderWaveformAndAudio()`:
    -   役割: Webデモにおいて、生成されたオーディオデータに基づいて波形をレンダリングし、再生可能なオーディオを準備します。
    -   引数: オーディオデータ (通常は `AudioBuffer`)。
    -   戻り値: なし (サイドエフェクトとしてCanvasへの描画とオーディオコンテキストの準備)。
    -   機能: ユーザーにオーディオの視覚的フィードバックを提供し、再生を可能にします。
-   `initialize()`:
    -   役割: Webデモアプリケーションの初期化処理を行います。
    -   引数: なし。
    -   戻り値: `Promise<void>` (非同期処理の完了を待機)。
    -   機能: WebAssemblyモジュールの読み込み、UI要素の初期設定、イベントリスナーの登録など、アプリケーションが動作するための初期準備を実行します。
-   `constructor()` (in `midiReader.ts`):
    -   役割: MIDIファイルデータを解析するための `MidiReader` クラスのインスタンスを生成します。
    -   引数: MIDIファイルのバイトデータ。
    -   戻り値: なし (オブジェクトの初期化)。
    -   機能: 提供されたMIDIデータを内部で処理可能な形式に設定し、解析準備を行います。
-   `treeToJSON()`:
    -   役割: `tree-sitter` で解析されたMMLの構文木（AST）をJSON形式の文字列に変換します。
    -   引数: `tree-sitter` の `Tree` オブジェクト。
    -   戻り値: JSON形式の文字列。
    -   機能: デバッグや中間結果の可視化のために、構造化された構文木を人間が読める形式に変換します。
-   `convertMML()`:
    -   役割: MML文字列をStandard MIDI Fileのバイトデータに変換します。WebAssemblyモジュールへのラッパー関数です。
    -   引数: MML形式の文字列。
    -   戻り値: Standard MIDI Fileのバイトデータ (`Uint8Array`)。
    -   機能: MML文字列を受け取り、バックエンドのRustライブラリ（WASM経由）を呼び出してMIDIデータを生成します。
-   `parseMidiNotes()`:
    -   役割: 解析されたMIDIデータから、個々の音符イベント（ノートオン、ノートオフ、持続時間、ベロシティなど）を抽出します。
    -   引数: MIDIデータ (解析済みのイベントリストや `MidiReader` のインスタンスから得られる情報)。
    -   戻り値: 音符イベントの構造化されたリスト。
    -   機能: MIDIファイルの内容から、音楽の演奏情報を詳細に取得し、視覚化やオーディオ生成に利用できる形式に整理します。
-   `deltaTicksToSeconds()`:
    -   役割: MIDIの内部時間単位であるデルタティック（delta ticks）を、実際の秒数に変換します。
    -   引数: `deltaTicks` (数値), `tempo` (現在のテンポ、BPM), `ticksPerBeat` (1拍あたりのティック数)。
    -   戻り値: 秒数 (数値)。
    -   機能: MIDIイベント間の相対時間から、実際の音楽時間における絶対的な間隔を計算します。
-   `smfToYM2151Json()`:
    -   役割: Standard MIDI Fileのデータを、ヤマハYM2151音源のログ形式のJSONに変換します。
    -   引数: MIDIファイルのバイトデータ。
    -   戻り値: YM2151ログ形式のJSON文字列。
    -   機能: SMFデータを特定の音源エミュレータ（YM2151）で利用するための、より低レベルなイベント列に変換します。
-   `showStatus()`:
    -   役割: Webデモのユーザーインターフェース上に、処理状況やエラーに関するメッセージを表示します。
    -   引数: `message` (表示する文字列), `type` (メッセージの種類、例: 'success', 'error', 'info')。
    -   戻り値: なし (サイドエフェクトとしてUI要素の更新)。
    -   機能: ユーザーに現在の操作のフィードバックや、発生した問題について通知します。
-   `loadExample()`:
    -   役割: WebデモのMML入力エリアに、あらかじめ用意されたMMLのサンプルコードを読み込みます。
    -   引数: `exampleId` (読み込むサンプルの識別子)。
    -   戻り値: なし (サイドエフェクトとしてUI入力フィールドの更新)。
    -   機能: ユーザーがMML変換機能を簡単に試せるように、手軽なサンプルを提供します。
-   `drawWaveform()`:
    -   役割: Webデモにおいて、オーディオデータから生成された波形をHTML Canvas要素に描画します。
    -   引数: `audioBuffer` (描画するオーディオデータを含む `AudioBuffer` オブジェクト), `context` (Canvasの描画コンテキスト `CanvasRenderingContext2D`)。
    -   戻り値: なし (サイドエフェクトとしてCanvasへの描画)。
    -   機能: オーディオデータを視覚的に表現し、ユーザーに音楽の構造を伝えます。
-   `visualizeRealtime()`:
    -   役割: リアルタイムで生成または再生されているオーディオの波形を視覚化します（デモでは未実装または準備中の機能）。
    -   引数: なし (またはWeb Audio APIからの直接のオーディオデータ)。
    -   戻り値: なし。
    -   機能: 音楽再生中の動的な波形表示を可能にすることを目指します。
-   `draw()`:
    -   役割: 視覚化処理のメインループ、またはフレームごとの描画更新を行います。
    -   引数: なし (内部でCanvasコンテキストやアニメーションフレームの情報を管理)。
    -   戻り値: なし (サイドエフェクトとしてCanvasの描画更新)。
    -   機能: アニメーションループを通じて、視覚化要素を連続的に更新・描画します。
-   `writeString()`:
    -   役割: WAVファイルのエクスポート処理において、指定されたバイトオフセットに文字列データをバイナリ形式で書き込みます。
    -   引数: `view` (`DataView` オブジェクト), `offset` (書き込み開始オフセット), `string` (書き込む文字列)。
    -   戻り値: なし (サイドエフェクトとして `DataView` の内容を変更)。
    -   機能: WAVファイルヘッダやチャンク情報など、固定文字列をバイナリデータに組み込むためのヘルパー関数です。
-   `audioBufferToWav()`:
    -   役割: Web Audio APIの `AudioBuffer` オブジェクトを、標準的なWAVファイル形式の `ArrayBuffer` に変換します。
    -   引数: `audioBuffer` (変換する `AudioBuffer` オブジェクト)。
    -   戻り値: WAVファイル形式のバイナリデータを含む `ArrayBuffer`。
    -   機能: ブラウザのメモリ内で生成されたオーディオデータを、一般的な音声ファイル形式にパッケージ化します。
-   `exportWav()`:
    -   役割: 現在生成されているオーディオデータをWAVファイルとしてユーザーのデバイスにダウンロードさせます。
    -   引数: なし (または内部でオーディオデータへの参照を持つ)。
    -   戻り値: `Promise<void>`。
    -   機能: ユーザーが生成された音楽をWAVファイルとして保存できるように、ダウンロードリンクを生成しトリガーします。

## 関数呼び出し階層ツリー
```
main.rs (CLIエントリーポイント)
└── (main関数)
    └── lib.rs (ライブラリのコア機能)
        └── (MML-to-SMF変換処理)
            ├── pass1_parser.rs (MMLトークン解析)
            ├── pass2_ast.rs (AST変換)
            ├── pass3_events.rs (MIDIイベント生成)
            └── pass4_midi.rs (SMFファイル作成)
            └── tree_sitter_mml.rs (tree-sitterパーサーとの連携)
            └── config.rs (設定読み込み)
            └── types.rs (データ構造定義)

mmlabc-to-smf-wasm/src/lib.rs (WASMラッパー)
└── (wasm_bindgenで公開される関数)
    └── lib.rs (上記Rustライブラリのコア機能)

demo/src/main.ts (Webデモのエントリーポイント)
├── initialize()
│   ├── mmlConverter.ts::convertMML() (WASM経由でRustコアライブラリを呼び出し)
│   │   ├── mmlConverter.ts::treeToJSON()
│   │   └── ui.ts::showStatus()
│   ├── audioPlayback.ts::playAudio() (イベントハンドラとして)
│   │   ├── audioPlayback.ts::stopAudio()
│   │   ├── ui.ts::showStatus()
│   │   └── visualization.ts::visualizeRealtime()
│   │       └── visualization.ts::draw()
│   ├── audioRenderer.ts::midiToAudio() (イベントハンドラとして)
│   │   ├── parseMidiNotes.ts::parseMidiNotes()
│   │   │   ├── midiReader.ts::constructor()
│   │   │   └── parseMidiNotes.ts::deltaTicksToSeconds()
│   │   └── visualization.ts::drawWaveform()
│   ├── smfToYm2151.ts::smfToYM2151Json() (イベントハンドラとして)
│   │   ├── midiReader.ts::constructor()
│   │   └── smfToYm2151.ts::deltaTicksToSeconds()
│   ├── wavExport.ts::exportWav() (イベントハンドラとして)
│   │   ├── wavExport.ts::audioBufferToWav()
│   │   │   └── wavExport.ts::writeString()
│   │   └── ui.ts::showStatus()
│   └── ui.ts::loadExample() (イベントハンドラとして)

---
Generated at: 2026-03-02 07:06:52 JST
