Last updated: 2026-02-05

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) 形式の文字列を、Standard MIDI File (SMF) へ変換するRustライブラリです。
- MML入力は4つの主要パス（トークン化、AST変換、MIDIイベント生成、SMF作成）を経て、多チャンネルのMIDIファイルとして出力されます。
- ネイティブアプリケーションだけでなく、WebAssemblyを通じてブラウザアプリケーションからも利用可能な音楽生成の基盤を提供します。

## 技術スタック
- フロントエンド: WASMライブラリとしてブラウザデモに利用される予定（直接的なフロントエンドフレームワークは記述なし）。
- 音楽・オーディオ: Music Macro Language (MML) 処理、Standard MIDI File (SMF) 生成、外部MIDIプレイヤー連携 (`cat-play-mml` (デフォルト), `timidity`, `fluidsynth`, `vlc`)。
- 開発ツール: Rust (プログラミング言語), Cargo (Rustのビルドシステム・パッケージマネージャー), tree-sitter (MML構文解析用パーサー生成), Node.js (tree-sitterパーサー生成環境), npx (Node.jsパッケージ実行ツール)。
- テスト: Rust標準のテストフレームワーク (`cargo test`)。35個の包括的テストケースが実装されています。
- ビルドツール: Cargo (Rustプロジェクトのビルド), tree-sitter-cli (MMLパーサーのC言語ファイル生成)。
- 言語機能: Rustの強力な型システムと所有権モデルによるメモリ安全性と安全な設計。
- 自動化・CI/CD: 明示的なCI/CDツールは記述されていませんが、`cargo fmt`や`cargo clippy`は自動化されたコード品質チェックに利用されます。
- 開発標準: `cargo clippy` (Linterによるコード品質チェック), `cargo fmt` (Formatterによるコードフォーマットの統一)。

## ファイル階層ツリー
```
.
├── Cargo.toml                              # プロジェクトの依存関係とメタデータ
├── Cargo.lock                              # 依存関係の正確なバージョンを記録
├── LICENSE                                 # プロジェクトのライセンス情報 (MIT License)
├── README.md                               # プロジェクトの概要と使い方 (英語)
├── README.ja.md                            # プロジェクトの概要と使い方 (日本語)
├── build.rs                                # ビルドスクリプト (tree-sitterパーサーの生成・コンパイル用)
├── mmlabc-to-smf-rust.toml.example         # カスタムMIDIプレイヤー設定の例
├── package.json                            # Node.jsパッケージの依存関係 (tree-sitter関連)
├── src/                                    # メインライブラリとCLIアプリケーションのソースコード
│   ├── main.rs                             # CLIアプリケーションのエントリーポイント
│   ├── lib.rs                              # ライブラリクレートのルートファイル
│   ├── config.rs                           # アプリケーション設定を管理するモジュール
│   ├── pass1_parser.rs                     # パス1: MML文字列のトークン化ロジック
│   ├── pass2_ast.rs                        # パス2: トークンから抽象構文木(AST)への変換ロジック
│   ├── pass3_events.rs                     # パス3: ASTからMIDIイベント生成ロジック
│   ├── pass4_midi.rs                       # パス4: MIDIイベントからStandard MIDI File作成ロジック
│   ├── tree_sitter_mml.rs                  # tree-sitter MMLパーサーとの統合モジュール
│   └── types.rs                            # プロジェクト全体で共有される共通データ型定義
├── tests/                                  # ユニットテストおよび統合テストコード
│   ├── integration_test.rs                 # 統合テスト
│   ├── test_channel.rs                     # チャンネル機能のテスト
│   ├── test_chord.rs                       # 和音機能のテスト
│   ├── test_cli.rs                         # コマンドラインインターフェースのテスト
│   ├── test_config.rs                      # 設定ファイルのテスト
│   ├── test_dotted_notes.rs                # 付点音符のテスト
│   ├── test_drum_channel.rs                # ドラムチャンネルのテスト
│   ├── test_key_transpose.rs               # キー移調のテスト
│   ├── test_length.rs                      # 音長指定のテスト
│   ├── test_modifier.rs                    # 修飾子（例: 音量）のテスト
│   ├── test_note_length.rs                 # 音符長に関するテスト
│   ├── test_octave.rs                      # オクターブ機能のテスト
│   ├── test_pass1.rs                       # パス1（トークン解析）のテスト
│   ├── test_pass2.rs                       # パス2（AST変換）のテスト
│   ├── test_pass3.rs                       # パス3（MIDIイベント生成）のテスト
│   ├── test_pass4.rs                       # パス4（MIDIファイル作成）のテスト
│   ├── test_program_change.rs              # プログラムチェンジのテスト
│   ├── test_rest.rs                        # 休符のテスト
│   ├── test_tempo.rs                       # テンポ指定のテスト
│   └── test_velocity.rs                    # ベロシティ（音量）のテスト
├── tree-sitter-mml/                        # tree-sitter MMLパーサーの定義と生成ファイル
│   ├── grammar.js                          # tree-sitter MML文法の定義ファイル (JavaScript形式)
│   ├── package.json                        # tree-sitterパーサーのNode.js依存関係
│   ├── src/                                # 生成されたパーサー関連ファイル
│   │   ├── grammar.json                    # 生成された文法定義 (JSON形式)
│   │   ├── node-types.json                 # 生成されたASTノードタイプ定義 (JSON形式)
│   │   ├── parser.c                        # 生成されたC言語パーサーソース
│   │   └── tree_sitter/                    # tree-sitterランタイムのヘッダファイル群
│   │       ├── alloc.h
│   │       ├── array.h
│   │       └── parser.h
│   └── tree-sitter-mml.wasm                # WebAssembly形式のMMLパーサーバイナリ
└── mmlabc-to-smf-wasm/                     # WebAssembly向けライブラリのサブクレート
    ├── Cargo.lock
    ├── Cargo.toml
    └── src/
        └── lib.rs                          # WASMライブラリのルートファイル
```

## ファイル詳細説明
- `Cargo.toml`: Rustプロジェクトのビルド設定、依存クレート、およびメタデータを定義するファイルです。
- `Cargo.lock`: `Cargo.toml`に基づいてビルドされた実際の依存クレートのバージョンを記録し、再現性のあるビルドを保証します。
- `LICENSE`: プロジェクトのライセンス情報（MIT License）が記載されています。
- `README.md`: プロジェクトの概要、機能、使い方、開発方法などを説明する英語の主要なドキュメントです。
- `README.ja.md`: プロジェクトの概要、機能、使い方、開発方法などを説明する日本語のドキュメントです。
- `build.rs`: Rustのビルドスクリプトで、特に`tree-sitter`パーサーのC言語ファイルを`grammar.js`から自動生成・コンパイルするために使用されます。
- `mmlabc-to-smf-rust.toml.example`: カスタムMIDIプレイヤー設定の例を示すTOML形式の設定ファイルです。
- `package.json`: Node.jsプロジェクトの依存関係とスクリプトを定義するファイルで、特に`tree-sitter-cli`の管理に使用されます。
- `src/main.rs`: コマンドラインインターフェース (CLI) アプリケーションのエントリーポイントです。コマンドライン引数の解析やMML変換処理のオーケストレーションを行います。
- `src/lib.rs`: `mmlabc-to-smf-rust`ライブラリクレートのルートファイルで、MMLからSMFへの変換ロジックの主要なインターフェースを提供します。
- `src/config.rs`: アプリケーションの動作設定（例：外部MIDIプレイヤー）を読み込み・管理するモジュールです。
- `src/pass1_parser.rs`: MML文字列を`tree-sitter`を使用して解析し、トークンストリームに変換する第一パスのロジックを含みます。
- `src/pass2_ast.rs`: パス1で生成されたトークンストリームから抽象構文木 (AST) を構築する第二パスのロジックを含みます。
- `src/pass3_events.rs`: パス2で生成されたASTを走査し、MIDIイベントのシーケンスを生成する第三パスのロジックを含みます。
- `src/pass4_midi.rs`: パス3で生成されたMIDIイベントのシーケンスから、最終的なStandard MIDI File (SMF) バイナリデータを構築する第四パスのロジックを含みます。
- `src/tree_sitter_mml.rs`: `tree-sitter` MMLパーサーをRustコードに統合するためのバインディングとユーティリティを提供します。
- `src/types.rs`: プロジェクト全体で共有されるカスタムデータ構造や列挙型などの型定義を含みます。
- `tests/integration_test.rs`: プロジェクト全体の主要な機能の統合的な動作を検証するテストコードです。
- `tests/test_channel.rs`: MMLの多チャンネル機能に関するテストコードです。
- `tests/test_chord.rs`: MMLの和音機能に関するテストコードです。
- `tests/test_cli.rs`: コマンドラインインターフェースの引数処理や出力に関するテストコードです。
- `tests/test_config.rs`: 設定ファイルの読み込みと適用に関するテストコードです。
- `tests/test_dotted_notes.rs`: 付点音符の正確な変換に関するテストコードです。
- `tests/test_drum_channel.rs`: ドラムチャンネルの処理に関するテストコードです。
- `tests/test_key_transpose.rs`: キー移調機能に関するテストコードです。
- `tests/test_length.rs`: 音長指定（例: `c4`, `c8`）のテストコードです。
- `tests/test_modifier.rs`: MMLの修飾子（例: 音量、パン）に関するテストコードです。
- `tests/test_note_length.rs`: 音符の長さ計算に関するテストコードです。
- `tests/test_octave.rs`: オクターブ変更コマンド（例: `<`, `>`)に関するテストコードです。
- `tests/test_pass1.rs`: パス1（トークン解析）のユニットテストコードです。
- `tests/test_pass2.rs`: パス2（AST変換）のユニットテストコードです。
- `tests/test_pass3.rs`: パス3（MIDIイベント生成）のユニットテストコードです。
- `tests/test_pass4.rs`: パス4（MIDIファイル作成）のユニットテストコードです。
- `tests/test_program_change.rs`: プログラムチェンジ（音色変更）に関するテストコードです。
- `tests/test_rest.rs`: 休符の処理に関するテストコードです。
- `tests/test_tempo.rs`: テンポ変更コマンドに関するテストコードです。
- `tests/test_velocity.rs`: ベロシティ（音量）変更コマンドに関するテストコードです。
- `tree-sitter-mml/grammar.js`: `tree-sitter`パーサーの文法規則をJavaScript形式で定義したファイルです。このファイルからパーサーのC言語ソースやWASMバイナリが生成されます。
- `tree-sitter-mml/package.json`: `tree-sitter-mml`パーサーのビルドに必要なNode.js依存関係を定義します。
- `tree-sitter-mml/src/grammar.json`: `grammar.js`から自動生成されたJSON形式の文法定義です。
- `tree-sitter-mml/src/node-types.json`: `grammar.js`から自動生成されたASTノードタイプの定義です。
- `tree-sitter-mml/src/parser.c`: `grammar.js`から自動生成されたC言語のパーサー実装ファイルです。
- `tree-sitter-mml/src/tree_sitter/alloc.h`, `array.h`, `parser.h`: `tree-sitter`ランタイムライブラリの内部で使用されるヘッダファイル群です。
- `tree-sitter-mml/tree-sitter-mml.wasm`: `tree-sitter` MMLパーサーをWebAssembly形式にコンパイルしたバイナリファイルです。
- `mmlabc-to-smf-wasm/Cargo.lock`: WASM版クレートの依存関係の正確なバージョンをロックするファイルです。
- `mmlabc-to-smf-wasm/Cargo.toml`: WASM版クレートのビルド設定と依存クレート、メタデータを定義するファイルです。
- `mmlabc-to-smf-wasm/src/lib.rs`: WebAssemblyターゲット向けライブラリのルートファイルで、WASMバインディングを提供します。

## 関数詳細説明
- `main()` (src/main.rs):
  - 役割: コマンドラインインターフェース (CLI) アプリケーションのメインエントリーポイントです。プログラムの実行を開始し、コマンドライン引数を解析後、MML変換処理を調整し、必要に応じてMIDIファイル再生をトリガーします。
  - 引数: なし。
  - 戻り値: なし (通常、成功時に`()`を返しますが、エラー発生時にはプロセスを終了する場合があります)。
  - 機能: CLIからのMML入力の受け付け、オプション解析、変換ライブラリの呼び出し、出力ファイルの書き出し、外部MIDIプレイヤーによる再生制御を行います。
- `mml_to_smf(mml_string: &str, options: Option<Config>) -> Result<Vec<u8>, Error>` (src/lib.rs):
  - 役割: Music Macro Language (MML) 形式の文字列をStandard MIDI File (SMF) のバイナリデータに変換する、ライブラリの主要な公開関数です。
  - 引数: `mml_string` (変換対象のMML文字列), `options` (変換に適用する設定情報を含む`Config`構造体のOptionalなインスタンス)。
  - 戻り値: 成功した場合はSMFのバイナリデータ (`Vec<u8>`) を、失敗した場合はエラー情報 (`Error`) を返します。
  - 機能: プロジェクトの4パスアーキテクチャ（トークン解析、AST構築、MIDIイベント生成、SMF作成）を統括して実行し、MMLをMIDIデータに変換します。
- `parse_mml_tokens(mml_string: &str) -> Result<Vec<Token>, Error>` (src/pass1_parser.rs):
  - 役割: 入力されたMML文字列を`tree-sitter`パーサーを用いて構文解析し、一連のトークンに分解します。
  - 引数: `mml_string` (解析対象のMML文字列)。
  - 戻り値: 成功した場合は解析されたトークンのリスト (`Vec<Token>`) を、失敗した場合はエラー情報 (`Error`) を返します。
  - 機能: MMLの構文要素（音符、コマンド、チャンネル区切りなど）を識別し、後続のパスで処理しやすい形式に変換します。
- `build_ast_from_tokens(tokens: Vec<Token>) -> Result<AstNode, Error>` (src/pass2_ast.rs):
  - 役割: パス1で生成されたトークンリストを基に、MMLの論理構造を表す抽象構文木 (AST) を構築します。
  - 引数: `tokens` (パス1で得られたトークンのリスト)。
  - 戻り値: 成功した場合はASTのルートノード (`AstNode`) を、失敗した場合はエラー情報 (`Error`) を返します。
  - 機能: トークン間の関係性を構造化し、MMLの音楽的意味を解釈するための準備をします。
- `generate_midi_events(ast: AstNode, config: &Config) -> Result<Vec<MidiEvent>, Error>` (src/pass3_events.rs):
  - 役割: パス2で構築されたASTを走査し、Standard MIDI Fileの内部表現として使用されるMIDIイベントのシーケンスを生成します。
  - 引数: `ast` (パス2で得られたAST), `config` (変換設定)。
  - 戻り値: 成功した場合はMIDIイベントのリスト (`Vec<MidiEvent>`) を、失敗した場合はエラー情報 (`Error`) を返します。
  - 機能: 音符のピッチ、長さ、ベロシティ、テンポ変更、プログラムチェンジ、チャンネル割り当てなどを具体的なMIDIイベントに変換します。
- `create_smf_from_events(events: Vec<MidiEvent>) -> Result<Vec<u8>, Error>` (src/pass4_midi.rs):
  - 役割: パス3で生成されたMIDIイベントのシーケンスから、最終的なStandard MIDI File (SMF) 形式のバイナリデータを構築します。
  - 引数: `events` (パス3で得られたMIDIイベントのリスト)。
  - 戻り値: 成功した場合はSMFバイナリデータ (`Vec<u8>`) を、失敗した場合はエラー情報 (`Error`) を返します。
  - 機能: MIDIイベントをSMFフォーマットのヘッダ、トラックチャンク、イベントデータとして適切にパッケージ化し、出力可能なファイル形式にします。
- `load_config() -> Config` (src/config.rs):
  - 役割: アプリケーションの設定ファイル（例: `mmlabc-to-smf-rust.toml`）から設定情報を読み込み、`Config`構造体として提供します。
  - 引数: なし。
  - 戻り値: 読み込まれた設定情報 (`Config`構造体)。
  - 機能: 外部MIDIプレイヤーの指定、デバッグ出力オプションなどのアプリケーション全体の挙動を制御する設定値をロードします。
- `play_midi_file(file_path: &Path, player: &str)` (src/main.rs):
  - 役割: 指定されたMIDIファイルを、ユーザー設定またはデフォルトの外部MIDIプレイヤーで再生します。
  - 引数: `file_path` (再生するMIDIファイルのパス), `player` (使用するMIDIプレイヤーのコマンド名)。
  - 戻り値: なし。
  - 機能: MML開発中に生成されたMIDIファイルの内容を即座に音で確認できるよう、外部プロセスを起動して再生します。
- `convert_mml_to_smf_wasm(mml_string: String) -> Result<Vec<u8>, String>` (mmlabc-to-smf-wasm/src/lib.rs):
  - 役割: WebAssembly環境から呼び出されることを想定した、MML文字列をStandard MIDI File (SMF) バイナリデータに変換する関数です。コアライブラリの機能をWASMインターフェースとして公開します。
  - 引数: `mml_string` (MML形式の入力文字列)。
  - 戻り値: 成功した場合はSMFバイナリデータ (`Vec<u8>`) を、失敗した場合はエラーメッセージ (`String`) を返します。
  - 機能: ブラウザなどのWebAssembly実行環境からMML変換機能を利用可能にします。

## 関数呼び出し階層ツリー
```
main() (src/main.rs)
├── 設定の読み込み (src/config.rs::load_config)
├── MML文字列からSMFへの変換 (src/lib.rs::mml_to_smf)
│   ├── パス1: MMLトークン解析 (src/pass1_parser.rs::parse_mml_tokens)
│   ├── パス2: ASTへの変換 (src/pass2_ast.rs::build_ast_from_tokens)
│   ├── パス3: MIDIイベント生成 (src/pass3_events.rs::generate_midi_events)
│   └── パス4: SMFファイル作成 (src/pass4_midi.rs::create_smf_from_events)
└── MIDIファイル再生 (src/main.rs::play_midi_file) (オプション)

WebAssemblyインターフェース (mmlabc-to-smf-wasm/src/lib.rs::convert_mml_to_smf_wasm)
└── MML文字列からSMFへの変換 (src/lib.rs::mml_to_smf)
    ├── パス1: MMLトークン解析 (src/pass1_parser.rs::parse_mml_tokens)
    ├── パス2: ASTへの変換 (src/pass2_ast.rs::build_ast_from_tokens)
    ├── パス3: MIDIイベント生成 (src/pass3_events.rs::generate_midi_events)
    └── パス4: SMFファイル作成 (src/pass4_midi.rs::create_smf_from_events)

---
Generated at: 2026-02-05 07:07:06 JST
