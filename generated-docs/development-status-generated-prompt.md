Last updated: 2026-03-02

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
- .github/IMPLEMENTATION_SUMMARY.md
- .github/actions-tmp/.github/workflows/call-callgraph.yml
- .github/actions-tmp/.github/workflows/call-check-large-files.yml
- .github/actions-tmp/.github/workflows/call-daily-project-summary.yml
- .github/actions-tmp/.github/workflows/call-issue-note.yml
- .github/actions-tmp/.github/workflows/call-rust-windows-check.yml
- .github/actions-tmp/.github/workflows/call-translate-readme.yml
- .github/actions-tmp/.github/workflows/callgraph.yml
- .github/actions-tmp/.github/workflows/check-large-files.yml
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
- .github/actions-tmp/.github_automation/check-large-files/README.md
- .github/actions-tmp/.github_automation/check-large-files/check-large-files.toml.default
- .github/actions-tmp/.github_automation/check-large-files/scripts/check_large_files.py
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
- .github/actions-tmp/issue-notes/35.md
- .github/actions-tmp/issue-notes/38.md
- .github/actions-tmp/issue-notes/4.md
- .github/actions-tmp/issue-notes/40.md
- .github/actions-tmp/issue-notes/44.md
- .github/actions-tmp/issue-notes/46.md
- .github/actions-tmp/issue-notes/7.md
- .github/actions-tmp/issue-notes/8.md
- .github/actions-tmp/issue-notes/9.md
- .github/actions-tmp/package-lock.json
- .github/actions-tmp/package.json
- .github/actions-tmp/src/main.js
- .github/copilot-instructions.md
- .github/scripts/README.md
- .github/scripts/create-build-failure-issue.js
- .github/workflows/call-check-large-files.yml
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
- demo/FEATURES.md
- demo/README.md
- demo/index.html
- demo/package.json
- demo/src/audioPlayback.ts
- demo/src/audioRenderer.ts
- demo/src/main.ts
- demo/src/midiReader.ts
- demo/src/mmlConverter.ts
- demo/src/parseMidiNotes.ts
- demo/src/smfToYm2151.ts
- demo/src/state.ts
- demo/src/ui.ts
- demo/src/visualization.ts
- demo/src/wavExport.ts
- demo-library/index.html
- demo-library/package.json
- googled947dc864c270e07.html
- issue-notes/39.md
- issue-notes/44.md
- issue-notes/89.md
- issue-notes/97.md
- mmlabc-to-smf-rust.toml.example
- mmlabc-to-smf-wasm/Cargo.lock
- mmlabc-to-smf-wasm/Cargo.toml
- mmlabc-to-smf-wasm/src/lib.rs
- package.json
- scripts/README.md
- scripts/build-demo.sh
- scripts/transform-demo-paths.sh
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
- tests/test_drum_channel.rs
- tests/test_key_transpose.rs
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
## [Issue #100](../issue-notes/100.md): 大きなファイルの検出: 82個のファイルが500行を超えています
以下のファイルが500行を超えています。リファクタリングを検討してください。

## 検出されたファイル

| ファイル | 行数 | 超過行数 |
|---------|------|----------|
| `_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_s...
ラベル: refactoring, code-quality, automated
--- issue-notes/100.md の内容 ---

```markdown

```

## [Issue #99](../issue-notes/99.md): Replace custom SMF→audio pipeline with web-ym2151 and smf-to-ym2151log-rust libraries
- [x] Explore codebase and understand current demo implementation
- [x] Update `demo/package.json` to add `smf-to-ym2151log-rust` and `web-ym2151` dependencies
- [x] Update `demo/.gitignore` for new build artifacts
- [x] Update `scripts/build-demo.sh` to copy new library files and update esbuild ext...
ラベル: 
--- issue-notes/99.md の内容 ---

```markdown

```

## [Issue #90](../issue-notes/90.md): demoでソースをみるとweb-ym2151ライブラリを使わず自前のsmf to Tone.js演奏をしているように見受けられる。web-ym2151を使うよう修正する

ラベル: 
--- issue-notes/90.md の内容 ---

```markdown

```

## ドキュメントで言及されているファイルの内容
### .github/actions-tmp/.gitignore
```gitignore
{% raw %}
.secrets
copilot-instructions.md
__pycache__/

{% endraw %}
```

### .gitignore
```gitignore
{% raw %}
# Generated by Cargo
# will have compiled files and executables
debug
target

# These are backup files generated by rustfmt
**/*.rs.bk

# MSVC Windows builds of rustc generate these, which store debugging information
*.pdb

# Generated by cargo mutants
# Contains mutation testing data
**/mutants.out*/

# RustRover
#  JetBrains specific template is maintained in a separate JetBrains.gitignore that can
#  be found at https://github.com/github/gitignore/blob/main/Global/JetBrains.gitignore
#  and can be added to the global gitignore or merged into this file.  For a more nuclear
#  option (not recommended) you can uncomment the following to ignore the entire idea folder.
#.idea/


# Added by cargo

/target

# Debug output files
pass*.json
*.mid
!tests/**/*.mid

# TOML configuration file (user-specific)
mmlabc-to-smf-rust.toml

# Node modules and npm artifacts
node_modules/
package-lock.json

# tree-sitter generated files and npm artifacts
tree-sitter-mml/node_modules/
tree-sitter-mml/package-lock.json
tree-sitter-mml/Cargo.toml
tree-sitter-mml/binding.gyp
tree-sitter-mml/bindings/

# Demo node_modules
demo/node_modules/
demo/package-lock.json

# Demo copied web-tree-sitter files
demo/web-tree-sitter.js
demo/web-tree-sitter.wasm

# WASM package output
mmlabc-to-smf-wasm/pkg/
mmlabc-to-smf-wasm/target/

# GitHub Pages deployment artifacts
_site/

{% endraw %}
```

### demo/.gitignore
```gitignore
{% raw %}
# Node.js dependencies
node_modules/

# Build output
pkg/

# Web tree-sitter files (copied from node_modules)
web-tree-sitter.js
web-tree-sitter.wasm

# Tone.js directory (copied from node_modules)
tone/

# Compiled TypeScript bundle
app.js

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

### .github/actions-tmp/package.json
```json
{% raw %}
{
  "name": "actions-tmp",
  "version": "1.0.0",
  "description": "This repository is a **collection of GitHub Actions shared workflows reusable across multiple projects.**",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [],
  "author": "",
  "license": "ISC",
  "dependencies": {
    "@google/generative-ai": "^0.24.1",
    "@octokit/rest": "^22.0.1"
  }
}

{% endraw %}
```

### demo/package.json
```json
{% raw %}
{
  "name": "demo",
  "version": "1.0.0",
  "description": "This demo demonstrates browser-based MML to SMF conversion using Option A architecture: - **JavaScript (web-tree-sitter)**: Parses MML text and generates parse tree - **Rust WASM**: Receives parse tree JSON, extracts tokens, and generates SMF binary",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [],
  "author": "",
  "license": "ISC",
  "dependencies": {
    "web-tree-sitter": "^0.26.5",
    "tone": "^15.0.4"
  },
  "devDependencies": {
    "esbuild": "^0.24.0"
  }
}

{% endraw %}
```

### demo-library/package.json
```json
{% raw %}
{
  "name": "mmlabc-to-smf-demo-library",
  "version": "0.1.0",
  "description": "Minimal demo for using mmlabc-to-smf (WASM) from a GitHub npm install.",
  "scripts": {
    "fetch:cdn": "bash -lc 'set -euo pipefail; BASE=https://cat2151.github.io/mmlabc-to-smf-rust; mkdir -p ../mmlabc-to-smf-wasm/pkg ../tree-sitter-mml ../demo; curl -L \"$BASE/mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js\" -o ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js; curl -L \"$BASE/mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm_bg.wasm\" -o ../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm_bg.wasm; curl -L \"$BASE/tree-sitter-mml/tree-sitter-mml.wasm\" -o ../tree-sitter-mml/tree-sitter-mml.wasm; curl -L \"$BASE/demo/web-tree-sitter.js\" -o ../demo/web-tree-sitter.js; curl -L \"$BASE/demo/web-tree-sitter.wasm\" -o ../demo/web-tree-sitter.wasm'",
    "build": "cd .. && bash ./scripts/build-demo.sh",
    "serve": "npm exec -- http-server .. -p 8080 -o /demo-library/"
  },
  "author": "cat2151",
  "license": "MIT",
  "type": "module",
  "devDependencies": {
    "http-server": "^14.1.1"
  }
}

{% endraw %}
```

### package.json
```json
{% raw %}
{
  "dependencies": {
    "tree-sitter-cli": "^0.26.5"
  }
}

{% endraw %}
```

### tree-sitter-mml/package.json
```json
{% raw %}
{
  "name": "tree-sitter-mml",
  "version": "1.0.0",
  "description": "Music Macro Language grammar for tree-sitter",
  "main": "bindings/node",
  "keywords": [
    "tree-sitter",
    "parser",
    "mml"
  ],
  "author": "cat2151",
  "license": "MIT",
  "dependencies": {
    "nan": "^2.17.0"
  },
  "devDependencies": {
    "tree-sitter-cli": "^0.20.8"
  },
  "tree-sitter": [
    {
      "scope": "source.mml",
      "file-types": [
        "mml"
      ]
    }
  ]
}

{% endraw %}
```

### _codeql_detected_source_root
```_codeql_detected_source_root
{% raw %}
（ファイル読み込み失敗: EISDIR: illegal operation on a directory, read）
{% endraw %}
```

### scripts/build-demo.sh
```sh
{% raw %}
#!/bin/bash
set -e

# Build script for GitHub Pages demo
# This script can be run standalone or called by GitHub Actions

echo "=== Building MML to SMF WASM Demo ==="

# Get the root directory (parent of scripts/)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

echo "Root directory: ${ROOT_DIR}"

# Step 1: Install npm dependencies for demo
echo ""
echo "Step 1/4: Installing demo npm dependencies..."
cd "${ROOT_DIR}/demo"
npm install

# Step 2: Copy web-tree-sitter, bundle Tone.js, and compile TypeScript source
echo ""
echo "Step 2/4: Copying web-tree-sitter, bundling Tone.js, and compiling TypeScript..."
cp node_modules/web-tree-sitter/web-tree-sitter.js web-tree-sitter.js
cp node_modules/web-tree-sitter/web-tree-sitter.wasm web-tree-sitter.wasm
# Bundle Tone.js and its dependencies into a single ESM file for browsers
rm -rf tone
mkdir -p tone
npx esbuild node_modules/tone/build/esm/index.js --bundle --format=esm --platform=browser --outfile=tone/index.js
# Compile TypeScript source modules into a single bundled app.js
npx esbuild src/main.ts --bundle --format=esm --platform=browser \
    --external:./web-tree-sitter.js \
    --external:./tone/index.js \
    '--external:../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js' \
    --outfile=app.js
echo "✓ Copied web-tree-sitter files, bundled Tone.js, and compiled TypeScript"

# Step 3: Build the WASM module
echo ""
echo "Step 3/4: Building Rust WASM module..."
cd "${ROOT_DIR}/mmlabc-to-smf-wasm"

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed"
    echo "Please install it with: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

wasm-pack build --target web --release

# Step 4: Build tree-sitter-mml.wasm
echo ""
echo "Step 4/4: Building tree-sitter-mml.wasm..."
cd "${ROOT_DIR}/tree-sitter-mml"

# Ensure tree-sitter-cli is installed
echo "Installing/updating tree-sitter-cli..."
npm install

# Build the WASM grammar
echo "Running tree-sitter build-wasm..."
npx tree-sitter build-wasm

if [ -f "${ROOT_DIR}/tree-sitter-mml/tree-sitter-mml.wasm" ]; then
    echo "✓ tree-sitter-mml.wasm built successfully"
else
    echo "Error: Failed to build tree-sitter-mml.wasm"
    exit 1
fi

echo ""
echo "=== Build completed successfully! ==="
echo ""
echo "Demo files are ready in:"
echo "  - demo/ (HTML, JS, copied web-tree-sitter and Tone.js files)"
echo "  - mmlabc-to-smf-wasm/pkg/ (Rust WASM module)"
echo "  - tree-sitter-mml/ (tree-sitter-mml.wasm)"
echo ""
echo "To test locally, run from repository root:"
echo "  python3 -m http.server 8000"
echo "Then open: http://localhost:8000/"

{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
5c18906 Merge pull request #98 from cat2151/copilot/translate-copilot-instructions-to-japanese
cdd4c0a translate copilot-instructions.md from English to Japanese
0c04230 Initial plan
c8c7ec4 Merge pull request #96 from cat2151/copilot/fix-harmony-parsing-issue
74c4bb9 Support chord length on any note position (first, middle, or last)
66071b3 Address review feedback: fix comment clarity and rename ambiguous test
e8bea68 Fix chord note length inheritance (pass2) and chord duration time advance (pass3)
92fd064 Add issue note for #97 [auto]
6a68200 Add instructions for using libraries in demo
80a91a0 Initial plan

### 変更されたファイル:
.github/copilot-instructions.md
.github/workflows/call-check-large-files.yml
demo/.gitignore
demo/index.html
demo/src/audioPlayback.ts
demo/src/audioRenderer.ts
demo/src/main.ts
demo/src/midiReader.ts
demo/src/mmlConverter.ts
demo/src/parseMidiNotes.ts
demo/src/smfToYm2151.ts
demo/src/state.ts
demo/src/ui.ts
demo/src/visualization.ts
demo/src/wavExport.ts
issue-notes/91.md
issue-notes/92.md
issue-notes/93.md
issue-notes/97.md
scripts/build-demo.sh
scripts/transform-demo-paths.sh
src/pass2_ast.rs
src/pass3_events.rs
tests/test_chord.rs


---
Generated at: 2026-03-02 07:05:43 JST
