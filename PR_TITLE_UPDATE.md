# PR Title Update Request

## Current Title
```
Add WebAssembly support for browser-based MML to SMF conversion - Investigation of Emscripten approach complete
```

## New Title (Japanese)
```
WASM実装調査完了：現在のクレート構造のままでは実現不可能、構造変更が必要と判明
```

## Alternative New Title (Japanese, with more context)
```
【調査完了・マージ不要】WASM実装は現在のクレート構造のままでは実現不可能、構造変更が必要と判明
```

## Rationale
- 現在のタイトルは「実装追加」を示唆しているが、実際は「実現不可能の証明」
- 「構造変更が必要」という重要な結論を明示
- マージしないことを明確化（オプション）
- 日本語で記載（ユーザー要求）

## Summary for PR Description Update
PRの冒頭も以下のように更新することを推奨：

```markdown
## ❌ 調査結果：現在のクレート構造では実現不可能

### 調査完了の確認

このPRは、ブラウザJavaScriptから利用できるWASM版の実現可能性を調査しました。
**結論：現在のクレート構造のままでは、SSOT（Single Source of Truth）を維持したWASM実装は不可能**

### 調査で判明した技術的制約

**Option B (Emscripten) の調査結果：**
- ✅ Emscripten SDK導入成功
- ✅ parser.cのWASMコンパイル成功
- ❌ wasm-bindgenとの統合：根本的なアーキテクチャ非互換により不可能
- 結論：Emscripten方式では ブラウザ統合不可能

**現在の実装の問題：**
- ❌ SSOT違反：JavaScriptのextractTokens関数がRustのpass1_parser.rsと重複
- ❌ 制約未達成：「JavaScriptで簡易parserを実装はNG」に違反

### 今後の方向性

**Option A（Parse Tree JSON経由）実装には構造変更が必須：**
- メインクレートとWASMクレートでコード共有可能な構造への変更
- tree-sitter依存の分離
- 共有ライブラリクレートの作成

### このPRについて

- **マージしない**：SSOT違反コードを含むため
- **価値**：Option B以外の解決方法が実現不可能であることを実証
- **次のステップ**：構造変更を伴うOption A実装を新しいIssue/PRで開始
```
