Last updated: 2026-03-15

# Development Status

## 現在のIssues
- [Issue #123](../issue-notes/123.md): デモのMML入力欄を5行に固定し、デバウンス付き自動演奏機能を実装、FFT表示の改善（周波数ラベルとモノトーン化）および波形ノーマライズ表示、examplesのプルダウン化を計画しています。
- [Issue #121](../issue-notes/121.md): 新たに追加されたissue noteで、WASMモジュールとデモ間の連携における軽微な改善点や調整が必要となる可能性があります。
- [Issue #103](../issue-notes/103.md): プロジェクトの長期的な健全性のためのコードベースのリファクタリング、特に内部パーサーの構造改善が検討されています。

## 次の一手候補
1.  [Issue #123](../issue-notes/123.md): デモのMML入力欄を5行に修正
    -   最初の小さな一歩: `demo/src/ui.ts` ファイルを開き、MML入力エリアのHTML要素または関連するスタイル設定を調整して表示行数を5行に固定する。
    -   Agent実行プロンプ:
        ```
        対象ファイル: `demo/src/ui.ts`

        実行内容: `demo/src/ui.ts`ファイルを分析し、MML入力エリア（textareaなど）の行数を5行に設定する変更を行ってください。具体的には、HTML要素の`rows`属性またはCSSの`height`プロパティを調整します。

        確認事項: 変更後にデモページをブラウザで開き、MML入力欄が正しく5行表示になっていることを確認してください。他のUI要素にレイアウトの崩れが発生していないかも合わせて確認します。

        期待する出力: `demo/src/ui.ts`の変更内容を記述した差分（diff）形式のMarkdown、および変更後のMML入力欄のスクリーンショット（想像）を説明するMarkdown。
        ```

2.  [Issue #123](../issue-notes/123.md): デモのMML自動演奏機能にデバウンスを導入
    -   最初の小さな一歩: `demo/src/main.ts`または`demo/src/ui.ts`内で、MML入力値の変更を監視している箇所を特定し、入力イベントに1秒のデバウンス処理を追加する。
    -   Agent実行プロンプ:
        ```
        対象ファイル: `demo/src/main.ts`, `demo/src/ui.ts`

        実行内容: `demo/src/main.ts`と`demo/src/ui.ts`ファイルを分析し、MML入力欄の内容変更時に自動演奏が開始されるロジックに1秒のデバウンス機能を追加してください。`setTimeout`やデバウンスユーティリティ関数（もしあれば）を使用して実装します。

        確認事項: デバウンス処理が正しく動作し、MML入力後1秒間操作がなければ自動演奏が開始され、その間に再入力があった場合はタイマーがリセットされることを確認してください。

        期待する出力: デバウンス処理が追加された`demo/src/main.ts`または`demo/src/ui.ts`の変更内容を記述した差分（diff）形式のMarkdown。
        ```

3.  [Issue #121](../issue-notes/121.md): WASMモジュールとJavaScript間のエラーハンドリングの改善
    -   最初の小さな一歩: `mmlabc-to-smf-wasm/src/lib.rs`内の主要な公開関数において、エラー発生時の詳細なエラー情報をJavaScript側へ適切に渡せるよう、エラー型を定義または既存のエラー型を調整する。
    -   Agent実行プロンプ:
        ```
        対象ファイル: `mmlabc-to-smf-wasm/src/lib.rs`, `mmlabc-to-smf-wasm/src/token_extractor.rs`, `demo/src/main.ts`

        実行内容: `mmlabc-to-smf-wasm`クレートのエラーハンドリングを強化し、WASMからJavaScriptへ詳細なエラーメッセージを返す仕組みを実装してください。具体的には、Rust側でカスタムエラー型を定義し、それを`wasm_bindgen`を使ってJavaScriptにエクスポートできるようにします。その後、`demo/src/main.ts`でそのエラーを捕捉し、ユーザーに分かりやすく表示する簡単な処理を追加します。

        確認事項: WASMモジュール内の意図的なエラー発生パス（例: 不正なMML入力）を作成し、JavaScript側でそのエラーが捕捉され、期待通りの詳細情報が表示されることを確認してください。

        期待する出力: `mmlabc-to-smf-wasm/src/lib.rs`および関連するRustファイル、`demo/src/main.ts`の変更内容を記述した差分（diff）形式のMarkdown。

---
Generated at: 2026-03-15 07:07:20 JST
