Last updated: 2026-02-04

# 開発状況生成プロンプト（開発者向け）

## 生成するもの：
- 現在openされているissuesを3行で要約する
- 次の一手の候補を3つlistする
- 次の一手の候補3つそれぞれについて、極力小さく分解して、その最初の小さな一歩を書く

## 生成しないもの：
- 「今日のissue目標」などuserに提案するもの
  - ハルシネーションの温床なので生成しない
- ハルシネーションしそうなものは生成しない（例、無価値なtaskや新issueを勝手に妄想してそれをuserに提案する等）
- プロジェクト構造情報（来訪者向け情報のため、別ファイルで管理）

## 「Agent実行プロンプト」生成ガイドライン：
「Agent実行プロンプト」作成時は以下の要素を必ず含めてください：

### 必須要素
1. **対象ファイル**: 分析/編集する具体的なファイルパス
2. **実行内容**: 具体的な分析や変更内容（「分析してください」ではなく「XXXファイルのYYY機能を分析し、ZZZの観点でmarkdown形式で出力してください」）
3. **確認事項**: 変更前に確認すべき依存関係や制約
4. **期待する出力**: markdown形式での結果や、具体的なファイル変更

### Agent実行プロンプト例

**良い例（上記「必須要素」4項目を含む具体的なプロンプト形式）**:
```
対象ファイル: `.github/workflows/translate-readme.yml`と`.github/workflows/call-translate-readme.yml`

実行内容: 対象ファイルについて、外部プロジェクトから利用する際に必要な設定項目を洗い出し、以下の観点から分析してください：
1) 必須入力パラメータ（target-branch等）
2) 必須シークレット（GEMINI_API_KEY）
3) ファイル配置の前提条件（README.ja.mdの存在）
4) 外部プロジェクトでの利用時に必要な追加設定

確認事項: 作業前に既存のworkflowファイルとの依存関係、および他のREADME関連ファイルとの整合性を確認してください。

期待する出力: 外部プロジェクトがこの`call-translate-readme.yml`を導入する際の手順書をmarkdown形式で生成してください。具体的には：必須パラメータの設定方法、シークレットの登録手順、前提条件の確認項目を含めてください。
```

**避けるべき例**:
- callgraphについて調べてください
- ワークフローを分析してください
- issue-noteの処理フローを確認してください

## 出力フォーマット：
以下のMarkdown形式で出力してください：

```markdown
# Development Status

## 現在のIssues
[以下の形式で3行でオープン中のissuesを要約。issue番号を必ず書く]
- [1行目の説明]
- [2行目の説明]
- [3行目の説明]

## 次の一手候補
1. [候補1のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```

2. [候補2のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```

3. [候補3のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```
```


# 開発状況情報
- 以下の開発状況情報を参考にしてください。
- Issue番号を記載する際は、必ず [Issue #番号](../issue-notes/番号.md) の形式でMarkdownリンクとして記載してください。

## プロジェクトのファイル一覧
- .editorconfig
- .github/actions-tmp/.github/workflows/call-callgraph.yml
- .github/actions-tmp/.github/workflows/call-daily-project-summary.yml
- .github/actions-tmp/.github/workflows/call-issue-note.yml
- .github/actions-tmp/.github/workflows/call-rust-windows-check.yml
- .github/actions-tmp/.github/workflows/call-translate-readme.yml
- .github/actions-tmp/.github/workflows/callgraph.yml
- .github/actions-tmp/.github/workflows/check-recent-human-commit.yml
- .github/actions-tmp/.github/workflows/daily-project-summary.yml
- .github/actions-tmp/.github/workflows/issue-note.yml
- .github/actions-tmp/.github/workflows/rust-windows-check.yml
- .github/actions-tmp/.github/workflows/translate-readme.yml
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/callgraph.ql
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/codeql-pack.lock.yml
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/qlpack.yml
- .github/actions-tmp/.github_automation/callgraph/config/example.json
- .github/actions-tmp/.github_automation/callgraph/docs/callgraph.md
- .github/actions-tmp/.github_automation/callgraph/presets/callgraph.js
- .github/actions-tmp/.github_automation/callgraph/presets/style.css
- .github/actions-tmp/.github_automation/callgraph/scripts/analyze-codeql.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/callgraph-utils.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/check-codeql-exists.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/check-node-version.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/common-utils.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/copy-commit-results.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/extract-sarif-info.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/find-process-results.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/generate-html-graph.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/generateHTML.cjs
- .github/actions-tmp/.github_automation/check_recent_human_commit/scripts/check-recent-human-commit.cjs
- .github/actions-tmp/.github_automation/project_summary/docs/daily-summary-setup.md
- .github/actions-tmp/.github_automation/project_summary/prompts/development-status-prompt.md
- .github/actions-tmp/.github_automation/project_summary/prompts/project-overview-prompt.md
- .github/actions-tmp/.github_automation/project_summary/scripts/ProjectSummaryCoordinator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/DevelopmentStatusGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/GitUtils.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/IssueTracker.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/generate-project-summary.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/CodeAnalyzer.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectAnalysisOrchestrator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectDataCollector.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectDataFormatter.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectOverviewGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/BaseGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/FileSystemUtils.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/ProjectFileUtils.cjs
- .github/actions-tmp/.github_automation/translate/docs/TRANSLATION_SETUP.md
- .github/actions-tmp/.github_automation/translate/scripts/translate-readme.cjs
- .github/actions-tmp/.gitignore
- .github/actions-tmp/.vscode/settings.json
- .github/actions-tmp/LICENSE
- .github/actions-tmp/README.ja.md
- .github/actions-tmp/README.md
- .github/actions-tmp/_config.yml
- .github/actions-tmp/generated-docs/callgraph.html
- .github/actions-tmp/generated-docs/callgraph.js
- .github/actions-tmp/generated-docs/development-status-generated-prompt.md
- .github/actions-tmp/generated-docs/development-status.md
- .github/actions-tmp/generated-docs/project-overview-generated-prompt.md
- .github/actions-tmp/generated-docs/project-overview.md
- .github/actions-tmp/generated-docs/style.css
- .github/actions-tmp/googled947dc864c270e07.html
- .github/actions-tmp/issue-notes/10.md
- .github/actions-tmp/issue-notes/11.md
- .github/actions-tmp/issue-notes/12.md
- .github/actions-tmp/issue-notes/13.md
- .github/actions-tmp/issue-notes/14.md
- .github/actions-tmp/issue-notes/15.md
- .github/actions-tmp/issue-notes/16.md
- .github/actions-tmp/issue-notes/17.md
- .github/actions-tmp/issue-notes/18.md
- .github/actions-tmp/issue-notes/19.md
- .github/actions-tmp/issue-notes/2.md
- .github/actions-tmp/issue-notes/20.md
- .github/actions-tmp/issue-notes/21.md
- .github/actions-tmp/issue-notes/22.md
- .github/actions-tmp/issue-notes/23.md
- .github/actions-tmp/issue-notes/24.md
- .github/actions-tmp/issue-notes/25.md
- .github/actions-tmp/issue-notes/26.md
- .github/actions-tmp/issue-notes/27.md
- .github/actions-tmp/issue-notes/28.md
- .github/actions-tmp/issue-notes/29.md
- .github/actions-tmp/issue-notes/3.md
- .github/actions-tmp/issue-notes/30.md
- .github/actions-tmp/issue-notes/4.md
- .github/actions-tmp/issue-notes/7.md
- .github/actions-tmp/issue-notes/8.md
- .github/actions-tmp/issue-notes/9.md
- .github/actions-tmp/package-lock.json
- .github/actions-tmp/package.json
- .github/actions-tmp/src/main.js
- .github/copilot-instructions.md
- .github/workflows/call-daily-project-summary.yml
- .github/workflows/call-issue-note.yml
- .github/workflows/call-translate-readme.yml
- .github/workflows/deploy-github-pages.yml
- .gitignore
- .vscode/settings.json
- Cargo.lock
- Cargo.toml
- IMPLEMENTATION_REPORT.md
- LICENSE
- OPTION_A_IMPLEMENTATION.md
- README.ja.md
- README.md
- _codeql_detected_source_root
- _config.yml
- build.rs
- demo/.gitignore
- demo/README.md
- demo/index.html
- demo/package.json
- googled947dc864c270e07.html
- index.html
- issue-notes/14.md
- issue-notes/17.md
- issue-notes/18.md
- issue-notes/19.md
- issue-notes/20.md
- issue-notes/21.md
- issue-notes/22.md
- issue-notes/23.md
- issue-notes/24.md
- issue-notes/30.md
- issue-notes/36.md
- issue-notes/37.md
- issue-notes/39.md
- issue-notes/40.md
- issue-notes/42.md
- issue-notes/44.md
- issue-notes/46.md
- issue-notes/48.md
- issue-notes/50.md
- issue-notes/52.md
- issue-notes/54.md
- issue-notes/55.md
- issue-notes/56.md
- mmlabc-to-smf-rust.toml.example
- mmlabc-to-smf-wasm/Cargo.lock
- mmlabc-to-smf-wasm/Cargo.toml
- mmlabc-to-smf-wasm/src/lib.rs
- package.json
- scripts/README.md
- scripts/build-demo.sh
- src/config.rs
- src/lib.rs
- src/main.rs
- src/pass1_parser.rs
- src/pass2_ast.rs
- src/pass3_events.rs
- src/pass4_midi.rs
- src/tree_sitter_mml.rs
- src/types.rs
- tests/integration_test.rs
- tests/test_channel.rs
- tests/test_chord.rs
- tests/test_cli.rs
- tests/test_config.rs
- tests/test_dotted_notes.rs
- tests/test_length.rs
- tests/test_modifier.rs
- tests/test_note_length.rs
- tests/test_octave.rs
- tests/test_pass1.rs
- tests/test_pass2.rs
- tests/test_pass3.rs
- tests/test_pass4.rs
- tests/test_program_change.rs
- tests/test_rest.rs
- tests/test_tempo.rs
- tests/test_velocity.rs
- tree-sitter-mml/grammar.js
- tree-sitter-mml/package.json
- tree-sitter-mml/src/grammar.json
- tree-sitter-mml/src/node-types.json
- tree-sitter-mml/src/parser.c
- tree-sitter-mml/src/tree_sitter/alloc.h
- tree-sitter-mml/src/tree_sitter/array.h
- tree-sitter-mml/src/tree_sitter/parser.h
- tree-sitter-mml/tree-sitter-mml.wasm

## 現在のオープンIssues
## [Issue #55](../issue-notes/55.md): README.ja.mdを改善し、SSOTであるgrammer.jsはすべてURL linkとし、クリックでそれを読者がすぐ読めるようにする
[issue-notes/55.md](https://github.com/cat2151/mmlabc-to-smf-rust/blob/main/issue-notes/55.md)

...
ラベル: good first issue
--- issue-notes/55.md の内容 ---

```markdown
# issue README.ja.mdを改善し、SSOTであるgrammer.jsはすべてURL linkとし、クリックでそれを読者がすぐ読めるようにする #55
[issues #55](https://github.com/cat2151/mmlabc-to-smf-rust/issues/55)



```

## [Issue #39](../issue-notes/39.md): ktコマンドを実装する。key transposeである。`kt1 c`は、note number 61となる。`kt-1 c` は、note number 59となる。これはmmlabcフォーマット準拠である
[issue-notes/39.md](https://github.com/cat2151/mmlabc-to-smf-rust/blob/main/issue-notes/39.md)

...
ラベル: good first issue
--- issue-notes/39.md の内容 ---

```markdown
# issue ktコマンドを実装する。key transposeである。は、note number 61となる。 は、note number 59となる。これはmmlabcフォーマット準拠である #39
[issues #39](https://github.com/cat2151/mmlabc-to-smf-rust/issues/39)

# 補足
- demoも追加実装すること

```

## [Issue #37](../issue-notes/37.md): `@128`のあるtrack（trackとは「;」で区切られる文字列グループそれぞれを意味する）は、MIDI channel 9（0base）として扱う、つまりGeneral MIDIの慣習に準拠したdrum channelとして扱う。これはmmlabcフォーマットに準拠している
[issue-notes/37.md](https://github.com/cat2151/mmlabc-to-smf-rust/blob/main/issue-notes/37.md)

...
ラベル: good first issue
--- issue-notes/37.md の内容 ---

```markdown
# issue のあるtrack（trackとは「;」で区切られる文字列グループそれぞれを意味する）は、MIDI channel 9（0base）として扱う、つまりGeneral MIDIの慣習に準拠したdrum channelとして扱う。これはmmlabcフォーマットに準拠している #37
[issues #37](https://github.com/cat2151/mmlabc-to-smf-rust/issues/37)



```

## ドキュメントで言及されているファイルの内容
### .github/actions-tmp/README.ja.md
```md
{% raw %}
# GitHub Actions 共通ワークフロー集

このリポジトリは、**複数プロジェクトで使い回せるGitHub Actions共通ワークフロー集**です

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

# 3行で説明
- 🚀 プロジェクトごとのGitHub Actions管理をもっと楽に
- 🔗 共通化されたワークフローで、どのプロジェクトからも呼ぶだけでOK
- ✅ メンテは一括、プロジェクト開発に集中できます

## Quick Links
| 項目 | リンク |
|------|--------|
| 📖 プロジェクト概要 | [generated-docs/project-overview.md](generated-docs/project-overview.md) |
| 📖 コールグラフ | [generated-docs/callgraph.html](https://cat2151.github.io/github-actions/generated-docs/callgraph.html) |
| 📊 開発状況 | [generated-docs/development-status.md](generated-docs/development-status.md) |

# notes
- まだ共通化の作業中です
- まだワークフロー内容を改善中です

※README.md は README.ja.md を元にGeminiの翻訳でGitHub Actionsで自動生成しています

{% endraw %}
```

### README.ja.md
```md
{% raw %}
# mmlabc-to-smf-rust

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
  <a href="https://deepwiki.com/cat2151/mmlabc-to-smf-rust"><img src="https://img.shields.io/badge/📖-DeepWiki-blue.svg" alt="DeepWiki"></a>
</p>

Music Macro Language (MML) から Standard MIDI File (SMF) への変換ライブラリ

## 概要

このライブラリは、Music Macro Language（MML）形式の文字列を、Standard MIDI Fileに変換します。Rustで書かれています。

## 用途

ライブラリとして `cat-play-mml` から利用しています

## 状況

頻繁に破壊的変更をしています

READMEがメンテ不足です。実際はもっと多数のMMLコマンドが実装済みです。あとでREADMEをメンテ予定です

実装されたMMLを知りたい場合、まず `tree-sitter-mml/grammar.js` をお読みください（ただし今後、破壊的変更されます）

### 実装済み機能 ✅
- **基本音符変換**: `cdefgab` → MIDI音符への変換
- **4パスアーキテクチャ**: 完全実装済み
  - パス1: MML文字列のトークン化（tree-sitterパーサー使用）
  - パス2: トークンからAST（抽象構文木）への変換
  - パス3: ASTからMIDIイベントの生成
  - パス4: MIDIイベントからStandard MIDI File作成
- **tree-sitter統合**: MML構文解析のための完全なtree-sitterパーサー統合
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
  - **パス1**: MML文字列をトークンに解析（tree-sitterパーサー使用）
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

### tree-sitter パーサーファイル

tree-sitter パーサーファイル（`tree-sitter-mml/src/` 配下）は、crates.io での信頼性のある配布のため、tree-sitter のベストプラクティスに従い **git で追跡されています**。

**開発ワークフロー：**
- C言語ソースファイル（`parser.c`、`grammar.json`、`node-types.json`、および `tree_sitter/` ディレクトリ）は、`grammar.js` が変更されたときに自動的に再生成されます
- ビルドスクリプトがファイルの更新時刻をチェックし、必要な場合にのみ再生成します
- **必要条件**：文法を更新する場合は、システムに Node.js と npx がインストールされている必要があります
- 通常のビルド（文法変更なし）は、コミット済みのC言語ファイルを使用するため、Node.js なしで動作します

**生成ファイルをコミットする理由**
これは tree-sitter エコシステムのベストプラクティスに従っています：
- crates.io からインストールするユーザーは Node.js や tree-sitter-cli を必要としません
- 文法とパーサーのバージョンが正確に一致することを保証します
- CI/CD とクロスプラットフォームビルドを簡素化します
- すべての tree-sitter 言語クレートの標準的な慣行です

**文法の更新：**
`tree-sitter-mml/grammar.js` を変更する場合：
1. `cargo build` を実行 - ビルドスクリプトが変更を検出し、パーサーファイルを再生成します
2. grammar.js と再生成されたC言語ファイルの両方を一緒にコミットします
3. これにより、文法とパーサーが同期した状態を保ちます

パーサーファイルを手動で再生成する場合：
```bash
cd tree-sitter-mml
npm install  # tree-sitter-cli がまだインストールされていない場合
npx tree-sitter generate
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

{% endraw %}
```

### .github/actions-tmp/issue-notes/7.md
```md
{% raw %}
# issue issue note生成できるかのtest用 #7
[issues #7](https://github.com/cat2151/github-actions/issues/7)

- 生成できた
- closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/9.md
```md
{% raw %}
# issue 関数コールグラフhtmlビジュアライズが0件なので、原因を可視化する #9
[issues #9](https://github.com/cat2151/github-actions/issues/9)

# agentに修正させたり、人力で修正したりした
- agentがハルシネーションし、いろいろ根の深いバグにつながる、エラー隠蔽などを仕込んでいたため、検知が遅れた
- 詳しくはcommit logを参照のこと
- WSL + actの環境を少し変更、act起動時のコマンドライン引数を変更し、generated-docsをmountする（ほかはデフォルト挙動であるcpだけにする）ことで、デバッグ情報をコンテナ外に出力できるようにし、デバッグを効率化した

# test green

# closeとする

{% endraw %}
```

### issue-notes/37.md
```md
{% raw %}
# issue のあるtrack（trackとは「;」で区切られる文字列グループそれぞれを意味する）は、MIDI channel 9（0base）として扱う、つまりGeneral MIDIの慣習に準拠したdrum channelとして扱う。これはmmlabcフォーマットに準拠している #37
[issues #37](https://github.com/cat2151/mmlabc-to-smf-rust/issues/37)



{% endraw %}
```

### issue-notes/39.md
```md
{% raw %}
# issue ktコマンドを実装する。key transposeである。は、note number 61となる。 は、note number 59となる。これはmmlabcフォーマット準拠である #39
[issues #39](https://github.com/cat2151/mmlabc-to-smf-rust/issues/39)

# 補足
- demoも追加実装すること

{% endraw %}
```

### issue-notes/55.md
```md
{% raw %}
# issue README.ja.mdを改善し、SSOTであるgrammer.jsはすべてURL linkとし、クリックでそれを読者がすぐ読めるようにする #55
[issues #55](https://github.com/cat2151/mmlabc-to-smf-rust/issues/55)



{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
a2156f7 Merge pull request #58 from cat2151/copilot/fix-semicolon-parsing-issue
959c596 Fix indentation with cargo fmt
788766d Add documentation for extract_tokens and extract_note_and_modifier functions
5f66adf Add test for channel_groups in WASM module
8b5c3b0 Update WASM lib to support channel_groups and add demo example
6306ae1 Add semicolon support to grammar.js and update CLI parser
0410eb0 Initial plan
7655c0e Merge pull request #57 from cat2151/copilot/add-dark-mode-support
34b31f3 Add OS dark mode support to demo HTML files
f7d5852 Initial plan

### 変更されたファイル:
.gitignore
demo/README.md
demo/index.html
index.html
issue-notes/52.md
issue-notes/54.md
issue-notes/55.md
issue-notes/56.md
mmlabc-to-smf-wasm/src/lib.rs
package.json
scripts/build-demo.sh
src/pass1_parser.rs
tests/test_pass1.rs
tree-sitter-mml/grammar.js
tree-sitter-mml/src/grammar.json
tree-sitter-mml/src/node-types.json
tree-sitter-mml/src/parser.c
tree-sitter-mml/src/tree_sitter/array.h


---
Generated at: 2026-02-04 07:09:57 JST
