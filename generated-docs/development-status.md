Last updated: 2026-03-09

# Development Status

## 現在のIssues
- [Issue #112](../issue-notes/112.md) では、MMLからSMFとは別に添付JSONを生成する仮仕様の実装が進行中で、MIDIイベントからユニークな`ProgramChange`を収集し、smf-to-ym2151log-rust互換フォーマットでの出力を目指しています。
- [Issue #111](../issue-notes/111.md) は、MMLからSMFファイルとは個別に添付JSONを出力できるか、その際の具体的な影響範囲と仮仕様を整理することを目的としています。
- 添付JSONはNRPN等よりも破壊的変更コストが低い自己記述的フォーマットとして検討されており、`@N`コマンドがない場合のプログラム0のデフォルト出力や、複数の`@N`のデデュープ・ソート済み出力が計画されています。

## 次の一手候補
1.  [Issue #111](../issue-notes/111.md): MMLからの添付JSON出力の仮仕様と影響範囲の整理
    -   最初の小さな一歩: 添付JSON出力の具体的なユースケースと、既存のMML-to-SMF変換処理への影響を文書化する。
    -   Agent実行プロンプト:
        ```
        対象ファイル: `src/pass4_midi.rs`, `src/lib.rs`, `mmlabc-to-smf-rust.toml.example`, `demo/src/smfToYm2151.ts`

        実行内容: [Issue #111](../issue-notes/111.md) の内容を基に、MMLからSMFとは別に添付JSONを出力する際の以下の影響範囲を分析し、Markdown形式で整理してください：
        1. 既存のMML解析/SMF生成フロー（`src/pass*`モジュール）への変更点の検討。
        2. 新しい出力ファイル（添付JSON）のスキーマの検討と、既存の出力（SMF）との整合性。
        3. 設定ファイル（`mmlabc-to-smf-rust.toml.example`など）での出力オプションの追加の可能性。
        4. デモ（`demo/src/smfToYm2151.ts`など）やその他のツールが添付JSONを利用する際のインターフェース。

        確認事項: 添付JSONがSMFと独立したファイルであること、既存のSMF出力フローに不要な副作用を与えないことを確認してください。また、smf-to-ym2151log-rustとの互換性についても考慮してください。

        期待する出力: 「添付JSON出力の影響範囲分析」と題したMarkdownファイル。各分析項目について詳細な説明と、可能な場合はコードスニペットの例を含めてください。
        ```

2.  [Issue #112](../issue-notes/112.md): MMLから添付JSONを出力する仮仕様の実装
    -   最初の小さな一歩: MIDIイベントからProgramChangeを収集し、仮の添付JSONフォーマットで出力するモック関数を`src/attachment_json.rs`として新規作成する。
    -   Agent実行プロンプト:
        ```
        対象ファイル: `src/pass3_events.rs`, `src/pass4_midi.rs`, `src/lib.rs`, `src/main.rs`, `src/attachment_json.rs` (新規作成)

        実行内容: [Issue #112](../issue-notes/112.md) で言及されている「MIDIイベントからユニークな`ProgramChange`を収集し、エントリを生成」するロジックを`src/attachment_json.rs`として新規作成し、`src/lib.rs`または`src/main.rs`から呼び出せるように実装してください。`@N`コマンドがない場合はプログラム0をデフォルト出力し、複数の`@N`はデデュープしてソート済みで出力するようにします。出力フォーマットはsmf-to-ym2151log-rust互換を意識してください。

        確認事項: 新規作成する`src/attachment_json.rs`が既存のMIDIイベント処理 (`src/pass3_events.rs`, `src/pass4_midi.rs`) から必要な情報を適切に取得できるか確認してください。また、SMF生成処理に影響を与えないこと、そしてプログラム変更イベントが正しく収集・整形されることを確認してください。

        期待する出力: `src/attachment_json.rs`ファイルとその呼び出し部分を含む`src/lib.rs`または`src/main.rs`の修正差分。実装された関数のテストコードの提案もあれば含めてください。
        ```

3.  [Issue #112](../issue-notes/112.md) の実装に伴うデモの更新と結合テストの準備
    -   最初の小さな一歩: 添付JSON出力機能をMMLパーサーに追加した後、デモ環境 (`demo/`) で新しい添付JSONファイルが正しく生成され、読み込まれることを確認するためのテスト計画を立てる。
    -   Agent実行プロンプト:
        ```
        対象ファイル: `demo/src/mmlConverter.ts`, `demo/src/smfToYm2151.ts`, `demo/index.html`, `scripts/build-demo.sh`, `tests/integration_test.rs`

        実行内容: [Issue #112](../issue-notes/112.md) で実装される添付JSON出力機能が完成した際に、その機能をデモ (`demo/`) で活用するための準備と、機能が正しく動作するかを確認するための結合テスト計画を策定してください。具体的には、
        1. デモ環境で添付JSONファイルを読み込み、表示または利用するための`demo/src/mmlConverter.ts`および`demo/src/smfToYm2151.ts`の変更点の検討。
        2. `scripts/build-demo.sh`に添付JSON出力処理を組み込む必要性の分析。
        3. 添付JSONの出力内容が正しいかを確認するための統合テストシナリオ (`tests/integration_test.rs`の拡張を考慮) の提案。

        確認事項: デモのUIや既存のSMF再生機能に影響を与えないこと。添付JSONの読み込み/利用が非同期的に行われる場合の影響も考慮すること。WASMモジュールとの連携がスムーズであること。

        期待する出力: 添付JSONを利用するデモの更新計画と、統合テストの具体的なシナリオを記述したMarkdownファイル。必要に応じて擬似コードやコマンド例を含めてください。

---
Generated at: 2026-03-09 07:06:14 JST
