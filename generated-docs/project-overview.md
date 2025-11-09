Last updated: 2025-11-10

# Project Overview

## プロジェクト概要
- `mmlabc-to-smf-rust`は、Music Macro Language (MML) 形式の文字列をStandard MIDI File (SMF) へ変換するRust製のコマンドラインツールです。
- MMLのトークン化からMIDIファイル作成までを4つのパスで処理し、各パスで中間結果をJSONデバッグ出力できる点が特徴です。
- 基本音符変換や多チャンネル対応が実装済みで、高いメモリ安全性と包括的なテストによって信頼性の高い動作を提供します。

## 技術スタック
- フロントエンド: このプロジェクトはコマンドラインインターフェース（CLI）ツールであるため、特定のフロントエンド技術は使用していません。
- 音楽・オーディオ:
    - Standard MIDI File (SMF): 音楽データをデジタルで表現し保存するための標準フォーマットで、このプロジェクトの最終出力形式です。
    - Music Macro Language (MML): 音楽をテキストベースのマクロ言語で記述するための記法で、このプロジェクトの入力形式です。
    - `cat-play-mml`: 生成されたMIDIファイルを自動再生するために利用される外部コマンド（macOSでの利用を想定）。
- 開発ツール:
    - Rust: 高速性、安全性、並行性を重視したシステムプログラミング言語で、このプロジェクトの主要な開発言語です。
    - Cargo: Rustの公式ビルドシステムおよびパッケージマネージャーで、依存関係の管理、ビルド、テスト、ドキュメント生成などを担当します。
    - tree-sitter: （今後の目標として）堅牢な構文解析を可能にするパーサー生成フレームワークで、より複雑なMML構文の解析のために統合が検討されています。
- テスト:
    - Rust組み込みテストフレームワーク: `cargo test`コマンドを通じて実行される、Rust言語に標準で備わっているテスト機能群です。ユニットテストから統合テストまでを記述し、コードの品質を保証します。
- ビルドツール:
    - Cargo: Rustのプロジェクトのビルドプロセス全体を管理します。
    - MMLパーサー (Pass 1): 入力されたMML文字列を個々の意味のある要素（トークン）に分解する機能を提供します。
    - ASTジェネレーター (Pass 2): トークン化されたMMLデータから、プログラムが扱いやすい抽象構文木（AST）を構築する機能です。
    - MIDIイベントジェネレーター (Pass 3): 構築されたASTに基づいて、MIDIイベントのシーケンス（音符のオン/オフ、テンポ変更など）を生成する機能です。
    - MIDIファイルライター (Pass 4): 生成されたMIDIイベントのシーケンスを、SMFフォーマットに準拠したバイナリファイルとして書き出す機能です。
- 言語機能:
    - Rustの型システム: コンパイル時に型安全性を厳密にチェックし、多くの実行時エラーを防ぎます。
    - 所有権モデル: メモリ管理の安全性をコンパイル時に保証するRust独自の仕組みで、ヌルポインタ参照やデータ競合などの一般的なバグを排除します。
- 自動化・CI/CD: このプロジェクト自体は特定のCI/CDツールを明示していませんが、`cargo test`, `cargo clippy`, `cargo fmt`といったコマンドは通常CI/CDパイプラインに組み込まれ、自動的な品質チェックに利用されます。
- 開発標準:
    - `cargo clippy`: Rustコードに対するより厳密なリンティング（静的解析）ツールで、潜在的なバグや非効率なコードパターンを検出します。
    - `cargo fmt`: Rustコードのフォーマットを自動的に統一するツールで、コードベースの一貫性と可読性を高めます。
    - `.editorconfig`: 異なるエディタやIDE間でコードスタイルの一貫性を維持するための設定ファイルです。

## ファイル階層ツリー
```
.
├── .editorconfig
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
├── generated-docs/
│   └── development-status-generated-prompt.md
├── src/
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
│   ├── test_cli.rs
│   ├── test_pass1.rs
│   ├── test_pass2.rs
│   ├── test_pass3.rs
│   └── test_pass4.rs
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
- **`.editorconfig`**: 複数の開発者やエディタ間でコードのスタイル（インデント、改行コードなど）を一貫させるための設定ファイルです。
- **`.gitignore`**: Gitがバージョン管理の対象外とするファイルやディレクトリのパターンを定義します（例: ビルド成果物、一時ファイル）。
- **`.vscode/settings.json`**: Visual Studio Codeエディタ固有の設定を定義するファイルです。
- **`Cargo.lock`**: `Cargo.toml`で指定された依存関係のバージョンを正確に固定し、再現性のあるビルドを保証するファイルです。
- **`Cargo.toml`**: Rustプロジェクトのメタデータ（プロジェクト名、バージョンなど）と、外部ライブラリの依存関係を定義するファイルです。
- **`LICENSE`**: このプロジェクトのライセンス情報（MIT License）が記載されています。
- **`README.ja.md`**: プロジェクトの概要、使い方、開発方法などを日本語で記述した主要なドキュメントファイルです。
- **`README.md`**: プロジェクトの概要、使い方、開発方法などを英語で記述した主要なドキュメントファイルです。
- **`_config.yml`**: GitHub Pagesなどのサイト生成ツールで利用される設定ファイルである可能性があります。
- **`build.rs`**: Rustプロジェクトのビルド時に実行されるカスタムビルドスクリプトです。例えば、C/C++ライブラリ（tree-sitterのCコードなど）をコンパイルし、Rustコードとリンクさせるために使用されます。
- **`generated-docs/`**: 自動生成されたドキュメントやプロンプトの出力などを格納するディレクトリです。
    - **`development-status-generated-prompt.md`**: 開発状況に関する自動生成されたプロンプトの出力ファイルです。
- **`src/`**: プロジェクトの主要なRustソースコードを格納するディレクトリです。
    - **`src/lib.rs`**: クレート（ライブラリ）のエントリーポイントであり、プロジェクトの公開モジュールや関数を定義します。
    - **`src/main.rs`**: コマンドラインインターフェース (CLI) のエントリーポイントであり、プログラムの実行開始点です。
    - **`src/pass1_parser.rs`**: MML文字列をトークン（最小単位の要素）に解析する「パス1」のロジックが実装されています。
    - **`src/pass2_ast.rs`**: トークンリストを抽象構文木（AST）に変換する「パス2」のロジックが実装されています。
    - **`src/pass3_events.rs`**: ASTからStandard MIDI Fileのイベントデータを生成する「パス3」のロジックが実装されています。
    - **`src/pass4_midi.rs`**: 生成されたMIDIイベントデータから、最終的なStandard MIDI Fileを構築する「パス4」のロジックが実装されています。
    - **`src/tree_sitter_mml.rs`**: tree-sitterパーサーフレームワークとMML文法を統合するためのロジックが記述されています（今後の機能として）。
    - **`src/types.rs`**: プロジェクト全体で共通して利用されるデータ構造や型定義がまとめられています。
- **`tests/`**: プロジェクトのテストコードを格納するディレクトリです。
    - **`tests/integration_test.rs`**: プロジェクトの複数のコンポーネントが連携して正しく動作するかを検証する統合テストです。
    - **`tests/test_channel.rs`**: MMLの多チャンネル機能が正しく処理されるかを検証するテストです。
    - **`tests/test_cli.rs`**: コマンドラインインターフェースの引数処理や振る舞いを検証するテストです。
    - **`tests/test_pass1.rs`**: MMLパーサー（パス1）のトークン化処理が正しく動作するかを検証するテストです。
    - **`tests/test_pass2.rs`**: AST変換（パス2）のロジックが正しく動作するかを検証するテストです。
    - **`tests/test_pass3.rs`**: MIDIイベント生成（パス3）のロジックが正しく動作するかを検証するテストです。
    - **`tests/test_pass4.rs`**: MIDIファイル作成（パス4）のロジックが正しく動作するかを検証するテストです。
- **`tree-sitter-mml/`**: tree-sitter用のMML文法定義とその関連ファイルを格納するディレクトリです。
    - **`tree-sitter-mml/grammar.js`**: tree-sitterがMMLを解析するために使用する文法ルールをJavaScriptで定義したファイルです。
    - **`tree-sitter-mml/package.json`**: tree-sitterパーサーのメタデータや依存関係を定義するNode.jsのパッケージファイルです。
    - **`tree-sitter-mml/src/`**: tree-sitterパーサーの生成されたソースコードを格納するディレクトリです。
        - **`tree-sitter-mml/src/grammar.json`**: `grammar.js`から生成されるJSON形式の文法定義ファイルです。
        - **`tree-sitter-mml/src/node-types.json`**: 生成されたASTノードの型情報を定義するファイルです。
        - **`tree-sitter-mml/src/parser.c`**: `grammar.js`から生成されるC言語のパーサー実装ファイルです。
        - **`tree-sitter-mml/src/tree_sitter/`**: tree-sitterパーサーのC言語ランタイムライブラリのヘッダーファイルが含まれるディレクトリです。
            - **`tree-sitter-mml/src/tree_sitter/alloc.h`**: メモリ割り当て関連のヘッダーファイル。
            - **`tree-sitter-mml/src/tree_sitter/array.h`**: 配列操作関連のヘッダーファイル。
            - **`tree-sitter-mml/src/tree_sitter/parser.h`**: tree-sitterパーサーのAPI定義ヘッダーファイル。

## 関数詳細説明
具体的な関数名やシグネチャはプロジェクト情報に明示されていませんが、プロジェクトの4パスアーキテクチャに基づき、主要な処理を担う関数グループについて抽象的に説明します。

- **`main()`**
    - **役割**: コマンドライン引数を解析し、MMLからSMFへの変換処理全体をオーケストレーションします。
    - **引数**: なし（コマンドライン引数は環境から直接取得）。
    - **戻り値**: `Result<(), Error>`（成功またはエラー）。
    - **機能**: 入力MML文字列の取得、出力ファイルパスの決定、自動再生オプションの処理、そして4つの変換パスの順次呼び出しを行います。各パスの中間結果のデバッグ出力も管理します。

- **`pass1::tokenize_mml(mml_string: &str) -> Vec<Token>`**
    - **役割**: 入力されたMML文字列を、解析可能な最小単位であるトークン（例: `c`, `;`, `>`など）のリストに変換します。
    - **引数**: `mml_string` (入力MML文字列)。
    - **戻り値**: `Vec<Token>` (MMLトークンのベクタ)。
    - **機能**: MML文字列を一文字ずつ走査し、定義されたルールに基づいて音符、チャンネル区切りなどのトークンに分割します。

- **`pass2::build_ast(tokens: Vec<Token>) -> AstNode`**
    - **役割**: パス1で生成されたトークンのリストから、MMLの構造を階層的に表現する抽象構文木（AST）を構築します。
    - **引数**: `tokens` (MMLトークンのベクタ)。
    - **戻り値**: `AstNode` (ASTのルートノード)。
    - **機能**: トークンの意味的な関係性を解析し、例えば「音符グループ」や「チャンネル」といった構造を持つ木構造データを生成します。

- **`pass3::generate_midi_events(ast: AstNode) -> Vec<MidiEvent>`**
    - **役割**: パス2で構築されたASTを基に、MIDIプロトコルに準拠したイベント（例: ノートオン、ノートオフ）のシーケンスを生成します。
    - **引数**: `ast` (ASTのルートノード)。
    - **戻り値**: `Vec<MidiEvent>` (MIDIイベントのベクタ)。
    - **機能**: ASTの各ノードを巡回し、対応するMIDIメッセージ（例: `c`を中央Cのノートオン/オフイベントに）とタイミング情報を付与してイベントリストを作成します。

- **`pass4::write_midi_file(midi_events: Vec<MidiEvent>, output_path: &Path) -> Result<(), Error>`**
    - **役割**: パス3で生成されたMIDIイベントのシーケンスを、Standard MIDI File（SMF）のフォーマットに従って指定されたファイルに書き出します。
    - **引数**: `midi_events` (MIDIイベントのベクタ), `output_path` (出力ファイルパス)。
    - **戻り値**: `Result<(), Error>` (ファイル書き込みの成否)。
    - **機能**: MIDIイベントデータをSMFのヘッダー、トラックデータ、イベントエンコーディングルールに従ってバイナリ形式に変換し、ディスク上のファイルに保存します。

- **`util::play_midi_file(file_path: &Path)`**
    - **役割**: 生成されたMIDIファイルを外部の再生ツール（`cat-play-mml`）を使用して自動再生します。
    - **引数**: `file_path` (再生するMIDIファイルのパス)。
    - **戻り値**: なし。
    - **機能**: 指定されたパスのMIDIファイルを`cat-play-mml`コマンドに渡し、サブプロセスとして実行することで再生をトリガーします。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした

---
Generated at: 2025-11-10 07:05:34 JST
