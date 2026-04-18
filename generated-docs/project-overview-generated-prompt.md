Last updated: 2026-04-19


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
名前: Unknown
説明: # mmlabc-to-smf-rust

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
  <a href="https://deepwiki.com/cat2151/mmlabc-to-smf-rust"><img src="https://img.shields.io/badge/📖-DeepWiki-blue.svg" alt="DeepWiki"></a>
</p>

Music Macro Language (MML) を Standard MIDI File (SMF) に変換する Rust 製ライブラリ兼 CLI です。

## 概要

- Rust ライブラリ: `mmlabc_to_smf`
- CLI バイナリ: `mmlabc-to-smf`
- 4 パス構成で MML を SMF に変換
- 各パスの中間結果を JSON で出力
- ブラウザ向けに `mmlabc-to-smf-wasm/`、動作確認用に `demo/` と `demo-library/` を同梱

## 現在の実装状況

README の内容を実装に合わせて更新しました。現在のコードベースでは、少なくとも以下が実装・テストされています。

- 基本音符: `c d e f g a b`
- シャープ / フラット: `+`, `-`
- 休符: `r`
- 音長指定: `c4`, `d8`, `l8`, `l4.`
- 付点音符: `c4.`, `c4..`, `c1....`
- オクターブ操作: `<`, `>`, `o4`, `o5`
- 和音: `'ceg'`
- 複数チャンネル: `;`
- テンポ: `t120`
- ベロシティ: `v1` 〜 `v15`
- プログラムチェンジ: `@0` 〜 `@127`
- キー移調: `kt1`, `kt-2`
- 添付 JSON 出力: `--attachment-output`
- MML 先頭の埋め込み添付 JSON: `[{"ProgramChange":1,"Tone":{"events":[]}}]@1cde`
- `@128` を含むチャンネルのドラムチャンネル割当用特殊マーカー（設定で無効化可能）

テスト件数や一覧は `cargo test -- --list` で確認できます。

## 使い方

### CLI

```bash
cargo run -- "cdefgab" --no-play
cargo run -- "t120 l4 c d e f" --no-play
cargo run -- "o4 'ceg' r8 >c" --no-play
cargo run -- "@0c;@128d;@1e" --no-play -o output.mid
```

主なオプション:

- `-o, --output <PATH>`: 出力 SMF ファイル（既定値: `output.mid`）
- `--attachment-output <PATH>`: 添付 JSON を出力
- `--no-play`: 生成後の自動再生を無効化

デフォルトでは生成した MIDI を `cat-play-mml` で再生しようとします。設定ファイルで別のプレイヤーに変更できます。

### ライブラリ

公開モジュール:

- `attachment_json`
- `config`
- `mml_preprocessor`
- `pass2_ast`
- `pass3_events`
- `pass4_midi`
- `types`
- `pass1_parser`, `tree_sitter_mml`（`cli` feature 有効時）

## 対応している MML 記法

| 種別 | 記法 | 例 |
| --- | --- | --- |
| 音符 | `cdefgab` | `cde` |
| 修飾 | `+`, `-` | `c+ d-` |
| 休符 | `r` | `cr8d` |
| 音長 | 数字 / `l` | `c4`, `l8cde` |
| 付点 | `.` | `c4.`, `l4..c` |
| オクターブ | `<` = 1オクターブ上げる, `>` = 1オクターブ下げる, `oN` = オクターブ設定 | `o4c<d>e` |
| 和音 | `'...'` | `'ceg'` |
| チャンネル分割 | `;` | `c;e;g` |
| テンポ | `tN` | `t120c` |
| ベロシティ | `vN` | `v8cde` |
| プログラム | `@N` | `@1c` |
| キー移調 | `ktN`, `kt-N` | `kt2c`, `kt-1d` |

補足:

- デフォルト音長は `l8`（8 分音符）です。
- `v1`〜`v15` は内部で MIDI velocity (`0`〜`127`) に変換されます。
- `@128` は `;` で区切られたチャンネル内で使われた場合に、既定で MIDI channel 9 (0-based) に割り当てられます。

## 添付 JSON

`--attachment-output` を使うと、Program Change ごとの添付 JSON を出力できます。

```bash
cargo run -- "@1cde" --no-play \
  --attachment-output attachment.json \
  -o output.mid
```

また、MML 文字列の先頭に JSON オブジェクトまたは JSON 配列を書くと、その部分を添付 JSON として抽出して利用します。

```text
[{"ProgramChange":1,"Tone":{"events":[]}}]@1cde
```

## 設定ファイル

実行ディレクトリに `mmlabc-to-smf-rust.toml` を置くと挙動を変更できます。

```toml
external_smf_player = "cat-play-mml"
use_drum_channel_for_128 = true
```

- `external_smf_player`: 自動再生に使うコマンド
- `use_drum_channel_for_128`: `@128` を含むチャンネルをドラムチャンネルに割り当てるかどうか

詳細は [`mmlabc-to-smf-rust.toml.example`](mmlabc-to-smf-rust.toml.example) を参照してください。

## デバッグ出力

CLI 実行時には以下のファイルが出力されます。

- `pass1_tokens.json`
- `pass2_ast.json`
- `pass3_events.json`
- `output.mid`（または `--output` で指定したファイル）
- `--attachment-output` 指定時は添付 JSON

## 開発

### ビルド / テスト / Lint

```bash
cargo build
cargo test
cargo clippy --all-targets --all-features
cargo fmt --check
```

必要に応じて整形:

```bash
cargo fmt
```

### tree-sitter 文法を変更する場合

`tree-sitter-mml/grammar.js` を更新したときは、生成物も合わせて更新します。

```bash
cargo build
# または
cd tree-sitter-mml
npm install
npx tree-sitter generate
```

通常の Rust ビルドだけなら、コミット済みの生成物があるため Node.js は必須ではありません。

### デモ

- `demo/`: ブラウザデモ
- `demo-library/`: ライブラリ利用例
- `mmlabc-to-smf-wasm/`: Web 向け WASM クレート

## 参考

- mmlabc コマンド体系: [cat2151/mml2abc](https://github.com/cat2151/mml2abc)
- オリジナル Python 実装: [cat2151/mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf)

## ライセンス

MIT License. 詳細は [LICENSE](LICENSE) を参照してください。


依存関係:
{
  "dependencies": {
    "tree-sitter-cli": "^0.26.5"
  },
  "devDependencies": {}
}

## ファイル階層ツリー
📄 .editorconfig
📄 .gitattributes
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
  📄 .gitattributes
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
    📄 .gitattributes
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
      📄 .gitattributes
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
        📄 .gitattributes
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
          📄 .gitattributes
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
            📄 .gitattributes
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
              📄 .gitattributes
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
                📄 .gitattributes
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
                  📄 .gitattributes
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
                    📄 .gitattributes
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
                      📄 .gitattributes
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
                        📄 .gitattributes
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
                          📄 .gitattributes
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
                            📄 .gitattributes
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
                              📄 .gitattributes
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
                                📄 .gitattributes
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
                                  📄 .gitattributes
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
                                    📄 .gitattributes
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
                                      📄 .gitattributes
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
                                        📄 .gitattributes
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
                                          📄 .gitattributes
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
                                            📄 .gitattributes
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
                                              📄 .gitattributes
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
                                                📄 .gitattributes
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
                                                  📄 .gitattributes
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
                                                    📄 .gitattributes
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
                                                      📄 .gitattributes
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
                                                        📄 .gitattributes
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
                                                          📄 .gitattributes
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
                                                            📄 .gitattributes
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
                                                              📄 .gitattributes
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
                                                                📄 .gitattributes
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
                                                                  📄 .gitattributes
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
                                                                    📄 .gitattributes
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
                                                                      📄 .gitattributes
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
                                                                        📄 .gitattributes
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
                                                                          📄 .gitattributes
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
                                                                            📄 .gitattributes
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
                                                                              📄 .gitattributes
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
                                                                                📄 .gitattributes
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
                                                                                📁 demo/
                                                                                  📄 .gitignore
                                                                                  📖 FEATURES.md
                                                                                  📖 README.md
                                                                                  🌐 index.html
                                                                                  📊 package.json
                                                                                  📁 src/
                                                                                    📘 audioPlayback.ts
                                                                                    📘 audioRenderer.ts
                                                                                    📘 main.ts
                                                                                    📘 midiReader.ts
                                                                                    📘 mmlConverter.ts
                                                                                    📘 parseMidiNotes.ts
                                                                                    📘 smfToYm2151.ts
                                                                                    📘 state.ts
                                                                                    📘 treeToJSON.ts
                                                                                    📘 ui.ts
                                                                                    📘 visualization.ts
                                                                                    📘 wavExport.ts
                                                                                  📄 test-loader.mjs
                                                                                  📄 test-register.mjs
                                                                                  📁 tests/
                                                                                    📘 audioBufferToWav.test.ts
                                                                                    📘 midiReader.test.ts
                                                                                    📘 parseMidiNotes.test.ts
                                                                                    📘 treeToJSON.test.ts
                                                                                📁 demo-library/
                                                                                  🌐 index.html
                                                                                  📊 package.json
                                                                                📁 generated-docs/
                                                                                🌐 googled947dc864c270e07.html
                                                                                📁 issue-notes/
                                                                                  📖 103.md
                                                                                  📖 123.md
                                                                                  📖 133.md
                                                                                  📖 39.md
                                                                                  📖 44.md
                                                                                📄 mmlabc-to-smf-rust.toml.example
                                                                                📁 mmlabc-to-smf-wasm/
                                                                                  📄 Cargo.lock
                                                                                  📄 Cargo.toml
                                                                                  📁 src/
                                                                                    📄 lib.rs
                                                                                    📄 token_extractor.rs
                                                                                  📁 tests/
                                                                                    📄 parity.rs
                                                                                📊 package.json
                                                                                📁 scripts/
                                                                                  📖 README.md
                                                                                  📄 build-demo.sh
                                                                                  📄 transform-demo-paths.sh
                                                                                📁 src/
                                                                                  📄 attachment_json.rs
                                                                                  📄 config.rs
                                                                                  📄 lib.rs
                                                                                  📄 main.rs
                                                                                  📄 mml_preprocessor.rs
                                                                                  📄 parse_tree_tokens.rs
                                                                                  📄 pass1_parser.rs
                                                                                  📄 pass2_ast.rs
                                                                                  📄 pass3_events.rs
                                                                                  📄 pass4_midi.rs
                                                                                  📄 tree_sitter_mml.rs
                                                                                  📄 types.rs
                                                                                📁 tests/
                                                                                  📄 integration_test.rs
                                                                                  📄 test_attachment_json.rs
                                                                                  📄 test_c1_vs_c64.rs
                                                                                  📄 test_channel.rs
                                                                                  📄 test_chord.rs
                                                                                  📄 test_cli.rs
                                                                                  📄 test_config.rs
                                                                                  📄 test_dotted_notes.rs
                                                                                  📄 test_drum_channel.rs
                                                                                  📄 test_key_transpose.rs
                                                                                  📄 test_length.rs
                                                                                  📄 test_modifier.rs
                                                                                  📄 test_note_length.rs
                                                                                  📄 test_octave.rs
                                                                                  📄 test_pass1.rs
                                                                                  📄 test_pass2.rs
                                                                                  📄 test_pass3.rs
                                                                                  📄 test_pass4.rs
                                                                                  📄 test_program_change.rs
                                                                                  📄 test_rest.rs
                                                                                  📄 test_tempo.rs
                                                                                  📄 test_velocity.rs
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
                                                                                  📄 tree-sitter-mml.wasm
                                                                              📄 _config.yml
                                                                              📄 build.rs
                                                                              📁 demo/
                                                                                📄 .gitignore
                                                                                📖 FEATURES.md
                                                                                📖 README.md
                                                                                🌐 index.html
                                                                                📊 package.json
                                                                                📁 src/
                                                                                  📘 audioPlayback.ts
                                                                                  📘 audioRenderer.ts
                                                                                  📘 main.ts
                                                                                  📘 midiReader.ts
                                                                                  📘 mmlConverter.ts
                                                                                  📘 parseMidiNotes.ts
                                                                                  📘 smfToYm2151.ts
                                                                                  📘 state.ts
                                                                                  📘 treeToJSON.ts
                                                                                  📘 ui.ts
                                                                                  📘 visualization.ts
                                                                                  📘 wavExport.ts
                                                                                📄 test-loader.mjs
                                                                                📄 test-register.mjs
                                                                                📁 tests/
                                                                                  📘 audioBufferToWav.test.ts
                                                                                  📘 midiReader.test.ts
                                                                                  📘 parseMidiNotes.test.ts
                                                                                  📘 treeToJSON.test.ts
                                                                              📁 demo-library/
                                                                                🌐 index.html
                                                                                📊 package.json
                                                                              📁 generated-docs/
                                                                              🌐 googled947dc864c270e07.html
                                                                              📁 issue-notes/
                                                                                📖 103.md
                                                                                📖 123.md
                                                                                📖 133.md
                                                                                📖 39.md
                                                                                📖 44.md
                                                                              📄 mmlabc-to-smf-rust.toml.example
                                                                              📁 mmlabc-to-smf-wasm/
                                                                                📄 Cargo.lock
                                                                                📄 Cargo.toml
                                                                                📁 src/
                                                                                  📄 lib.rs
                                                                                  📄 token_extractor.rs
                                                                                📁 tests/
                                                                                  📄 parity.rs
                                                                              📊 package.json
                                                                              📁 scripts/
                                                                                📖 README.md
                                                                                📄 build-demo.sh
                                                                                📄 transform-demo-paths.sh
                                                                              📁 src/
                                                                                📄 attachment_json.rs
                                                                                📄 config.rs
                                                                                📄 lib.rs
                                                                                📄 main.rs
                                                                                📄 mml_preprocessor.rs
                                                                                📄 parse_tree_tokens.rs
                                                                                📄 pass1_parser.rs
                                                                                📄 pass2_ast.rs
                                                                                📄 pass3_events.rs
                                                                                📄 pass4_midi.rs
                                                                                📄 tree_sitter_mml.rs
                                                                                📄 types.rs
                                                                              📁 tests/
                                                                                📄 integration_test.rs
                                                                                📄 test_attachment_json.rs
                                                                                📄 test_c1_vs_c64.rs
                                                                                📄 test_channel.rs
                                                                                📄 test_chord.rs
                                                                                📄 test_cli.rs
                                                                                📄 test_config.rs
                                                                                📄 test_dotted_notes.rs
                                                                                📄 test_drum_channel.rs
                                                                                📄 test_key_transpose.rs
                                                                                📄 test_length.rs
                                                                                📄 test_modifier.rs
                                                                                📄 test_note_length.rs
                                                                                📄 test_octave.rs
                                                                                📄 test_pass1.rs
                                                                                📄 test_pass2.rs
                                                                                📄 test_pass3.rs
                                                                                📄 test_pass4.rs
                                                                                📄 test_program_change.rs
                                                                                📄 test_rest.rs
                                                                                📄 test_tempo.rs
                                                                                📄 test_velocity.rs
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
                                                                                📄 tree-sitter-mml.wasm
                                                                            📄 _config.yml
                                                                            📄 build.rs
                                                                            📁 demo/
                                                                              📄 .gitignore
                                                                              📖 FEATURES.md
                                                                              📖 README.md
                                                                              🌐 index.html
                                                                              📊 package.json
                                                                              📁 src/
                                                                                📘 audioPlayback.ts
                                                                                📘 audioRenderer.ts
                                                                                📘 main.ts
                                                                                📘 midiReader.ts
                                                                                📘 mmlConverter.ts
                                                                                📘 parseMidiNotes.ts
                                                                                📘 smfToYm2151.ts
                                                                                📘 state.ts
                                                                                📘 treeToJSON.ts
                                                                                📘 ui.ts
                                                                                📘 visualization.ts
                                                                                📘 wavExport.ts
                                                                              📄 test-loader.mjs
                                                                              📄 test-register.mjs
                                                                              📁 tests/
                                                                                📘 audioBufferToWav.test.ts
                                                                                📘 midiReader.test.ts
                                                                                📘 parseMidiNotes.test.ts
                                                                                📘 treeToJSON.test.ts
                                                                            📁 demo-library/
                                                                              🌐 index.html
                                                                              📊 package.json
                                                                            📁 generated-docs/
                                                                            🌐 googled947dc864c270e07.html
                                                                            📁 issue-notes/
                                                                              📖 103.md
                                                                              📖 123.md
                                                                              📖 133.md
                                                                              📖 39.md
                                                                              📖 44.md
                                                                            📄 mmlabc-to-smf-rust.toml.example
                                                                            📁 mmlabc-to-smf-wasm/
                                                                              📄 Cargo.lock
                                                                              📄 Cargo.toml
                                                                              📁 src/
                                                                                📄 lib.rs
                                                                                📄 token_extractor.rs
                                                                              📁 tests/
                                                                                📄 parity.rs
                                                                            📊 package.json
                                                                            📁 scripts/
                                                                              📖 README.md
                                                                              📄 build-demo.sh
                                                                              📄 transform-demo-paths.sh
                                                                            📁 src/
                                                                              📄 attachment_json.rs
                                                                              📄 config.rs
                                                                              📄 lib.rs
                                                                              📄 main.rs
                                                                              📄 mml_preprocessor.rs
                                                                              📄 parse_tree_tokens.rs
                                                                              📄 pass1_parser.rs
                                                                              📄 pass2_ast.rs
                                                                              📄 pass3_events.rs
                                                                              📄 pass4_midi.rs
                                                                              📄 tree_sitter_mml.rs
                                                                              📄 types.rs
                                                                            📁 tests/
                                                                              📄 integration_test.rs
                                                                              📄 test_attachment_json.rs
                                                                              📄 test_c1_vs_c64.rs
                                                                              📄 test_channel.rs
                                                                              📄 test_chord.rs
                                                                              📄 test_cli.rs
                                                                              📄 test_config.rs
                                                                              📄 test_dotted_notes.rs
                                                                              📄 test_drum_channel.rs
                                                                              📄 test_key_transpose.rs
                                                                              📄 test_length.rs
                                                                              📄 test_modifier.rs
                                                                              📄 test_note_length.rs
                                                                              📄 test_octave.rs
                                                                              📄 test_pass1.rs
                                                                              📄 test_pass2.rs
                                                                              📄 test_pass3.rs
                                                                              📄 test_pass4.rs
                                                                              📄 test_program_change.rs
                                                                              📄 test_rest.rs
                                                                              📄 test_tempo.rs
                                                                              📄 test_velocity.rs
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
                                                                              📄 tree-sitter-mml.wasm
                                                                          📄 _config.yml
                                                                          📄 build.rs
                                                                          📁 demo/
                                                                            📄 .gitignore
                                                                            📖 FEATURES.md
                                                                            📖 README.md
                                                                            🌐 index.html
                                                                            📊 package.json
                                                                            📁 src/
                                                                              📘 audioPlayback.ts
                                                                              📘 audioRenderer.ts
                                                                              📘 main.ts
                                                                              📘 midiReader.ts
                                                                              📘 mmlConverter.ts
                                                                              📘 parseMidiNotes.ts
                                                                              📘 smfToYm2151.ts
                                                                              📘 state.ts
                                                                              📘 treeToJSON.ts
                                                                              📘 ui.ts
                                                                              📘 visualization.ts
                                                                              📘 wavExport.ts
                                                                            📄 test-loader.mjs
                                                                            📄 test-register.mjs
                                                                            📁 tests/
                                                                              📘 audioBufferToWav.test.ts
                                                                              📘 midiReader.test.ts
                                                                              📘 parseMidiNotes.test.ts
                                                                              📘 treeToJSON.test.ts
                                                                          📁 demo-library/
                                                                            🌐 index.html
                                                                            📊 package.json
                                                                          📁 generated-docs/
                                                                          🌐 googled947dc864c270e07.html
                                                                          📁 issue-notes/
                                                                            📖 103.md
                                                                            📖 123.md
                                                                            📖 133.md
                                                                            📖 39.md
                                                                            📖 44.md
                                                                          📄 mmlabc-to-smf-rust.toml.example
                                                                          📁 mmlabc-to-smf-wasm/
                                                                            📄 Cargo.lock
                                                                            📄 Cargo.toml
                                                                            📁 src/
                                                                              📄 lib.rs
                                                                              📄 token_extractor.rs
                                                                            📁 tests/
                                                                              📄 parity.rs
                                                                          📊 package.json
                                                                          📁 scripts/
                                                                            📖 README.md
                                                                            📄 build-demo.sh
                                                                            📄 transform-demo-paths.sh
                                                                          📁 src/
                                                                            📄 attachment_json.rs
                                                                            📄 config.rs
                                                                            📄 lib.rs
                                                                            📄 main.rs
                                                                            📄 mml_preprocessor.rs
                                                                            📄 parse_tree_tokens.rs
                                                                            📄 pass1_parser.rs
                                                                            📄 pass2_ast.rs
                                                                            📄 pass3_events.rs
                                                                            📄 pass4_midi.rs
                                                                            📄 tree_sitter_mml.rs
                                                                            📄 types.rs
                                                                          📁 tests/
                                                                            📄 integration_test.rs
                                                                            📄 test_attachment_json.rs
                                                                            📄 test_c1_vs_c64.rs
                                                                            📄 test_channel.rs
                                                                            📄 test_chord.rs
                                                                            📄 test_cli.rs
                                                                            📄 test_config.rs
                                                                            📄 test_dotted_notes.rs
                                                                            📄 test_drum_channel.rs
                                                                            📄 test_key_transpose.rs
                                                                            📄 test_length.rs
                                                                            📄 test_modifier.rs
                                                                            📄 test_note_length.rs
                                                                            📄 test_octave.rs
                                                                            📄 test_pass1.rs
                                                                            📄 test_pass2.rs
                                                                            📄 test_pass3.rs
                                                                            📄 test_pass4.rs
                                                                            📄 test_program_change.rs
                                                                            📄 test_rest.rs
                                                                            📄 test_tempo.rs
                                                                            📄 test_velocity.rs
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
                                                                            📄 tree-sitter-mml.wasm
                                                                        📄 _config.yml
                                                                        📄 build.rs
                                                                        📁 demo/
                                                                          📄 .gitignore
                                                                          📖 FEATURES.md
                                                                          📖 README.md
                                                                          🌐 index.html
                                                                          📊 package.json
                                                                          📁 src/
                                                                            📘 audioPlayback.ts
                                                                            📘 audioRenderer.ts
                                                                            📘 main.ts
                                                                            📘 midiReader.ts
                                                                            📘 mmlConverter.ts
                                                                            📘 parseMidiNotes.ts
                                                                            📘 smfToYm2151.ts
                                                                            📘 state.ts
                                                                            📘 treeToJSON.ts
                                                                            📘 ui.ts
                                                                            📘 visualization.ts
                                                                            📘 wavExport.ts
                                                                          📄 test-loader.mjs
                                                                          📄 test-register.mjs
                                                                          📁 tests/
                                                                            📘 audioBufferToWav.test.ts
                                                                            📘 midiReader.test.ts
                                                                            📘 parseMidiNotes.test.ts
                                                                            📘 treeToJSON.test.ts
                                                                        📁 demo-library/
                                                                          🌐 index.html
                                                                          📊 package.json
                                                                        📁 generated-docs/
                                                                        🌐 googled947dc864c270e07.html
                                                                        📁 issue-notes/
                                                                          📖 103.md
                                                                          📖 123.md
                                                                          📖 133.md
                                                                          📖 39.md
                                                                          📖 44.md
                                                                        📄 mmlabc-to-smf-rust.toml.example
                                                                        📁 mmlabc-to-smf-wasm/
                                                                          📄 Cargo.lock
                                                                          📄 Cargo.toml
                                                                          📁 src/
                                                                            📄 lib.rs
                                                                            📄 token_extractor.rs
                                                                          📁 tests/
                                                                            📄 parity.rs
                                                                        📊 package.json
                                                                        📁 scripts/
                                                                          📖 README.md
                                                                          📄 build-demo.sh
                                                                          📄 transform-demo-paths.sh
                                                                        📁 src/
                                                                          📄 attachment_json.rs
                                                                          📄 config.rs
                                                                          📄 lib.rs
                                                                          📄 main.rs
                                                                          📄 mml_preprocessor.rs
                                                                          📄 parse_tree_tokens.rs
                                                                          📄 pass1_parser.rs
                                                                          📄 pass2_ast.rs
                                                                          📄 pass3_events.rs
                                                                          📄 pass4_midi.rs
                                                                          📄 tree_sitter_mml.rs
                                                                          📄 types.rs
                                                                        📁 tests/
                                                                          📄 integration_test.rs
                                                                          📄 test_attachment_json.rs
                                                                          📄 test_c1_vs_c64.rs
                                                                          📄 test_channel.rs
                                                                          📄 test_chord.rs
                                                                          📄 test_cli.rs
                                                                          📄 test_config.rs
                                                                          📄 test_dotted_notes.rs
                                                                          📄 test_drum_channel.rs
                                                                          📄 test_key_transpose.rs
                                                                          📄 test_length.rs
                                                                          📄 test_modifier.rs
                                                                          📄 test_note_length.rs
                                                                          📄 test_octave.rs
                                                                          📄 test_pass1.rs
                                                                          📄 test_pass2.rs
                                                                          📄 test_pass3.rs
                                                                          📄 test_pass4.rs
                                                                          📄 test_program_change.rs
                                                                          📄 test_rest.rs
                                                                          📄 test_tempo.rs
                                                                          📄 test_velocity.rs
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
                                                                          📄 tree-sitter-mml.wasm
                                                                      📄 _config.yml
                                                                      📄 build.rs
                                                                      📁 demo/
                                                                        📄 .gitignore
                                                                        📖 FEATURES.md
                                                                        📖 README.md
                                                                        🌐 index.html
                                                                        📊 package.json
                                                                        📁 src/
                                                                          📘 audioPlayback.ts
                                                                          📘 audioRenderer.ts
                                                                          📘 main.ts
                                                                          📘 midiReader.ts
                                                                          📘 mmlConverter.ts
                                                                          📘 parseMidiNotes.ts
                                                                          📘 smfToYm2151.ts
                                                                          📘 state.ts
                                                                          📘 treeToJSON.ts
                                                                          📘 ui.ts
                                                                          📘 visualization.ts
                                                                          📘 wavExport.ts
                                                                        📄 test-loader.mjs
                                                                        📄 test-register.mjs
                                                                        📁 tests/
                                                                          📘 audioBufferToWav.test.ts
                                                                          📘 midiReader.test.ts
                                                                          📘 parseMidiNotes.test.ts
                                                                          📘 treeToJSON.test.ts
                                                                      📁 demo-library/
                                                                        🌐 index.html
                                                                        📊 package.json
                                                                      📁 generated-docs/
                                                                      🌐 googled947dc864c270e07.html
                                                                      📁 issue-notes/
                                                                        📖 103.md
                                                                        📖 123.md
                                                                        📖 133.md
                                                                        📖 39.md
                                                                        📖 44.md
                                                                      📄 mmlabc-to-smf-rust.toml.example
                                                                      📁 mmlabc-to-smf-wasm/
                                                                        📄 Cargo.lock
                                                                        📄 Cargo.toml
                                                                        📁 src/
                                                                          📄 lib.rs
                                                                          📄 token_extractor.rs
                                                                        📁 tests/
                                                                          📄 parity.rs
                                                                      📊 package.json
                                                                      📁 scripts/
                                                                        📖 README.md
                                                                        📄 build-demo.sh
                                                                        📄 transform-demo-paths.sh
                                                                      📁 src/
                                                                        📄 attachment_json.rs
                                                                        📄 config.rs
                                                                        📄 lib.rs
                                                                        📄 main.rs
                                                                        📄 mml_preprocessor.rs
                                                                        📄 parse_tree_tokens.rs
                                                                        📄 pass1_parser.rs
                                                                        📄 pass2_ast.rs
                                                                        📄 pass3_events.rs
                                                                        📄 pass4_midi.rs
                                                                        📄 tree_sitter_mml.rs
                                                                        📄 types.rs
                                                                      📁 tests/
                                                                        📄 integration_test.rs
                                                                        📄 test_attachment_json.rs
                                                                        📄 test_c1_vs_c64.rs
                                                                        📄 test_channel.rs
                                                                        📄 test_chord.rs
                                                                        📄 test_cli.rs
                                                                        📄 test_config.rs
                                                                        📄 test_dotted_notes.rs
                                                                        📄 test_drum_channel.rs
                                                                        📄 test_key_transpose.rs
                                                                        📄 test_length.rs
                                                                        📄 test_modifier.rs
                                                                        📄 test_note_length.rs
                                                                        📄 test_octave.rs
                                                                        📄 test_pass1.rs
                                                                        📄 test_pass2.rs
                                                                        📄 test_pass3.rs
                                                                        📄 test_pass4.rs
                                                                        📄 test_program_change.rs
                                                                        📄 test_rest.rs
                                                                        📄 test_tempo.rs
                                                                        📄 test_velocity.rs
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
                                                                        📄 tree-sitter-mml.wasm
                                                                    📄 _config.yml
                                                                    📄 build.rs
                                                                    📁 demo/
                                                                      📄 .gitignore
                                                                      📖 FEATURES.md
                                                                      📖 README.md
                                                                      🌐 index.html
                                                                      📊 package.json
                                                                      📁 src/
                                                                        📘 audioPlayback.ts
                                                                        📘 audioRenderer.ts
                                                                        📘 main.ts
                                                                        📘 midiReader.ts
                                                                        📘 mmlConverter.ts
                                                                        📘 parseMidiNotes.ts
                                                                        📘 smfToYm2151.ts
                                                                        📘 state.ts
                                                                        📘 treeToJSON.ts
                                                                        📘 ui.ts
                                                                        📘 visualization.ts
                                                                        📘 wavExport.ts
                                                                      📄 test-loader.mjs
                                                                      📄 test-register.mjs
                                                                      📁 tests/
                                                                        📘 audioBufferToWav.test.ts
                                                                        📘 midiReader.test.ts
                                                                        📘 parseMidiNotes.test.ts
                                                                        📘 treeToJSON.test.ts
                                                                    📁 demo-library/
                                                                      🌐 index.html
                                                                      📊 package.json
                                                                    📁 generated-docs/
                                                                    🌐 googled947dc864c270e07.html
                                                                    📁 issue-notes/
                                                                      📖 103.md
                                                                      📖 123.md
                                                                      📖 133.md
                                                                      📖 39.md
                                                                      📖 44.md
                                                                    📄 mmlabc-to-smf-rust.toml.example
                                                                    📁 mmlabc-to-smf-wasm/
                                                                      📄 Cargo.lock
                                                                      📄 Cargo.toml
                                                                      📁 src/
                                                                        📄 lib.rs
                                                                        📄 token_extractor.rs
                                                                      📁 tests/
                                                                        📄 parity.rs
                                                                    📊 package.json
                                                                    📁 scripts/
                                                                      📖 README.md
                                                                      📄 build-demo.sh
                                                                      📄 transform-demo-paths.sh
                                                                    📁 src/
                                                                      📄 attachment_json.rs
                                                                      📄 config.rs
                                                                      📄 lib.rs
                                                                      📄 main.rs
                                                                      📄 mml_preprocessor.rs
                                                                      📄 parse_tree_tokens.rs
                                                                      📄 pass1_parser.rs
                                                                      📄 pass2_ast.rs
                                                                      📄 pass3_events.rs
                                                                      📄 pass4_midi.rs
                                                                      📄 tree_sitter_mml.rs
                                                                      📄 types.rs
                                                                    📁 tests/
                                                                      📄 integration_test.rs
                                                                      📄 test_attachment_json.rs
                                                                      📄 test_c1_vs_c64.rs
                                                                      📄 test_channel.rs
                                                                      📄 test_chord.rs
                                                                      📄 test_cli.rs
                                                                      📄 test_config.rs
                                                                      📄 test_dotted_notes.rs
                                                                      📄 test_drum_channel.rs
                                                                      📄 test_key_transpose.rs
                                                                      📄 test_length.rs
                                                                      📄 test_modifier.rs
                                                                      📄 test_note_length.rs
                                                                      📄 test_octave.rs
                                                                      📄 test_pass1.rs
                                                                      📄 test_pass2.rs
                                                                      📄 test_pass3.rs
                                                                      📄 test_pass4.rs
                                                                      📄 test_program_change.rs
                                                                      📄 test_rest.rs
                                                                      📄 test_tempo.rs
                                                                      📄 test_velocity.rs
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
                                                                      📄 tree-sitter-mml.wasm
                                                                  📄 _config.yml
                                                                  📄 build.rs
                                                                  📁 demo/
                                                                    📄 .gitignore
                                                                    📖 FEATURES.md
                                                                    📖 README.md
                                                                    🌐 index.html
                                                                    📊 package.json
                                                                    📁 src/
                                                                      📘 audioPlayback.ts
                                                                      📘 audioRenderer.ts
                                                                      📘 main.ts
                                                                      📘 midiReader.ts
                                                                      📘 mmlConverter.ts
                                                                      📘 parseMidiNotes.ts
                                                                      📘 smfToYm2151.ts
                                                                      📘 state.ts
                                                                      📘 treeToJSON.ts
                                                                      📘 ui.ts
                                                                      📘 visualization.ts
                                                                      📘 wavExport.ts
                                                                    📄 test-loader.mjs
                                                                    📄 test-register.mjs
                                                                    📁 tests/
                                                                      📘 audioBufferToWav.test.ts
                                                                      📘 midiReader.test.ts
                                                                      📘 parseMidiNotes.test.ts
                                                                      📘 treeToJSON.test.ts
                                                                  📁 demo-library/
                                                                    🌐 index.html
                                                                    📊 package.json
                                                                  📁 generated-docs/
                                                                  🌐 googled947dc864c270e07.html
                                                                  📁 issue-notes/
                                                                    📖 103.md
                                                                    📖 123.md
                                                                    📖 133.md
                                                                    📖 39.md
                                                                    📖 44.md
                                                                  📄 mmlabc-to-smf-rust.toml.example
                                                                  📁 mmlabc-to-smf-wasm/
                                                                    📄 Cargo.lock
                                                                    📄 Cargo.toml
                                                                    📁 src/
                                                                      📄 lib.rs
                                                                      📄 token_extractor.rs
                                                                    📁 tests/
                                                                      📄 parity.rs
                                                                  📊 package.json
                                                                  📁 scripts/
                                                                    📖 README.md
                                                                    📄 build-demo.sh
                                                                    📄 transform-demo-paths.sh
                                                                  📁 src/
                                                                    📄 attachment_json.rs
                                                                    📄 config.rs
                                                                    📄 lib.rs
                                                                    📄 main.rs
                                                                    📄 mml_preprocessor.rs
                                                                    📄 parse_tree_tokens.rs
                                                                    📄 pass1_parser.rs
                                                                    📄 pass2_ast.rs
                                                                    📄 pass3_events.rs
                                                                    📄 pass4_midi.rs
                                                                    📄 tree_sitter_mml.rs
                                                                    📄 types.rs
                                                                  📁 tests/
                                                                    📄 integration_test.rs
                                                                    📄 test_attachment_json.rs
                                                                    📄 test_c1_vs_c64.rs
                                                                    📄 test_channel.rs
                                                                    📄 test_chord.rs
                                                                    📄 test_cli.rs
                                                                    📄 test_config.rs
                                                                    📄 test_dotted_notes.rs
                                                                    📄 test_drum_channel.rs
                                                                    📄 test_key_transpose.rs
                                                                    📄 test_length.rs
                                                                    📄 test_modifier.rs
                                                                    📄 test_note_length.rs
                                                                    📄 test_octave.rs
                                                                    📄 test_pass1.rs
                                                                    📄 test_pass2.rs
                                                                    📄 test_pass3.rs
                                                                    📄 test_pass4.rs
                                                                    📄 test_program_change.rs
                                                                    📄 test_rest.rs
                                                                    📄 test_tempo.rs
                                                                    📄 test_velocity.rs
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
                                                                    📄 tree-sitter-mml.wasm
                                                                📄 _config.yml
                                                                📄 build.rs
                                                                📁 demo/
                                                                  📄 .gitignore
                                                                  📖 FEATURES.md
                                                                  📖 README.md
                                                                  🌐 index.html
                                                                  📊 package.json
                                                                  📁 src/
                                                                    📘 audioPlayback.ts
                                                                    📘 audioRenderer.ts
                                                                    📘 main.ts
                                                                    📘 midiReader.ts
                                                                    📘 mmlConverter.ts
                                                                    📘 parseMidiNotes.ts
                                                                    📘 smfToYm2151.ts
                                                                    📘 state.ts
                                                                    📘 treeToJSON.ts
                                                                    📘 ui.ts
                                                                    📘 visualization.ts
                                                                    📘 wavExport.ts
                                                                  📄 test-loader.mjs
                                                                  📄 test-register.mjs
                                                                  📁 tests/
                                                                    📘 audioBufferToWav.test.ts
                                                                    📘 midiReader.test.ts
                                                                    📘 parseMidiNotes.test.ts
                                                                    📘 treeToJSON.test.ts
                                                                📁 demo-library/
                                                                  🌐 index.html
                                                                  📊 package.json
                                                                📁 generated-docs/
                                                                🌐 googled947dc864c270e07.html
                                                                📁 issue-notes/
                                                                  📖 103.md
                                                                  📖 123.md
                                                                  📖 133.md
                                                                  📖 39.md
                                                                  📖 44.md
                                                                📄 mmlabc-to-smf-rust.toml.example
                                                                📁 mmlabc-to-smf-wasm/
                                                                  📄 Cargo.lock
                                                                  📄 Cargo.toml
                                                                  📁 src/
                                                                    📄 lib.rs
                                                                    📄 token_extractor.rs
                                                                  📁 tests/
                                                                    📄 parity.rs
                                                                📊 package.json
                                                                📁 scripts/
                                                                  📖 README.md
                                                                  📄 build-demo.sh
                                                                  📄 transform-demo-paths.sh
                                                                📁 src/
                                                                  📄 attachment_json.rs
                                                                  📄 config.rs
                                                                  📄 lib.rs
                                                                  📄 main.rs
                                                                  📄 mml_preprocessor.rs
                                                                  📄 parse_tree_tokens.rs
                                                                  📄 pass1_parser.rs
                                                                  📄 pass2_ast.rs
                                                                  📄 pass3_events.rs
                                                                  📄 pass4_midi.rs
                                                                  📄 tree_sitter_mml.rs
                                                                  📄 types.rs
                                                                📁 tests/
                                                                  📄 integration_test.rs
                                                                  📄 test_attachment_json.rs
                                                                  📄 test_c1_vs_c64.rs
                                                                  📄 test_channel.rs
                                                                  📄 test_chord.rs
                                                                  📄 test_cli.rs
                                                                  📄 test_config.rs
                                                                  📄 test_dotted_notes.rs
                                                                  📄 test_drum_channel.rs
                                                                  📄 test_key_transpose.rs
                                                                  📄 test_length.rs
                                                                  📄 test_modifier.rs
                                                                  📄 test_note_length.rs
                                                                  📄 test_octave.rs
                                                                  📄 test_pass1.rs
                                                                  📄 test_pass2.rs
                                                                  📄 test_pass3.rs
                                                                  📄 test_pass4.rs
                                                                  📄 test_program_change.rs
                                                                  📄 test_rest.rs
                                                                  📄 test_tempo.rs
                                                                  📄 test_velocity.rs
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
                                                                  📄 tree-sitter-mml.wasm
                                                              📄 _config.yml
                                                              📄 build.rs
                                                              📁 demo/
                                                                📄 .gitignore
                                                                📖 FEATURES.md
                                                                📖 README.md
                                                                🌐 index.html
                                                                📊 package.json
                                                                📁 src/
                                                                  📘 audioPlayback.ts
                                                                  📘 audioRenderer.ts
                                                                  📘 main.ts
                                                                  📘 midiReader.ts
                                                                  📘 mmlConverter.ts
                                                                  📘 parseMidiNotes.ts
                                                                  📘 smfToYm2151.ts
                                                                  📘 state.ts
                                                                  📘 treeToJSON.ts
                                                                  📘 ui.ts
                                                                  📘 visualization.ts
                                                                  📘 wavExport.ts
                                                                📄 test-loader.mjs
                                                                📄 test-register.mjs
                                                                📁 tests/
                                                                  📘 audioBufferToWav.test.ts
                                                                  📘 midiReader.test.ts
                                                                  📘 parseMidiNotes.test.ts
                                                                  📘 treeToJSON.test.ts
                                                              📁 demo-library/
                                                                🌐 index.html
                                                                📊 package.json
                                                              📁 generated-docs/
                                                              🌐 googled947dc864c270e07.html
                                                              📁 issue-notes/
                                                                📖 103.md
                                                                📖 123.md
                                                                📖 133.md
                                                                📖 39.md
                                                                📖 44.md
                                                              📄 mmlabc-to-smf-rust.toml.example
                                                              📁 mmlabc-to-smf-wasm/
                                                                📄 Cargo.lock
                                                                📄 Cargo.toml
                                                                📁 src/
                                                                  📄 lib.rs
                                                                  📄 token_extractor.rs
                                                                📁 tests/
                                                                  📄 parity.rs
                                                              📊 package.json
                                                              📁 scripts/
                                                                📖 README.md
                                                                📄 build-demo.sh
                                                                📄 transform-demo-paths.sh
                                                              📁 src/
                                                                📄 attachment_json.rs
                                                                📄 config.rs
                                                                📄 lib.rs
                                                                📄 main.rs
                                                                📄 mml_preprocessor.rs
                                                                📄 parse_tree_tokens.rs
                                                                📄 pass1_parser.rs
                                                                📄 pass2_ast.rs
                                                                📄 pass3_events.rs
                                                                📄 pass4_midi.rs
                                                                📄 tree_sitter_mml.rs
                                                                📄 types.rs
                                                              📁 tests/
                                                                📄 integration_test.rs
                                                                📄 test_attachment_json.rs
                                                                📄 test_c1_vs_c64.rs
                                                                📄 test_channel.rs
                                                                📄 test_chord.rs
                                                                📄 test_cli.rs
                                                                📄 test_config.rs
                                                                📄 test_dotted_notes.rs
                                                                📄 test_drum_channel.rs
                                                                📄 test_key_transpose.rs
                                                                📄 test_length.rs
                                                                📄 test_modifier.rs
                                                                📄 test_note_length.rs
                                                                📄 test_octave.rs
                                                                📄 test_pass1.rs
                                                                📄 test_pass2.rs
                                                                📄 test_pass3.rs
                                                                📄 test_pass4.rs
                                                                📄 test_program_change.rs
                                                                📄 test_rest.rs
                                                                📄 test_tempo.rs
                                                                📄 test_velocity.rs
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
                                                                📄 tree-sitter-mml.wasm
                                                            📄 _config.yml
                                                            📄 build.rs
                                                            📁 demo/
                                                              📄 .gitignore
                                                              📖 FEATURES.md
                                                              📖 README.md
                                                              🌐 index.html
                                                              📊 package.json
                                                              📁 src/
                                                                📘 audioPlayback.ts
                                                                📘 audioRenderer.ts
                                                                📘 main.ts
                                                                📘 midiReader.ts
                                                                📘 mmlConverter.ts
                                                                📘 parseMidiNotes.ts
                                                                📘 smfToYm2151.ts
                                                                📘 state.ts
                                                                📘 treeToJSON.ts
                                                                📘 ui.ts
                                                                📘 visualization.ts
                                                                📘 wavExport.ts
                                                              📄 test-loader.mjs
                                                              📄 test-register.mjs
                                                              📁 tests/
                                                                📘 audioBufferToWav.test.ts
                                                                📘 midiReader.test.ts
                                                                📘 parseMidiNotes.test.ts
                                                                📘 treeToJSON.test.ts
                                                            📁 demo-library/
                                                              🌐 index.html
                                                              📊 package.json
                                                            📁 generated-docs/
                                                            🌐 googled947dc864c270e07.html
                                                            📁 issue-notes/
                                                              📖 103.md
                                                              📖 123.md
                                                              📖 133.md
                                                              📖 39.md
                                                              📖 44.md
                                                            📄 mmlabc-to-smf-rust.toml.example
                                                            📁 mmlabc-to-smf-wasm/
                                                              📄 Cargo.lock
                                                              📄 Cargo.toml
                                                              📁 src/
                                                                📄 lib.rs
                                                                📄 token_extractor.rs
                                                              📁 tests/
                                                                📄 parity.rs
                                                            📊 package.json
                                                            📁 scripts/
                                                              📖 README.md
                                                              📄 build-demo.sh
                                                              📄 transform-demo-paths.sh
                                                            📁 src/
                                                              📄 attachment_json.rs
                                                              📄 config.rs
                                                              📄 lib.rs
                                                              📄 main.rs
                                                              📄 mml_preprocessor.rs
                                                              📄 parse_tree_tokens.rs
                                                              📄 pass1_parser.rs
                                                              📄 pass2_ast.rs
                                                              📄 pass3_events.rs
                                                              📄 pass4_midi.rs
                                                              📄 tree_sitter_mml.rs
                                                              📄 types.rs
                                                            📁 tests/
                                                              📄 integration_test.rs
                                                              📄 test_attachment_json.rs
                                                              📄 test_c1_vs_c64.rs
                                                              📄 test_channel.rs
                                                              📄 test_chord.rs
                                                              📄 test_cli.rs
                                                              📄 test_config.rs
                                                              📄 test_dotted_notes.rs
                                                              📄 test_drum_channel.rs
                                                              📄 test_key_transpose.rs
                                                              📄 test_length.rs
                                                              📄 test_modifier.rs
                                                              📄 test_note_length.rs
                                                              📄 test_octave.rs
                                                              📄 test_pass1.rs
                                                              📄 test_pass2.rs
                                                              📄 test_pass3.rs
                                                              📄 test_pass4.rs
                                                              📄 test_program_change.rs
                                                              📄 test_rest.rs
                                                              📄 test_tempo.rs
                                                              📄 test_velocity.rs
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
                                                              📄 tree-sitter-mml.wasm
                                                          📄 _config.yml
                                                          📄 build.rs
                                                          📁 demo/
                                                            📄 .gitignore
                                                            📖 FEATURES.md
                                                            📖 README.md
                                                            🌐 index.html
                                                            📊 package.json
                                                            📁 src/
                                                              📘 audioPlayback.ts
                                                              📘 audioRenderer.ts
                                                              📘 main.ts
                                                              📘 midiReader.ts
                                                              📘 mmlConverter.ts
                                                              📘 parseMidiNotes.ts
                                                              📘 smfToYm2151.ts
                                                              📘 state.ts
                                                              📘 treeToJSON.ts
                                                              📘 ui.ts
                                                              📘 visualization.ts
                                                              📘 wavExport.ts
                                                            📄 test-loader.mjs
                                                            📄 test-register.mjs
                                                            📁 tests/
                                                              📘 audioBufferToWav.test.ts
                                                              📘 midiReader.test.ts
                                                              📘 parseMidiNotes.test.ts
                                                              📘 treeToJSON.test.ts
                                                          📁 demo-library/
                                                            🌐 index.html
                                                            📊 package.json
                                                          📁 generated-docs/
                                                          🌐 googled947dc864c270e07.html
                                                          📁 issue-notes/
                                                            📖 103.md
                                                            📖 123.md
                                                            📖 133.md
                                                            📖 39.md
                                                            📖 44.md
                                                          📄 mmlabc-to-smf-rust.toml.example
                                                          📁 mmlabc-to-smf-wasm/
                                                            📄 Cargo.lock
                                                            📄 Cargo.toml
                                                            📁 src/
                                                              📄 lib.rs
                                                              📄 token_extractor.rs
                                                            📁 tests/
                                                              📄 parity.rs
                                                          📊 package.json
                                                          📁 scripts/
                                                            📖 README.md
                                                            📄 build-demo.sh
                                                            📄 transform-demo-paths.sh
                                                          📁 src/
                                                            📄 attachment_json.rs
                                                            📄 config.rs
                                                            📄 lib.rs
                                                            📄 main.rs
                                                            📄 mml_preprocessor.rs
                                                            📄 parse_tree_tokens.rs
                                                            📄 pass1_parser.rs
                                                            📄 pass2_ast.rs
                                                            📄 pass3_events.rs
                                                            📄 pass4_midi.rs
                                                            📄 tree_sitter_mml.rs
                                                            📄 types.rs
                                                          📁 tests/
                                                            📄 integration_test.rs
                                                            📄 test_attachment_json.rs
                                                            📄 test_c1_vs_c64.rs
                                                            📄 test_channel.rs
                                                            📄 test_chord.rs
                                                            📄 test_cli.rs
                                                            📄 test_config.rs
                                                            📄 test_dotted_notes.rs
                                                            📄 test_drum_channel.rs
                                                            📄 test_key_transpose.rs
                                                            📄 test_length.rs
                                                            📄 test_modifier.rs
                                                            📄 test_note_length.rs
                                                            📄 test_octave.rs
                                                            📄 test_pass1.rs
                                                            📄 test_pass2.rs
                                                            📄 test_pass3.rs
                                                            📄 test_pass4.rs
                                                            📄 test_program_change.rs
                                                            📄 test_rest.rs
                                                            📄 test_tempo.rs
                                                            📄 test_velocity.rs
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
                                                            📄 tree-sitter-mml.wasm
                                                        📄 _config.yml
                                                        📄 build.rs
                                                        📁 demo/
                                                          📄 .gitignore
                                                          📖 FEATURES.md
                                                          📖 README.md
                                                          🌐 index.html
                                                          📊 package.json
                                                          📁 src/
                                                            📘 audioPlayback.ts
                                                            📘 audioRenderer.ts
                                                            📘 main.ts
                                                            📘 midiReader.ts
                                                            📘 mmlConverter.ts
                                                            📘 parseMidiNotes.ts
                                                            📘 smfToYm2151.ts
                                                            📘 state.ts
                                                            📘 treeToJSON.ts
                                                            📘 ui.ts
                                                            📘 visualization.ts
                                                            📘 wavExport.ts
                                                          📄 test-loader.mjs
                                                          📄 test-register.mjs
                                                          📁 tests/
                                                            📘 audioBufferToWav.test.ts
                                                            📘 midiReader.test.ts
                                                            📘 parseMidiNotes.test.ts
                                                            📘 treeToJSON.test.ts
                                                        📁 demo-library/
                                                          🌐 index.html
                                                          📊 package.json
                                                        📁 generated-docs/
                                                        🌐 googled947dc864c270e07.html
                                                        📁 issue-notes/
                                                          📖 103.md
                                                          📖 123.md
                                                          📖 133.md
                                                          📖 39.md
                                                          📖 44.md
                                                        📄 mmlabc-to-smf-rust.toml.example
                                                        📁 mmlabc-to-smf-wasm/
                                                          📄 Cargo.lock
                                                          📄 Cargo.toml
                                                          📁 src/
                                                            📄 lib.rs
                                                            📄 token_extractor.rs
                                                          📁 tests/
                                                            📄 parity.rs
                                                        📊 package.json
                                                        📁 scripts/
                                                          📖 README.md
                                                          📄 build-demo.sh
                                                          📄 transform-demo-paths.sh
                                                        📁 src/
                                                          📄 attachment_json.rs
                                                          📄 config.rs
                                                          📄 lib.rs
                                                          📄 main.rs
                                                          📄 mml_preprocessor.rs
                                                          📄 parse_tree_tokens.rs
                                                          📄 pass1_parser.rs
                                                          📄 pass2_ast.rs
                                                          📄 pass3_events.rs
                                                          📄 pass4_midi.rs
                                                          📄 tree_sitter_mml.rs
                                                          📄 types.rs
                                                        📁 tests/
                                                          📄 integration_test.rs
                                                          📄 test_attachment_json.rs
                                                          📄 test_c1_vs_c64.rs
                                                          📄 test_channel.rs
                                                          📄 test_chord.rs
                                                          📄 test_cli.rs
                                                          📄 test_config.rs
                                                          📄 test_dotted_notes.rs
                                                          📄 test_drum_channel.rs
                                                          📄 test_key_transpose.rs
                                                          📄 test_length.rs
                                                          📄 test_modifier.rs
                                                          📄 test_note_length.rs
                                                          📄 test_octave.rs
                                                          📄 test_pass1.rs
                                                          📄 test_pass2.rs
                                                          📄 test_pass3.rs
                                                          📄 test_pass4.rs
                                                          📄 test_program_change.rs
                                                          📄 test_rest.rs
                                                          📄 test_tempo.rs
                                                          📄 test_velocity.rs
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
                                                          📄 tree-sitter-mml.wasm
                                                      📄 _config.yml
                                                      📄 build.rs
                                                      📁 demo/
                                                        📄 .gitignore
                                                        📖 FEATURES.md
                                                        📖 README.md
                                                        🌐 index.html
                                                        📊 package.json
                                                        📁 src/
                                                          📘 audioPlayback.ts
                                                          📘 audioRenderer.ts
                                                          📘 main.ts
                                                          📘 midiReader.ts
                                                          📘 mmlConverter.ts
                                                          📘 parseMidiNotes.ts
                                                          📘 smfToYm2151.ts
                                                          📘 state.ts
                                                          📘 treeToJSON.ts
                                                          📘 ui.ts
                                                          📘 visualization.ts
                                                          📘 wavExport.ts
                                                        📄 test-loader.mjs
                                                        📄 test-register.mjs
                                                        📁 tests/
                                                          📘 audioBufferToWav.test.ts
                                                          📘 midiReader.test.ts
                                                          📘 parseMidiNotes.test.ts
                                                          📘 treeToJSON.test.ts
                                                      📁 demo-library/
                                                        🌐 index.html
                                                        📊 package.json
                                                      📁 generated-docs/
                                                      🌐 googled947dc864c270e07.html
                                                      📁 issue-notes/
                                                        📖 103.md
                                                        📖 123.md
                                                        📖 133.md
                                                        📖 39.md
                                                        📖 44.md
                                                      📄 mmlabc-to-smf-rust.toml.example
                                                      📁 mmlabc-to-smf-wasm/
                                                        📄 Cargo.lock
                                                        📄 Cargo.toml
                                                        📁 src/
                                                          📄 lib.rs
                                                          📄 token_extractor.rs
                                                        📁 tests/
                                                          📄 parity.rs
                                                      📊 package.json
                                                      📁 scripts/
                                                        📖 README.md
                                                        📄 build-demo.sh
                                                        📄 transform-demo-paths.sh
                                                      📁 src/
                                                        📄 attachment_json.rs
                                                        📄 config.rs
                                                        📄 lib.rs
                                                        📄 main.rs
                                                        📄 mml_preprocessor.rs
                                                        📄 parse_tree_tokens.rs
                                                        📄 pass1_parser.rs
                                                        📄 pass2_ast.rs
                                                        📄 pass3_events.rs
                                                        📄 pass4_midi.rs
                                                        📄 tree_sitter_mml.rs
                                                        📄 types.rs
                                                      📁 tests/
                                                        📄 integration_test.rs
                                                        📄 test_attachment_json.rs
                                                        📄 test_c1_vs_c64.rs
                                                        📄 test_channel.rs
                                                        📄 test_chord.rs
                                                        📄 test_cli.rs
                                                        📄 test_config.rs
                                                        📄 test_dotted_notes.rs
                                                        📄 test_drum_channel.rs
                                                        📄 test_key_transpose.rs
                                                        📄 test_length.rs
                                                        📄 test_modifier.rs
                                                        📄 test_note_length.rs
                                                        📄 test_octave.rs
                                                        📄 test_pass1.rs
                                                        📄 test_pass2.rs
                                                        📄 test_pass3.rs
                                                        📄 test_pass4.rs
                                                        📄 test_program_change.rs
                                                        📄 test_rest.rs
                                                        📄 test_tempo.rs
                                                        📄 test_velocity.rs
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
                                                        📄 tree-sitter-mml.wasm
                                                    📄 _config.yml
                                                    📄 build.rs
                                                    📁 demo/
                                                      📄 .gitignore
                                                      📖 FEATURES.md
                                                      📖 README.md
                                                      🌐 index.html
                                                      📊 package.json
                                                      📁 src/
                                                        📘 audioPlayback.ts
                                                        📘 audioRenderer.ts
                                                        📘 main.ts
                                                        📘 midiReader.ts
                                                        📘 mmlConverter.ts
                                                        📘 parseMidiNotes.ts
                                                        📘 smfToYm2151.ts
                                                        📘 state.ts
                                                        📘 treeToJSON.ts
                                                        📘 ui.ts
                                                        📘 visualization.ts
                                                        📘 wavExport.ts
                                                      📄 test-loader.mjs
                                                      📄 test-register.mjs
                                                      📁 tests/
                                                        📘 audioBufferToWav.test.ts
                                                        📘 midiReader.test.ts
                                                        📘 parseMidiNotes.test.ts
                                                        📘 treeToJSON.test.ts
                                                    📁 demo-library/
                                                      🌐 index.html
                                                      📊 package.json
                                                    📁 generated-docs/
                                                    🌐 googled947dc864c270e07.html
                                                    📁 issue-notes/
                                                      📖 103.md
                                                      📖 123.md
                                                      📖 133.md
                                                      📖 39.md
                                                      📖 44.md
                                                    📄 mmlabc-to-smf-rust.toml.example
                                                    📁 mmlabc-to-smf-wasm/
                                                      📄 Cargo.lock
                                                      📄 Cargo.toml
                                                      📁 src/
                                                        📄 lib.rs
                                                        📄 token_extractor.rs
                                                      📁 tests/
                                                        📄 parity.rs
                                                    📊 package.json
                                                    📁 scripts/
                                                      📖 README.md
                                                      📄 build-demo.sh
                                                      📄 transform-demo-paths.sh
                                                    📁 src/
                                                      📄 attachment_json.rs
                                                      📄 config.rs
                                                      📄 lib.rs
                                                      📄 main.rs
                                                      📄 mml_preprocessor.rs
                                                      📄 parse_tree_tokens.rs
                                                      📄 pass1_parser.rs
                                                      📄 pass2_ast.rs
                                                      📄 pass3_events.rs
                                                      📄 pass4_midi.rs
                                                      📄 tree_sitter_mml.rs
                                                      📄 types.rs
                                                    📁 tests/
                                                      📄 integration_test.rs
                                                      📄 test_attachment_json.rs
                                                      📄 test_c1_vs_c64.rs
                                                      📄 test_channel.rs
                                                      📄 test_chord.rs
                                                      📄 test_cli.rs
                                                      📄 test_config.rs
                                                      📄 test_dotted_notes.rs
                                                      📄 test_drum_channel.rs
                                                      📄 test_key_transpose.rs
                                                      📄 test_length.rs
                                                      📄 test_modifier.rs
                                                      📄 test_note_length.rs
                                                      📄 test_octave.rs
                                                      📄 test_pass1.rs
                                                      📄 test_pass2.rs
                                                      📄 test_pass3.rs
                                                      📄 test_pass4.rs
                                                      📄 test_program_change.rs
                                                      📄 test_rest.rs
                                                      📄 test_tempo.rs
                                                      📄 test_velocity.rs
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
                                                      📄 tree-sitter-mml.wasm
                                                  📄 _config.yml
                                                  📄 build.rs
                                                  📁 demo/
                                                    📄 .gitignore
                                                    📖 FEATURES.md
                                                    📖 README.md
                                                    🌐 index.html
                                                    📊 package.json
                                                    📁 src/
                                                      📘 audioPlayback.ts
                                                      📘 audioRenderer.ts
                                                      📘 main.ts
                                                      📘 midiReader.ts
                                                      📘 mmlConverter.ts
                                                      📘 parseMidiNotes.ts
                                                      📘 smfToYm2151.ts
                                                      📘 state.ts
                                                      📘 treeToJSON.ts
                                                      📘 ui.ts
                                                      📘 visualization.ts
                                                      📘 wavExport.ts
                                                    📄 test-loader.mjs
                                                    📄 test-register.mjs
                                                    📁 tests/
                                                      📘 audioBufferToWav.test.ts
                                                      📘 midiReader.test.ts
                                                      📘 parseMidiNotes.test.ts
                                                      📘 treeToJSON.test.ts
                                                  📁 demo-library/
                                                    🌐 index.html
                                                    📊 package.json
                                                  📁 generated-docs/
                                                  🌐 googled947dc864c270e07.html
                                                  📁 issue-notes/
                                                    📖 103.md
                                                    📖 123.md
                                                    📖 133.md
                                                    📖 39.md
                                                    📖 44.md
                                                  📄 mmlabc-to-smf-rust.toml.example
                                                  📁 mmlabc-to-smf-wasm/
                                                    📄 Cargo.lock
                                                    📄 Cargo.toml
                                                    📁 src/
                                                      📄 lib.rs
                                                      📄 token_extractor.rs
                                                    📁 tests/
                                                      📄 parity.rs
                                                  📊 package.json
                                                  📁 scripts/
                                                    📖 README.md
                                                    📄 build-demo.sh
                                                    📄 transform-demo-paths.sh
                                                  📁 src/
                                                    📄 attachment_json.rs
                                                    📄 config.rs
                                                    📄 lib.rs
                                                    📄 main.rs
                                                    📄 mml_preprocessor.rs
                                                    📄 parse_tree_tokens.rs
                                                    📄 pass1_parser.rs
                                                    📄 pass2_ast.rs
                                                    📄 pass3_events.rs
                                                    📄 pass4_midi.rs
                                                    📄 tree_sitter_mml.rs
                                                    📄 types.rs
                                                  📁 tests/
                                                    📄 integration_test.rs
                                                    📄 test_attachment_json.rs
                                                    📄 test_c1_vs_c64.rs
                                                    📄 test_channel.rs
                                                    📄 test_chord.rs
                                                    📄 test_cli.rs
                                                    📄 test_config.rs
                                                    📄 test_dotted_notes.rs
                                                    📄 test_drum_channel.rs
                                                    📄 test_key_transpose.rs
                                                    📄 test_length.rs
                                                    📄 test_modifier.rs
                                                    📄 test_note_length.rs
                                                    📄 test_octave.rs
                                                    📄 test_pass1.rs
                                                    📄 test_pass2.rs
                                                    📄 test_pass3.rs
                                                    📄 test_pass4.rs
                                                    📄 test_program_change.rs
                                                    📄 test_rest.rs
                                                    📄 test_tempo.rs
                                                    📄 test_velocity.rs
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
                                                    📄 tree-sitter-mml.wasm
                                                📄 _config.yml
                                                📄 build.rs
                                                📁 demo/
                                                  📄 .gitignore
                                                  📖 FEATURES.md
                                                  📖 README.md
                                                  🌐 index.html
                                                  📊 package.json
                                                  📁 src/
                                                    📘 audioPlayback.ts
                                                    📘 audioRenderer.ts
                                                    📘 main.ts
                                                    📘 midiReader.ts
                                                    📘 mmlConverter.ts
                                                    📘 parseMidiNotes.ts
                                                    📘 smfToYm2151.ts
                                                    📘 state.ts
                                                    📘 treeToJSON.ts
                                                    📘 ui.ts
                                                    📘 visualization.ts
                                                    📘 wavExport.ts
                                                  📄 test-loader.mjs
                                                  📄 test-register.mjs
                                                  📁 tests/
                                                    📘 audioBufferToWav.test.ts
                                                    📘 midiReader.test.ts
                                                    📘 parseMidiNotes.test.ts
                                                    📘 treeToJSON.test.ts
                                                📁 demo-library/
                                                  🌐 index.html
                                                  📊 package.json
                                                📁 generated-docs/
                                                🌐 googled947dc864c270e07.html
                                                📁 issue-notes/
                                                  📖 103.md
                                                  📖 123.md
                                                  📖 133.md
                                                  📖 39.md
                                                  📖 44.md
                                                📄 mmlabc-to-smf-rust.toml.example
                                                📁 mmlabc-to-smf-wasm/
                                                  📄 Cargo.lock
                                                  📄 Cargo.toml
                                                  📁 src/
                                                    📄 lib.rs
                                                    📄 token_extractor.rs
                                                  📁 tests/
                                                    📄 parity.rs
                                                📊 package.json
                                                📁 scripts/
                                                  📖 README.md
                                                  📄 build-demo.sh
                                                  📄 transform-demo-paths.sh
                                                📁 src/
                                                  📄 attachment_json.rs
                                                  📄 config.rs
                                                  📄 lib.rs
                                                  📄 main.rs
                                                  📄 mml_preprocessor.rs
                                                  📄 parse_tree_tokens.rs
                                                  📄 pass1_parser.rs
                                                  📄 pass2_ast.rs
                                                  📄 pass3_events.rs
                                                  📄 pass4_midi.rs
                                                  📄 tree_sitter_mml.rs
                                                  📄 types.rs
                                                📁 tests/
                                                  📄 integration_test.rs
                                                  📄 test_attachment_json.rs
                                                  📄 test_c1_vs_c64.rs
                                                  📄 test_channel.rs
                                                  📄 test_chord.rs
                                                  📄 test_cli.rs
                                                  📄 test_config.rs
                                                  📄 test_dotted_notes.rs
                                                  📄 test_drum_channel.rs
                                                  📄 test_key_transpose.rs
                                                  📄 test_length.rs
                                                  📄 test_modifier.rs
                                                  📄 test_note_length.rs
                                                  📄 test_octave.rs
                                                  📄 test_pass1.rs
                                                  📄 test_pass2.rs
                                                  📄 test_pass3.rs
                                                  📄 test_pass4.rs
                                                  📄 test_program_change.rs
                                                  📄 test_rest.rs
                                                  📄 test_tempo.rs
                                                  📄 test_velocity.rs
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
                                                  📄 tree-sitter-mml.wasm
                                              📄 _config.yml
                                              📄 build.rs
                                              📁 demo/
                                                📄 .gitignore
                                                📖 FEATURES.md
                                                📖 README.md
                                                🌐 index.html
                                                📊 package.json
                                                📁 src/
                                                  📘 audioPlayback.ts
                                                  📘 audioRenderer.ts
                                                  📘 main.ts
                                                  📘 midiReader.ts
                                                  📘 mmlConverter.ts
                                                  📘 parseMidiNotes.ts
                                                  📘 smfToYm2151.ts
                                                  📘 state.ts
                                                  📘 treeToJSON.ts
                                                  📘 ui.ts
                                                  📘 visualization.ts
                                                  📘 wavExport.ts
                                                📄 test-loader.mjs
                                                📄 test-register.mjs
                                                📁 tests/
                                                  📘 audioBufferToWav.test.ts
                                                  📘 midiReader.test.ts
                                                  📘 parseMidiNotes.test.ts
                                                  📘 treeToJSON.test.ts
                                              📁 demo-library/
                                                🌐 index.html
                                                📊 package.json
                                              📁 generated-docs/
                                              🌐 googled947dc864c270e07.html
                                              📁 issue-notes/
                                                📖 103.md
                                                📖 123.md
                                                📖 133.md
                                                📖 39.md
                                                📖 44.md
                                              📄 mmlabc-to-smf-rust.toml.example
                                              📁 mmlabc-to-smf-wasm/
                                                📄 Cargo.lock
                                                📄 Cargo.toml
                                                📁 src/
                                                  📄 lib.rs
                                                  📄 token_extractor.rs
                                                📁 tests/
                                                  📄 parity.rs
                                              📊 package.json
                                              📁 scripts/
                                                📖 README.md
                                                📄 build-demo.sh
                                                📄 transform-demo-paths.sh
                                              📁 src/
                                                📄 attachment_json.rs
                                                📄 config.rs
                                                📄 lib.rs
                                                📄 main.rs
                                                📄 mml_preprocessor.rs
                                                📄 parse_tree_tokens.rs
                                                📄 pass1_parser.rs
                                                📄 pass2_ast.rs
                                                📄 pass3_events.rs
                                                📄 pass4_midi.rs
                                                📄 tree_sitter_mml.rs
                                                📄 types.rs
                                              📁 tests/
                                                📄 integration_test.rs
                                                📄 test_attachment_json.rs
                                                📄 test_c1_vs_c64.rs
                                                📄 test_channel.rs
                                                📄 test_chord.rs
                                                📄 test_cli.rs
                                                📄 test_config.rs
                                                📄 test_dotted_notes.rs
                                                📄 test_drum_channel.rs
                                                📄 test_key_transpose.rs
                                                📄 test_length.rs
                                                📄 test_modifier.rs
                                                📄 test_note_length.rs
                                                📄 test_octave.rs
                                                📄 test_pass1.rs
                                                📄 test_pass2.rs
                                                📄 test_pass3.rs
                                                📄 test_pass4.rs
                                                📄 test_program_change.rs
                                                📄 test_rest.rs
                                                📄 test_tempo.rs
                                                📄 test_velocity.rs
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
                                                📄 tree-sitter-mml.wasm
                                            📄 _config.yml
                                            📄 build.rs
                                            📁 demo/
                                              📄 .gitignore
                                              📖 FEATURES.md
                                              📖 README.md
                                              🌐 index.html
                                              📊 package.json
                                              📁 src/
                                                📘 audioPlayback.ts
                                                📘 audioRenderer.ts
                                                📘 main.ts
                                                📘 midiReader.ts
                                                📘 mmlConverter.ts
                                                📘 parseMidiNotes.ts
                                                📘 smfToYm2151.ts
                                                📘 state.ts
                                                📘 treeToJSON.ts
                                                📘 ui.ts
                                                📘 visualization.ts
                                                📘 wavExport.ts
                                              📄 test-loader.mjs
                                              📄 test-register.mjs
                                              📁 tests/
                                                📘 audioBufferToWav.test.ts
                                                📘 midiReader.test.ts
                                                📘 parseMidiNotes.test.ts
                                                📘 treeToJSON.test.ts
                                            📁 demo-library/
                                              🌐 index.html
                                              📊 package.json
                                            📁 generated-docs/
                                            🌐 googled947dc864c270e07.html
                                            📁 issue-notes/
                                              📖 103.md
                                              📖 123.md
                                              📖 133.md
                                              📖 39.md
                                              📖 44.md
                                            📄 mmlabc-to-smf-rust.toml.example
                                            📁 mmlabc-to-smf-wasm/
                                              📄 Cargo.lock
                                              📄 Cargo.toml
                                              📁 src/
                                                📄 lib.rs
                                                📄 token_extractor.rs
                                              📁 tests/
                                                📄 parity.rs
                                            📊 package.json
                                            📁 scripts/
                                              📖 README.md
                                              📄 build-demo.sh
                                              📄 transform-demo-paths.sh
                                            📁 src/
                                              📄 attachment_json.rs
                                              📄 config.rs
                                              📄 lib.rs
                                              📄 main.rs
                                              📄 mml_preprocessor.rs
                                              📄 parse_tree_tokens.rs
                                              📄 pass1_parser.rs
                                              📄 pass2_ast.rs
                                              📄 pass3_events.rs
                                              📄 pass4_midi.rs
                                              📄 tree_sitter_mml.rs
                                              📄 types.rs
                                            📁 tests/
                                              📄 integration_test.rs
                                              📄 test_attachment_json.rs
                                              📄 test_c1_vs_c64.rs
                                              📄 test_channel.rs
                                              📄 test_chord.rs
                                              📄 test_cli.rs
                                              📄 test_config.rs
                                              📄 test_dotted_notes.rs
                                              📄 test_drum_channel.rs
                                              📄 test_key_transpose.rs
                                              📄 test_length.rs
                                              📄 test_modifier.rs
                                              📄 test_note_length.rs
                                              📄 test_octave.rs
                                              📄 test_pass1.rs
                                              📄 test_pass2.rs
                                              📄 test_pass3.rs
                                              📄 test_pass4.rs
                                              📄 test_program_change.rs
                                              📄 test_rest.rs
                                              📄 test_tempo.rs
                                              📄 test_velocity.rs
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
                                              📄 tree-sitter-mml.wasm
                                          📄 _config.yml
                                          📄 build.rs
                                          📁 demo/
                                            📄 .gitignore
                                            📖 FEATURES.md
                                            📖 README.md
                                            🌐 index.html
                                            📊 package.json
                                            📁 src/
                                              📘 audioPlayback.ts
                                              📘 audioRenderer.ts
                                              📘 main.ts
                                              📘 midiReader.ts
                                              📘 mmlConverter.ts
                                              📘 parseMidiNotes.ts
                                              📘 smfToYm2151.ts
                                              📘 state.ts
                                              📘 treeToJSON.ts
                                              📘 ui.ts
                                              📘 visualization.ts
                                              📘 wavExport.ts
                                            📄 test-loader.mjs
                                            📄 test-register.mjs
                                            📁 tests/
                                              📘 audioBufferToWav.test.ts
                                              📘 midiReader.test.ts
                                              📘 parseMidiNotes.test.ts
                                              📘 treeToJSON.test.ts
                                          📁 demo-library/
                                            🌐 index.html
                                            📊 package.json
                                          📁 generated-docs/
                                          🌐 googled947dc864c270e07.html
                                          📁 issue-notes/
                                            📖 103.md
                                            📖 123.md
                                            📖 133.md
                                            📖 39.md
                                            📖 44.md
                                          📄 mmlabc-to-smf-rust.toml.example
                                          📁 mmlabc-to-smf-wasm/
                                            📄 Cargo.lock
                                            📄 Cargo.toml
                                            📁 src/
                                              📄 lib.rs
                                              📄 token_extractor.rs
                                            📁 tests/
                                              📄 parity.rs
                                          📊 package.json
                                          📁 scripts/
                                            📖 README.md
                                            📄 build-demo.sh
                                            📄 transform-demo-paths.sh
                                          📁 src/
                                            📄 attachment_json.rs
                                            📄 config.rs
                                            📄 lib.rs
                                            📄 main.rs
                                            📄 mml_preprocessor.rs
                                            📄 parse_tree_tokens.rs
                                            📄 pass1_parser.rs
                                            📄 pass2_ast.rs
                                            📄 pass3_events.rs
                                            📄 pass4_midi.rs
                                            📄 tree_sitter_mml.rs
                                            📄 types.rs
                                          📁 tests/
                                            📄 integration_test.rs
                                            📄 test_attachment_json.rs
                                            📄 test_c1_vs_c64.rs
                                            📄 test_channel.rs
                                            📄 test_chord.rs
                                            📄 test_cli.rs
                                            📄 test_config.rs
                                            📄 test_dotted_notes.rs
                                            📄 test_drum_channel.rs
                                            📄 test_key_transpose.rs
                                            📄 test_length.rs
                                            📄 test_modifier.rs
                                            📄 test_note_length.rs
                                            📄 test_octave.rs
                                            📄 test_pass1.rs
                                            📄 test_pass2.rs
                                            📄 test_pass3.rs
                                            📄 test_pass4.rs
                                            📄 test_program_change.rs
                                            📄 test_rest.rs
                                            📄 test_tempo.rs
                                            📄 test_velocity.rs
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
                                            📄 tree-sitter-mml.wasm
                                        📄 _config.yml
                                        📄 build.rs
                                        📁 demo/
                                          📄 .gitignore
                                          📖 FEATURES.md
                                          📖 README.md
                                          🌐 index.html
                                          📊 package.json
                                          📁 src/
                                            📘 audioPlayback.ts
                                            📘 audioRenderer.ts
                                            📘 main.ts
                                            📘 midiReader.ts
                                            📘 mmlConverter.ts
                                            📘 parseMidiNotes.ts
                                            📘 smfToYm2151.ts
                                            📘 state.ts
                                            📘 treeToJSON.ts
                                            📘 ui.ts
                                            📘 visualization.ts
                                            📘 wavExport.ts
                                          📄 test-loader.mjs
                                          📄 test-register.mjs
                                          📁 tests/
                                            📘 audioBufferToWav.test.ts
                                            📘 midiReader.test.ts
                                            📘 parseMidiNotes.test.ts
                                            📘 treeToJSON.test.ts
                                        📁 demo-library/
                                          🌐 index.html
                                          📊 package.json
                                        📁 generated-docs/
                                        🌐 googled947dc864c270e07.html
                                        📁 issue-notes/
                                          📖 103.md
                                          📖 123.md
                                          📖 133.md
                                          📖 39.md
                                          📖 44.md
                                        📄 mmlabc-to-smf-rust.toml.example
                                        📁 mmlabc-to-smf-wasm/
                                          📄 Cargo.lock
                                          📄 Cargo.toml
                                          📁 src/
                                            📄 lib.rs
                                            📄 token_extractor.rs
                                          📁 tests/
                                            📄 parity.rs
                                        📊 package.json
                                        📁 scripts/
                                          📖 README.md
                                          📄 build-demo.sh
                                          📄 transform-demo-paths.sh
                                        📁 src/
                                          📄 attachment_json.rs
                                          📄 config.rs
                                          📄 lib.rs
                                          📄 main.rs
                                          📄 mml_preprocessor.rs
                                          📄 parse_tree_tokens.rs
                                          📄 pass1_parser.rs
                                          📄 pass2_ast.rs
                                          📄 pass3_events.rs
                                          📄 pass4_midi.rs
                                          📄 tree_sitter_mml.rs
                                          📄 types.rs
                                        📁 tests/
                                          📄 integration_test.rs
                                          📄 test_attachment_json.rs
                                          📄 test_c1_vs_c64.rs
                                          📄 test_channel.rs
                                          📄 test_chord.rs
                                          📄 test_cli.rs
                                          📄 test_config.rs
                                          📄 test_dotted_notes.rs
                                          📄 test_drum_channel.rs
                                          📄 test_key_transpose.rs
                                          📄 test_length.rs
                                          📄 test_modifier.rs
                                          📄 test_note_length.rs
                                          📄 test_octave.rs
                                          📄 test_pass1.rs
                                          📄 test_pass2.rs
                                          📄 test_pass3.rs
                                          📄 test_pass4.rs
                                          📄 test_program_change.rs
                                          📄 test_rest.rs
                                          📄 test_tempo.rs
                                          📄 test_velocity.rs
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
                                          📄 tree-sitter-mml.wasm
                                      📄 _config.yml
                                      📄 build.rs
                                      📁 demo/
                                        📄 .gitignore
                                        📖 FEATURES.md
                                        📖 README.md
                                        🌐 index.html
                                        📊 package.json
                                        📁 src/
                                          📘 audioPlayback.ts
                                          📘 audioRenderer.ts
                                          📘 main.ts
                                          📘 midiReader.ts
                                          📘 mmlConverter.ts
                                          📘 parseMidiNotes.ts
                                          📘 smfToYm2151.ts
                                          📘 state.ts
                                          📘 treeToJSON.ts
                                          📘 ui.ts
                                          📘 visualization.ts
                                          📘 wavExport.ts
                                        📄 test-loader.mjs
                                        📄 test-register.mjs
                                        📁 tests/
                                          📘 audioBufferToWav.test.ts
                                          📘 midiReader.test.ts
                                          📘 parseMidiNotes.test.ts
                                          📘 treeToJSON.test.ts
                                      📁 demo-library/
                                        🌐 index.html
                                        📊 package.json
                                      📁 generated-docs/
                                      🌐 googled947dc864c270e07.html
                                      📁 issue-notes/
                                        📖 103.md
                                        📖 123.md
                                        📖 133.md
                                        📖 39.md
                                        📖 44.md
                                      📄 mmlabc-to-smf-rust.toml.example
                                      📁 mmlabc-to-smf-wasm/
                                        📄 Cargo.lock
                                        📄 Cargo.toml
                                        📁 src/
                                          📄 lib.rs
                                          📄 token_extractor.rs
                                        📁 tests/
                                          📄 parity.rs
                                      📊 package.json
                                      📁 scripts/
                                        📖 README.md
                                        📄 build-demo.sh
                                        📄 transform-demo-paths.sh
                                      📁 src/
                                        📄 attachment_json.rs
                                        📄 config.rs
                                        📄 lib.rs
                                        📄 main.rs
                                        📄 mml_preprocessor.rs
                                        📄 parse_tree_tokens.rs
                                        📄 pass1_parser.rs
                                        📄 pass2_ast.rs
                                        📄 pass3_events.rs
                                        📄 pass4_midi.rs
                                        📄 tree_sitter_mml.rs
                                        📄 types.rs
                                      📁 tests/
                                        📄 integration_test.rs
                                        📄 test_attachment_json.rs
                                        📄 test_c1_vs_c64.rs
                                        📄 test_channel.rs
                                        📄 test_chord.rs
                                        📄 test_cli.rs
                                        📄 test_config.rs
                                        📄 test_dotted_notes.rs
                                        📄 test_drum_channel.rs
                                        📄 test_key_transpose.rs
                                        📄 test_length.rs
                                        📄 test_modifier.rs
                                        📄 test_note_length.rs
                                        📄 test_octave.rs
                                        📄 test_pass1.rs
                                        📄 test_pass2.rs
                                        📄 test_pass3.rs
                                        📄 test_pass4.rs
                                        📄 test_program_change.rs
                                        📄 test_rest.rs
                                        📄 test_tempo.rs
                                        📄 test_velocity.rs
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
                                        📄 tree-sitter-mml.wasm
                                    📄 _config.yml
                                    📄 build.rs
                                    📁 demo/
                                      📄 .gitignore
                                      📖 FEATURES.md
                                      📖 README.md
                                      🌐 index.html
                                      📊 package.json
                                      📁 src/
                                        📘 audioPlayback.ts
                                        📘 audioRenderer.ts
                                        📘 main.ts
                                        📘 midiReader.ts
                                        📘 mmlConverter.ts
                                        📘 parseMidiNotes.ts
                                        📘 smfToYm2151.ts
                                        📘 state.ts
                                        📘 treeToJSON.ts
                                        📘 ui.ts
                                        📘 visualization.ts
                                        📘 wavExport.ts
                                      📄 test-loader.mjs
                                      📄 test-register.mjs
                                      📁 tests/
                                        📘 audioBufferToWav.test.ts
                                        📘 midiReader.test.ts
                                        📘 parseMidiNotes.test.ts
                                        📘 treeToJSON.test.ts
                                    📁 demo-library/
                                      🌐 index.html
                                      📊 package.json
                                    📁 generated-docs/
                                    🌐 googled947dc864c270e07.html
                                    📁 issue-notes/
                                      📖 103.md
                                      📖 123.md
                                      📖 133.md
                                      📖 39.md
                                      📖 44.md
                                    📄 mmlabc-to-smf-rust.toml.example
                                    📁 mmlabc-to-smf-wasm/
                                      📄 Cargo.lock
                                      📄 Cargo.toml
                                      📁 src/
                                        📄 lib.rs
                                        📄 token_extractor.rs
                                      📁 tests/
                                        📄 parity.rs
                                    📊 package.json
                                    📁 scripts/
                                      📖 README.md
                                      📄 build-demo.sh
                                      📄 transform-demo-paths.sh
                                    📁 src/
                                      📄 attachment_json.rs
                                      📄 config.rs
                                      📄 lib.rs
                                      📄 main.rs
                                      📄 mml_preprocessor.rs
                                      📄 parse_tree_tokens.rs
                                      📄 pass1_parser.rs
                                      📄 pass2_ast.rs
                                      📄 pass3_events.rs
                                      📄 pass4_midi.rs
                                      📄 tree_sitter_mml.rs
                                      📄 types.rs
                                    📁 tests/
                                      📄 integration_test.rs
                                      📄 test_attachment_json.rs
                                      📄 test_c1_vs_c64.rs
                                      📄 test_channel.rs
                                      📄 test_chord.rs
                                      📄 test_cli.rs
                                      📄 test_config.rs
                                      📄 test_dotted_notes.rs
                                      📄 test_drum_channel.rs
                                      📄 test_key_transpose.rs
                                      📄 test_length.rs
                                      📄 test_modifier.rs
                                      📄 test_note_length.rs
                                      📄 test_octave.rs
                                      📄 test_pass1.rs
                                      📄 test_pass2.rs
                                      📄 test_pass3.rs
                                      📄 test_pass4.rs
                                      📄 test_program_change.rs
                                      📄 test_rest.rs
                                      📄 test_tempo.rs
                                      📄 test_velocity.rs
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
                                      📄 tree-sitter-mml.wasm
                                  📄 _config.yml
                                  📄 build.rs
                                  📁 demo/
                                    📄 .gitignore
                                    📖 FEATURES.md
                                    📖 README.md
                                    🌐 index.html
                                    📊 package.json
                                    📁 src/
                                      📘 audioPlayback.ts
                                      📘 audioRenderer.ts
                                      📘 main.ts
                                      📘 midiReader.ts
                                      📘 mmlConverter.ts
                                      📘 parseMidiNotes.ts
                                      📘 smfToYm2151.ts
                                      📘 state.ts
                                      📘 treeToJSON.ts
                                      📘 ui.ts
                                      📘 visualization.ts
                                      📘 wavExport.ts
                                    📄 test-loader.mjs
                                    📄 test-register.mjs
                                    📁 tests/
                                      📘 audioBufferToWav.test.ts
                                      📘 midiReader.test.ts
                                      📘 parseMidiNotes.test.ts
                                      📘 treeToJSON.test.ts
                                  📁 demo-library/
                                    🌐 index.html
                                    📊 package.json
                                  📁 generated-docs/
                                  🌐 googled947dc864c270e07.html
                                  📁 issue-notes/
                                    📖 103.md
                                    📖 123.md
                                    📖 133.md
                                    📖 39.md
                                    📖 44.md
                                  📄 mmlabc-to-smf-rust.toml.example
                                  📁 mmlabc-to-smf-wasm/
                                    📄 Cargo.lock
                                    📄 Cargo.toml
                                    📁 src/
                                      📄 lib.rs
                                      📄 token_extractor.rs
                                    📁 tests/
                                      📄 parity.rs
                                  📊 package.json
                                  📁 scripts/
                                    📖 README.md
                                    📄 build-demo.sh
                                    📄 transform-demo-paths.sh
                                  📁 src/
                                    📄 attachment_json.rs
                                    📄 config.rs
                                    📄 lib.rs
                                    📄 main.rs
                                    📄 mml_preprocessor.rs
                                    📄 parse_tree_tokens.rs
                                    📄 pass1_parser.rs
                                    📄 pass2_ast.rs
                                    📄 pass3_events.rs
                                    📄 pass4_midi.rs
                                    📄 tree_sitter_mml.rs
                                    📄 types.rs
                                  📁 tests/
                                    📄 integration_test.rs
                                    📄 test_attachment_json.rs
                                    📄 test_c1_vs_c64.rs
                                    📄 test_channel.rs
                                    📄 test_chord.rs
                                    📄 test_cli.rs
                                    📄 test_config.rs
                                    📄 test_dotted_notes.rs
                                    📄 test_drum_channel.rs
                                    📄 test_key_transpose.rs
                                    📄 test_length.rs
                                    📄 test_modifier.rs
                                    📄 test_note_length.rs
                                    📄 test_octave.rs
                                    📄 test_pass1.rs
                                    📄 test_pass2.rs
                                    📄 test_pass3.rs
                                    📄 test_pass4.rs
                                    📄 test_program_change.rs
                                    📄 test_rest.rs
                                    📄 test_tempo.rs
                                    📄 test_velocity.rs
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
                                    📄 tree-sitter-mml.wasm
                                📄 _config.yml
                                📄 build.rs
                                📁 demo/
                                  📄 .gitignore
                                  📖 FEATURES.md
                                  📖 README.md
                                  🌐 index.html
                                  📊 package.json
                                  📁 src/
                                    📘 audioPlayback.ts
                                    📘 audioRenderer.ts
                                    📘 main.ts
                                    📘 midiReader.ts
                                    📘 mmlConverter.ts
                                    📘 parseMidiNotes.ts
                                    📘 smfToYm2151.ts
                                    📘 state.ts
                                    📘 treeToJSON.ts
                                    📘 ui.ts
                                    📘 visualization.ts
                                    📘 wavExport.ts
                                  📄 test-loader.mjs
                                  📄 test-register.mjs
                                  📁 tests/
                                    📘 audioBufferToWav.test.ts
                                    📘 midiReader.test.ts
                                    📘 parseMidiNotes.test.ts
                                    📘 treeToJSON.test.ts
                                📁 demo-library/
                                  🌐 index.html
                                  📊 package.json
                                📁 generated-docs/
                                🌐 googled947dc864c270e07.html
                                📁 issue-notes/
                                  📖 103.md
                                  📖 123.md
                                  📖 133.md
                                  📖 39.md
                                  📖 44.md
                                📄 mmlabc-to-smf-rust.toml.example
                                📁 mmlabc-to-smf-wasm/
                                  📄 Cargo.lock
                                  📄 Cargo.toml
                                  📁 src/
                                    📄 lib.rs
                                    📄 token_extractor.rs
                                  📁 tests/
                                    📄 parity.rs
                                📊 package.json
                                📁 scripts/
                                  📖 README.md
                                  📄 build-demo.sh
                                  📄 transform-demo-paths.sh
                                📁 src/
                                  📄 attachment_json.rs
                                  📄 config.rs
                                  📄 lib.rs
                                  📄 main.rs
                                  📄 mml_preprocessor.rs
                                  📄 parse_tree_tokens.rs
                                  📄 pass1_parser.rs
                                  📄 pass2_ast.rs
                                  📄 pass3_events.rs
                                  📄 pass4_midi.rs
                                  📄 tree_sitter_mml.rs
                                  📄 types.rs
                                📁 tests/
                                  📄 integration_test.rs
                                  📄 test_attachment_json.rs
                                  📄 test_c1_vs_c64.rs
                                  📄 test_channel.rs
                                  📄 test_chord.rs
                                  📄 test_cli.rs
                                  📄 test_config.rs
                                  📄 test_dotted_notes.rs
                                  📄 test_drum_channel.rs
                                  📄 test_key_transpose.rs
                                  📄 test_length.rs
                                  📄 test_modifier.rs
                                  📄 test_note_length.rs
                                  📄 test_octave.rs
                                  📄 test_pass1.rs
                                  📄 test_pass2.rs
                                  📄 test_pass3.rs
                                  📄 test_pass4.rs
                                  📄 test_program_change.rs
                                  📄 test_rest.rs
                                  📄 test_tempo.rs
                                  📄 test_velocity.rs
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
                                  📄 tree-sitter-mml.wasm
                              📄 _config.yml
                              📄 build.rs
                              📁 demo/
                                📄 .gitignore
                                📖 FEATURES.md
                                📖 README.md
                                🌐 index.html
                                📊 package.json
                                📁 src/
                                  📘 audioPlayback.ts
                                  📘 audioRenderer.ts
                                  📘 main.ts
                                  📘 midiReader.ts
                                  📘 mmlConverter.ts
                                  📘 parseMidiNotes.ts
                                  📘 smfToYm2151.ts
                                  📘 state.ts
                                  📘 treeToJSON.ts
                                  📘 ui.ts
                                  📘 visualization.ts
                                  📘 wavExport.ts
                                📄 test-loader.mjs
                                📄 test-register.mjs
                                📁 tests/
                                  📘 audioBufferToWav.test.ts
                                  📘 midiReader.test.ts
                                  📘 parseMidiNotes.test.ts
                                  📘 treeToJSON.test.ts
                              📁 demo-library/
                                🌐 index.html
                                📊 package.json
                              📁 generated-docs/
                              🌐 googled947dc864c270e07.html
                              📁 issue-notes/
                                📖 103.md
                                📖 123.md
                                📖 133.md
                                📖 39.md
                                📖 44.md
                              📄 mmlabc-to-smf-rust.toml.example
                              📁 mmlabc-to-smf-wasm/
                                📄 Cargo.lock
                                📄 Cargo.toml
                                📁 src/
                                  📄 lib.rs
                                  📄 token_extractor.rs
                                📁 tests/
                                  📄 parity.rs
                              📊 package.json
                              📁 scripts/
                                📖 README.md
                                📄 build-demo.sh
                                📄 transform-demo-paths.sh
                              📁 src/
                                📄 attachment_json.rs
                                📄 config.rs
                                📄 lib.rs
                                📄 main.rs
                                📄 mml_preprocessor.rs
                                📄 parse_tree_tokens.rs
                                📄 pass1_parser.rs
                                📄 pass2_ast.rs
                                📄 pass3_events.rs
                                📄 pass4_midi.rs
                                📄 tree_sitter_mml.rs
                                📄 types.rs
                              📁 tests/
                                📄 integration_test.rs
                                📄 test_attachment_json.rs
                                📄 test_c1_vs_c64.rs
                                📄 test_channel.rs
                                📄 test_chord.rs
                                📄 test_cli.rs
                                📄 test_config.rs
                                📄 test_dotted_notes.rs
                                📄 test_drum_channel.rs
                                📄 test_key_transpose.rs
                                📄 test_length.rs
                                📄 test_modifier.rs
                                📄 test_note_length.rs
                                📄 test_octave.rs
                                📄 test_pass1.rs
                                📄 test_pass2.rs
                                📄 test_pass3.rs
                                📄 test_pass4.rs
                                📄 test_program_change.rs
                                📄 test_rest.rs
                                📄 test_tempo.rs
                                📄 test_velocity.rs
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
                                📄 tree-sitter-mml.wasm
                            📄 _config.yml
                            📄 build.rs
                            📁 demo/
                              📄 .gitignore
                              📖 FEATURES.md
                              📖 README.md
                              🌐 index.html
                              📊 package.json
                              📁 src/
                                📘 audioPlayback.ts
                                📘 audioRenderer.ts
                                📘 main.ts
                                📘 midiReader.ts
                                📘 mmlConverter.ts
                                📘 parseMidiNotes.ts
                                📘 smfToYm2151.ts
                                📘 state.ts
                                📘 treeToJSON.ts
                                📘 ui.ts
                                📘 visualization.ts
                                📘 wavExport.ts
                              📄 test-loader.mjs
                              📄 test-register.mjs
                              📁 tests/
                                📘 audioBufferToWav.test.ts
                                📘 midiReader.test.ts
                                📘 parseMidiNotes.test.ts
                                📘 treeToJSON.test.ts
                            📁 demo-library/
                              🌐 index.html
                              📊 package.json
                            📁 generated-docs/
                            🌐 googled947dc864c270e07.html
                            📁 issue-notes/
                              📖 103.md
                              📖 123.md
                              📖 133.md
                              📖 39.md
                              📖 44.md
                            📄 mmlabc-to-smf-rust.toml.example
                            📁 mmlabc-to-smf-wasm/
                              📄 Cargo.lock
                              📄 Cargo.toml
                              📁 src/
                                📄 lib.rs
                                📄 token_extractor.rs
                              📁 tests/
                                📄 parity.rs
                            📊 package.json
                            📁 scripts/
                              📖 README.md
                              📄 build-demo.sh
                              📄 transform-demo-paths.sh
                            📁 src/
                              📄 attachment_json.rs
                              📄 config.rs
                              📄 lib.rs
                              📄 main.rs
                              📄 mml_preprocessor.rs
                              📄 parse_tree_tokens.rs
                              📄 pass1_parser.rs
                              📄 pass2_ast.rs
                              📄 pass3_events.rs
                              📄 pass4_midi.rs
                              📄 tree_sitter_mml.rs
                              📄 types.rs
                            📁 tests/
                              📄 integration_test.rs
                              📄 test_attachment_json.rs
                              📄 test_c1_vs_c64.rs
                              📄 test_channel.rs
                              📄 test_chord.rs
                              📄 test_cli.rs
                              📄 test_config.rs
                              📄 test_dotted_notes.rs
                              📄 test_drum_channel.rs
                              📄 test_key_transpose.rs
                              📄 test_length.rs
                              📄 test_modifier.rs
                              📄 test_note_length.rs
                              📄 test_octave.rs
                              📄 test_pass1.rs
                              📄 test_pass2.rs
                              📄 test_pass3.rs
                              📄 test_pass4.rs
                              📄 test_program_change.rs
                              📄 test_rest.rs
                              📄 test_tempo.rs
                              📄 test_velocity.rs
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
                              📄 tree-sitter-mml.wasm
                          📄 _config.yml
                          📄 build.rs
                          📁 demo/
                            📄 .gitignore
                            📖 FEATURES.md
                            📖 README.md
                            🌐 index.html
                            📊 package.json
                            📁 src/
                              📘 audioPlayback.ts
                              📘 audioRenderer.ts
                              📘 main.ts
                              📘 midiReader.ts
                              📘 mmlConverter.ts
                              📘 parseMidiNotes.ts
                              📘 smfToYm2151.ts
                              📘 state.ts
                              📘 treeToJSON.ts
                              📘 ui.ts
                              📘 visualization.ts
                              📘 wavExport.ts
                            📄 test-loader.mjs
                            📄 test-register.mjs
                            📁 tests/
                              📘 audioBufferToWav.test.ts
                              📘 midiReader.test.ts
                              📘 parseMidiNotes.test.ts
                              📘 treeToJSON.test.ts
                          📁 demo-library/
                            🌐 index.html
                            📊 package.json
                          📁 generated-docs/
                          🌐 googled947dc864c270e07.html
                          📁 issue-notes/
                            📖 103.md
                            📖 123.md
                            📖 133.md
                            📖 39.md
                            📖 44.md
                          📄 mmlabc-to-smf-rust.toml.example
                          📁 mmlabc-to-smf-wasm/
                            📄 Cargo.lock
                            📄 Cargo.toml
                            📁 src/
                              📄 lib.rs
                              📄 token_extractor.rs
                            📁 tests/
                              📄 parity.rs
                          📊 package.json
                          📁 scripts/
                            📖 README.md
                            📄 build-demo.sh
                            📄 transform-demo-paths.sh
                          📁 src/
                            📄 attachment_json.rs
                            📄 config.rs
                            📄 lib.rs
                            📄 main.rs
                            📄 mml_preprocessor.rs
                            📄 parse_tree_tokens.rs
                            📄 pass1_parser.rs
                            📄 pass2_ast.rs
                            📄 pass3_events.rs
                            📄 pass4_midi.rs
                            📄 tree_sitter_mml.rs
                            📄 types.rs
                          📁 tests/
                            📄 integration_test.rs
                            📄 test_attachment_json.rs
                            📄 test_c1_vs_c64.rs
                            📄 test_channel.rs
                            📄 test_chord.rs
                            📄 test_cli.rs
                            📄 test_config.rs
                            📄 test_dotted_notes.rs
                            📄 test_drum_channel.rs
                            📄 test_key_transpose.rs
                            📄 test_length.rs
                            📄 test_modifier.rs
                            📄 test_note_length.rs
                            📄 test_octave.rs
                            📄 test_pass1.rs
                            📄 test_pass2.rs
                            📄 test_pass3.rs
                            📄 test_pass4.rs
                            📄 test_program_change.rs
                            📄 test_rest.rs
                            📄 test_tempo.rs
                            📄 test_velocity.rs
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
                            📄 tree-sitter-mml.wasm
                        📄 _config.yml
                        📄 build.rs
                        📁 demo/
                          📄 .gitignore
                          📖 FEATURES.md
                          📖 README.md
                          🌐 index.html
                          📊 package.json
                          📁 src/
                            📘 audioPlayback.ts
                            📘 audioRenderer.ts
                            📘 main.ts
                            📘 midiReader.ts
                            📘 mmlConverter.ts
                            📘 parseMidiNotes.ts
                            📘 smfToYm2151.ts
                            📘 state.ts
                            📘 treeToJSON.ts
                            📘 ui.ts
                            📘 visualization.ts
                            📘 wavExport.ts
                          📄 test-loader.mjs
                          📄 test-register.mjs
                          📁 tests/
                            📘 audioBufferToWav.test.ts
                            📘 midiReader.test.ts
                            📘 parseMidiNotes.test.ts
                            📘 treeToJSON.test.ts
                        📁 demo-library/
                          🌐 index.html
                          📊 package.json
                        📁 generated-docs/
                          📖 development-status-generated-prompt.md
                        🌐 googled947dc864c270e07.html
                        📁 issue-notes/
                          📖 103.md
                          📖 123.md
                          📖 133.md
                          📖 39.md
                          📖 44.md
                        📄 mmlabc-to-smf-rust.toml.example
                        📁 mmlabc-to-smf-wasm/
                          📄 Cargo.lock
                          📄 Cargo.toml
                          📁 src/
                            📄 lib.rs
                            📄 token_extractor.rs
                          📁 tests/
                            📄 parity.rs
                        📊 package.json
                        📁 scripts/
                          📖 README.md
                          📄 build-demo.sh
                          📄 transform-demo-paths.sh
                        📁 src/
                          📄 attachment_json.rs
                          📄 config.rs
                          📄 lib.rs
                          📄 main.rs
                          📄 mml_preprocessor.rs
                          📄 parse_tree_tokens.rs
                          📄 pass1_parser.rs
                          📄 pass2_ast.rs
                          📄 pass3_events.rs
                          📄 pass4_midi.rs
                          📄 tree_sitter_mml.rs
                          📄 types.rs
                        📁 tests/
                          📄 integration_test.rs
                          📄 test_attachment_json.rs
                          📄 test_c1_vs_c64.rs
                          📄 test_channel.rs
                          📄 test_chord.rs
                          📄 test_cli.rs
                          📄 test_config.rs
                          📄 test_dotted_notes.rs
                          📄 test_drum_channel.rs
                          📄 test_key_transpose.rs
                          📄 test_length.rs
                          📄 test_modifier.rs
                          📄 test_note_length.rs
                          📄 test_octave.rs
                          📄 test_pass1.rs
                          📄 test_pass2.rs
                          📄 test_pass3.rs
                          📄 test_pass4.rs
                          📄 test_program_change.rs
                          📄 test_rest.rs
                          📄 test_tempo.rs
                          📄 test_velocity.rs
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
                          📄 tree-sitter-mml.wasm
                      📄 _config.yml
                      📄 build.rs
                      📁 demo/
                        📄 .gitignore
                        📖 FEATURES.md
                        📖 README.md
                        🌐 index.html
                        📊 package.json
                        📁 src/
                          📘 audioPlayback.ts
                          📘 audioRenderer.ts
                          📘 main.ts
                          📘 midiReader.ts
                          📘 mmlConverter.ts
                          📘 parseMidiNotes.ts
                          📘 smfToYm2151.ts
                          📘 state.ts
                          📘 treeToJSON.ts
                          📘 ui.ts
                          📘 visualization.ts
                          📘 wavExport.ts
                        📄 test-loader.mjs
                        📄 test-register.mjs
                        📁 tests/
                          📘 audioBufferToWav.test.ts
                          📘 midiReader.test.ts
                          📘 parseMidiNotes.test.ts
                          📘 treeToJSON.test.ts
                      📁 demo-library/
                        🌐 index.html
                        📊 package.json
                      📁 generated-docs/
                        📖 development-status-generated-prompt.md
                      🌐 googled947dc864c270e07.html
                      📁 issue-notes/
                        📖 103.md
                        📖 123.md
                        📖 133.md
                        📖 39.md
                        📖 44.md
                      📄 mmlabc-to-smf-rust.toml.example
                      📁 mmlabc-to-smf-wasm/
                        📄 Cargo.lock
                        📄 Cargo.toml
                        📁 src/
                          📄 lib.rs
                          📄 token_extractor.rs
                        📁 tests/
                          📄 parity.rs
                      📊 package.json
                      📁 scripts/
                        📖 README.md
                        📄 build-demo.sh
                        📄 transform-demo-paths.sh
                      📁 src/
                        📄 attachment_json.rs
                        📄 config.rs
                        📄 lib.rs
                        📄 main.rs
                        📄 mml_preprocessor.rs
                        📄 parse_tree_tokens.rs
                        📄 pass1_parser.rs
                        📄 pass2_ast.rs
                        📄 pass3_events.rs
                        📄 pass4_midi.rs
                        📄 tree_sitter_mml.rs
                        📄 types.rs
                      📁 tests/
                        📄 integration_test.rs
                        📄 test_attachment_json.rs
                        📄 test_c1_vs_c64.rs
                        📄 test_channel.rs
                        📄 test_chord.rs
                        📄 test_cli.rs
                        📄 test_config.rs
                        📄 test_dotted_notes.rs
                        📄 test_drum_channel.rs
                        📄 test_key_transpose.rs
                        📄 test_length.rs
                        📄 test_modifier.rs
                        📄 test_note_length.rs
                        📄 test_octave.rs
                        📄 test_pass1.rs
                        📄 test_pass2.rs
                        📄 test_pass3.rs
                        📄 test_pass4.rs
                        📄 test_program_change.rs
                        📄 test_rest.rs
                        📄 test_tempo.rs
                        📄 test_velocity.rs
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
                        📄 tree-sitter-mml.wasm
                    📄 _config.yml
                    📄 build.rs
                    📁 demo/
                      📄 .gitignore
                      📖 FEATURES.md
                      📖 README.md
                      🌐 index.html
                      📊 package.json
                      📁 src/
                        📘 audioPlayback.ts
                        📘 audioRenderer.ts
                        📘 main.ts
                        📘 midiReader.ts
                        📘 mmlConverter.ts
                        📘 parseMidiNotes.ts
                        📘 smfToYm2151.ts
                        📘 state.ts
                        📘 treeToJSON.ts
                        📘 ui.ts
                        📘 visualization.ts
                        📘 wavExport.ts
                      📄 test-loader.mjs
                      📄 test-register.mjs
                      📁 tests/
                        📘 audioBufferToWav.test.ts
                        📘 midiReader.test.ts
                        📘 parseMidiNotes.test.ts
                        📘 treeToJSON.test.ts
                    📁 demo-library/
                      🌐 index.html
                      📊 package.json
                    📁 generated-docs/
                      📖 development-status-generated-prompt.md
                    🌐 googled947dc864c270e07.html
                    📁 issue-notes/
                      📖 103.md
                      📖 123.md
                      📖 133.md
                      📖 39.md
                      📖 44.md
                    📄 mmlabc-to-smf-rust.toml.example
                    📁 mmlabc-to-smf-wasm/
                      📄 Cargo.lock
                      📄 Cargo.toml
                      📁 src/
                        📄 lib.rs
                        📄 token_extractor.rs
                      📁 tests/
                        📄 parity.rs
                    📊 package.json
                    📁 scripts/
                      📖 README.md
                      📄 build-demo.sh
                      📄 transform-demo-paths.sh
                    📁 src/
                      📄 attachment_json.rs
                      📄 config.rs
                      📄 lib.rs
                      📄 main.rs
                      📄 mml_preprocessor.rs
                      📄 parse_tree_tokens.rs
                      📄 pass1_parser.rs
                      📄 pass2_ast.rs
                      📄 pass3_events.rs
                      📄 pass4_midi.rs
                      📄 tree_sitter_mml.rs
                      📄 types.rs
                    📁 tests/
                      📄 integration_test.rs
                      📄 test_attachment_json.rs
                      📄 test_c1_vs_c64.rs
                      📄 test_channel.rs
                      📄 test_chord.rs
                      📄 test_cli.rs
                      📄 test_config.rs
                      📄 test_dotted_notes.rs
                      📄 test_drum_channel.rs
                      📄 test_key_transpose.rs
                      📄 test_length.rs
                      📄 test_modifier.rs
                      📄 test_note_length.rs
                      📄 test_octave.rs
                      📄 test_pass1.rs
                      📄 test_pass2.rs
                      📄 test_pass3.rs
                      📄 test_pass4.rs
                      📄 test_program_change.rs
                      📄 test_rest.rs
                      📄 test_tempo.rs
                      📄 test_velocity.rs
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
                      📄 tree-sitter-mml.wasm
                  📄 _config.yml
                  📄 build.rs
                  📁 demo/
                    📄 .gitignore
                    📖 FEATURES.md
                    📖 README.md
                    🌐 index.html
                    📊 package.json
                    📁 src/
                      📘 audioPlayback.ts
                      📘 audioRenderer.ts
                      📘 main.ts
                      📘 midiReader.ts
                      📘 mmlConverter.ts
                      📘 parseMidiNotes.ts
                      📘 smfToYm2151.ts
                      📘 state.ts
                      📘 treeToJSON.ts
                      📘 ui.ts
                      📘 visualization.ts
                      📘 wavExport.ts
                    📄 test-loader.mjs
                    📄 test-register.mjs
                    📁 tests/
                      📘 audioBufferToWav.test.ts
                      📘 midiReader.test.ts
                      📘 parseMidiNotes.test.ts
                      📘 treeToJSON.test.ts
                  📁 demo-library/
                    🌐 index.html
                    📊 package.json
                  📁 generated-docs/
                    📖 development-status-generated-prompt.md
                  🌐 googled947dc864c270e07.html
                  📁 issue-notes/
                    📖 103.md
                    📖 123.md
                    📖 133.md
                    📖 39.md
                    📖 44.md
                  📄 mmlabc-to-smf-rust.toml.example
                  📁 mmlabc-to-smf-wasm/
                    📄 Cargo.lock
                    📄 Cargo.toml
                    📁 src/
                      📄 lib.rs
                      📄 token_extractor.rs
                    📁 tests/
                      📄 parity.rs
                  📊 package.json
                  📁 scripts/
                    📖 README.md
                    📄 build-demo.sh
                    📄 transform-demo-paths.sh
                  📁 src/
                    📄 attachment_json.rs
                    📄 config.rs
                    📄 lib.rs
                    📄 main.rs
                    📄 mml_preprocessor.rs
                    📄 parse_tree_tokens.rs
                    📄 pass1_parser.rs
                    📄 pass2_ast.rs
                    📄 pass3_events.rs
                    📄 pass4_midi.rs
                    📄 tree_sitter_mml.rs
                    📄 types.rs
                  📁 tests/
                    📄 integration_test.rs
                    📄 test_attachment_json.rs
                    📄 test_c1_vs_c64.rs
                    📄 test_channel.rs
                    📄 test_chord.rs
                    📄 test_cli.rs
                    📄 test_config.rs
                    📄 test_dotted_notes.rs
                    📄 test_drum_channel.rs
                    📄 test_key_transpose.rs
                    📄 test_length.rs
                    📄 test_modifier.rs
                    📄 test_note_length.rs
                    📄 test_octave.rs
                    📄 test_pass1.rs
                    📄 test_pass2.rs
                    📄 test_pass3.rs
                    📄 test_pass4.rs
                    📄 test_program_change.rs
                    📄 test_rest.rs
                    📄 test_tempo.rs
                    📄 test_velocity.rs
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
                    📄 tree-sitter-mml.wasm
                📄 _config.yml
                📄 build.rs
                📁 demo/
                  📄 .gitignore
                  📖 FEATURES.md
                  📖 README.md
                  🌐 index.html
                  📊 package.json
                  📁 src/
                    📘 audioPlayback.ts
                    📘 audioRenderer.ts
                    📘 main.ts
                    📘 midiReader.ts
                    📘 mmlConverter.ts
                    📘 parseMidiNotes.ts
                    📘 smfToYm2151.ts
                    📘 state.ts
                    📘 treeToJSON.ts
                    📘 ui.ts
                    📘 visualization.ts
                    📘 wavExport.ts
                  📄 test-loader.mjs
                  📄 test-register.mjs
                  📁 tests/
                    📘 audioBufferToWav.test.ts
                    📘 midiReader.test.ts
                    📘 parseMidiNotes.test.ts
                    📘 treeToJSON.test.ts
                📁 demo-library/
                  🌐 index.html
                  📊 package.json
                📁 generated-docs/
                  📖 development-status-generated-prompt.md
                🌐 googled947dc864c270e07.html
                📁 issue-notes/
                  📖 103.md
                  📖 123.md
                  📖 133.md
                  📖 39.md
                  📖 44.md
                📄 mmlabc-to-smf-rust.toml.example
                📁 mmlabc-to-smf-wasm/
                  📄 Cargo.lock
                  📄 Cargo.toml
                  📁 src/
                    📄 lib.rs
                    📄 token_extractor.rs
                  📁 tests/
                    📄 parity.rs
                📊 package.json
                📁 scripts/
                  📖 README.md
                  📄 build-demo.sh
                  📄 transform-demo-paths.sh
                📁 src/
                  📄 attachment_json.rs
                  📄 config.rs
                  📄 lib.rs
                  📄 main.rs
                  📄 mml_preprocessor.rs
                  📄 parse_tree_tokens.rs
                  📄 pass1_parser.rs
                  📄 pass2_ast.rs
                  📄 pass3_events.rs
                  📄 pass4_midi.rs
                  📄 tree_sitter_mml.rs
                  📄 types.rs
                📁 tests/
                  📄 integration_test.rs
                  📄 test_attachment_json.rs
                  📄 test_c1_vs_c64.rs
                  📄 test_channel.rs
                  📄 test_chord.rs
                  📄 test_cli.rs
                  📄 test_config.rs
                  📄 test_dotted_notes.rs
                  📄 test_drum_channel.rs
                  📄 test_key_transpose.rs
                  📄 test_length.rs
                  📄 test_modifier.rs
                  📄 test_note_length.rs
                  📄 test_octave.rs
                  📄 test_pass1.rs
                  📄 test_pass2.rs
                  📄 test_pass3.rs
                  📄 test_pass4.rs
                  📄 test_program_change.rs
                  📄 test_rest.rs
                  📄 test_tempo.rs
                  📄 test_velocity.rs
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
                  📄 tree-sitter-mml.wasm
              📄 _config.yml
              📄 build.rs
              📁 demo/
                📄 .gitignore
                📖 FEATURES.md
                📖 README.md
                🌐 index.html
                📊 package.json
                📁 src/
                  📘 audioPlayback.ts
                  📘 audioRenderer.ts
                  📘 main.ts
                  📘 midiReader.ts
                  📘 mmlConverter.ts
                  📘 parseMidiNotes.ts
                  📘 smfToYm2151.ts
                  📘 state.ts
                  📘 treeToJSON.ts
                  📘 ui.ts
                  📘 visualization.ts
                  📘 wavExport.ts
                📄 test-loader.mjs
                📄 test-register.mjs
                📁 tests/
                  📘 audioBufferToWav.test.ts
                  📘 midiReader.test.ts
                  📘 parseMidiNotes.test.ts
                  📘 treeToJSON.test.ts
              📁 demo-library/
                🌐 index.html
                📊 package.json
              📁 generated-docs/
                📖 development-status-generated-prompt.md
              🌐 googled947dc864c270e07.html
              📁 issue-notes/
                📖 103.md
                📖 123.md
                📖 133.md
                📖 39.md
                📖 44.md
              📄 mmlabc-to-smf-rust.toml.example
              📁 mmlabc-to-smf-wasm/
                📄 Cargo.lock
                📄 Cargo.toml
                📁 src/
                  📄 lib.rs
                  📄 token_extractor.rs
                📁 tests/
                  📄 parity.rs
              📊 package.json
              📁 scripts/
                📖 README.md
                📄 build-demo.sh
                📄 transform-demo-paths.sh
              📁 src/
                📄 attachment_json.rs
                📄 config.rs
                📄 lib.rs
                📄 main.rs
                📄 mml_preprocessor.rs
                📄 parse_tree_tokens.rs
                📄 pass1_parser.rs
                📄 pass2_ast.rs
                📄 pass3_events.rs
                📄 pass4_midi.rs
                📄 tree_sitter_mml.rs
                📄 types.rs
              📁 tests/
                📄 integration_test.rs
                📄 test_attachment_json.rs
                📄 test_c1_vs_c64.rs
                📄 test_channel.rs
                📄 test_chord.rs
                📄 test_cli.rs
                📄 test_config.rs
                📄 test_dotted_notes.rs
                📄 test_drum_channel.rs
                📄 test_key_transpose.rs
                📄 test_length.rs
                📄 test_modifier.rs
                📄 test_note_length.rs
                📄 test_octave.rs
                📄 test_pass1.rs
                📄 test_pass2.rs
                📄 test_pass3.rs
                📄 test_pass4.rs
                📄 test_program_change.rs
                📄 test_rest.rs
                📄 test_tempo.rs
                📄 test_velocity.rs
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
                📄 tree-sitter-mml.wasm
            📄 _config.yml
            📄 build.rs
            📁 demo/
              📄 .gitignore
              📖 FEATURES.md
              📖 README.md
              🌐 index.html
              📊 package.json
              📁 src/
                📘 audioPlayback.ts
                📘 audioRenderer.ts
                📘 main.ts
                📘 midiReader.ts
                📘 mmlConverter.ts
                📘 parseMidiNotes.ts
                📘 smfToYm2151.ts
                📘 state.ts
                📘 treeToJSON.ts
                📘 ui.ts
                📘 visualization.ts
                📘 wavExport.ts
              📄 test-loader.mjs
              📄 test-register.mjs
              📁 tests/
                📘 audioBufferToWav.test.ts
                📘 midiReader.test.ts
                📘 parseMidiNotes.test.ts
                📘 treeToJSON.test.ts
            📁 demo-library/
              🌐 index.html
              📊 package.json
            📁 generated-docs/
              📖 development-status-generated-prompt.md
            🌐 googled947dc864c270e07.html
            📁 issue-notes/
              📖 103.md
              📖 123.md
              📖 133.md
              📖 39.md
              📖 44.md
            📄 mmlabc-to-smf-rust.toml.example
            📁 mmlabc-to-smf-wasm/
              📄 Cargo.lock
              📄 Cargo.toml
              📁 src/
                📄 lib.rs
                📄 token_extractor.rs
              📁 tests/
                📄 parity.rs
            📊 package.json
            📁 scripts/
              📖 README.md
              📄 build-demo.sh
              📄 transform-demo-paths.sh
            📁 src/
              📄 attachment_json.rs
              📄 config.rs
              📄 lib.rs
              📄 main.rs
              📄 mml_preprocessor.rs
              📄 parse_tree_tokens.rs
              📄 pass1_parser.rs
              📄 pass2_ast.rs
              📄 pass3_events.rs
              📄 pass4_midi.rs
              📄 tree_sitter_mml.rs
              📄 types.rs
            📁 tests/
              📄 integration_test.rs
              📄 test_attachment_json.rs
              📄 test_c1_vs_c64.rs
              📄 test_channel.rs
              📄 test_chord.rs
              📄 test_cli.rs
              📄 test_config.rs
              📄 test_dotted_notes.rs
              📄 test_drum_channel.rs
              📄 test_key_transpose.rs
              📄 test_length.rs
              📄 test_modifier.rs
              📄 test_note_length.rs
              📄 test_octave.rs
              📄 test_pass1.rs
              📄 test_pass2.rs
              📄 test_pass3.rs
              📄 test_pass4.rs
              📄 test_program_change.rs
              📄 test_rest.rs
              📄 test_tempo.rs
              📄 test_velocity.rs
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
              📄 tree-sitter-mml.wasm
          📄 _config.yml
          📄 build.rs
          📁 demo/
            📄 .gitignore
            📖 FEATURES.md
            📖 README.md
            🌐 index.html
            📊 package.json
            📁 src/
              📘 audioPlayback.ts
              📘 audioRenderer.ts
              📘 main.ts
              📘 midiReader.ts
              📘 mmlConverter.ts
              📘 parseMidiNotes.ts
              📘 smfToYm2151.ts
              📘 state.ts
              📘 treeToJSON.ts
              📘 ui.ts
              📘 visualization.ts
              📘 wavExport.ts
            📄 test-loader.mjs
            📄 test-register.mjs
            📁 tests/
              📘 audioBufferToWav.test.ts
              📘 midiReader.test.ts
              📘 parseMidiNotes.test.ts
              📘 treeToJSON.test.ts
          📁 demo-library/
            🌐 index.html
            📊 package.json
          📁 generated-docs/
            📖 development-status-generated-prompt.md
          🌐 googled947dc864c270e07.html
          📁 issue-notes/
            📖 103.md
            📖 123.md
            📖 133.md
            📖 39.md
            📖 44.md
          📄 mmlabc-to-smf-rust.toml.example
          📁 mmlabc-to-smf-wasm/
            📄 Cargo.lock
            📄 Cargo.toml
            📁 src/
              📄 lib.rs
              📄 token_extractor.rs
            📁 tests/
              📄 parity.rs
          📊 package.json
          📁 scripts/
            📖 README.md
            📄 build-demo.sh
            📄 transform-demo-paths.sh
          📁 src/
            📄 attachment_json.rs
            📄 config.rs
            📄 lib.rs
            📄 main.rs
            📄 mml_preprocessor.rs
            📄 parse_tree_tokens.rs
            📄 pass1_parser.rs
            📄 pass2_ast.rs
            📄 pass3_events.rs
            📄 pass4_midi.rs
            📄 tree_sitter_mml.rs
            📄 types.rs
          📁 tests/
            📄 integration_test.rs
            📄 test_attachment_json.rs
            📄 test_c1_vs_c64.rs
            📄 test_channel.rs
            📄 test_chord.rs
            📄 test_cli.rs
            📄 test_config.rs
            📄 test_dotted_notes.rs
            📄 test_drum_channel.rs
            📄 test_key_transpose.rs
            📄 test_length.rs
            📄 test_modifier.rs
            📄 test_note_length.rs
            📄 test_octave.rs
            📄 test_pass1.rs
            📄 test_pass2.rs
            📄 test_pass3.rs
            📄 test_pass4.rs
            📄 test_program_change.rs
            📄 test_rest.rs
            📄 test_tempo.rs
            📄 test_velocity.rs
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
            📄 tree-sitter-mml.wasm
        📄 _config.yml
        📄 build.rs
        📁 demo/
          📄 .gitignore
          📖 FEATURES.md
          📖 README.md
          🌐 index.html
          📊 package.json
          📁 src/
            📘 audioPlayback.ts
            📘 audioRenderer.ts
            📘 main.ts
            📘 midiReader.ts
            📘 mmlConverter.ts
            📘 parseMidiNotes.ts
            📘 smfToYm2151.ts
            📘 state.ts
            📘 treeToJSON.ts
            📘 ui.ts
            📘 visualization.ts
            📘 wavExport.ts
          📄 test-loader.mjs
          📄 test-register.mjs
          📁 tests/
            📘 audioBufferToWav.test.ts
            📘 midiReader.test.ts
            📘 parseMidiNotes.test.ts
            📘 treeToJSON.test.ts
        📁 demo-library/
          🌐 index.html
          📊 package.json
        📁 generated-docs/
          📖 development-status-generated-prompt.md
        🌐 googled947dc864c270e07.html
        📁 issue-notes/
          📖 103.md
          📖 123.md
          📖 133.md
          📖 39.md
          📖 44.md
        📄 mmlabc-to-smf-rust.toml.example
        📁 mmlabc-to-smf-wasm/
          📄 Cargo.lock
          📄 Cargo.toml
          📁 src/
            📄 lib.rs
            📄 token_extractor.rs
          📁 tests/
            📄 parity.rs
        📊 package.json
        📁 scripts/
          📖 README.md
          📄 build-demo.sh
          📄 transform-demo-paths.sh
        📁 src/
          📄 attachment_json.rs
          📄 config.rs
          📄 lib.rs
          📄 main.rs
          📄 mml_preprocessor.rs
          📄 parse_tree_tokens.rs
          📄 pass1_parser.rs
          📄 pass2_ast.rs
          📄 pass3_events.rs
          📄 pass4_midi.rs
          📄 tree_sitter_mml.rs
          📄 types.rs
        📁 tests/
          📄 integration_test.rs
          📄 test_attachment_json.rs
          📄 test_c1_vs_c64.rs
          📄 test_channel.rs
          📄 test_chord.rs
          📄 test_cli.rs
          📄 test_config.rs
          📄 test_dotted_notes.rs
          📄 test_drum_channel.rs
          📄 test_key_transpose.rs
          📄 test_length.rs
          📄 test_modifier.rs
          📄 test_note_length.rs
          📄 test_octave.rs
          📄 test_pass1.rs
          📄 test_pass2.rs
          📄 test_pass3.rs
          📄 test_pass4.rs
          📄 test_program_change.rs
          📄 test_rest.rs
          📄 test_tempo.rs
          📄 test_velocity.rs
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
          📄 tree-sitter-mml.wasm
      📄 _config.yml
      📄 build.rs
      📁 demo/
        📄 .gitignore
        📖 FEATURES.md
        📖 README.md
        🌐 index.html
        📊 package.json
        📁 src/
          📘 audioPlayback.ts
          📘 audioRenderer.ts
          📘 main.ts
          📘 midiReader.ts
          📘 mmlConverter.ts
          📘 parseMidiNotes.ts
          📘 smfToYm2151.ts
          📘 state.ts
          📘 treeToJSON.ts
          📘 ui.ts
          📘 visualization.ts
          📘 wavExport.ts
        📄 test-loader.mjs
        📄 test-register.mjs
        📁 tests/
          📘 audioBufferToWav.test.ts
          📘 midiReader.test.ts
          📘 parseMidiNotes.test.ts
          📘 treeToJSON.test.ts
      📁 demo-library/
        🌐 index.html
        📊 package.json
      📁 generated-docs/
        📖 development-status-generated-prompt.md
      🌐 googled947dc864c270e07.html
      📁 issue-notes/
        📖 103.md
        📖 123.md
        📖 133.md
        📖 39.md
        📖 44.md
      📄 mmlabc-to-smf-rust.toml.example
      📁 mmlabc-to-smf-wasm/
        📄 Cargo.lock
        📄 Cargo.toml
        📁 src/
          📄 lib.rs
          📄 token_extractor.rs
        📁 tests/
          📄 parity.rs
      📊 package.json
      📁 scripts/
        📖 README.md
        📄 build-demo.sh
        📄 transform-demo-paths.sh
      📁 src/
        📄 attachment_json.rs
        📄 config.rs
        📄 lib.rs
        📄 main.rs
        📄 mml_preprocessor.rs
        📄 parse_tree_tokens.rs
        📄 pass1_parser.rs
        📄 pass2_ast.rs
        📄 pass3_events.rs
        📄 pass4_midi.rs
        📄 tree_sitter_mml.rs
        📄 types.rs
      📁 tests/
        📄 integration_test.rs
        📄 test_attachment_json.rs
        📄 test_c1_vs_c64.rs
        📄 test_channel.rs
        📄 test_chord.rs
        📄 test_cli.rs
        📄 test_config.rs
        📄 test_dotted_notes.rs
        📄 test_drum_channel.rs
        📄 test_key_transpose.rs
        📄 test_length.rs
        📄 test_modifier.rs
        📄 test_note_length.rs
        📄 test_octave.rs
        📄 test_pass1.rs
        📄 test_pass2.rs
        📄 test_pass3.rs
        📄 test_pass4.rs
        📄 test_program_change.rs
        📄 test_rest.rs
        📄 test_tempo.rs
        📄 test_velocity.rs
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
        📄 tree-sitter-mml.wasm
    📄 _config.yml
    📄 build.rs
    📁 demo/
      📄 .gitignore
      📖 FEATURES.md
      📖 README.md
      🌐 index.html
      📊 package.json
      📁 src/
        📘 audioPlayback.ts
        📘 audioRenderer.ts
        📘 main.ts
        📘 midiReader.ts
        📘 mmlConverter.ts
        📘 parseMidiNotes.ts
        📘 smfToYm2151.ts
        📘 state.ts
        📘 treeToJSON.ts
        📘 ui.ts
        📘 visualization.ts
        📘 wavExport.ts
      📄 test-loader.mjs
      📄 test-register.mjs
      📁 tests/
        📘 audioBufferToWav.test.ts
        📘 midiReader.test.ts
        📘 parseMidiNotes.test.ts
        📘 treeToJSON.test.ts
    📁 demo-library/
      🌐 index.html
      📊 package.json
    📁 generated-docs/
      📖 development-status-generated-prompt.md
    🌐 googled947dc864c270e07.html
    📁 issue-notes/
      📖 103.md
      📖 123.md
      📖 133.md
      📖 39.md
      📖 44.md
    📄 mmlabc-to-smf-rust.toml.example
    📁 mmlabc-to-smf-wasm/
      📄 Cargo.lock
      📄 Cargo.toml
      📁 src/
        📄 lib.rs
        📄 token_extractor.rs
      📁 tests/
        📄 parity.rs
    📊 package.json
    📁 scripts/
      📖 README.md
      📄 build-demo.sh
      📄 transform-demo-paths.sh
    📁 src/
      📄 attachment_json.rs
      📄 config.rs
      📄 lib.rs
      📄 main.rs
      📄 mml_preprocessor.rs
      📄 parse_tree_tokens.rs
      📄 pass1_parser.rs
      📄 pass2_ast.rs
      📄 pass3_events.rs
      📄 pass4_midi.rs
      📄 tree_sitter_mml.rs
      📄 types.rs
    📁 tests/
      📄 integration_test.rs
      📄 test_attachment_json.rs
      📄 test_c1_vs_c64.rs
      📄 test_channel.rs
      📄 test_chord.rs
      📄 test_cli.rs
      📄 test_config.rs
      📄 test_dotted_notes.rs
      📄 test_drum_channel.rs
      📄 test_key_transpose.rs
      📄 test_length.rs
      📄 test_modifier.rs
      📄 test_note_length.rs
      📄 test_octave.rs
      📄 test_pass1.rs
      📄 test_pass2.rs
      📄 test_pass3.rs
      📄 test_pass4.rs
      📄 test_program_change.rs
      📄 test_rest.rs
      📄 test_tempo.rs
      📄 test_velocity.rs
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
      📄 tree-sitter-mml.wasm
  📄 _config.yml
  📄 build.rs
  📁 demo/
    📄 .gitignore
    📖 FEATURES.md
    📖 README.md
    🌐 index.html
    📊 package.json
    📁 src/
      📘 audioPlayback.ts
      📘 audioRenderer.ts
      📘 main.ts
      📘 midiReader.ts
      📘 mmlConverter.ts
      📘 parseMidiNotes.ts
      📘 smfToYm2151.ts
      📘 state.ts
      📘 treeToJSON.ts
      📘 ui.ts
      📘 visualization.ts
      📘 wavExport.ts
    📄 test-loader.mjs
    📄 test-register.mjs
    📁 tests/
      📘 audioBufferToWav.test.ts
      📘 midiReader.test.ts
      📘 parseMidiNotes.test.ts
      📘 treeToJSON.test.ts
  📁 demo-library/
    🌐 index.html
    📊 package.json
  📁 generated-docs/
    📖 development-status-generated-prompt.md
  🌐 googled947dc864c270e07.html
  📁 issue-notes/
    📖 103.md
    📖 123.md
    📖 133.md
    📖 39.md
    📖 44.md
  📄 mmlabc-to-smf-rust.toml.example
  📁 mmlabc-to-smf-wasm/
    📄 Cargo.lock
    📄 Cargo.toml
    📁 src/
      📄 lib.rs
      📄 token_extractor.rs
    📁 tests/
      📄 parity.rs
  📊 package.json
  📁 scripts/
    📖 README.md
    📄 build-demo.sh
    📄 transform-demo-paths.sh
  📁 src/
    📄 attachment_json.rs
    📄 config.rs
    📄 lib.rs
    📄 main.rs
    📄 mml_preprocessor.rs
    📄 parse_tree_tokens.rs
    📄 pass1_parser.rs
    📄 pass2_ast.rs
    📄 pass3_events.rs
    📄 pass4_midi.rs
    📄 tree_sitter_mml.rs
    📄 types.rs
  📁 tests/
    📄 integration_test.rs
    📄 test_attachment_json.rs
    📄 test_c1_vs_c64.rs
    📄 test_channel.rs
    📄 test_chord.rs
    📄 test_cli.rs
    📄 test_config.rs
    📄 test_dotted_notes.rs
    📄 test_drum_channel.rs
    📄 test_key_transpose.rs
    📄 test_length.rs
    📄 test_modifier.rs
    📄 test_note_length.rs
    📄 test_octave.rs
    📄 test_pass1.rs
    📄 test_pass2.rs
    📄 test_pass3.rs
    📄 test_pass4.rs
    📄 test_program_change.rs
    📄 test_rest.rs
    📄 test_tempo.rs
    📄 test_velocity.rs
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
    📄 tree-sitter-mml.wasm
📄 _config.yml
📄 build.rs
📁 demo/
  📄 .gitignore
  📖 FEATURES.md
  📖 README.md
  🌐 index.html
  📊 package.json
  📁 src/
    📘 audioPlayback.ts
    📘 audioRenderer.ts
    📘 main.ts
    📘 midiReader.ts
    📘 mmlConverter.ts
    📘 parseMidiNotes.ts
    📘 smfToYm2151.ts
    📘 state.ts
    📘 treeToJSON.ts
    📘 ui.ts
    📘 visualization.ts
    📘 wavExport.ts
  📄 test-loader.mjs
  📄 test-register.mjs
  📁 tests/
    📘 audioBufferToWav.test.ts
    📘 midiReader.test.ts
    📘 parseMidiNotes.test.ts
    📘 treeToJSON.test.ts
📁 demo-library/
  🌐 index.html
  📊 package.json
📁 generated-docs/
  📖 development-status-generated-prompt.md
🌐 googled947dc864c270e07.html
📁 issue-notes/
  📖 103.md
  📖 123.md
  📖 133.md
  📖 39.md
  📖 44.md
📄 mmlabc-to-smf-rust.toml.example
📁 mmlabc-to-smf-wasm/
  📄 Cargo.lock
  📄 Cargo.toml
  📁 src/
    📄 lib.rs
    📄 token_extractor.rs
  📁 tests/
    📄 parity.rs
📊 package.json
📁 scripts/
  📖 README.md
  📄 build-demo.sh
  📄 transform-demo-paths.sh
📁 src/
  📄 attachment_json.rs
  📄 config.rs
  📄 lib.rs
  📄 main.rs
  📄 mml_preprocessor.rs
  📄 parse_tree_tokens.rs
  📄 pass1_parser.rs
  📄 pass2_ast.rs
  📄 pass3_events.rs
  📄 pass4_midi.rs
  📄 tree_sitter_mml.rs
  📄 types.rs
📁 tests/
  📄 integration_test.rs
  📄 test_attachment_json.rs
  📄 test_c1_vs_c64.rs
  📄 test_channel.rs
  📄 test_chord.rs
  📄 test_cli.rs
  📄 test_config.rs
  📄 test_dotted_notes.rs
  📄 test_drum_channel.rs
  📄 test_key_transpose.rs
  📄 test_length.rs
  📄 test_modifier.rs
  📄 test_note_length.rs
  📄 test_octave.rs
  📄 test_pass1.rs
  📄 test_pass2.rs
  📄 test_pass3.rs
  📄 test_pass4.rs
  📄 test_program_change.rs
  📄 test_rest.rs
  📄 test_tempo.rs
  📄 test_velocity.rs
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
  📄 tree-sitter-mml.wasm

## ファイル詳細分析
**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**_codeql_detected_source_root/demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**_codeql_detected_source_root/demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**_codeql_detected_source_root/demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**_codeql_detected_source_root/demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**_codeql_detected_source_root/demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**_codeql_detected_source_root/demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**_codeql_detected_source_root/demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**_codeql_detected_source_root/demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**_codeql_detected_source_root/demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**_codeql_detected_source_root/demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**_codeql_detected_source_root/demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**_codeql_detected_source_root/demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**_codeql_detected_source_root/demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**_codeql_detected_source_root/tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

**demo/index.html** (325行, 10598バイト)
  - 関数: なし
  - インポート: なし

**demo/src/audioPlayback.ts** (50行, 1479バイト)
  - 関数: playAudio, stopAudio, if, catch
  - インポート: ./tone/index.js, ./state.js, ./ui.js

**demo/src/audioRenderer.ts** (109行, 3805バイト)
  - 関数: waitForWebYm2151, calculateDuration, renderWaveformAndAudio, check, if, for, catch
  - インポート: ./visualization.js, ./state.js, ./smfToYm2151.js

**demo/src/main.ts** (75行, 2790バイト)
  - 関数: initialize, if, catch
  - インポート: ./web-tree-sitter.js, ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js

**demo/src/midiReader.ts** (74行, 2049バイト)
  - 関数: constructor, if, while
  - インポート: なし

**demo/src/mmlConverter.ts** (98行, 4304バイト)
  - 関数: convertMML, if, catch
  - インポート: ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js, ./state.js, ./ui.js

**demo/src/parseMidiNotes.ts** (154行, 5418バイト)
  - 関数: parseMidiNotes, deltaTicksToSeconds, if, for, while, catch
  - インポート: ./midiReader.js

**demo/src/smfToYm2151.ts** (37行, 1113バイト)
  - 関数: ensureInitialized, smfToYM2151Json, if
  - インポート: ./smf-to-ym2151log-wasm/smf_to_ym2151log.js

**demo/src/state.ts** (13行, 441バイト)
  - 関数: なし
  - インポート: なし

**demo/src/treeToJSON.ts** (20行, 702バイト)
  - 関数: treeToJSON, if, for
  - インポート: なし

**demo/src/ui.ts** (14行, 466バイト)
  - 関数: showStatus, loadExample
  - インポート: なし

**demo/src/visualization.ts** (46行, 1655バイト)
  - 関数: drawWaveform, for
  - インポート: なし

**demo/src/wavExport.ts** (75行, 2528バイト)
  - 関数: writeString, audioBufferToWav, exportWav, for, if, catch
  - インポート: ./state.js, ./ui.js

**demo/tests/audioBufferToWav.test.ts** (75行, 2868バイト)
  - 関数: mockAudioBuffer, for
  - インポート: node:test, node:assert/strict, ../src/wavExport.ts

**demo/tests/midiReader.test.ts** (64行, 2306バイト)
  - 関数: なし
  - インポート: node:test, node:assert/strict, ../src/midiReader.ts

**demo/tests/parseMidiNotes.test.ts** (102行, 3901バイト)
  - 関数: buildSmf, for, while
  - インポート: node:test, node:assert/strict, ../src/parseMidiNotes.ts

**demo/tests/treeToJSON.test.ts** (73行, 3169バイト)
  - 関数: mockNode
  - インポート: node:test, node:assert/strict, ../src/treeToJSON.ts

**demo-library/index.html** (320行, 10573バイト)
  - 関数: なし
  - インポート: なし

**googled947dc864c270e07.html** (1行, 53バイト)
  - 関数: なし
  - インポート: なし

**tree-sitter-mml/grammar.js** (69行, 1537バイト)
  - 関数: なし
  - インポート: なし

## 関数呼び出し階層
- if (_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts)
  - playAudio (_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts)
    - stopAudio ()
      - showStatus ()
  - waitForWebYm2151 (_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts)
    - calculateDuration ()
      - renderWaveformAndAudio ()
      - check ()
      - drawWaveform ()
  - initialize (_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/main.ts)
    - convertMML ()
      - smfToYM2151Json ()
      - treeToJSON ()
    - loadExample ()
  - constructor (undefined)
  - parseMidiNotes (_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts)
    - deltaTicksToSeconds ()
  - catch (_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioPlayback.ts)
    - writeString (_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/wavExport.ts)
      - audioBufferToWav ()
      - exportWav ()
  - ensureInitialized (_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/smfToYm2151.ts)
- for (_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/audioRenderer.ts)
  - mockAudioBuffer (_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/audioBufferToWav.test.ts)
  - buildSmf (_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/parseMidiNotes.test.ts)
- while (_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/src/parseMidiNotes.ts)
- mockNode (_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/tests/treeToJSON.test.ts)


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
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/FEATURES.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/FEATURES.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/FEATURES.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/FEATURES.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/FEATURES.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo/FEATURES.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html
_codeql_detected_source_root/_codeql_detected_source_root/demo/FEATURES.md
_codeql_detected_source_root/_codeql_detected_source_root/demo-library/index.html
_codeql_detected_source_root/_codeql_detected_source_root/googled947dc864c270e07.html
_codeql_detected_source_root/demo/FEATURES.md
_codeql_detected_source_root/demo-library/index.html
_codeql_detected_source_root/googled947dc864c270e07.html
demo/FEATURES.md
demo-library/index.html
googled947dc864c270e07.html

上記の情報を基に、プロンプトで指定された形式でプロジェクト概要を生成してください。
特に以下の点を重視してください：
- 技術スタックは各カテゴリごとに整理して説明
- ファイル階層ツリーは提供された構造をそのまま使用
- ファイルの説明は各ファイルの実際の内容と機能に基づく
- 関数の説明は実際に検出された関数の役割に基づく
- 関数呼び出し階層は実際の呼び出し関係に基づく


---
Generated at: 2026-04-19 07:10:03 JST
