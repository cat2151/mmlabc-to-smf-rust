Last updated: 2025-11-14


# プロジェクト概要生成プロンプト（来訪者向け）

## 生成するもの：
- projectを3行で要約する
- プロジェクトで使用されている技術スタックをカテゴリ別に整理して説明する
- プロジェクト全体のファイル階層ツリー（ディレクトリ構造を図解）
- プロジェクト全体のファイルそれぞれの説明
- プロジェクト全体の関数それぞれの説明
- プロジェクト全体の関数の呼び出し階層ツリー

## 生成しないもの：
- Issues情報（開発者向け情報のため）
- 次の一手候補（開発者向け情報のため）
- ハルシネーションしそうなもの（例、存在しない機能や計画を勝手に妄想する等）

## 出力フォーマット：
以下のMarkdown形式で出力してください：

```markdown
# Project Overview

## プロジェクト概要
[以下の形式で3行でプロジェクトを要約]
- [1行目の説明]
- [2行目の説明]
- [3行目の説明]

## 技術スタック
[使用している技術をカテゴリ別に整理して説明]
- フロントエンド: [フロントエンド技術とその説明]
- 音楽・オーディオ: [音楽・オーディオ関連技術とその説明]
- 開発ツール: [開発支援ツールとその説明]
- テスト: [テスト関連技術とその説明]
- ビルドツール: [ビルド・パース関連技術とその説明]
- 言語機能: [言語仕様・機能とその説明]
- 自動化・CI/CD: [自動化・継続的統合関連技術とその説明]
- 開発標準: [コード品質・統一ルール関連技術とその説明]

## ファイル階層ツリー
```
[プロジェクトのディレクトリ構造をツリー形式で表現]
```

## ファイル詳細説明
[各ファイルの役割と機能を詳細に説明]

## 関数詳細説明
[各関数の役割、引数、戻り値、機能を詳細に説明]

## 関数呼び出し階層ツリー
```
[関数間の呼び出し関係をツリー形式で表現]
```
```


以下のプロジェクト情報を参考にして要約を生成してください：

## プロジェクト情報
名前: 
説明: # mmlabc-to-smf-rust

Music Macro Language (MML) から Standard MIDI File (SMF) への変換ツール（Rust版）

## 概要

このプロジェクトは、[mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf) のRust実装版です。
Music Macro Language形式の文字列を、包括的なデバッグ出力を備えた4パスアーキテクチャを使用してStandard MIDI Fileに変換します。

## WIP

開発中です。現状、c～bを認識できるのみで、ほかのMMLはこれから実装します。

### 実装済み機能 ✅
- **基本音符変換**: `cdefgab` → MIDI音符への変換
- **4パスアーキテクチャ**: 完全実装済み
  - パス1: MML文字列のトークン化（単純パーサー）
  - パス2: トークンからAST（抽象構文木）への変換
  - パス3: ASTからMIDIイベントの生成
  - パス4: MIDIイベントからStandard MIDI File作成
- **チャンネル機能**: セミコロン（`;`）による多チャンネル対応
- **JSON デバッグ出力**: 各パスの中間結果をJSONで出力
- **CLI**: コマンドライン引数による基本操作
- **包括的テスト**: 35個のテストケースがすべて通過

### 動作確認
```bash
# 基本音階変換
cargo run -- "cdefgab"

# 多チャンネル
cargo run -- "c;e;g"

# カスタム出力ファイル
cargo run -- "cde" -o my_song.mid
```

## 今後の見通し

### 短期目標 🚧
- **tree-sitter統合**: より複雑なMML構文の解析に向けて
- **リポジトリ設定**: フォーマッター、リンター等の設定整備
- **エラーハンドリング**: より詳細なエラーメッセージ

### 長期目標 🎯
- **mmlabcコマンド実装**: 完全なmmlabcフォーマット対応
  - 音長指定（4分音符、8分音符等）
  - オクターブ指定（`>`, `<`）
  - テンポ、音量等の制御コマンド
  - 和音機能の拡張
- **パフォーマンス最適化**: 大規模MMLファイルの高速処理

### 参考資料
- mmlabcについては、[mml2abc](https://github.com/cat2151/mml2abc)リポジトリを参照

## 特徴

- **4パスアーキテクチャ**:
  - **パス1**: MML文字列をトークンに解析（現在：単純パーサー、将来：tree-sitter）
  - **パス2**: トークンを抽象構文木（AST）に変換
  - **パス3**: ASTからMIDIイベントを生成
  - **パス4**: Standard MIDI Fileを作成
- **多チャンネル対応**: セミコロン（`;`）による同時発音チャンネル分離
- **JSON デバッグ出力**: 各パスの中間結果をJSON形式で保存・確認可能
- **包括的テスト**: ユニットテスト・統合テスト合計35個のテストケース
- **安全な設計**: Rustの型システムと所有権モデルによるメモリ安全性

## 必要要件

- Rust 1.70.0以上
- Cargo

## インストール

### 開発版（現在の状態）

```bash
git clone https://github.com/cat2151/mmlabc-to-smf-rust
cd mmlabc-to-smf-rust
cargo build --release
```

### 直接実行（Cargo経由）

```bash
cargo run -- "cdefgab"
```

## 使い方

### 基本的な使い方

```bash
# 基本音階の変換（デフォルトでcat-play-mmlで自動再生されます）
cargo run -- "cdefgab"

# 多チャンネル（同時発音）
cargo run -- "c;e;g"  # Cメジャーコード

# カスタム出力ファイル
cargo run -- "cde" -o my_song.mid

# 自動再生を無効化
cargo run -- "cde" --no-play
```

### 自動再生機能

デフォルトでは、MIDIファイル生成後に自動的に `cat-play-mml` コマンドで再生されます。
これによりMML開発時に即座に音を確認できます。

- 自動再生を無効化するには `--no-play` オプションを使用してください
- `cat-play-mml` がインストールされていない場合、警告メッセージが表示されますがMIDIファイルは正常に生成されます

#### カスタムプレイヤーの設定

ツールを実行するディレクトリに `mmlabc-to-smf-rust.toml` ファイルを作成することで、カスタムMIDIプレイヤーを設定できます。

設定ファイルの例：
```toml
# mmlabc-to-smf-rust.toml
external_smf_player = "timidity"
```

設定可能な一般的なMIDIプレイヤー：
- `timidity` - TiMidity++ MIDIプレイヤー
- `fluidsynth` - FluidSynthソフトウェアシンセサイザー
- `vlc` - VLCメディアプレイヤー
- `cat-play-mml` (デフォルト)

設定ファイルが存在しない場合、デフォルトで `cat-play-mml` が使用されます。

サンプル設定ファイルは `mmlabc-to-smf-rust.toml.example` を参照してください。

### 出力ファイル

実行すると以下のファイルが生成されます：
- `pass1_tokens.json` - パス1のトークン情報（デバッグ用）
- `pass2_ast.json` - パス2のAST情報（デバッグ用）
- `pass3_events.json` - パス3のMIDIイベント情報（デバッグ用）
- `output.mid` - 最終的なMIDIファイル

### 対応MML記法

現在対応している記法：
- **基本音符**: `c`, `d`, `e`, `f`, `g`, `a`, `b` (大文字・小文字対応)
- **多チャンネル**: `;` でチャンネル分離（同時発音）

例：
```
cdefgab     → ドレミファソラシの連続再生
c;e;g       → C・E・G音の同時再生（Cメジャーコード）
```

## 開発

### ビルド

```bash
cargo build        # デバッグビルド
cargo build --release  # リリースビルド
```

### テスト

```bash
cargo test         # 全テスト実行（35個のテストケース）
```

### フォーマット・Lint

```bash
cargo clippy       # コード品質チェック
cargo fmt --check  # フォーマットチェック
cargo fmt          # フォーマット適用
```

### プロジェクト構造

```
src/
├── main.rs              # CLI エントリーポイント
├── lib.rs               # ライブラリルート
├── pass1_parser.rs      # パス1: トークン解析
├── pass2_ast.rs         # パス2: AST変換
├── pass3_events.rs      # パス3: MIDIイベント生成
├── pass4_midi.rs        # パス4: MIDI ファイル作成
├── tree_sitter_mml.rs   # tree-sitter MML統合
└── types.rs             # 共通型定義

tests/
├── integration_test.rs  # 統合テスト
├── test_channel.rs      # チャンネル機能テスト
├── test_pass1.rs        # パス1テスト
├── test_pass2.rs        # パス2テスト
├── test_pass3.rs        # パス3テスト
└── test_pass4.rs        # パス4テスト
```

## ライセンス

MIT License - 詳細は [LICENSE](LICENSE) ファイルを参照してください。

## 参考

- オリジナルのPython実装: [cat2151/mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf)


依存関係:
{}

## ファイル階層ツリー
📄 .editorconfig
📄 .gitignore
📁 .vscode/
  📊 settings.json
📄 Cargo.lock
📄 Cargo.toml
📄 LICENSE
📖 README.ja.md
📖 README.md
📁 _codeql_detected_source_root/
  📄 .editorconfig
  📄 .gitignore
  📁 .vscode/
    📊 settings.json
  📄 Cargo.lock
  📄 Cargo.toml
  📄 LICENSE
  📖 README.ja.md
  📖 README.md
  📁 _codeql_detected_source_root/
    📄 .editorconfig
    📄 .gitignore
    📁 .vscode/
      📊 settings.json
    📄 Cargo.lock
    📄 Cargo.toml
    📄 LICENSE
    📖 README.ja.md
    📖 README.md
    📁 _codeql_detected_source_root/
      📄 .editorconfig
      📄 .gitignore
      📁 .vscode/
        📊 settings.json
      📄 Cargo.lock
      📄 Cargo.toml
      📄 LICENSE
      📖 README.ja.md
      📖 README.md
      📁 _codeql_detected_source_root/
        📄 .editorconfig
        📄 .gitignore
        📁 .vscode/
          📊 settings.json
        📄 Cargo.lock
        📄 Cargo.toml
        📄 LICENSE
        📖 README.ja.md
        📖 README.md
        📁 _codeql_detected_source_root/
          📄 .editorconfig
          📄 .gitignore
          📁 .vscode/
            📊 settings.json
          📄 Cargo.lock
          📄 Cargo.toml
          📄 LICENSE
          📖 README.ja.md
          📖 README.md
          📁 _codeql_detected_source_root/
            📄 .editorconfig
            📄 .gitignore
            📁 .vscode/
              📊 settings.json
            📄 Cargo.lock
            📄 Cargo.toml
            📄 LICENSE
            📖 README.ja.md
            📖 README.md
            📁 _codeql_detected_source_root/
              📄 .editorconfig
              📄 .gitignore
              📁 .vscode/
                📊 settings.json
              📄 Cargo.lock
              📄 Cargo.toml
              📄 LICENSE
              📖 README.ja.md
              📖 README.md
              📁 _codeql_detected_source_root/
                📄 .editorconfig
                📄 .gitignore
                📁 .vscode/
                  📊 settings.json
                📄 Cargo.lock
                📄 Cargo.toml
                📄 LICENSE
                📖 README.ja.md
                📖 README.md
                📁 _codeql_detected_source_root/
                  📄 .editorconfig
                  📄 .gitignore
                  📁 .vscode/
                    📊 settings.json
                  📄 Cargo.lock
                  📄 Cargo.toml
                  📄 LICENSE
                  📖 README.ja.md
                  📖 README.md
                  📁 _codeql_detected_source_root/
                    📄 .editorconfig
                    📄 .gitignore
                    📁 .vscode/
                      📊 settings.json
                    📄 Cargo.lock
                    📄 Cargo.toml
                    📄 LICENSE
                    📖 README.ja.md
                    📖 README.md
                    📁 _codeql_detected_source_root/
                      📄 .editorconfig
                      📄 .gitignore
                      📁 .vscode/
                        📊 settings.json
                      📄 Cargo.lock
                      📄 Cargo.toml
                      📄 LICENSE
                      📖 README.ja.md
                      📖 README.md
                      📁 _codeql_detected_source_root/
                        📄 .editorconfig
                        📄 .gitignore
                        📁 .vscode/
                          📊 settings.json
                        📄 Cargo.lock
                        📄 Cargo.toml
                        📄 LICENSE
                        📖 README.ja.md
                        📖 README.md
                        📁 _codeql_detected_source_root/
                          📄 .editorconfig
                          📄 .gitignore
                          📁 .vscode/
                            📊 settings.json
                          📄 Cargo.lock
                          📄 Cargo.toml
                          📄 LICENSE
                          📖 README.ja.md
                          📖 README.md
                          📁 _codeql_detected_source_root/
                            📄 .editorconfig
                            📄 .gitignore
                            📁 .vscode/
                              📊 settings.json
                            📄 Cargo.lock
                            📄 Cargo.toml
                            📄 LICENSE
                            📖 README.ja.md
                            📖 README.md
                            📁 _codeql_detected_source_root/
                              📄 .editorconfig
                              📄 .gitignore
                              📁 .vscode/
                                📊 settings.json
                              📄 Cargo.lock
                              📄 Cargo.toml
                              📄 LICENSE
                              📖 README.ja.md
                              📖 README.md
                              📁 _codeql_detected_source_root/
                                📄 .editorconfig
                                📄 .gitignore
                                📁 .vscode/
                                  📊 settings.json
                                📄 Cargo.lock
                                📄 Cargo.toml
                                📄 LICENSE
                                📖 README.ja.md
                                📖 README.md
                                📁 _codeql_detected_source_root/
                                  📄 .editorconfig
                                  📄 .gitignore
                                  📁 .vscode/
                                    📊 settings.json
                                  📄 Cargo.lock
                                  📄 Cargo.toml
                                  📄 LICENSE
                                  📖 README.ja.md
                                  📖 README.md
                                  📁 _codeql_detected_source_root/
                                    📄 .editorconfig
                                    📄 .gitignore
                                    📁 .vscode/
                                      📊 settings.json
                                    📄 Cargo.lock
                                    📄 Cargo.toml
                                    📄 LICENSE
                                    📖 README.ja.md
                                    📖 README.md
                                    📁 _codeql_detected_source_root/
                                      📄 .editorconfig
                                      📄 .gitignore
                                      📁 .vscode/
                                        📊 settings.json
                                      📄 Cargo.lock
                                      📄 Cargo.toml
                                      📄 LICENSE
                                      📖 README.ja.md
                                      📖 README.md
                                      📁 _codeql_detected_source_root/
                                        📄 .editorconfig
                                        📄 .gitignore
                                        📁 .vscode/
                                          📊 settings.json
                                        📄 Cargo.lock
                                        📄 Cargo.toml
                                        📄 LICENSE
                                        📖 README.ja.md
                                        📖 README.md
                                        📁 _codeql_detected_source_root/
                                          📄 .editorconfig
                                          📄 .gitignore
                                          📁 .vscode/
                                            📊 settings.json
                                          📄 Cargo.lock
                                          📄 Cargo.toml
                                          📄 LICENSE
                                          📖 README.ja.md
                                          📖 README.md
                                          📁 _codeql_detected_source_root/
                                            📄 .editorconfig
                                            📄 .gitignore
                                            📁 .vscode/
                                              📊 settings.json
                                            📄 Cargo.lock
                                            📄 Cargo.toml
                                            📄 LICENSE
                                            📖 README.ja.md
                                            📖 README.md
                                            📁 _codeql_detected_source_root/
                                              📄 .editorconfig
                                              📄 .gitignore
                                              📁 .vscode/
                                                📊 settings.json
                                              📄 Cargo.lock
                                              📄 Cargo.toml
                                              📄 LICENSE
                                              📖 README.ja.md
                                              📖 README.md
                                              📁 _codeql_detected_source_root/
                                                📄 .editorconfig
                                                📄 .gitignore
                                                📁 .vscode/
                                                  📊 settings.json
                                                📄 Cargo.lock
                                                📄 Cargo.toml
                                                📄 LICENSE
                                                📖 README.ja.md
                                                📖 README.md
                                                📁 _codeql_detected_source_root/
                                                  📄 .editorconfig
                                                  📄 .gitignore
                                                  📁 .vscode/
                                                    📊 settings.json
                                                  📄 Cargo.lock
                                                  📄 Cargo.toml
                                                  📄 LICENSE
                                                  📖 README.ja.md
                                                  📖 README.md
                                                  📁 _codeql_detected_source_root/
                                                    📄 .editorconfig
                                                    📄 .gitignore
                                                    📁 .vscode/
                                                      📊 settings.json
                                                    📄 Cargo.lock
                                                    📄 Cargo.toml
                                                    📄 LICENSE
                                                    📖 README.ja.md
                                                    📖 README.md
                                                    📁 _codeql_detected_source_root/
                                                      📄 .editorconfig
                                                      📄 .gitignore
                                                      📁 .vscode/
                                                        📊 settings.json
                                                      📄 Cargo.lock
                                                      📄 Cargo.toml
                                                      📄 LICENSE
                                                      📖 README.ja.md
                                                      📖 README.md
                                                      📁 _codeql_detected_source_root/
                                                        📄 .editorconfig
                                                        📄 .gitignore
                                                        📁 .vscode/
                                                          📊 settings.json
                                                        📄 Cargo.lock
                                                        📄 Cargo.toml
                                                        📄 LICENSE
                                                        📖 README.ja.md
                                                        📖 README.md
                                                        📁 _codeql_detected_source_root/
                                                          📄 .editorconfig
                                                          📄 .gitignore
                                                          📁 .vscode/
                                                            📊 settings.json
                                                          📄 Cargo.lock
                                                          📄 Cargo.toml
                                                          📄 LICENSE
                                                          📖 README.ja.md
                                                          📖 README.md
                                                          📁 _codeql_detected_source_root/
                                                            📄 .editorconfig
                                                            📄 .gitignore
                                                            📁 .vscode/
                                                              📊 settings.json
                                                            📄 Cargo.lock
                                                            📄 Cargo.toml
                                                            📄 LICENSE
                                                            📖 README.ja.md
                                                            📖 README.md
                                                            📁 _codeql_detected_source_root/
                                                              📄 .editorconfig
                                                              📄 .gitignore
                                                              📁 .vscode/
                                                                📊 settings.json
                                                              📄 Cargo.lock
                                                              📄 Cargo.toml
                                                              📄 LICENSE
                                                              📖 README.ja.md
                                                              📖 README.md
                                                              📁 _codeql_detected_source_root/
                                                                📄 .editorconfig
                                                                📄 .gitignore
                                                                📁 .vscode/
                                                                  📊 settings.json
                                                                📄 Cargo.lock
                                                                📄 Cargo.toml
                                                                📄 LICENSE
                                                                📖 README.ja.md
                                                                📖 README.md
                                                                📁 _codeql_detected_source_root/
                                                                  📄 .editorconfig
                                                                  📄 .gitignore
                                                                  📁 .vscode/
                                                                    📊 settings.json
                                                                  📄 Cargo.lock
                                                                  📄 Cargo.toml
                                                                  📄 LICENSE
                                                                  📖 README.ja.md
                                                                  📖 README.md
                                                                  📁 _codeql_detected_source_root/
                                                                    📄 .editorconfig
                                                                    📄 .gitignore
                                                                    📁 .vscode/
                                                                      📊 settings.json
                                                                    📄 Cargo.lock
                                                                    📄 Cargo.toml
                                                                    📄 LICENSE
                                                                    📖 README.ja.md
                                                                    📖 README.md
                                                                    📁 _codeql_detected_source_root/
                                                                      📄 .editorconfig
                                                                      📄 .gitignore
                                                                      📁 .vscode/
                                                                        📊 settings.json
                                                                      📄 Cargo.lock
                                                                      📄 Cargo.toml
                                                                      📄 LICENSE
                                                                      📖 README.ja.md
                                                                      📖 README.md
                                                                      📁 _codeql_detected_source_root/
                                                                        📄 .editorconfig
                                                                        📄 .gitignore
                                                                        📁 .vscode/
                                                                          📊 settings.json
                                                                        📄 Cargo.lock
                                                                        📄 Cargo.toml
                                                                        📄 LICENSE
                                                                        📖 README.ja.md
                                                                        📖 README.md
                                                                        📁 _codeql_detected_source_root/
                                                                          📄 .editorconfig
                                                                          📄 .gitignore
                                                                          📁 .vscode/
                                                                            📊 settings.json
                                                                          📄 Cargo.lock
                                                                          📄 Cargo.toml
                                                                          📄 LICENSE
                                                                          📖 README.ja.md
                                                                          📖 README.md
                                                                          📁 _codeql_detected_source_root/
                                                                            📄 .editorconfig
                                                                            📄 .gitignore
                                                                            📁 .vscode/
                                                                              📊 settings.json
                                                                            📄 Cargo.lock
                                                                            📄 Cargo.toml
                                                                            📄 LICENSE
                                                                            📖 README.ja.md
                                                                            📖 README.md
                                                                            📁 _codeql_detected_source_root/
                                                                              📄 .editorconfig
                                                                              📄 .gitignore
                                                                              📁 .vscode/
                                                                                📊 settings.json
                                                                              📄 Cargo.lock
                                                                              📄 Cargo.toml
                                                                              📄 LICENSE
                                                                              📖 README.ja.md
                                                                              📖 README.md
                                                                              📁 _codeql_detected_source_root/
                                                                                📄 .editorconfig
                                                                                📄 .gitignore
                                                                                📁 .vscode/
                                                                                  📊 settings.json
                                                                                📄 Cargo.lock
                                                                                📄 Cargo.toml
                                                                                📄 LICENSE
                                                                                📖 README.ja.md
                                                                                📖 README.md
                                                                                📄 _config.yml
                                                                                📄 build.rs
                                                                                📁 generated-docs/
                                                                                📁 issue-notes/
                                                                                  📖 14.md
                                                                                📄 mmlabc-to-smf-rust.toml.example
                                                                                📁 src/
                                                                                  📄 config.rs
                                                                                  📄 lib.rs
                                                                                  📄 main.rs
                                                                                  📄 pass1_parser.rs
                                                                                  📄 pass2_ast.rs
                                                                                  📄 pass3_events.rs
                                                                                  📄 pass4_midi.rs
                                                                                  📄 tree_sitter_mml.rs
                                                                                  📄 types.rs
                                                                                📁 tests/
                                                                                  📄 integration_test.rs
                                                                                  📄 test_channel.rs
                                                                                  📄 test_cli.rs
                                                                                  📄 test_config.rs
                                                                                  📄 test_pass1.rs
                                                                                  📄 test_pass2.rs
                                                                                  📄 test_pass3.rs
                                                                                  📄 test_pass4.rs
                                                                                📁 tree-sitter-mml/
                                                                                  📜 grammar.js
                                                                                  📊 package.json
                                                                                  📁 src/
                                                                                    📊 grammar.json
                                                                                    📊 node-types.json
                                                                                    📄 parser.c
                                                                                    📁 tree_sitter/
                                                                                      📄 alloc.h
                                                                                      📄 array.h
                                                                                      📄 parser.h
                                                                              📄 _config.yml
                                                                              📄 build.rs
                                                                              📁 generated-docs/
                                                                              📁 issue-notes/
                                                                                📖 14.md
                                                                              📄 mmlabc-to-smf-rust.toml.example
                                                                              📁 src/
                                                                                📄 config.rs
                                                                                📄 lib.rs
                                                                                📄 main.rs
                                                                                📄 pass1_parser.rs
                                                                                📄 pass2_ast.rs
                                                                                📄 pass3_events.rs
                                                                                📄 pass4_midi.rs
                                                                                📄 tree_sitter_mml.rs
                                                                                📄 types.rs
                                                                              📁 tests/
                                                                                📄 integration_test.rs
                                                                                📄 test_channel.rs
                                                                                📄 test_cli.rs
                                                                                📄 test_config.rs
                                                                                📄 test_pass1.rs
                                                                                📄 test_pass2.rs
                                                                                📄 test_pass3.rs
                                                                                📄 test_pass4.rs
                                                                              📁 tree-sitter-mml/
                                                                                📜 grammar.js
                                                                                📊 package.json
                                                                                📁 src/
                                                                                  📊 grammar.json
                                                                                  📊 node-types.json
                                                                                  📄 parser.c
                                                                                  📁 tree_sitter/
                                                                                    📄 alloc.h
                                                                                    📄 array.h
                                                                                    📄 parser.h
                                                                            📄 _config.yml
                                                                            📄 build.rs
                                                                            📁 generated-docs/
                                                                            📁 issue-notes/
                                                                              📖 14.md
                                                                            📄 mmlabc-to-smf-rust.toml.example
                                                                            📁 src/
                                                                              📄 config.rs
                                                                              📄 lib.rs
                                                                              📄 main.rs
                                                                              📄 pass1_parser.rs
                                                                              📄 pass2_ast.rs
                                                                              📄 pass3_events.rs
                                                                              📄 pass4_midi.rs
                                                                              📄 tree_sitter_mml.rs
                                                                              📄 types.rs
                                                                            📁 tests/
                                                                              📄 integration_test.rs
                                                                              📄 test_channel.rs
                                                                              📄 test_cli.rs
                                                                              📄 test_config.rs
                                                                              📄 test_pass1.rs
                                                                              📄 test_pass2.rs
                                                                              📄 test_pass3.rs
                                                                              📄 test_pass4.rs
                                                                            📁 tree-sitter-mml/
                                                                              📜 grammar.js
                                                                              📊 package.json
                                                                              📁 src/
                                                                                📊 grammar.json
                                                                                📊 node-types.json
                                                                                📄 parser.c
                                                                                📁 tree_sitter/
                                                                                  📄 alloc.h
                                                                                  📄 array.h
                                                                                  📄 parser.h
                                                                          📄 _config.yml
                                                                          📄 build.rs
                                                                          📁 generated-docs/
                                                                          📁 issue-notes/
                                                                            📖 14.md
                                                                          📄 mmlabc-to-smf-rust.toml.example
                                                                          📁 src/
                                                                            📄 config.rs
                                                                            📄 lib.rs
                                                                            📄 main.rs
                                                                            📄 pass1_parser.rs
                                                                            📄 pass2_ast.rs
                                                                            📄 pass3_events.rs
                                                                            📄 pass4_midi.rs
                                                                            📄 tree_sitter_mml.rs
                                                                            📄 types.rs
                                                                          📁 tests/
                                                                            📄 integration_test.rs
                                                                            📄 test_channel.rs
                                                                            📄 test_cli.rs
                                                                            📄 test_config.rs
                                                                            📄 test_pass1.rs
                                                                            📄 test_pass2.rs
                                                                            📄 test_pass3.rs
                                                                            📄 test_pass4.rs
                                                                          📁 tree-sitter-mml/
                                                                            📜 grammar.js
                                                                            📊 package.json
                                                                            📁 src/
                                                                              📊 grammar.json
                                                                              📊 node-types.json
                                                                              📄 parser.c
                                                                              📁 tree_sitter/
                                                                                📄 alloc.h
                                                                                📄 array.h
                                                                                📄 parser.h
                                                                        📄 _config.yml
                                                                        📄 build.rs
                                                                        📁 generated-docs/
                                                                        📁 issue-notes/
                                                                          📖 14.md
                                                                        📄 mmlabc-to-smf-rust.toml.example
                                                                        📁 src/
                                                                          📄 config.rs
                                                                          📄 lib.rs
                                                                          📄 main.rs
                                                                          📄 pass1_parser.rs
                                                                          📄 pass2_ast.rs
                                                                          📄 pass3_events.rs
                                                                          📄 pass4_midi.rs
                                                                          📄 tree_sitter_mml.rs
                                                                          📄 types.rs
                                                                        📁 tests/
                                                                          📄 integration_test.rs
                                                                          📄 test_channel.rs
                                                                          📄 test_cli.rs
                                                                          📄 test_config.rs
                                                                          📄 test_pass1.rs
                                                                          📄 test_pass2.rs
                                                                          📄 test_pass3.rs
                                                                          📄 test_pass4.rs
                                                                        📁 tree-sitter-mml/
                                                                          📜 grammar.js
                                                                          📊 package.json
                                                                          📁 src/
                                                                            📊 grammar.json
                                                                            📊 node-types.json
                                                                            📄 parser.c
                                                                            📁 tree_sitter/
                                                                              📄 alloc.h
                                                                              📄 array.h
                                                                              📄 parser.h
                                                                      📄 _config.yml
                                                                      📄 build.rs
                                                                      📁 generated-docs/
                                                                      📁 issue-notes/
                                                                        📖 14.md
                                                                      📄 mmlabc-to-smf-rust.toml.example
                                                                      📁 src/
                                                                        📄 config.rs
                                                                        📄 lib.rs
                                                                        📄 main.rs
                                                                        📄 pass1_parser.rs
                                                                        📄 pass2_ast.rs
                                                                        📄 pass3_events.rs
                                                                        📄 pass4_midi.rs
                                                                        📄 tree_sitter_mml.rs
                                                                        📄 types.rs
                                                                      📁 tests/
                                                                        📄 integration_test.rs
                                                                        📄 test_channel.rs
                                                                        📄 test_cli.rs
                                                                        📄 test_config.rs
                                                                        📄 test_pass1.rs
                                                                        📄 test_pass2.rs
                                                                        📄 test_pass3.rs
                                                                        📄 test_pass4.rs
                                                                      📁 tree-sitter-mml/
                                                                        📜 grammar.js
                                                                        📊 package.json
                                                                        📁 src/
                                                                          📊 grammar.json
                                                                          📊 node-types.json
                                                                          📄 parser.c
                                                                          📁 tree_sitter/
                                                                            📄 alloc.h
                                                                            📄 array.h
                                                                            📄 parser.h
                                                                    📄 _config.yml
                                                                    📄 build.rs
                                                                    📁 generated-docs/
                                                                    📁 issue-notes/
                                                                      📖 14.md
                                                                    📄 mmlabc-to-smf-rust.toml.example
                                                                    📁 src/
                                                                      📄 config.rs
                                                                      📄 lib.rs
                                                                      📄 main.rs
                                                                      📄 pass1_parser.rs
                                                                      📄 pass2_ast.rs
                                                                      📄 pass3_events.rs
                                                                      📄 pass4_midi.rs
                                                                      📄 tree_sitter_mml.rs
                                                                      📄 types.rs
                                                                    📁 tests/
                                                                      📄 integration_test.rs
                                                                      📄 test_channel.rs
                                                                      📄 test_cli.rs
                                                                      📄 test_config.rs
                                                                      📄 test_pass1.rs
                                                                      📄 test_pass2.rs
                                                                      📄 test_pass3.rs
                                                                      📄 test_pass4.rs
                                                                    📁 tree-sitter-mml/
                                                                      📜 grammar.js
                                                                      📊 package.json
                                                                      📁 src/
                                                                        📊 grammar.json
                                                                        📊 node-types.json
                                                                        📄 parser.c
                                                                        📁 tree_sitter/
                                                                          📄 alloc.h
                                                                          📄 array.h
                                                                          📄 parser.h
                                                                  📄 _config.yml
                                                                  📄 build.rs
                                                                  📁 generated-docs/
                                                                  📁 issue-notes/
                                                                    📖 14.md
                                                                  📄 mmlabc-to-smf-rust.toml.example
                                                                  📁 src/
                                                                    📄 config.rs
                                                                    📄 lib.rs
                                                                    📄 main.rs
                                                                    📄 pass1_parser.rs
                                                                    📄 pass2_ast.rs
                                                                    📄 pass3_events.rs
                                                                    📄 pass4_midi.rs
                                                                    📄 tree_sitter_mml.rs
                                                                    📄 types.rs
                                                                  📁 tests/
                                                                    📄 integration_test.rs
                                                                    📄 test_channel.rs
                                                                    📄 test_cli.rs
                                                                    📄 test_config.rs
                                                                    📄 test_pass1.rs
                                                                    📄 test_pass2.rs
                                                                    📄 test_pass3.rs
                                                                    📄 test_pass4.rs
                                                                  📁 tree-sitter-mml/
                                                                    📜 grammar.js
                                                                    📊 package.json
                                                                    📁 src/
                                                                      📊 grammar.json
                                                                      📊 node-types.json
                                                                      📄 parser.c
                                                                      📁 tree_sitter/
                                                                        📄 alloc.h
                                                                        📄 array.h
                                                                        📄 parser.h
                                                                📄 _config.yml
                                                                📄 build.rs
                                                                📁 generated-docs/
                                                                📁 issue-notes/
                                                                  📖 14.md
                                                                📄 mmlabc-to-smf-rust.toml.example
                                                                📁 src/
                                                                  📄 config.rs
                                                                  📄 lib.rs
                                                                  📄 main.rs
                                                                  📄 pass1_parser.rs
                                                                  📄 pass2_ast.rs
                                                                  📄 pass3_events.rs
                                                                  📄 pass4_midi.rs
                                                                  📄 tree_sitter_mml.rs
                                                                  📄 types.rs
                                                                📁 tests/
                                                                  📄 integration_test.rs
                                                                  📄 test_channel.rs
                                                                  📄 test_cli.rs
                                                                  📄 test_config.rs
                                                                  📄 test_pass1.rs
                                                                  📄 test_pass2.rs
                                                                  📄 test_pass3.rs
                                                                  📄 test_pass4.rs
                                                                📁 tree-sitter-mml/
                                                                  📜 grammar.js
                                                                  📊 package.json
                                                                  📁 src/
                                                                    📊 grammar.json
                                                                    📊 node-types.json
                                                                    📄 parser.c
                                                                    📁 tree_sitter/
                                                                      📄 alloc.h
                                                                      📄 array.h
                                                                      📄 parser.h
                                                              📄 _config.yml
                                                              📄 build.rs
                                                              📁 generated-docs/
                                                              📁 issue-notes/
                                                                📖 14.md
                                                              📄 mmlabc-to-smf-rust.toml.example
                                                              📁 src/
                                                                📄 config.rs
                                                                📄 lib.rs
                                                                📄 main.rs
                                                                📄 pass1_parser.rs
                                                                📄 pass2_ast.rs
                                                                📄 pass3_events.rs
                                                                📄 pass4_midi.rs
                                                                📄 tree_sitter_mml.rs
                                                                📄 types.rs
                                                              📁 tests/
                                                                📄 integration_test.rs
                                                                📄 test_channel.rs
                                                                📄 test_cli.rs
                                                                📄 test_config.rs
                                                                📄 test_pass1.rs
                                                                📄 test_pass2.rs
                                                                📄 test_pass3.rs
                                                                📄 test_pass4.rs
                                                              📁 tree-sitter-mml/
                                                                📜 grammar.js
                                                                📊 package.json
                                                                📁 src/
                                                                  📊 grammar.json
                                                                  📊 node-types.json
                                                                  📄 parser.c
                                                                  📁 tree_sitter/
                                                                    📄 alloc.h
                                                                    📄 array.h
                                                                    📄 parser.h
                                                            📄 _config.yml
                                                            📄 build.rs
                                                            📁 generated-docs/
                                                            📁 issue-notes/
                                                              📖 14.md
                                                            📄 mmlabc-to-smf-rust.toml.example
                                                            📁 src/
                                                              📄 config.rs
                                                              📄 lib.rs
                                                              📄 main.rs
                                                              📄 pass1_parser.rs
                                                              📄 pass2_ast.rs
                                                              📄 pass3_events.rs
                                                              📄 pass4_midi.rs
                                                              📄 tree_sitter_mml.rs
                                                              📄 types.rs
                                                            📁 tests/
                                                              📄 integration_test.rs
                                                              📄 test_channel.rs
                                                              📄 test_cli.rs
                                                              📄 test_config.rs
                                                              📄 test_pass1.rs
                                                              📄 test_pass2.rs
                                                              📄 test_pass3.rs
                                                              📄 test_pass4.rs
                                                            📁 tree-sitter-mml/
                                                              📜 grammar.js
                                                              📊 package.json
                                                              📁 src/
                                                                📊 grammar.json
                                                                📊 node-types.json
                                                                📄 parser.c
                                                                📁 tree_sitter/
                                                                  📄 alloc.h
                                                                  📄 array.h
                                                                  📄 parser.h
                                                          📄 _config.yml
                                                          📄 build.rs
                                                          📁 generated-docs/
                                                          📁 issue-notes/
                                                            📖 14.md
                                                          📄 mmlabc-to-smf-rust.toml.example
                                                          📁 src/
                                                            📄 config.rs
                                                            📄 lib.rs
                                                            📄 main.rs
                                                            📄 pass1_parser.rs
                                                            📄 pass2_ast.rs
                                                            📄 pass3_events.rs
                                                            📄 pass4_midi.rs
                                                            📄 tree_sitter_mml.rs
                                                            📄 types.rs
                                                          📁 tests/
                                                            📄 integration_test.rs
                                                            📄 test_channel.rs
                                                            📄 test_cli.rs
                                                            📄 test_config.rs
                                                            📄 test_pass1.rs
                                                            📄 test_pass2.rs
                                                            📄 test_pass3.rs
                                                            📄 test_pass4.rs
                                                          📁 tree-sitter-mml/
                                                            📜 grammar.js
                                                            📊 package.json
                                                            📁 src/
                                                              📊 grammar.json
                                                              📊 node-types.json
                                                              📄 parser.c
                                                              📁 tree_sitter/
                                                                📄 alloc.h
                                                                📄 array.h
                                                                📄 parser.h
                                                        📄 _config.yml
                                                        📄 build.rs
                                                        📁 generated-docs/
                                                        📁 issue-notes/
                                                          📖 14.md
                                                        📄 mmlabc-to-smf-rust.toml.example
                                                        📁 src/
                                                          📄 config.rs
                                                          📄 lib.rs
                                                          📄 main.rs
                                                          📄 pass1_parser.rs
                                                          📄 pass2_ast.rs
                                                          📄 pass3_events.rs
                                                          📄 pass4_midi.rs
                                                          📄 tree_sitter_mml.rs
                                                          📄 types.rs
                                                        📁 tests/
                                                          📄 integration_test.rs
                                                          📄 test_channel.rs
                                                          📄 test_cli.rs
                                                          📄 test_config.rs
                                                          📄 test_pass1.rs
                                                          📄 test_pass2.rs
                                                          📄 test_pass3.rs
                                                          📄 test_pass4.rs
                                                        📁 tree-sitter-mml/
                                                          📜 grammar.js
                                                          📊 package.json
                                                          📁 src/
                                                            📊 grammar.json
                                                            📊 node-types.json
                                                            📄 parser.c
                                                            📁 tree_sitter/
                                                              📄 alloc.h
                                                              📄 array.h
                                                              📄 parser.h
                                                      📄 _config.yml
                                                      📄 build.rs
                                                      📁 generated-docs/
                                                      📁 issue-notes/
                                                        📖 14.md
                                                      📄 mmlabc-to-smf-rust.toml.example
                                                      📁 src/
                                                        📄 config.rs
                                                        📄 lib.rs
                                                        📄 main.rs
                                                        📄 pass1_parser.rs
                                                        📄 pass2_ast.rs
                                                        📄 pass3_events.rs
                                                        📄 pass4_midi.rs
                                                        📄 tree_sitter_mml.rs
                                                        📄 types.rs
                                                      📁 tests/
                                                        📄 integration_test.rs
                                                        📄 test_channel.rs
                                                        📄 test_cli.rs
                                                        📄 test_config.rs
                                                        📄 test_pass1.rs
                                                        📄 test_pass2.rs
                                                        📄 test_pass3.rs
                                                        📄 test_pass4.rs
                                                      📁 tree-sitter-mml/
                                                        📜 grammar.js
                                                        📊 package.json
                                                        📁 src/
                                                          📊 grammar.json
                                                          📊 node-types.json
                                                          📄 parser.c
                                                          📁 tree_sitter/
                                                            📄 alloc.h
                                                            📄 array.h
                                                            📄 parser.h
                                                    📄 _config.yml
                                                    📄 build.rs
                                                    📁 generated-docs/
                                                    📁 issue-notes/
                                                      📖 14.md
                                                    📄 mmlabc-to-smf-rust.toml.example
                                                    📁 src/
                                                      📄 config.rs
                                                      📄 lib.rs
                                                      📄 main.rs
                                                      📄 pass1_parser.rs
                                                      📄 pass2_ast.rs
                                                      📄 pass3_events.rs
                                                      📄 pass4_midi.rs
                                                      📄 tree_sitter_mml.rs
                                                      📄 types.rs
                                                    📁 tests/
                                                      📄 integration_test.rs
                                                      📄 test_channel.rs
                                                      📄 test_cli.rs
                                                      📄 test_config.rs
                                                      📄 test_pass1.rs
                                                      📄 test_pass2.rs
                                                      📄 test_pass3.rs
                                                      📄 test_pass4.rs
                                                    📁 tree-sitter-mml/
                                                      📜 grammar.js
                                                      📊 package.json
                                                      📁 src/
                                                        📊 grammar.json
                                                        📊 node-types.json
                                                        📄 parser.c
                                                        📁 tree_sitter/
                                                          📄 alloc.h
                                                          📄 array.h
                                                          📄 parser.h
                                                  📄 _config.yml
                                                  📄 build.rs
                                                  📁 generated-docs/
                                                  📁 issue-notes/
                                                    📖 14.md
                                                  📄 mmlabc-to-smf-rust.toml.example
                                                  📁 src/
                                                    📄 config.rs
                                                    📄 lib.rs
                                                    📄 main.rs
                                                    📄 pass1_parser.rs
                                                    📄 pass2_ast.rs
                                                    📄 pass3_events.rs
                                                    📄 pass4_midi.rs
                                                    📄 tree_sitter_mml.rs
                                                    📄 types.rs
                                                  📁 tests/
                                                    📄 integration_test.rs
                                                    📄 test_channel.rs
                                                    📄 test_cli.rs
                                                    📄 test_config.rs
                                                    📄 test_pass1.rs
                                                    📄 test_pass2.rs
                                                    📄 test_pass3.rs
                                                    📄 test_pass4.rs
                                                  📁 tree-sitter-mml/
                                                    📜 grammar.js
                                                    📊 package.json
                                                    📁 src/
                                                      📊 grammar.json
                                                      📊 node-types.json
                                                      📄 parser.c
                                                      📁 tree_sitter/
                                                        📄 alloc.h
                                                        📄 array.h
                                                        📄 parser.h
                                                📄 _config.yml
                                                📄 build.rs
                                                📁 generated-docs/
                                                📁 issue-notes/
                                                  📖 14.md
                                                📄 mmlabc-to-smf-rust.toml.example
                                                📁 src/
                                                  📄 config.rs
                                                  📄 lib.rs
                                                  📄 main.rs
                                                  📄 pass1_parser.rs
                                                  📄 pass2_ast.rs
                                                  📄 pass3_events.rs
                                                  📄 pass4_midi.rs
                                                  📄 tree_sitter_mml.rs
                                                  📄 types.rs
                                                📁 tests/
                                                  📄 integration_test.rs
                                                  📄 test_channel.rs
                                                  📄 test_cli.rs
                                                  📄 test_config.rs
                                                  📄 test_pass1.rs
                                                  📄 test_pass2.rs
                                                  📄 test_pass3.rs
                                                  📄 test_pass4.rs
                                                📁 tree-sitter-mml/
                                                  📜 grammar.js
                                                  📊 package.json
                                                  📁 src/
                                                    📊 grammar.json
                                                    📊 node-types.json
                                                    📄 parser.c
                                                    📁 tree_sitter/
                                                      📄 alloc.h
                                                      📄 array.h
                                                      📄 parser.h
                                              📄 _config.yml
                                              📄 build.rs
                                              📁 generated-docs/
                                              📁 issue-notes/
                                                📖 14.md
                                              📄 mmlabc-to-smf-rust.toml.example
                                              📁 src/
                                                📄 config.rs
                                                📄 lib.rs
                                                📄 main.rs
                                                📄 pass1_parser.rs
                                                📄 pass2_ast.rs
                                                📄 pass3_events.rs
                                                📄 pass4_midi.rs
                                                📄 tree_sitter_mml.rs
                                                📄 types.rs
                                              📁 tests/
                                                📄 integration_test.rs
                                                📄 test_channel.rs
                                                📄 test_cli.rs
                                                📄 test_config.rs
                                                📄 test_pass1.rs
                                                📄 test_pass2.rs
                                                📄 test_pass3.rs
                                                📄 test_pass4.rs
                                              📁 tree-sitter-mml/
                                                📜 grammar.js
                                                📊 package.json
                                                📁 src/
                                                  📊 grammar.json
                                                  📊 node-types.json
                                                  📄 parser.c
                                                  📁 tree_sitter/
                                                    📄 alloc.h
                                                    📄 array.h
                                                    📄 parser.h
                                            📄 _config.yml
                                            📄 build.rs
                                            📁 generated-docs/
                                            📁 issue-notes/
                                              📖 14.md
                                            📄 mmlabc-to-smf-rust.toml.example
                                            📁 src/
                                              📄 config.rs
                                              📄 lib.rs
                                              📄 main.rs
                                              📄 pass1_parser.rs
                                              📄 pass2_ast.rs
                                              📄 pass3_events.rs
                                              📄 pass4_midi.rs
                                              📄 tree_sitter_mml.rs
                                              📄 types.rs
                                            📁 tests/
                                              📄 integration_test.rs
                                              📄 test_channel.rs
                                              📄 test_cli.rs
                                              📄 test_config.rs
                                              📄 test_pass1.rs
                                              📄 test_pass2.rs
                                              📄 test_pass3.rs
                                              📄 test_pass4.rs
                                            📁 tree-sitter-mml/
                                              📜 grammar.js
                                              📊 package.json
                                              📁 src/
                                                📊 grammar.json
                                                📊 node-types.json
                                                📄 parser.c
                                                📁 tree_sitter/
                                                  📄 alloc.h
                                                  📄 array.h
                                                  📄 parser.h
                                          📄 _config.yml
                                          📄 build.rs
                                          📁 generated-docs/
                                          📁 issue-notes/
                                            📖 14.md
                                          📄 mmlabc-to-smf-rust.toml.example
                                          📁 src/
                                            📄 config.rs
                                            📄 lib.rs
                                            📄 main.rs
                                            📄 pass1_parser.rs
                                            📄 pass2_ast.rs
                                            📄 pass3_events.rs
                                            📄 pass4_midi.rs
                                            📄 tree_sitter_mml.rs
                                            📄 types.rs
                                          📁 tests/
                                            📄 integration_test.rs
                                            📄 test_channel.rs
                                            📄 test_cli.rs
                                            📄 test_config.rs
                                            📄 test_pass1.rs
                                            📄 test_pass2.rs
                                            📄 test_pass3.rs
                                            📄 test_pass4.rs
                                          📁 tree-sitter-mml/
                                            📜 grammar.js
                                            📊 package.json
                                            📁 src/
                                              📊 grammar.json
                                              📊 node-types.json
                                              📄 parser.c
                                              📁 tree_sitter/
                                                📄 alloc.h
                                                📄 array.h
                                                📄 parser.h
                                        📄 _config.yml
                                        📄 build.rs
                                        📁 generated-docs/
                                        📁 issue-notes/
                                          📖 14.md
                                        📄 mmlabc-to-smf-rust.toml.example
                                        📁 src/
                                          📄 config.rs
                                          📄 lib.rs
                                          📄 main.rs
                                          📄 pass1_parser.rs
                                          📄 pass2_ast.rs
                                          📄 pass3_events.rs
                                          📄 pass4_midi.rs
                                          📄 tree_sitter_mml.rs
                                          📄 types.rs
                                        📁 tests/
                                          📄 integration_test.rs
                                          📄 test_channel.rs
                                          📄 test_cli.rs
                                          📄 test_config.rs
                                          📄 test_pass1.rs
                                          📄 test_pass2.rs
                                          📄 test_pass3.rs
                                          📄 test_pass4.rs
                                        📁 tree-sitter-mml/
                                          📜 grammar.js
                                          📊 package.json
                                          📁 src/
                                            📊 grammar.json
                                            📊 node-types.json
                                            📄 parser.c
                                            📁 tree_sitter/
                                              📄 alloc.h
                                              📄 array.h
                                              📄 parser.h
                                      📄 _config.yml
                                      📄 build.rs
                                      📁 generated-docs/
                                      📁 issue-notes/
                                        📖 14.md
                                      📄 mmlabc-to-smf-rust.toml.example
                                      📁 src/
                                        📄 config.rs
                                        📄 lib.rs
                                        📄 main.rs
                                        📄 pass1_parser.rs
                                        📄 pass2_ast.rs
                                        📄 pass3_events.rs
                                        📄 pass4_midi.rs
                                        📄 tree_sitter_mml.rs
                                        📄 types.rs
                                      📁 tests/
                                        📄 integration_test.rs
                                        📄 test_channel.rs
                                        📄 test_cli.rs
                                        📄 test_config.rs
                                        📄 test_pass1.rs
                                        📄 test_pass2.rs
                                        📄 test_pass3.rs
                                        📄 test_pass4.rs
                                      📁 tree-sitter-mml/
                                        📜 grammar.js
                                        📊 package.json
                                        📁 src/
                                          📊 grammar.json
                                          📊 node-types.json
                                          📄 parser.c
                                          📁 tree_sitter/
                                            📄 alloc.h
                                            📄 array.h
                                            📄 parser.h
                                    📄 _config.yml
                                    📄 build.rs
                                    📁 generated-docs/
                                    📁 issue-notes/
                                      📖 14.md
                                    📄 mmlabc-to-smf-rust.toml.example
                                    📁 src/
                                      📄 config.rs
                                      📄 lib.rs
                                      📄 main.rs
                                      📄 pass1_parser.rs
                                      📄 pass2_ast.rs
                                      📄 pass3_events.rs
                                      📄 pass4_midi.rs
                                      📄 tree_sitter_mml.rs
                                      📄 types.rs
                                    📁 tests/
                                      📄 integration_test.rs
                                      📄 test_channel.rs
                                      📄 test_cli.rs
                                      📄 test_config.rs
                                      📄 test_pass1.rs
                                      📄 test_pass2.rs
                                      📄 test_pass3.rs
                                      📄 test_pass4.rs
                                    📁 tree-sitter-mml/
                                      📜 grammar.js
                                      📊 package.json
                                      📁 src/
                                        📊 grammar.json
                                        📊 node-types.json
                                        📄 parser.c
                                        📁 tree_sitter/
                                          📄 alloc.h
                                          📄 array.h
                                          📄 parser.h
                                  📄 _config.yml
                                  📄 build.rs
                                  📁 generated-docs/
                                  📁 issue-notes/
                                    📖 14.md
                                  📄 mmlabc-to-smf-rust.toml.example
                                  📁 src/
                                    📄 config.rs
                                    📄 lib.rs
                                    📄 main.rs
                                    📄 pass1_parser.rs
                                    📄 pass2_ast.rs
                                    📄 pass3_events.rs
                                    📄 pass4_midi.rs
                                    📄 tree_sitter_mml.rs
                                    📄 types.rs
                                  📁 tests/
                                    📄 integration_test.rs
                                    📄 test_channel.rs
                                    📄 test_cli.rs
                                    📄 test_config.rs
                                    📄 test_pass1.rs
                                    📄 test_pass2.rs
                                    📄 test_pass3.rs
                                    📄 test_pass4.rs
                                  📁 tree-sitter-mml/
                                    📜 grammar.js
                                    📊 package.json
                                    📁 src/
                                      📊 grammar.json
                                      📊 node-types.json
                                      📄 parser.c
                                      📁 tree_sitter/
                                        📄 alloc.h
                                        📄 array.h
                                        📄 parser.h
                                📄 _config.yml
                                📄 build.rs
                                📁 generated-docs/
                                📁 issue-notes/
                                  📖 14.md
                                📄 mmlabc-to-smf-rust.toml.example
                                📁 src/
                                  📄 config.rs
                                  📄 lib.rs
                                  📄 main.rs
                                  📄 pass1_parser.rs
                                  📄 pass2_ast.rs
                                  📄 pass3_events.rs
                                  📄 pass4_midi.rs
                                  📄 tree_sitter_mml.rs
                                  📄 types.rs
                                📁 tests/
                                  📄 integration_test.rs
                                  📄 test_channel.rs
                                  📄 test_cli.rs
                                  📄 test_config.rs
                                  📄 test_pass1.rs
                                  📄 test_pass2.rs
                                  📄 test_pass3.rs
                                  📄 test_pass4.rs
                                📁 tree-sitter-mml/
                                  📜 grammar.js
                                  📊 package.json
                                  📁 src/
                                    📊 grammar.json
                                    📊 node-types.json
                                    📄 parser.c
                                    📁 tree_sitter/
                                      📄 alloc.h
                                      📄 array.h
                                      📄 parser.h
                              📄 _config.yml
                              📄 build.rs
                              📁 generated-docs/
                              📁 issue-notes/
                                📖 14.md
                              📄 mmlabc-to-smf-rust.toml.example
                              📁 src/
                                📄 config.rs
                                📄 lib.rs
                                📄 main.rs
                                📄 pass1_parser.rs
                                📄 pass2_ast.rs
                                📄 pass3_events.rs
                                📄 pass4_midi.rs
                                📄 tree_sitter_mml.rs
                                📄 types.rs
                              📁 tests/
                                📄 integration_test.rs
                                📄 test_channel.rs
                                📄 test_cli.rs
                                📄 test_config.rs
                                📄 test_pass1.rs
                                📄 test_pass2.rs
                                📄 test_pass3.rs
                                📄 test_pass4.rs
                              📁 tree-sitter-mml/
                                📜 grammar.js
                                📊 package.json
                                📁 src/
                                  📊 grammar.json
                                  📊 node-types.json
                                  📄 parser.c
                                  📁 tree_sitter/
                                    📄 alloc.h
                                    📄 array.h
                                    📄 parser.h
                            📄 _config.yml
                            📄 build.rs
                            📁 generated-docs/
                            📁 issue-notes/
                              📖 14.md
                            📄 mmlabc-to-smf-rust.toml.example
                            📁 src/
                              📄 config.rs
                              📄 lib.rs
                              📄 main.rs
                              📄 pass1_parser.rs
                              📄 pass2_ast.rs
                              📄 pass3_events.rs
                              📄 pass4_midi.rs
                              📄 tree_sitter_mml.rs
                              📄 types.rs
                            📁 tests/
                              📄 integration_test.rs
                              📄 test_channel.rs
                              📄 test_cli.rs
                              📄 test_config.rs
                              📄 test_pass1.rs
                              📄 test_pass2.rs
                              📄 test_pass3.rs
                              📄 test_pass4.rs
                            📁 tree-sitter-mml/
                              📜 grammar.js
                              📊 package.json
                              📁 src/
                                📊 grammar.json
                                📊 node-types.json
                                📄 parser.c
                                📁 tree_sitter/
                                  📄 alloc.h
                                  📄 array.h
                                  📄 parser.h
                          📄 _config.yml
                          📄 build.rs
                          📁 generated-docs/
                          📁 issue-notes/
                            📖 14.md
                          📄 mmlabc-to-smf-rust.toml.example
                          📁 src/
                            📄 config.rs
                            📄 lib.rs
                            📄 main.rs
                            📄 pass1_parser.rs
                            📄 pass2_ast.rs
                            📄 pass3_events.rs
                            📄 pass4_midi.rs
                            📄 tree_sitter_mml.rs
                            📄 types.rs
                          📁 tests/
                            📄 integration_test.rs
                            📄 test_channel.rs
                            📄 test_cli.rs
                            📄 test_config.rs
                            📄 test_pass1.rs
                            📄 test_pass2.rs
                            📄 test_pass3.rs
                            📄 test_pass4.rs
                          📁 tree-sitter-mml/
                            📜 grammar.js
                            📊 package.json
                            📁 src/
                              📊 grammar.json
                              📊 node-types.json
                              📄 parser.c
                              📁 tree_sitter/
                                📄 alloc.h
                                📄 array.h
                                📄 parser.h
                        📄 _config.yml
                        📄 build.rs
                        📁 generated-docs/
                        📁 issue-notes/
                          📖 14.md
                        📄 mmlabc-to-smf-rust.toml.example
                        📁 src/
                          📄 config.rs
                          📄 lib.rs
                          📄 main.rs
                          📄 pass1_parser.rs
                          📄 pass2_ast.rs
                          📄 pass3_events.rs
                          📄 pass4_midi.rs
                          📄 tree_sitter_mml.rs
                          📄 types.rs
                        📁 tests/
                          📄 integration_test.rs
                          📄 test_channel.rs
                          📄 test_cli.rs
                          📄 test_config.rs
                          📄 test_pass1.rs
                          📄 test_pass2.rs
                          📄 test_pass3.rs
                          📄 test_pass4.rs
                        📁 tree-sitter-mml/
                          📜 grammar.js
                          📊 package.json
                          📁 src/
                            📊 grammar.json
                            📊 node-types.json
                            📄 parser.c
                            📁 tree_sitter/
                              📄 alloc.h
                              📄 array.h
                              📄 parser.h
                      📄 _config.yml
                      📄 build.rs
                      📁 generated-docs/
                      📁 issue-notes/
                        📖 14.md
                      📄 mmlabc-to-smf-rust.toml.example
                      📁 src/
                        📄 config.rs
                        📄 lib.rs
                        📄 main.rs
                        📄 pass1_parser.rs
                        📄 pass2_ast.rs
                        📄 pass3_events.rs
                        📄 pass4_midi.rs
                        📄 tree_sitter_mml.rs
                        📄 types.rs
                      📁 tests/
                        📄 integration_test.rs
                        📄 test_channel.rs
                        📄 test_cli.rs
                        📄 test_config.rs
                        📄 test_pass1.rs
                        📄 test_pass2.rs
                        📄 test_pass3.rs
                        📄 test_pass4.rs
                      📁 tree-sitter-mml/
                        📜 grammar.js
                        📊 package.json
                        📁 src/
                          📊 grammar.json
                          📊 node-types.json
                          📄 parser.c
                          📁 tree_sitter/
                            📄 alloc.h
                            📄 array.h
                            📄 parser.h
                    📄 _config.yml
                    📄 build.rs
                    📁 generated-docs/
                    📁 issue-notes/
                      📖 14.md
                    📄 mmlabc-to-smf-rust.toml.example
                    📁 src/
                      📄 config.rs
                      📄 lib.rs
                      📄 main.rs
                      📄 pass1_parser.rs
                      📄 pass2_ast.rs
                      📄 pass3_events.rs
                      📄 pass4_midi.rs
                      📄 tree_sitter_mml.rs
                      📄 types.rs
                    📁 tests/
                      📄 integration_test.rs
                      📄 test_channel.rs
                      📄 test_cli.rs
                      📄 test_config.rs
                      📄 test_pass1.rs
                      📄 test_pass2.rs
                      📄 test_pass3.rs
                      📄 test_pass4.rs
                    📁 tree-sitter-mml/
                      📜 grammar.js
                      📊 package.json
                      📁 src/
                        📊 grammar.json
                        📊 node-types.json
                        📄 parser.c
                        📁 tree_sitter/
                          📄 alloc.h
                          📄 array.h
                          📄 parser.h
                  📄 _config.yml
                  📄 build.rs
                  📁 generated-docs/
                  📁 issue-notes/
                    📖 14.md
                  📄 mmlabc-to-smf-rust.toml.example
                  📁 src/
                    📄 config.rs
                    📄 lib.rs
                    📄 main.rs
                    📄 pass1_parser.rs
                    📄 pass2_ast.rs
                    📄 pass3_events.rs
                    📄 pass4_midi.rs
                    📄 tree_sitter_mml.rs
                    📄 types.rs
                  📁 tests/
                    📄 integration_test.rs
                    📄 test_channel.rs
                    📄 test_cli.rs
                    📄 test_config.rs
                    📄 test_pass1.rs
                    📄 test_pass2.rs
                    📄 test_pass3.rs
                    📄 test_pass4.rs
                  📁 tree-sitter-mml/
                    📜 grammar.js
                    📊 package.json
                    📁 src/
                      📊 grammar.json
                      📊 node-types.json
                      📄 parser.c
                      📁 tree_sitter/
                        📄 alloc.h
                        📄 array.h
                        📄 parser.h
                📄 _config.yml
                📄 build.rs
                📁 generated-docs/
                📁 issue-notes/
                  📖 14.md
                📄 mmlabc-to-smf-rust.toml.example
                📁 src/
                  📄 config.rs
                  📄 lib.rs
                  📄 main.rs
                  📄 pass1_parser.rs
                  📄 pass2_ast.rs
                  📄 pass3_events.rs
                  📄 pass4_midi.rs
                  📄 tree_sitter_mml.rs
                  📄 types.rs
                📁 tests/
                  📄 integration_test.rs
                  📄 test_channel.rs
                  📄 test_cli.rs
                  📄 test_config.rs
                  📄 test_pass1.rs
                  📄 test_pass2.rs
                  📄 test_pass3.rs
                  📄 test_pass4.rs
                📁 tree-sitter-mml/
                  📜 grammar.js
                  📊 package.json
                  📁 src/
                    📊 grammar.json
                    📊 node-types.json
                    📄 parser.c
                    📁 tree_sitter/
                      📄 alloc.h
                      📄 array.h
                      📄 parser.h
              📄 _config.yml
              📄 build.rs
              📁 generated-docs/
              📁 issue-notes/
                📖 14.md
              📄 mmlabc-to-smf-rust.toml.example
              📁 src/
                📄 config.rs
                📄 lib.rs
                📄 main.rs
                📄 pass1_parser.rs
                📄 pass2_ast.rs
                📄 pass3_events.rs
                📄 pass4_midi.rs
                📄 tree_sitter_mml.rs
                📄 types.rs
              📁 tests/
                📄 integration_test.rs
                📄 test_channel.rs
                📄 test_cli.rs
                📄 test_config.rs
                📄 test_pass1.rs
                📄 test_pass2.rs
                📄 test_pass3.rs
                📄 test_pass4.rs
              📁 tree-sitter-mml/
                📜 grammar.js
                📊 package.json
                📁 src/
                  📊 grammar.json
                  📊 node-types.json
                  📄 parser.c
                  📁 tree_sitter/
                    📄 alloc.h
                    📄 array.h
                    📄 parser.h
            📄 _config.yml
            📄 build.rs
            📁 generated-docs/
            📁 issue-notes/
              📖 14.md
            📄 mmlabc-to-smf-rust.toml.example
            📁 src/
              📄 config.rs
              📄 lib.rs
              📄 main.rs
              📄 pass1_parser.rs
              📄 pass2_ast.rs
              📄 pass3_events.rs
              📄 pass4_midi.rs
              📄 tree_sitter_mml.rs
              📄 types.rs
            📁 tests/
              📄 integration_test.rs
              📄 test_channel.rs
              📄 test_cli.rs
              📄 test_config.rs
              📄 test_pass1.rs
              📄 test_pass2.rs
              📄 test_pass3.rs
              📄 test_pass4.rs
            📁 tree-sitter-mml/
              📜 grammar.js
              📊 package.json
              📁 src/
                📊 grammar.json
                📊 node-types.json
                📄 parser.c
                📁 tree_sitter/
                  📄 alloc.h
                  📄 array.h
                  📄 parser.h
          📄 _config.yml
          📄 build.rs
          📁 generated-docs/
          📁 issue-notes/
            📖 14.md
          📄 mmlabc-to-smf-rust.toml.example
          📁 src/
            📄 config.rs
            📄 lib.rs
            📄 main.rs
            📄 pass1_parser.rs
            📄 pass2_ast.rs
            📄 pass3_events.rs
            📄 pass4_midi.rs
            📄 tree_sitter_mml.rs
            📄 types.rs
          📁 tests/
            📄 integration_test.rs
            📄 test_channel.rs
            📄 test_cli.rs
            📄 test_config.rs
            📄 test_pass1.rs
            📄 test_pass2.rs
            📄 test_pass3.rs
            📄 test_pass4.rs
          📁 tree-sitter-mml/
            📜 grammar.js
            📊 package.json
            📁 src/
              📊 grammar.json
              📊 node-types.json
              📄 parser.c
              📁 tree_sitter/
                📄 alloc.h
                📄 array.h
                📄 parser.h
        📄 _config.yml
        📄 build.rs
        📁 generated-docs/
        📁 issue-notes/
          📖 14.md
        📄 mmlabc-to-smf-rust.toml.example
        📁 src/
          📄 config.rs
          📄 lib.rs
          📄 main.rs
          📄 pass1_parser.rs
          📄 pass2_ast.rs
          📄 pass3_events.rs
          📄 pass4_midi.rs
          📄 tree_sitter_mml.rs
          📄 types.rs
        📁 tests/
          📄 integration_test.rs
          📄 test_channel.rs
          📄 test_cli.rs
          📄 test_config.rs
          📄 test_pass1.rs
          📄 test_pass2.rs
          📄 test_pass3.rs
          📄 test_pass4.rs
        📁 tree-sitter-mml/
          📜 grammar.js
          📊 package.json
          📁 src/
            📊 grammar.json
            📊 node-types.json
            📄 parser.c
            📁 tree_sitter/
              📄 alloc.h
              📄 array.h
              📄 parser.h
      📄 _config.yml
      📄 build.rs
      📁 generated-docs/
      📁 issue-notes/
        📖 14.md
      📄 mmlabc-to-smf-rust.toml.example
      📁 src/
        📄 config.rs
        📄 lib.rs
        📄 main.rs
        📄 pass1_parser.rs
        📄 pass2_ast.rs
        📄 pass3_events.rs
        📄 pass4_midi.rs
        📄 tree_sitter_mml.rs
        📄 types.rs
      📁 tests/
        📄 integration_test.rs
        📄 test_channel.rs
        📄 test_cli.rs
        📄 test_config.rs
        📄 test_pass1.rs
        📄 test_pass2.rs
        📄 test_pass3.rs
        📄 test_pass4.rs
      📁 tree-sitter-mml/
        📜 grammar.js
        📊 package.json
        📁 src/
          📊 grammar.json
          📊 node-types.json
          📄 parser.c
          📁 tree_sitter/
            📄 alloc.h
            📄 array.h
            📄 parser.h
    📄 _config.yml
    📄 build.rs
    📁 generated-docs/
    📁 issue-notes/
      📖 14.md
    📄 mmlabc-to-smf-rust.toml.example
    📁 src/
      📄 config.rs
      📄 lib.rs
      📄 main.rs
      📄 pass1_parser.rs
      📄 pass2_ast.rs
      📄 pass3_events.rs
      📄 pass4_midi.rs
      📄 tree_sitter_mml.rs
      📄 types.rs
    📁 tests/
      📄 integration_test.rs
      📄 test_channel.rs
      📄 test_cli.rs
      📄 test_config.rs
      📄 test_pass1.rs
      📄 test_pass2.rs
      📄 test_pass3.rs
      📄 test_pass4.rs
    📁 tree-sitter-mml/
      📜 grammar.js
      📊 package.json
      📁 src/
        📊 grammar.json
        📊 node-types.json
        📄 parser.c
        📁 tree_sitter/
          📄 alloc.h
          📄 array.h
          📄 parser.h
  📄 _config.yml
  📄 build.rs
  📁 generated-docs/
  📁 issue-notes/
    📖 14.md
  📄 mmlabc-to-smf-rust.toml.example
  📁 src/
    📄 config.rs
    📄 lib.rs
    📄 main.rs
    📄 pass1_parser.rs
    📄 pass2_ast.rs
    📄 pass3_events.rs
    📄 pass4_midi.rs
    📄 tree_sitter_mml.rs
    📄 types.rs
  📁 tests/
    📄 integration_test.rs
    📄 test_channel.rs
    📄 test_cli.rs
    📄 test_config.rs
    📄 test_pass1.rs
    📄 test_pass2.rs
    📄 test_pass3.rs
    📄 test_pass4.rs
  📁 tree-sitter-mml/
    📜 grammar.js
    📊 package.json
    📁 src/
      📊 grammar.json
      📊 node-types.json
      📄 parser.c
      📁 tree_sitter/
        📄 alloc.h
        📄 array.h
        📄 parser.h
📄 _config.yml
📄 build.rs
📁 generated-docs/
📁 issue-notes/
  📖 14.md
📄 mmlabc-to-smf-rust.toml.example
📁 src/
  📄 config.rs
  📄 lib.rs
  📄 main.rs
  📄 pass1_parser.rs
  📄 pass2_ast.rs
  📄 pass3_events.rs
  📄 pass4_midi.rs
  📄 tree_sitter_mml.rs
  📄 types.rs
📁 tests/
  📄 integration_test.rs
  📄 test_channel.rs
  📄 test_cli.rs
  📄 test_config.rs
  📄 test_pass1.rs
  📄 test_pass2.rs
  📄 test_pass3.rs
  📄 test_pass4.rs
📁 tree-sitter-mml/
  📜 grammar.js
  📊 package.json
  📁 src/
    📊 grammar.json
    📊 node-types.json
    📄 parser.c
    📁 tree_sitter/
      📄 alloc.h
      📄 array.h
      📄 parser.h

## ファイル詳細分析
**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

**tree-sitter-mml/grammar.js** (14行, 183バイト)
  - 関数: なし
  - インポート: なし

## 関数呼び出し階層
関数呼び出し階層を分析できませんでした

## プロジェクト構造（ファイル一覧）
.vscode/settings.json
README.ja.md
README.md
_codeql_detected_source_root/.vscode/settings.json
_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/.vscode/settings.json
_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/.vscode/settings.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/.vscode/settings.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/.vscode/settings.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/.vscode/settings.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/.vscode/settings.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/.vscode/settings.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/.vscode/settings.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/14.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/14.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/14.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/14.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/14.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/14.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js
_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/14.md
_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js
_codeql_detected_source_root/issue-notes/14.md
_codeql_detected_source_root/tree-sitter-mml/grammar.js
issue-notes/14.md
tree-sitter-mml/grammar.js

上記の情報を基に、プロンプトで指定された形式でプロジェクト概要を生成してください。
特に以下の点を重視してください：
- 技術スタックは各カテゴリごとに整理して説明
- ファイル階層ツリーは提供された構造をそのまま使用
- ファイルの説明は各ファイルの実際の内容と機能に基づく
- 関数の説明は実際に検出された関数の役割に基づく
- 関数呼び出し階層は実際の呼び出し関係に基づく


---
Generated at: 2025-11-14 07:05:57 JST
