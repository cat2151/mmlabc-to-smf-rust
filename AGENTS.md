# AGENTS.md

このファイルはAIエージェント（GitHub Copilot等）向けの指示書です。

## 基本方針

### ライブラリの優先利用

**自前実装を避け、必ずnpmライブラリや既存の依存クレートを利用すること。**

- 既存のライブラリで実現できる機能を自前実装してはならない
- 自前実装はバグの温床となり、根が深いトラブルの原因になる
- 特にバイナリパース、音声合成、フォーマット変換など複雑な処理には必ずライブラリを使う

詳細はサブディレクトリの `AGENTS.md` を参照すること。

## プロジェクト概要

MML（Music Macro Language）をSMF（Standard MIDI File）に変換するRustライブラリ。
4パスアーキテクチャ（パース→AST→MIDIイベント→SMFバイナリ）で実装。

## ビルド・テスト

```bash
# Rustコード
cargo build
cargo test
cargo clippy
cargo fmt

# demoビルド
bash scripts/build-demo.sh
```

## コーディング規則

- `cargo clippy` の警告を0にすること
- `cargo fmt` でフォーマットすること
- 公開APIには `///` ドキュメントコメントを付けること
- 500行を超えるソースファイルは単一責任の原則に従って分割すること
  - 1関数をファイル移動 → テスト → 必要ならテスト修正、のサイクルを繰り返す
- ライブラリで実現できる機能を自前実装してはならない

詳細は `.github/copilot-instructions.md` を参照すること。
