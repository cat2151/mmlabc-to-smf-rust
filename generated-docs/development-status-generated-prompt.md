Last updated: 2026-04-20

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
- .gitattributes
- .github/actions-tmp/.gitattributes
- .github/actions-tmp/.github/workflows/call-callgraph.yml
- .github/actions-tmp/.github/workflows/call-check-large-files.yml
- .github/actions-tmp/.github/workflows/call-daily-project-summary.yml
- .github/actions-tmp/.github/workflows/call-issue-note.yml
- .github/actions-tmp/.github/workflows/call-rust-fmt-commit.yml
- .github/actions-tmp/.github/workflows/call-rust-windows-cargo-check.yml
- .github/actions-tmp/.github/workflows/call-rust-windows-check.yml
- .github/actions-tmp/.github/workflows/call-translate-readme.yml
- .github/actions-tmp/.github/workflows/callgraph.yml
- .github/actions-tmp/.github/workflows/check-large-files.yml
- .github/actions-tmp/.github/workflows/check-recent-human-commit.yml
- .github/actions-tmp/.github/workflows/daily-project-summary.yml
- .github/actions-tmp/.github/workflows/issue-note.yml
- .github/actions-tmp/.github/workflows/rust-fmt-commit.yml
- .github/actions-tmp/.github/workflows/rust-windows-cargo-check.yml
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
- .github/actions-tmp/.github_automation/check-large-files/scripts/test_check_large_files.py
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
- .github/actions-tmp/AGENTS.md
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
- .github/actions-tmp/issue-notes/57.md
- .github/actions-tmp/issue-notes/69.md
- .github/actions-tmp/issue-notes/7.md
- .github/actions-tmp/issue-notes/8.md
- .github/actions-tmp/issue-notes/9.md
- .github/actions-tmp/package-lock.json
- .github/actions-tmp/package.json
- .github/actions-tmp/src/main.js
- .github/check-large-files.toml
- .github/copilot-instructions.md
- .github/scripts/README.md
- .github/scripts/create-build-failure-issue.js
- .github/workflows/call-check-large-files.yml
- .github/workflows/call-daily-project-summary.yml
- .github/workflows/call-issue-note.yml
- .github/workflows/call-mmlabc-to-smf-rust-windows-cargo-check.yml
- .github/workflows/call-rust-fmt-commit.yml
- .github/workflows/call-translate-readme.yml
- .github/workflows/deploy-github-pages.yml
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
- demo/src/treeToJSON.ts
- demo/src/ui.ts
- demo/src/visualization.ts
- demo/src/wavExport.ts
- demo/test-loader.mjs
- demo/test-register.mjs
- demo/tests/audioBufferToWav.test.ts
- demo/tests/midiReader.test.ts
- demo/tests/parseMidiNotes.test.ts
- demo/tests/treeToJSON.test.ts
- demo-library/index.html
- demo-library/package.json
- googled947dc864c270e07.html
- issue-notes/103.md
- issue-notes/123.md
- issue-notes/133.md
- issue-notes/39.md
- issue-notes/44.md
- mmlabc-to-smf-rust.toml.example
- mmlabc-to-smf-wasm/Cargo.lock
- mmlabc-to-smf-wasm/Cargo.toml
- mmlabc-to-smf-wasm/src/lib.rs
- mmlabc-to-smf-wasm/src/token_extractor.rs
- mmlabc-to-smf-wasm/tests/parity.rs
- package.json
- scripts/README.md
- scripts/build-demo.sh
- scripts/transform-demo-paths.sh
- src/attachment_json.rs
- src/config.rs
- src/lib.rs
- src/main.rs
- src/mml_preprocessor.rs
- src/parse_tree_tokens.rs
- src/pass1_parser.rs
- src/pass2_ast.rs
- src/pass3_events.rs
- src/pass4_midi.rs
- src/tree_sitter_mml.rs
- src/types.rs
- tests/integration_test.rs
- tests/test_attachment_json.rs
- tests/test_c1_vs_c64.rs
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
## [Issue #138](../issue-notes/138.md): 大きなファイルの検出: 1個のファイルが500行を超えています
以下のファイルが500行を超えています。リファクタリングを検討してください。

## 検出されたファイル

| ファイル | 行数 | 超過行数 |
|---------|------|----------|
| `tests/test_chord.rs` | 507 | +7 |

## テスト実施のお願い

- リファクタリング前後にテストを実行し、それぞれのテスト失敗件数を報告してください
- リファクタリング前後のどちらかでテストがredの場合、まず別issueでtest greenにしてからリファクタリングしてください

## 推奨事項

1. 単一責任の原則に従い、ファイルを分割す...
ラベル: refactoring, code-quality, automated
--- issue-notes/138.md の内容 ---

```markdown

```

## ドキュメントで言及されているファイルの内容
### .github/actions-tmp/issue-notes/38.md
```md
{% raw %}
# issue PR 36 と PR 37 を取り込んだあと、存在しないissueでワークフローがエラー終了してしまった #38
[issues #38](https://github.com/cat2151/github-actions/issues/38)

# URL

- https://github.com/cat2151/wavlpf/actions/runs/21907996164/job/63253441830

# 実現したいこと

- issueが存在しないのは想定したことであるから、エラー終了にはしない。可用性を維持する。
  - それはそれとして、想定しないできごとが発生した場合は、fail fastする
    - 今回は「想定したできごとなので、fail fastしない」

{% endraw %}
```

### .github/actions-tmp/issue-notes/8.md
```md
{% raw %}
# issue 関数コールグラフhtmlビジュアライズ生成の対象ソースファイルを、呼び出し元ymlで指定できるようにする #8
[issues #8](https://github.com/cat2151/github-actions/issues/8)

# これまでの課題
- 以下が決め打ちになっていた
```
  const allowedFiles = [
    'src/main.js',
    'src/mml2json.js',
    'src/play.js'
  ];
```

# 対策
- 呼び出し元ymlで指定できるようにする

# agent
- agentにやらせることができれば楽なので、初手agentを試した
- 失敗
    - ハルシネーションしてscriptを大量破壊した
- 分析
    - 修正対象scriptはagentが生成したもの
    - 低品質な生成結果でありソースが巨大
    - ハルシネーションで破壊されやすいソース
    - AIの生成したソースは、必ずしもAIフレンドリーではない

# 人力リファクタリング
- 低品質コードを、最低限agentが扱えて、ハルシネーションによる大量破壊を防止できる内容、にする
- 手短にやる
    - そもそもビジュアライズは、agentに雑に指示してやらせたもので、
    - 今後別のビジュアライザを選ぶ可能性も高い
    - 今ここで手間をかけすぎてコンコルド効果（サンクコストバイアス）を増やすのは、project群をトータルで俯瞰して見たとき、損
- 対象
    - allowedFiles のあるソース
        - callgraph-utils.cjs
            - たかだか300行未満のソースである
            - この程度でハルシネーションされるのは予想外
            - やむなし、リファクタリングでソース分割を進める

# agentに修正させる
## prompt
```
allowedFilesを引数で受け取るようにしたいです。
ないならエラー。
最終的に呼び出し元すべてに波及して修正したいです。

呼び出し元をたどってエントリポイントも見つけて、
エントリポイントにおいては、
引数で受け取ったjsonファイル名 allowedFiles.js から
jsonファイル allowedFiles.jsonの内容をreadして
変数 allowedFilesに格納、
後続処理に引き渡す、としたいです。

まずplanしてください。
planにおいては、修正対象のソースファイル名と関数名を、呼び出し元を遡ってすべて特定し、listしてください。
```

# 修正が順調にできた
- コマンドライン引数から受け取る作りになっていなかったので、そこだけ指示して修正させた
- yml側は人力で修正した

# 他のリポジトリから呼び出した場合にバグらないよう修正する
- 気付いた
    - 共通ワークフローとして他のリポジトリから使った場合はバグるはず。
        - ymlから、共通ワークフロー側リポジトリのcheckoutが漏れているので。
- 他のyml同様に修正する
- あわせて全体にymlをリファクタリングし、修正しやすくし、今後のyml読み書きの学びにしやすくする

# local WSL + act : test green

# closeとする
- もし生成されたhtmlがNGの場合は、別issueとするつもり

{% endraw %}
```

### tests/test_chord.rs
```rs
{% raw %}
//! Tests for chord functionality (apostrophe operator)

use mmlabc_to_smf::{pass1_parser, pass2_ast, pass3_events, pass4_midi};

#[test]
fn test_parse_simple_chord() {
    let tokens = pass1_parser::parse_mml("'ceg'");
    assert_eq!(tokens.len(), 3);

    // All tokens should have the same chord_id
    assert_eq!(tokens[0].chord_id, Some(0));
    assert_eq!(tokens[1].chord_id, Some(0));
    assert_eq!(tokens[2].chord_id, Some(0));

    // All should be note tokens
    assert_eq!(tokens[0].token_type, "note");
    assert_eq!(tokens[1].token_type, "note");
    assert_eq!(tokens[2].token_type, "note");

    // Verify note values
    assert_eq!(tokens[0].value, "c");
    assert_eq!(tokens[1].value, "e");
    assert_eq!(tokens[2].value, "g");
}

#[test]
fn test_chord_to_ast() {
    let tokens = pass1_parser::parse_mml("'ceg'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);

    // All notes should have the same chord_id
    assert_eq!(ast.notes[0].chord_id, Some(0));
    assert_eq!(ast.notes[1].chord_id, Some(0));
    assert_eq!(ast.notes[2].chord_id, Some(0));

    // All notes should be on the same channel (None = default channel 0)
    assert_eq!(ast.notes[0].channel, None);
    assert_eq!(ast.notes[1].channel, None);
    assert_eq!(ast.notes[2].channel, None);

    // Verify MIDI pitches (C major chord)
    assert_eq!(ast.notes[0].pitch, 60); // C
    assert_eq!(ast.notes[1].pitch, 64); // E
    assert_eq!(ast.notes[2].pitch, 67); // G
}

#[test]
fn test_chord_events_simultaneous() {
    let tokens = pass1_parser::parse_mml("'ceg'");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    // Should have 6 events (3 note_on + 3 note_off)
    assert_eq!(events.len(), 6);

    // All note_on events should be at time 0 (simultaneous)
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 3);
    assert_eq!(note_on_events[0].time, 0);
    assert_eq!(note_on_events[1].time, 0);
    assert_eq!(note_on_events[2].time, 0);

    // All notes should be on the same channel (channel 0)
    assert_eq!(note_on_events[0].channel, 0);
    assert_eq!(note_on_events[1].channel, 0);
    assert_eq!(note_on_events[2].channel, 0);
}

#[test]
fn test_chord_midi_format() {
    let tokens = pass1_parser::parse_mml("'ceg'");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);
    let midi_data = pass4_midi::events_to_midi(&events).unwrap();

    // MIDI file should be created successfully
    assert!(!midi_data.is_empty());
    assert_eq!(&midi_data[0..4], b"MThd");
}

#[test]
fn test_sequential_notes_then_chord() {
    let tokens = pass1_parser::parse_mml("cd'eg'");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    assert_eq!(tokens.len(), 4);
    assert_eq!(ast.notes.len(), 4);

    // First two notes should not be in a chord
    assert_eq!(ast.notes[0].chord_id, None);
    assert_eq!(ast.notes[1].chord_id, None);

    // Last two notes should be in the same chord
    assert_eq!(ast.notes[2].chord_id, Some(0));
    assert_eq!(ast.notes[3].chord_id, Some(0));

    // Get note_on events
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // c at time 0, d at time 240, e and g both at time 480
    assert_eq!(note_on_events[0].time, 0); // c
    assert_eq!(note_on_events[1].time, 240); // d
    assert_eq!(note_on_events[2].time, 480); // e (chord)
    assert_eq!(note_on_events[3].time, 480); // g (chord)
}

#[test]
fn test_chord_then_sequential_notes() {
    let tokens = pass1_parser::parse_mml("'ceg'de");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    assert_eq!(tokens.len(), 5);
    assert_eq!(ast.notes.len(), 5);

    // First three notes should be in a chord
    assert_eq!(ast.notes[0].chord_id, Some(0));
    assert_eq!(ast.notes[1].chord_id, Some(0));
    assert_eq!(ast.notes[2].chord_id, Some(0));

    // Last two notes should not be in a chord
    assert_eq!(ast.notes[3].chord_id, None);
    assert_eq!(ast.notes[4].chord_id, None);

    // Get note_on events
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // c, e, g all at time 0 (chord), d at time 240, e at time 480
    assert_eq!(note_on_events[0].time, 0); // c (chord)
    assert_eq!(note_on_events[1].time, 0); // e (chord)
    assert_eq!(note_on_events[2].time, 0); // g (chord)
    assert_eq!(note_on_events[3].time, 240); // d (sequential)
    assert_eq!(note_on_events[4].time, 480); // e (sequential)
}

#[test]
fn test_multiple_chords() {
    let tokens = pass1_parser::parse_mml("'ce''df'");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    assert_eq!(tokens.len(), 4);
    assert_eq!(ast.notes.len(), 4);

    // First chord: c and e with chord_id 0
    assert_eq!(ast.notes[0].chord_id, Some(0));
    assert_eq!(ast.notes[1].chord_id, Some(0));

    // Second chord: d and f with chord_id 1
    assert_eq!(ast.notes[2].chord_id, Some(1));
    assert_eq!(ast.notes[3].chord_id, Some(1));

    // Get note_on events
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // First chord at time 0
    assert_eq!(note_on_events[0].time, 0); // c
    assert_eq!(note_on_events[1].time, 0); // e

    // Second chord at time 240
    assert_eq!(note_on_events[2].time, 240); // d
    assert_eq!(note_on_events[3].time, 240); // f
}

#[test]
fn test_two_note_chord() {
    let tokens = pass1_parser::parse_mml("'ce'");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    assert_eq!(tokens.len(), 2);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(events.len(), 4); // 2 notes * 2 events each

    // Both notes should be at time 0 on the same channel
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events[0].time, 0);
    assert_eq!(note_on_events[1].time, 0);
    assert_eq!(note_on_events[0].channel, 0);
    assert_eq!(note_on_events[1].channel, 0);
}

#[test]
fn test_sequential_notes_without_chords_unchanged() {
    // Verify that sequential notes (without chords) still work as before
    let tokens = pass1_parser::parse_mml("cde");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    // All tokens should have chord_id None
    assert_eq!(tokens[0].chord_id, None);
    assert_eq!(tokens[1].chord_id, None);
    assert_eq!(tokens[2].chord_id, None);

    // All notes should have chord_id None
    assert_eq!(ast.notes[0].chord_id, None);
    assert_eq!(ast.notes[1].chord_id, None);
    assert_eq!(ast.notes[2].chord_id, None);

    // Notes should be sequential, not simultaneous
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events[0].time, 0);
    assert_eq!(note_on_events[1].time, 240);
    assert_eq!(note_on_events[2].time, 480);
}

#[test]
fn test_full_pipeline_chord() {
    use std::env;
    use std::fs;
    use std::path::Path;

    let test_dir = env::temp_dir().join("test_chord");
    fs::create_dir_all(&test_dir).unwrap();

    // Pass 1
    let pass1_file = test_dir.join("pass1.json");
    let tokens = pass1_parser::process_pass1("'ceg'", pass1_file.to_str().unwrap()).unwrap();
    assert_eq!(tokens.len(), 3);

    // Pass 2
    let pass2_file = test_dir.join("pass2.json");
    let ast = pass2_ast::process_pass2(&tokens, pass2_file.to_str().unwrap()).unwrap();
    assert_eq!(ast.notes.len(), 3);

    // Pass 3
    let pass3_file = test_dir.join("pass3.json");
    let events = pass3_events::process_pass3(&ast, pass3_file.to_str().unwrap(), true).unwrap();
    assert_eq!(events.len(), 6);

    // Pass 4
    let output_file = test_dir.join("output_chord.mid");
    let midi_data = pass4_midi::process_pass4(&events, output_file.to_str().unwrap()).unwrap();
    assert!(Path::new(&output_file).exists());
    assert!(!midi_data.is_empty());

    // Verify all debug JSONs exist
    assert!(Path::new(&pass1_file).exists());
    assert!(Path::new(&pass2_file).exists());
    assert!(Path::new(&pass3_file).exists());

    // Cleanup
    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_chord_case_insensitive() {
    let tokens_lower = pass1_parser::parse_mml("'ceg'");
    let tokens_upper = pass1_parser::parse_mml("'CEG'");
    let tokens_mixed = pass1_parser::parse_mml("'CeG'");

    assert_eq!(tokens_lower.len(), tokens_upper.len());
    assert_eq!(tokens_lower.len(), tokens_mixed.len());

    // All should produce the same note values (lowercase)
    for i in 0..3 {
        assert_eq!(tokens_lower[i].value, tokens_upper[i].value);
        assert_eq!(tokens_lower[i].value, tokens_mixed[i].value);
    }
}

#[test]
fn test_chord_with_octave() {
    // Test that octave commands work with chords
    let tokens = pass1_parser::parse_mml("<'ceg'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);

    // All notes in the chord should be one octave higher
    assert_eq!(ast.notes[0].pitch, 72); // C5 (60 + 12)
    assert_eq!(ast.notes[1].pitch, 76); // E5 (64 + 12)
    assert_eq!(ast.notes[2].pitch, 79); // G5 (67 + 12)
}

#[test]
fn test_chord_with_internal_octave_commands_tokenized() {
    let tokens = pass1_parser::parse_mml("'c<eg'");

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, "note");
    assert_eq!(tokens[1].token_type, "octave_up");
    assert_eq!(tokens[2].token_type, "note");
    assert_eq!(tokens[3].token_type, "note");
    assert!(tokens.iter().all(|token| token.chord_id == Some(0)));
}

#[test]
fn test_chord_with_leading_and_trailing_octave_commands_tokenized() {
    let tokens = pass1_parser::parse_mml("'<c>'");

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type, "octave_up");
    assert_eq!(tokens[1].token_type, "note");
    assert_eq!(tokens[2].token_type, "octave_down");
    assert!(tokens.iter().all(|token| token.chord_id == Some(0)));
}

#[test]
fn test_chord_with_internal_octave_commands_affect_following_chord_notes() {
    let tokens = pass1_parser::parse_mml("'c<e>g'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);
    assert!(ast.notes.iter().all(|note| note.chord_id == Some(0)));
    assert_eq!(ast.notes[0].pitch, 60); // C4 in the default chord octave
    assert_eq!(ast.notes[1].pitch, 76); // E one octave up after <
    assert_eq!(ast.notes[2].pitch, 67); // G back to the original octave after >
}

#[test]
fn test_two_note_chord_with_internal_octave_command_affects_following_note() {
    let tokens = pass1_parser::parse_mml("'c<g'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);
    assert!(ast.notes.iter().all(|note| note.chord_id == Some(0)));
    assert_eq!(ast.notes[0].pitch, 60); // C4 in the default chord octave
    assert_eq!(ast.notes[1].pitch, 79); // G one octave up after <
}

#[test]
fn test_trailing_chord_octave_command_does_not_affect_next_chord() {
    let tokens = pass1_parser::parse_mml("'c<''c'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].chord_id, Some(0));
    assert_eq!(ast.notes[1].chord_id, Some(1));
    assert_eq!(ast.notes[0].pitch, 60); // First C in the default chord octave
    assert_eq!(ast.notes[1].pitch, ast.notes[0].pitch); // Next chord C stays in the same octave
}

#[test]
fn test_octave_only_chord_syntax_is_not_treated_as_a_chord() {
    let tokens = pass1_parser::parse_mml("'<>'");

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "octave_up");
    assert_eq!(tokens[1].token_type, "octave_down");
    assert!(tokens.iter().all(|token| token.chord_id.is_none()));
}

#[test]
fn test_chord_first_note_length_propagates_within_chord() {
    // 'c8eg' – only the first note has an explicit length (8).
    // The second and third notes should inherit that length.
    let tokens = pass1_parser::parse_mml("'c8eg'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].length, Some(8)); // c8 – explicit
    assert_eq!(ast.notes[1].length, Some(8)); // e – inherits from c8
    assert_eq!(ast.notes[2].length, Some(8)); // g – inherits from c8

    let events = pass3_events::ast_to_events(&ast, true);
    // All chord notes start at 0 and end at 240 (eighth-note = 240 ticks)
    assert_eq!(events[0].time, 0); // C on
    assert_eq!(events[1].time, 240); // C off
    assert_eq!(events[2].time, 0); // E on
    assert_eq!(events[3].time, 240); // E off
    assert_eq!(events[4].time, 0); // G on
    assert_eq!(events[5].time, 240); // G off
}

#[test]
fn test_chord_then_different_length_note() {
    // 'c8eg'd4 – chord is eighth notes, following note is a quarter note.
    // After the chord (duration 240) the next note should start at tick 240.
    let tokens = pass1_parser::parse_mml("'c8eg'd4");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 4);
    assert_eq!(note_on_events[0].time, 0); // c (chord)
    assert_eq!(note_on_events[1].time, 0); // e (chord)
    assert_eq!(note_on_events[2].time, 0); // g (chord)
    assert_eq!(note_on_events[3].time, 240); // d – starts after the chord ends
}

#[test]
fn test_chord_last_note_length_propagates_within_chord() {
    // 'ceg2.' – the length is on the LAST note; all notes should inherit it.
    // dotted half note: 1920/2 * 1.5 = 1440 ticks
    let tokens = pass1_parser::parse_mml("'ceg2.'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].length, Some(2)); // c – inherits from g2.
    assert_eq!(ast.notes[1].length, Some(2)); // e – inherits from g2.
    assert_eq!(ast.notes[2].length, Some(2)); // g2. – explicit

    // All dots should be 1
    assert_eq!(ast.notes[0].dots, Some(1));
    assert_eq!(ast.notes[1].dots, Some(1));
    assert_eq!(ast.notes[2].dots, Some(1));

    let events = pass3_events::ast_to_events(&ast, true);
    // All chord notes on at 0, off at 1440 (dotted half note)
    assert_eq!(events[0].time, 0); // C on
    assert_eq!(events[1].time, 1440); // C off
    assert_eq!(events[2].time, 0); // E on
    assert_eq!(events[3].time, 1440); // E off
    assert_eq!(events[4].time, 0); // G on
    assert_eq!(events[5].time, 1440); // G off
}

#[test]
fn test_chord_middle_note_length_propagates_within_chord() {
    // 'ce8g' – the length is on a middle note; all notes should inherit it.
    // eighth note: 1920/8 = 240 ticks
    let tokens = pass1_parser::parse_mml("'ce8g'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].length, Some(8)); // c – inherits from e8
    assert_eq!(ast.notes[1].length, Some(8)); // e8 – explicit
    assert_eq!(ast.notes[2].length, Some(8)); // g – inherits from e8

    let events = pass3_events::ast_to_events(&ast, true);
    // All chord notes on at 0, off at 240 (eighth note)
    assert_eq!(events[0].time, 0); // C on
    assert_eq!(events[1].time, 240); // C off
    assert_eq!(events[2].time, 0); // E on
    assert_eq!(events[3].time, 240); // E off
    assert_eq!(events[4].time, 0); // G on
    assert_eq!(events[5].time, 240); // G off
}

#[test]
fn test_two_chords_different_lengths() {
    // 'c8eg''d4fa' – first chord is eighth notes, second is quarter notes.
    // The second chord should start at tick 240 (end of first chord).
    let tokens = pass1_parser::parse_mml("'c8eg''d4fa'");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 6);
    // First chord at tick 0
    assert_eq!(note_on_events[0].time, 0); // c
    assert_eq!(note_on_events[1].time, 0); // e
    assert_eq!(note_on_events[2].time, 0); // g
                                           // Second chord starts after first chord (240 ticks)
    assert_eq!(note_on_events[3].time, 240); // d
    assert_eq!(note_on_events[4].time, 240); // f
    assert_eq!(note_on_events[5].time, 240); // a
}

#[test]
fn test_chord_and_channel_simultaneous() {
    // 'cg';e – C and G as a chord on channel 0, E on channel 1.
    // All three should sound simultaneously (C major chord spread across channels).
    let tokens = pass1_parser::parse_mml("'cg';e");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 3);

    // All three notes should start at time 0
    assert_eq!(note_on_events[0].time, 0); // c (chord, ch 0)
    assert_eq!(note_on_events[1].time, 0); // g (chord, ch 0)
    assert_eq!(note_on_events[2].time, 0); // e (ch 1)

    // C and G on channel 0, E on channel 1
    assert_eq!(note_on_events[0].channel, 0); // c
    assert_eq!(note_on_events[1].channel, 0); // g
    assert_eq!(note_on_events[2].channel, 1); // e
}

{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
a37c1cb fix: 和音内の末尾オクターブ変更を次の和音へ漏らさない
0e80684 Update project summaries (overview & development status) [auto]
c6aea99 CI で Rust vs WASM 整合性testも通すようにした
8861e01 Merge branch 'main' of github.com:cat2151/mmlabc-to-smf-rust
2705433 CI Rust
c0d1994 Merge pull request #136 from cat2151/copilot/fix-build-errors-github-actions
4cf91ed fix: enable cli dependency only for wasm tests
5173186 fix: avoid enabling cli feature in wasm build
6393b5a Initial plan
4d01ce4 fix: web版の和音内octaveバグを修正、そのため構成変更

### 変更されたファイル:
.gitattributes
.github/workflows/call-mmlabc-to-smf-rust-windows-cargo-check.yml
.github/workflows/call-rust-fmt-commit.yml
generated-docs/development-status-generated-prompt.md
generated-docs/development-status.md
generated-docs/project-overview-generated-prompt.md
generated-docs/project-overview.md
mmlabc-to-smf-wasm/Cargo.lock
mmlabc-to-smf-wasm/Cargo.toml
mmlabc-to-smf-wasm/src/token_extractor.rs
mmlabc-to-smf-wasm/tests/parity.rs
src/lib.rs
src/parse_tree_tokens.rs
src/pass1_parser.rs
src/pass2_ast.rs
tests/test_chord.rs
tests/test_cli.rs
tests/test_config.rs


---
Generated at: 2026-04-20 07:10:49 JST
