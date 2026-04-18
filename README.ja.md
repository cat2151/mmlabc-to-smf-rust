# mmlabc-to-smf-rust

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
- プログラムチェンジ: `@0` 〜 `@128`
- キー移調: `kt1`, `kt-2`
- 添付 JSON 出力: `--attachment-output`
- MML 先頭の埋め込み添付 JSON: `[{...}]@1cde`
- `@128` を含むチャンネルのドラムチャンネル割当（設定で無効化可能）

2026-04-18 時点で `cargo test -- --list` では 289 件のテストが列挙されます。

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
| オクターブ | `<`, `>`, `oN` | `o4c<d>e` |
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
