# mmlabc-to-smf-rust

Music Macro Language (MML) から Standard MIDI File (SMF) への変換ツール（Rust版）

## 概要

このプロジェクトは、[mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf) のRust実装版です。
Music Macro Language形式の文字列を、包括的なデバッグ出力を備えた4パスアーキテクチャを使用してStandard MIDI Fileに変換します。

## ステータス

🚧 **開発中** - 現在、実装計画策定フェーズです。

詳細は [IMPLEMENTATION_PLAN.md](./IMPLEMENTATION_PLAN.md) をご覧ください。

## 特徴（予定）

- **4パスアーキテクチャ**: 
  - パス1: MML文字列をトークンに解析
  - パス2: トークンを抽象構文木（AST）に変換
  - パス3: ASTからMIDIイベントを生成
  - パス4: Standard MIDI Fileを作成
- **デバッグJSON出力**: 各パスは、デバッグ用に中間結果をJSONとして保存
- **テスト駆動開発**: ユニットテストと統合テストを含む包括的なテストスイート
- **モジュール設計**: Rustの型システムと所有権モデルを活用した安全な実装

## 必要要件

- Rust 1.70.0以上
- Cargo

## インストール（実装後）

```bash
cargo install --path .
```

## 使い方（実装後）

### 基本的な使い方

```bash
mmlabc-to-smf "cde"
```

### カスタム出力ファイル

```bash
mmlabc-to-smf "cde" -o my_song.mid
```

## 開発

### ビルド

```bash
cargo build
```

### テスト

```bash
cargo test
```

### Lint

```bash
cargo clippy
cargo fmt --check
```

## ライセンス

MIT License - 詳細は [LICENSE](LICENSE) ファイルを参照してください。

## 参考

- オリジナルのPython実装: [cat2151/mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf)
