Last updated: 2026-01-21

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
- .gitignore
- .vscode/settings.json
- Cargo.lock
- Cargo.toml
- LICENSE
- README.ja.md
- README.md
- _codeql_detected_source_root
- _config.yml
- build.rs
- googled947dc864c270e07.html
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
- mmlabc-to-smf-rust.toml.example
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

## 現在のオープンIssues
## [Issue #44](../issue-notes/44.md): ブラウザで MML to SMF（バイナリバッファ） 変換を可能とするためのWASM版のクレート追加を、WASI Reactor (FFI export) の方法で実装を試す
[issue-notes/44.md](https://github.com/cat2151/mmlabc-to-smf-rust/blob/main/issue-notes/44.md)

...
ラベル: 
--- issue-notes/44.md の内容 ---

```markdown
# issue ブラウザで MML to SMF（バイナリバッファ） 変換を可能とするためのWASM版のクレート追加を、WASI Reactor (FFI export) の方法で実装を試す #44
[issues #44](https://github.com/cat2151/mmlabc-to-smf-rust/issues/44)

- これまでの課題
    - WASM版を実現したい
    - Tree-sitterのparser.cがあるため、直接WASMにできない
    - そのため、WASI Reactor (FFI export) による方法を試す
    - これならparser.cの問題（C言語依存なので、WASMにできない）を解決できる可能性がある
- 用途
    - ブラウザで、web-ym2151 において、MMLでブラウザでYM2151を鳴らすための技術スタックとして使う用
- 入出力仕様
    - input : text
    - output : バイナリバッファ（内容はSMF）
- 小さく始める
    - 体験の検証：
        - ブラウザで、MMLをtextareaに書いて、exportボタンを押したらSMFをダウンロード、
        - という小さいindex htmlのdemoで検証する
- 完了条件
    - ブラウザJavaScriptから利用できるWASM版が実現すること
    - 既存のCLI機能も維持されること

```

## [Issue #39](../issue-notes/39.md): ktコマンドを実装する。key transposeである。`kt1 c`は、note number 61となる。`kt-1 c` は、note number 59となる。これはmmlabcフォーマット準拠である
[issue-notes/39.md](https://github.com/cat2151/mmlabc-to-smf-rust/blob/main/issue-notes/39.md)

...
ラベル: 
--- issue-notes/39.md の内容 ---

```markdown
# issue ktコマンドを実装する。key transposeである。は、note number 61となる。 は、note number 59となる。これはmmlabcフォーマット準拠である #39
[issues #39](https://github.com/cat2151/mmlabc-to-smf-rust/issues/39)



```

## [Issue #37](../issue-notes/37.md): `@128`のあるtrack（trackとは「;」で区切られる文字列グループそれぞれを意味する）は、MIDI channel 9（0base）として扱う、つまりGeneral MIDIの慣習に準拠したdrum channelとして扱う。これはmmlabcフォーマットに準拠している
[issue-notes/37.md](https://github.com/cat2151/mmlabc-to-smf-rust/blob/main/issue-notes/37.md)

...
ラベル: 
--- issue-notes/37.md の内容 ---

```markdown
# issue のあるtrack（trackとは「;」で区切られる文字列グループそれぞれを意味する）は、MIDI channel 9（0base）として扱う、つまりGeneral MIDIの慣習に準拠したdrum channelとして扱う。これはmmlabcフォーマットに準拠している #37
[issues #37](https://github.com/cat2151/mmlabc-to-smf-rust/issues/37)



```

## ドキュメントで言及されているファイルの内容
### .github/actions-tmp/issue-notes/4.md
```md
{% raw %}
# issue GitHub Actions「project概要生成」を共通ワークフロー化する #4
[issues #4](https://github.com/cat2151/github-actions/issues/4)

# prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
このymlファイルを、以下の2つのファイルに分割してください。
1. 共通ワークフロー       cat2151/github-actions/.github/workflows/daily-project-summary.yml
2. 呼び出し元ワークフロー cat2151/github-actions/.github/workflows/call-daily-project-summary.yml
まずplanしてください
```

# 結果、あちこちハルシネーションのあるymlが生成された
- agentの挙動があからさまにハルシネーション
    - インデントが修正できない、「失敗した」という
    - 構文誤りを認識できない
- 人力で修正した

# このagentによるセルフレビューが信頼できないため、別のLLMによるセカンドオピニオンを試す
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
以下の2つのファイルをレビューしてください。最優先で、エラーが発生するかどうかだけレビューてください。エラー以外の改善事項のチェックをするかわりに、エラー発生有無チェックに最大限注力してください。

--- 呼び出し元

name: Call Daily Project Summary

on:
  schedule:
    # 日本時間 07:00 (UTC 22:00 前日)
    - cron: '0 22 * * *'
  workflow_dispatch:

jobs:
  call-daily-project-summary:
    uses: cat2151/github-actions/.github/workflows/daily-project-summary.yml
    secrets:
      GEMINI_API_KEY: ${{ secrets.GEMINI_API_KEY }}

--- 共通ワークフロー
name: Daily Project Summary
on:
  workflow_call:

jobs:
  generate-summary:
    runs-on: ubuntu-latest

    permissions:
      contents: write
      issues: read
      pull-requests: read

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
          fetch-depth: 0  # 履歴を取得するため

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install dependencies
        run: |
          # 一時的なディレクトリで依存関係をインストール
          mkdir -p /tmp/summary-deps
          cd /tmp/summary-deps
          npm init -y
          npm install @google/generative-ai @octokit/rest
          # generated-docsディレクトリを作成
          mkdir -p $GITHUB_WORKSPACE/generated-docs

      - name: Generate project summary
        env:
          GEMINI_API_KEY: ${{ secrets.GEMINI_API_KEY }}
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          GITHUB_REPOSITORY: ${{ github.repository }}
          NODE_PATH: /tmp/summary-deps/node_modules
        run: |
          node .github/scripts/generate-project-summary.cjs

      - name: Check for generated summaries
        id: check_summaries
        run: |
          if [ -f "generated-docs/project-overview.md" ] && [ -f "generated-docs/development-status.md" ]; then
            echo "summaries_generated=true" >> $GITHUB_OUTPUT
          else
            echo "summaries_generated=false" >> $GITHUB_OUTPUT
          fi

      - name: Commit and push summaries
        if: steps.check_summaries.outputs.summaries_generated == 'true'
        run: |
          git config --local user.email "action@github.com"
          git config --local user.name "GitHub Action"
          # package.jsonの変更のみリセット（generated-docsは保持）
          git restore package.json 2>/dev/null || true
          # サマリーファイルのみを追加
          git add generated-docs/project-overview.md
          git add generated-docs/development-status.md
          git commit -m "Update project summaries (overview & development status)"
          git push

      - name: Summary generation result
        run: |
          if [ "${{ steps.check_summaries.outputs.summaries_generated }}" == "true" ]; then
            echo "✅ Project summaries updated successfully"
            echo "📊 Generated: project-overview.md & development-status.md"
          else
            echo "ℹ️ No summaries generated (likely no user commits in the last 24 hours)"
          fi
```

# 上記promptで、2つのLLMにレビューさせ、合格した

# 細部を、先行する2つのymlを参照に手直しした

# ローカルtestをしてからcommitできるとよい。方法を検討する
- ローカルtestのメリット
    - 素早く修正のサイクルをまわせる
    - ムダにgit historyを汚さない
        - これまでの事例：「実装したつもり」「エラー。修正したつもり」「エラー。修正したつもり」...（以降エラー多数）
- 方法
    - ※検討、WSL + act を環境構築済みである。test可能であると判断する
    - 呼び出し元のURLをコメントアウトし、相対パス記述にする
    - ※備考、テスト成功すると結果がcommit pushされる。それでよしとする
- 結果
    - OK
    - secretsを簡略化できるか試した、できなかった、現状のsecrets記述が今わかっている範囲でベストと判断する
    - OK

# test green

# commit用に、yml 呼び出し元 uses をlocal用から本番用に書き換える

# closeとする

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



{% endraw %}
```

### issue-notes/44.md
```md
{% raw %}
# issue ブラウザで MML to SMF（バイナリバッファ） 変換を可能とするためのWASM版のクレート追加を、WASI Reactor (FFI export) の方法で実装を試す #44
[issues #44](https://github.com/cat2151/mmlabc-to-smf-rust/issues/44)

- これまでの課題
    - WASM版を実現したい
    - Tree-sitterのparser.cがあるため、直接WASMにできない
    - そのため、WASI Reactor (FFI export) による方法を試す
    - これならparser.cの問題（C言語依存なので、WASMにできない）を解決できる可能性がある
- 用途
    - ブラウザで、web-ym2151 において、MMLでブラウザでYM2151を鳴らすための技術スタックとして使う用
- 入出力仕様
    - input : text
    - output : バイナリバッファ（内容はSMF）
- 小さく始める
    - 体験の検証：
        - ブラウザで、MMLをtextareaに書いて、exportボタンを押したらSMFをダウンロード、
        - という小さいindex htmlのdemoで検証する
- 完了条件
    - ブラウザJavaScriptから利用できるWASM版が実現すること
    - 既存のCLI機能も維持されること

{% endraw %}
```

### tree-sitter-mml/src/parser.c
```c
{% raw %}
/* Automatically @generated by tree-sitter v0.25.10 */

#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 33
#define LARGE_STATE_COUNT 5
#define SYMBOL_COUNT 29
#define ALIAS_COUNT 0
#define TOKEN_COUNT 15
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 4
#define MAX_RESERVED_WORD_SET_SIZE 0
#define PRODUCTION_ID_COUNT 1
#define SUPERTYPE_COUNT 0

enum ts_symbol_identifiers {
  anon_sym_SQUOTE = 1,
  sym_note = 2,
  anon_sym_PLUS = 3,
  anon_sym_DASH = 4,
  aux_sym_note_length_token1 = 5,
  sym_dots = 6,
  aux_sym_rest_token1 = 7,
  sym_octave_up = 8,
  sym_octave_down = 9,
  anon_sym_o = 10,
  anon_sym_l = 11,
  anon_sym_AT = 12,
  anon_sym_t = 13,
  anon_sym_v = 14,
  sym_source_file = 15,
  sym__item = 16,
  sym_chord = 17,
  sym_note_with_modifier = 18,
  sym_modifier = 19,
  sym_note_length = 20,
  sym_rest = 21,
  sym_octave_set = 22,
  sym_length_set = 23,
  sym_program_change = 24,
  sym_tempo_set = 25,
  sym_velocity_set = 26,
  aux_sym_source_file_repeat1 = 27,
  aux_sym_chord_repeat1 = 28,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_SQUOTE] = "'",
  [sym_note] = "note",
  [anon_sym_PLUS] = "+",
  [anon_sym_DASH] = "-",
  [aux_sym_note_length_token1] = "note_length_token1",
  [sym_dots] = "dots",
  [aux_sym_rest_token1] = "rest_token1",
  [sym_octave_up] = "octave_up",
  [sym_octave_down] = "octave_down",
  [anon_sym_o] = "o",
  [anon_sym_l] = "l",
  [anon_sym_AT] = "@",
  [anon_sym_t] = "t",
  [anon_sym_v] = "v",
  [sym_source_file] = "source_file",
  [sym__item] = "_item",
  [sym_chord] = "chord",
  [sym_note_with_modifier] = "note_with_modifier",
  [sym_modifier] = "modifier",
  [sym_note_length] = "note_length",
  [sym_rest] = "rest",
  [sym_octave_set] = "octave_set",
  [sym_length_set] = "length_set",
  [sym_program_change] = "program_change",
  [sym_tempo_set] = "tempo_set",
  [sym_velocity_set] = "velocity_set",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_chord_repeat1] = "chord_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [sym_note] = sym_note,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_DASH] = anon_sym_DASH,
  [aux_sym_note_length_token1] = aux_sym_note_length_token1,
  [sym_dots] = sym_dots,
  [aux_sym_rest_token1] = aux_sym_rest_token1,
  [sym_octave_up] = sym_octave_up,
  [sym_octave_down] = sym_octave_down,
  [anon_sym_o] = anon_sym_o,
  [anon_sym_l] = anon_sym_l,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_t] = anon_sym_t,
  [anon_sym_v] = anon_sym_v,
  [sym_source_file] = sym_source_file,
  [sym__item] = sym__item,
  [sym_chord] = sym_chord,
  [sym_note_with_modifier] = sym_note_with_modifier,
  [sym_modifier] = sym_modifier,
  [sym_note_length] = sym_note_length,
  [sym_rest] = sym_rest,
  [sym_octave_set] = sym_octave_set,
  [sym_length_set] = sym_length_set,
  [sym_program_change] = sym_program_change,
  [sym_tempo_set] = sym_tempo_set,
  [sym_velocity_set] = sym_velocity_set,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_chord_repeat1] = aux_sym_chord_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [sym_note] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_note_length_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_dots] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_rest_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_octave_up] = {
    .visible = true,
    .named = true,
  },
  [sym_octave_down] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_o] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_l] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_t] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_v] = {
    .visible = true,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__item] = {
    .visible = false,
    .named = true,
  },
  [sym_chord] = {
    .visible = true,
    .named = true,
  },
  [sym_note_with_modifier] = {
    .visible = true,
    .named = true,
  },
  [sym_modifier] = {
    .visible = true,
    .named = true,
  },
  [sym_note_length] = {
    .visible = true,
    .named = true,
  },
  [sym_rest] = {
    .visible = true,
    .named = true,
  },
  [sym_octave_set] = {
    .visible = true,
    .named = true,
  },
  [sym_length_set] = {
    .visible = true,
    .named = true,
  },
  [sym_program_change] = {
    .visible = true,
    .named = true,
  },
  [sym_tempo_set] = {
    .visible = true,
    .named = true,
  },
  [sym_velocity_set] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_chord_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(1);
      ADVANCE_MAP(
        '\'', 2,
        '+', 4,
        '-', 5,
        '.', 7,
        '<', 9,
        '>', 10,
        '@', 13,
        'l', 12,
        'o', 11,
        't', 14,
        'v', 15,
        'R', 8,
        'r', 8,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(6);
      if (('A' <= lookahead && lookahead <= 'G') ||
          ('a' <= lookahead && lookahead <= 'g')) ADVANCE(3);
      END_STATE();
    case 1:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 2:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(sym_note);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(aux_sym_note_length_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(6);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(sym_dots);
      if (lookahead == '.') ADVANCE(7);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(aux_sym_rest_token1);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(sym_octave_up);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(sym_octave_down);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_o);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_l);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_t);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_v);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [STATE(0)] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [sym_note] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [aux_sym_note_length_token1] = ACTIONS(1),
    [sym_dots] = ACTIONS(1),
    [aux_sym_rest_token1] = ACTIONS(1),
    [sym_octave_up] = ACTIONS(1),
    [sym_octave_down] = ACTIONS(1),
    [anon_sym_o] = ACTIONS(1),
    [anon_sym_l] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [anon_sym_t] = ACTIONS(1),
    [anon_sym_v] = ACTIONS(1),
  },
  [STATE(1)] = {
    [sym_source_file] = STATE(30),
    [sym__item] = STATE(2),
    [sym_chord] = STATE(2),
    [sym_note_with_modifier] = STATE(2),
    [sym_rest] = STATE(2),
    [sym_octave_set] = STATE(2),
    [sym_length_set] = STATE(2),
    [sym_program_change] = STATE(2),
    [sym_tempo_set] = STATE(2),
    [sym_velocity_set] = STATE(2),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_SQUOTE] = ACTIONS(5),
    [sym_note] = ACTIONS(7),
    [aux_sym_rest_token1] = ACTIONS(9),
    [sym_octave_up] = ACTIONS(11),
    [sym_octave_down] = ACTIONS(11),
    [anon_sym_o] = ACTIONS(13),
    [anon_sym_l] = ACTIONS(15),
    [anon_sym_AT] = ACTIONS(17),
    [anon_sym_t] = ACTIONS(19),
    [anon_sym_v] = ACTIONS(21),
  },
  [STATE(2)] = {
    [sym__item] = STATE(3),
    [sym_chord] = STATE(3),
    [sym_note_with_modifier] = STATE(3),
    [sym_rest] = STATE(3),
    [sym_octave_set] = STATE(3),
    [sym_length_set] = STATE(3),
    [sym_program_change] = STATE(3),
    [sym_tempo_set] = STATE(3),
    [sym_velocity_set] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(23),
    [anon_sym_SQUOTE] = ACTIONS(5),
    [sym_note] = ACTIONS(7),
    [aux_sym_rest_token1] = ACTIONS(9),
    [sym_octave_up] = ACTIONS(25),
    [sym_octave_down] = ACTIONS(25),
    [anon_sym_o] = ACTIONS(13),
    [anon_sym_l] = ACTIONS(15),
    [anon_sym_AT] = ACTIONS(17),
    [anon_sym_t] = ACTIONS(19),
    [anon_sym_v] = ACTIONS(21),
  },
  [STATE(3)] = {
    [sym__item] = STATE(3),
    [sym_chord] = STATE(3),
    [sym_note_with_modifier] = STATE(3),
    [sym_rest] = STATE(3),
    [sym_octave_set] = STATE(3),
    [sym_length_set] = STATE(3),
    [sym_program_change] = STATE(3),
    [sym_tempo_set] = STATE(3),
    [sym_velocity_set] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(27),
    [anon_sym_SQUOTE] = ACTIONS(29),
    [sym_note] = ACTIONS(32),
    [aux_sym_rest_token1] = ACTIONS(35),
    [sym_octave_up] = ACTIONS(38),
    [sym_octave_down] = ACTIONS(38),
    [anon_sym_o] = ACTIONS(41),
    [anon_sym_l] = ACTIONS(44),
    [anon_sym_AT] = ACTIONS(47),
    [anon_sym_t] = ACTIONS(50),
    [anon_sym_v] = ACTIONS(53),
  },
  [STATE(4)] = {
    [sym_modifier] = STATE(5),
    [sym_note_length] = STATE(8),
    [ts_builtin_sym_end] = ACTIONS(56),
    [anon_sym_SQUOTE] = ACTIONS(56),
    [sym_note] = ACTIONS(56),
    [anon_sym_PLUS] = ACTIONS(58),
    [anon_sym_DASH] = ACTIONS(58),
    [aux_sym_note_length_token1] = ACTIONS(60),
    [sym_dots] = ACTIONS(62),
    [aux_sym_rest_token1] = ACTIONS(56),
    [sym_octave_up] = ACTIONS(56),
    [sym_octave_down] = ACTIONS(56),
    [anon_sym_o] = ACTIONS(56),
    [anon_sym_l] = ACTIONS(56),
    [anon_sym_AT] = ACTIONS(56),
    [anon_sym_t] = ACTIONS(56),
    [anon_sym_v] = ACTIONS(56),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(60), 1,
      aux_sym_note_length_token1,
    ACTIONS(66), 1,
      sym_dots,
    STATE(11), 1,
      sym_note_length,
    ACTIONS(64), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [23] = 4,
    ACTIONS(60), 1,
      aux_sym_note_length_token1,
    ACTIONS(70), 1,
      sym_dots,
    STATE(10), 1,
      sym_note_length,
    ACTIONS(68), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [46] = 1,
    ACTIONS(72), 13,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_note_length_token1,
      sym_dots,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [62] = 2,
    ACTIONS(66), 1,
      sym_dots,
    ACTIONS(64), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [79] = 1,
    ACTIONS(74), 12,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_dots,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [94] = 2,
    ACTIONS(78), 1,
      sym_dots,
    ACTIONS(76), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [111] = 2,
    ACTIONS(82), 1,
      sym_dots,
    ACTIONS(80), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [128] = 2,
    ACTIONS(86), 1,
      sym_dots,
    ACTIONS(84), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [145] = 1,
    ACTIONS(64), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [159] = 1,
    ACTIONS(76), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [173] = 1,
    ACTIONS(88), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [187] = 1,
    ACTIONS(90), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [201] = 1,
    ACTIONS(92), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [215] = 1,
    ACTIONS(94), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [229] = 1,
    ACTIONS(96), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [243] = 1,
    ACTIONS(98), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [257] = 1,
    ACTIONS(80), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [271] = 1,
    ACTIONS(100), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [285] = 1,
    ACTIONS(102), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      aux_sym_rest_token1,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [299] = 3,
    ACTIONS(7), 1,
      sym_note,
    ACTIONS(104), 1,
      anon_sym_SQUOTE,
    STATE(25), 2,
      sym_note_with_modifier,
      aux_sym_chord_repeat1,
  [310] = 3,
    ACTIONS(106), 1,
      anon_sym_SQUOTE,
    ACTIONS(108), 1,
      sym_note,
    STATE(25), 2,
      sym_note_with_modifier,
      aux_sym_chord_repeat1,
  [321] = 2,
    ACTIONS(7), 1,
      sym_note,
    STATE(24), 2,
      sym_note_with_modifier,
      aux_sym_chord_repeat1,
  [329] = 1,
    ACTIONS(111), 1,
      aux_sym_note_length_token1,
  [333] = 1,
    ACTIONS(113), 1,
      aux_sym_note_length_token1,
  [337] = 1,
    ACTIONS(115), 1,
      aux_sym_note_length_token1,
  [341] = 1,
    ACTIONS(117), 1,
      ts_builtin_sym_end,
  [345] = 1,
    ACTIONS(119), 1,
      aux_sym_note_length_token1,
  [349] = 1,
    ACTIONS(121), 1,
      aux_sym_note_length_token1,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(5)] = 0,
  [SMALL_STATE(6)] = 23,
  [SMALL_STATE(7)] = 46,
  [SMALL_STATE(8)] = 62,
  [SMALL_STATE(9)] = 79,
  [SMALL_STATE(10)] = 94,
  [SMALL_STATE(11)] = 111,
  [SMALL_STATE(12)] = 128,
  [SMALL_STATE(13)] = 145,
  [SMALL_STATE(14)] = 159,
  [SMALL_STATE(15)] = 173,
  [SMALL_STATE(16)] = 187,
  [SMALL_STATE(17)] = 201,
  [SMALL_STATE(18)] = 215,
  [SMALL_STATE(19)] = 229,
  [SMALL_STATE(20)] = 243,
  [SMALL_STATE(21)] = 257,
  [SMALL_STATE(22)] = 271,
  [SMALL_STATE(23)] = 285,
  [SMALL_STATE(24)] = 299,
  [SMALL_STATE(25)] = 310,
  [SMALL_STATE(26)] = 321,
  [SMALL_STATE(27)] = 329,
  [SMALL_STATE(28)] = 333,
  [SMALL_STATE(29)] = 337,
  [SMALL_STATE(30)] = 341,
  [SMALL_STATE(31)] = 345,
  [SMALL_STATE(32)] = 349,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [29] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(26),
  [32] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(4),
  [35] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(6),
  [38] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(3),
  [41] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(32),
  [44] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(31),
  [47] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(29),
  [50] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(28),
  [53] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(27),
  [56] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_note_with_modifier, 1, 0, 0),
  [58] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [60] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [62] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [64] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_note_with_modifier, 2, 0, 0),
  [66] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [68] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rest, 1, 0, 0),
  [70] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [72] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_modifier, 1, 0, 0),
  [74] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_note_length, 1, 0, 0),
  [76] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rest, 2, 0, 0),
  [78] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [80] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_note_with_modifier, 3, 0, 0),
  [82] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [84] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_length_set, 2, 0, 0),
  [86] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [88] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_program_change, 2, 0, 0),
  [90] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tempo_set, 2, 0, 0),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_velocity_set, 2, 0, 0),
  [94] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rest, 3, 0, 0),
  [96] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chord, 3, 0, 0),
  [98] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_octave_set, 2, 0, 0),
  [100] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_length_set, 3, 0, 0),
  [102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_note_with_modifier, 4, 0, 0),
  [104] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [106] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_chord_repeat1, 2, 0, 0),
  [108] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_chord_repeat1, 2, 0, 0), SHIFT_REPEAT(4),
  [111] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [113] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [115] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [117] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [119] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [121] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_mml(void) {
  static const TSLanguage language = {
    .abi_version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = (const void*)ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif

{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
d8ca0d4 Expand issue notes for WASM crate implementation #44
6379f66 Add issue note for #44 [auto]

### 変更されたファイル:
.github/copilot-instructions.md
README.ja.md
README.md
issue-notes/42.md
issue-notes/44.md


---
Generated at: 2026-01-21 07:05:55 JST
