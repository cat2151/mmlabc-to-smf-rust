Last updated: 2026-04-19

# Development Status

## 現在のIssues
現在オープン中のIssueはありません。最近のコミットでは、CIでRustとWASMの整合性テストが導入され、Web版の和音内のオクターブバグが修正されるなど、プロジェクトの安定化と機能改善が進められています。

## 次の一手候補
1. CI/CDワークフローの健全性評価と最適化
   - 最初の小さな一歩: 現在のCIワークフロー（特にRustとWASM関連）が意図通りに動作しているか、ログを確認し、ボトルネックや冗長なステップがないかレビューする。
   - Agent実行プロンプト:
     ```
     対象ファイル: .github/workflows/call-mmlabc-to-smf-rust-windows-cargo-check.yml, .github/workflows/call-rust-fmt-commit.yml, mmlabc-to-smf-wasm/tests/parity.rs および関連するワークフロー定義ファイル

     実行内容: これらのCIワークフローの最近の実行履歴を分析し、特に成功/失敗のパターン、実行時間、エラーメッセージの有無を確認してください。また、`mmlabc-to-smf-wasm/tests/parity.rs`がWASMビルド時にどのように実行されているか、そのテストカバレッジが十分か評価してください。

     確認事項: ワークフローのトリガー条件、依存ジョブ、および各ステップの目的を理解し、現在の設定がプロジェクトの品質保証目標と合致しているか確認してください。

     期待する出力: 現状のCIワークフローの健全性評価（例: 安定性、速度、カバレッジ）、改善が必要な具体的な箇所（例: テストケースの追加、ジョブの並列化、キャッシュの最適化）、および上記改善点それぞれについて、優先度と次のアクションの提案をmarkdown形式で出力してください。
     ```

2. WASM版コンバーターの入出力パリティテスト強化 [Issue #不明だが重要]
   - 最初の小さな一歩: WASM版 `mmlabc-to-smf-wasm/src/lib.rs` とRustネイティブ版 `src/lib.rs` の間で、特定のMML入力に対するSMF出力のパリティを検証するテストケースをさらに追加する。
   - Agent実行プロンプト:
     ```
     対象ファイル: mmlabc-to-smf-wasm/src/lib.rs, mmlabc-to-smf-wasm/tests/parity.rs, src/lib.rs, tests/integration_test.rs

     実行内容:
     1. 既存の`mmlabc-to-smf-wasm/tests/parity.rs`を分析し、どのようなMML入力がテストされているか把握してください。
     2. `src/lib.rs`の主要な変換ロジックをカバーしつつ、WASM版で特にバグが報告されやすい和音、オクターブ変更、テンポ変更などの複雑なMMLシーケンスを網羅する新たなテストケースのアイデアを5つ提案してください。
     3. 提案したテストケースが、どのようにWASMとネイティブの出力パリティ検証に貢献するか説明してください。

     確認事項: WASM版とネイティブ版の変換ロジックの違い、特にCLI機能とWebAssemblyのAPIの違いを考慮し、現実的なテストケースを提案できるか確認してください。

     期待する出力: `mmlabc-to-smf-wasm/tests/parity.rs`に追加すべき、MML入力文字列とその期待される検証ポイント（例: 生成されるSMFのノート数、テンポイベント、和音の正確性など）を記述したmarkdown形式のリスト。各テストケースについて、そのMML入力がWASM版とネイティブ版のパリティ検証にどのように貢献するかを簡潔に説明してください。
     ```

3. デモアプリケーションの構造と利用方法に関するドキュメント作成 [Issue #不明だが新規作成の価値あり]
   - 最初の小さな一歩: `demo/` ディレクトリ内の主要ファイル（`main.ts`, `mmlConverter.ts`, `audioPlayback.ts` など）を洗い出し、それらの役割と相互関係を簡潔にまとめる。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo/src/main.ts, demo/src/mmlConverter.ts, demo/src/audioPlayback.ts, demo/src/midiReader.ts, demo/README.md, demo/FEATURES.md

     実行内容: `demo/`ディレクトリ内のTypeScriptファイルを分析し、主要なコンポーネント（MMLからSMFへの変換、MIDI読み込み、オーディオ再生、UI、可視化）がどのように連携しているかを説明してください。`demo/README.md`と`demo/FEATURES.md`の内容を考慮し、現在のドキュメントで不足している情報を特定してください。

     確認事項: デモアプリケーションの主要な機能がカバーされているか、開発者がデモを理解し、拡張するために必要な情報が網羅されているか確認してください。

     期待する出力: `demo/docs/` ディレクトリに新たに作成すべきファイルとして、デモアプリケーションのアーキテクチャ概要、主要コンポーネントの役割、簡単なローカルでのビルド・実行方法、およびMML入力例と出力されるMIDIデータの関係性について記述したmarkdown形式のドラフト。また、既存の`demo/README.md`を更新すべき点があれば、その提案。

---
Generated at: 2026-04-19 07:10:18 JST
