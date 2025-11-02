# mmlabc-to-smf-rust 実装計画書

## 概要

このドキュメントは、Python実装の [mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf) を参考に、Rust版をどう実装するかを記載した実装計画書です。

## プロジェクトの目的

Music Macro Language (MML) から Standard MIDI File (SMF) への変換ツールをRustで実装する。参考実装と同様に、4パスアーキテクチャとデバッグ出力機能を持つ。

## 入出力

### 入力
- **MML文字列**: コマンドライン引数として受け取る
  - 例: `"cde"` (音符C、D、Eを表す)
  - 基本的な音符名をサポート: `c`, `d`, `e`, `f`, `g`, `a`, `b`

### 出力
1. **SMF (Standard MIDI File)**: `.mid` ファイル
   - デフォルト出力: `output.mid`
   - `-o` オプションでファイル名を指定可能
   
2. **デバッグJSON出力** (各パスごと):
   - `pass1_tokens.json`: パース結果のトークンリスト
   - `pass2_ast.json`: 抽象構文木 (AST)
   - `pass3_events.json`: MIDIイベントリスト

### 使用例
```bash
# 基本的な使い方
cargo run -- "cde"

# 出力ファイル名を指定
cargo run -- "cde" -o my_song.mid
```

### 期待される動作
```bash
$ cargo run -- "cde"
Converting MML: cde
Pass 1: Parsing MML...
  Generated 3 tokens → pass1_tokens.json
Pass 2: Creating AST...
  Generated AST with 3 notes → pass2_ast.json
Pass 3: Creating MIDI events...
  Generated 6 events → pass3_events.json
Pass 4: Creating MIDI file...
  Generated MIDI file → output.mid

Conversion complete!
Output files:
  - pass1_tokens.json (debug)
  - pass2_ast.json (debug)
  - pass3_events.json (debug)
  - output.mid (final output)
```

## アーキテクチャ設計

### 4パスアーキテクチャ

Python版と同様の4パス構造を採用：

```
MML文字列 → Pass 1 → トークン → Pass 2 → AST → Pass 3 → MIDIイベント → Pass 4 → SMFファイル
              ↓         ↓         ↓         ↓          ↓         ↓          ↓
         pass1_parser  JSON   pass2_ast   JSON   pass3_events  JSON   pass4_midi
```

#### Pass 1: パーサー (Lexer)
- **役割**: MML文字列を解析してトークンリストを生成
- **入力**: MML文字列 (例: `"cde"`)
- **出力**: トークンリスト
  ```json
  {
    "pass": 1,
    "description": "Parsed tokens",
    "tokens": [
      {"type": "note", "value": "c"},
      {"type": "note", "value": "d"},
      {"type": "note", "value": "e"}
    ]
  }
  ```
- **処理内容**:
  - 文字列を1文字ずつ走査
  - 音符文字 (c-g, a, b) を識別してトークン化
  - 小文字に正規化

#### Pass 2: AST生成
- **役割**: トークンから抽象構文木 (AST) を構築
- **入力**: トークンリスト
- **出力**: AST構造
  ```json
  {
    "pass": 2,
    "description": "Abstract Syntax Tree",
    "ast": {
      "type": "sequence",
      "notes": [
        {"type": "note", "pitch": 60, "name": "c"},
        {"type": "note", "pitch": 62, "name": "d"},
        {"type": "note", "pitch": 64, "name": "e"}
      ]
    }
  }
  ```
- **処理内容**:
  - 音符名をMIDIノート番号に変換
    - c=60 (Middle C), d=62, e=64, f=65, g=67, a=69, b=71
  - 音楽構造を表現するツリー構造を構築

#### Pass 3: MIDIイベント生成
- **役割**: ASTからMIDIイベントシーケンスを生成
- **入力**: AST
- **出力**: MIDIイベントリスト
  ```json
  {
    "pass": 3,
    "description": "MIDI events",
    "events": [
      {"type": "note_on", "time": 0, "note": 60, "velocity": 64},
      {"type": "note_off", "time": 480, "note": 60, "velocity": 0},
      {"type": "note_on", "time": 480, "note": 62, "velocity": 64},
      {"type": "note_off", "time": 960, "note": 62, "velocity": 0},
      {"type": "note_on", "time": 960, "note": 64, "velocity": 64},
      {"type": "note_off", "time": 1440, "note": 64, "velocity": 0}
    ]
  }
  ```
- **処理内容**:
  - 各音符に対してnote_onとnote_offイベントを生成
  - タイミング情報を追加 (デフォルト: 480 ticks per quarter note)
  - ベロシティ (音量) を設定 (デフォルト: 64)

#### Pass 4: SMFファイル作成
- **役割**: MIDIイベントからStandard MIDI Fileを生成
- **入力**: MIDIイベントリスト
- **出力**: `.mid` ファイル
- **処理内容**:
  - MIDIファイルヘッダーとトラックを作成
  - テンポ設定 (デフォルト: 120 BPM = 500000 μs/beat)
  - イベントをデルタタイムでエンコード
  - end_of_trackメタメッセージを追加

### モジュール構成

```
mmlabc-to-smf-rust/
├── Cargo.toml              # プロジェクト設定・依存関係
├── README.md               # プロジェクト概要（日本語版）
├── README.en.md            # プロジェクト概要（英語版）
├── IMPLEMENTATION_PLAN.md  # このファイル
├── src/
│   ├── main.rs            # メインエントリーポイント・CLI処理
│   ├── lib.rs             # ライブラリルート
│   ├── pass1_parser.rs    # Pass 1: MMLパーサー
│   ├── pass2_ast.rs       # Pass 2: AST生成
│   ├── pass3_events.rs    # Pass 3: MIDIイベント生成
│   ├── pass4_midi.rs      # Pass 4: SMFファイル作成
│   └── types.rs           # 共通型定義 (Token, AST, Event等)
└── tests/
    ├── test_pass1.rs      # Pass 1のユニットテスト
    ├── test_pass2.rs      # Pass 2のユニットテスト
    ├── test_pass3.rs      # Pass 3のユニットテスト
    ├── test_pass4.rs      # Pass 4のユニットテスト
    └── integration_test.rs # 統合テスト
```

## 利用ライブラリ

### 必須依存関係

1. **midly** (または **midi-msg** / **rimd**)
   - **用途**: MIDI/SMFファイルの読み書き
   - **理由**: Rust製の高品質MIDIライブラリ。型安全でゼロコスト抽象化。
   - **代替**: `midi-msg`, `rimd` も検討可能
   - **ecosystem**: cargo (Rust)

2. **serde** と **serde_json**
   - **用途**: デバッグJSON出力のシリアライズ
   - **理由**: Rustの標準的なシリアライズ/デシリアライズライブラリ
   - **ecosystem**: cargo (Rust)

3. **clap** (v4)
   - **用途**: コマンドライン引数のパース
   - **理由**: 使いやすく、型安全なCLIライブラリ。派生マクロで簡潔に記述可能
   - **ecosystem**: cargo (Rust)

### 開発・テスト用依存関係

4. **anyhow** / **thiserror**
   - **用途**: エラーハンドリング
   - **理由**: `anyhow`は簡潔なエラー処理、`thiserror`はカスタムエラー型定義に便利
   - **ecosystem**: cargo (Rust)

### Cargo.toml 例

```toml
[package]
name = "mmlabc-to-smf-rust"
version = "0.1.0"
edition = "2021"
authors = ["cat2151"]
license = "MIT"
description = "Music Macro Language to Standard MIDI File converter"

[[bin]]
name = "mmlabc-to-smf"
path = "src/main.rs"

[dependencies]
midly = "0.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4.5", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
# テスト用の追加依存関係があれば記載
```

## テスト方針

### 基本方針
- **テスト駆動開発 (TDD)**: Python版と同様、各パスに対応するユニットテストと統合テストを作成
- **カバレッジ目標**: 主要な処理パスのカバレッジ80%以上を目指す
- **自動テスト**: CIでビルド・テスト・lintを自動実行

### テストの種類

#### 1. ユニットテスト (各パスごと)

**test_pass1.rs**: パーサーのテスト
```rust
#[test]
fn test_parse_single_note() {
    let tokens = parse_mml("c");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].note_type, NoteType::Note);
    assert_eq!(tokens[0].value, "c");
}

#[test]
fn test_parse_multiple_notes() {
    let tokens = parse_mml("cde");
    assert_eq!(tokens.len(), 3);
}

#[test]
fn test_case_insensitive() {
    let tokens1 = parse_mml("CDE");
    let tokens2 = parse_mml("cde");
    assert_eq!(tokens1, tokens2);
}
```

**test_pass2.rs**: AST生成のテスト
```rust
#[test]
fn test_tokens_to_ast() {
    let tokens = vec![/* ... */];
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].pitch, 60); // C
}

#[test]
fn test_note_to_midi_mapping() {
    // c=60, d=62, e=64, ...
}
```

**test_pass3.rs**: MIDIイベント生成のテスト
```rust
#[test]
fn test_ast_to_events() {
    let ast = /* ... */;
    let events = ast_to_events(&ast);
    assert_eq!(events.len(), 6); // 3 notes * 2 events
}

#[test]
fn test_event_timing() {
    // タイミングが正しいか検証
}
```

**test_pass4.rs**: SMFファイル作成のテスト
```rust
#[test]
fn test_events_to_midi_file() {
    let events = /* ... */;
    let midi = events_to_midi(&events);
    // MIDIファイルの構造を検証
}
```

#### 2. 統合テスト

**integration_test.rs**: エンドツーエンドテスト
```rust
#[test]
fn test_full_pipeline_cde() {
    // MML "cde" → SMF ファイル生成までの全パイプラインをテスト
    let result = process_mml("cde", "test_output.mid");
    assert!(result.is_ok());
    
    // 出力ファイルが存在するか
    assert!(Path::new("test_output.mid").exists());
    
    // デバッグJSONが生成されているか
    assert!(Path::new("pass1_tokens.json").exists());
    assert!(Path::new("pass2_ast.json").exists());
    assert!(Path::new("pass3_events.json").exists());
    
    // MIDIファイルの内容を検証
    let midi = read_midi_file("test_output.mid");
    // ノート数、ピッチ等を検証
}
```

### テスト実行コマンド

```bash
# すべてのテストを実行
cargo test

# 詳細出力
cargo test -- --nocapture

# 特定のテストのみ実行
cargo test test_pass1

# 統合テストのみ実行
cargo test --test integration_test
```

## 段階的実装計画

### Phase 1: プロジェクトセットアップ (基本骨格)
**目標**: プロジェクト構造とビルド環境を整える

- [ ] Cargoプロジェクト初期化 (`cargo init`)
- [ ] `Cargo.toml` に依存関係を追加
- [ ] 基本的なモジュール構成を作成 (空ファイルでOK)
  - `src/main.rs`
  - `src/lib.rs`
  - `src/types.rs`
  - `src/pass1_parser.rs`
  - `src/pass2_ast.rs`
  - `src/pass3_events.rs`
  - `src/pass4_midi.rs`
- [ ] README.md (日本語版) を作成
- [ ] ビルド確認 (`cargo build`)

### Phase 2: Pass 1 実装 (MMLパーサー)
**目標**: MML文字列をトークン化する機能を実装

- [ ] `types.rs` にToken型を定義
- [ ] `pass1_parser.rs` に `parse_mml()` 関数を実装
  - 基本音符 (c-g, a, b) の認識
  - 小文字への正規化
- [ ] JSON出力機能を実装 (`save_tokens_to_json()`)
- [ ] `test_pass1.rs` でユニットテストを作成
- [ ] テスト実行・確認 (`cargo test test_pass1`)

### Phase 3: Pass 2 実装 (AST生成)
**目標**: トークンから抽象構文木を構築

- [ ] `types.rs` にAST型 (Note, Sequence等) を定義
- [ ] `pass2_ast.rs` に `tokens_to_ast()` 関数を実装
  - 音符名 → MIDIノート番号の変換
  - AST構造の構築
- [ ] JSON出力機能を実装 (`save_ast_to_json()`)
- [ ] `test_pass2.rs` でユニットテストを作成
- [ ] テスト実行・確認

### Phase 4: Pass 3 実装 (MIDIイベント生成)
**目標**: ASTからMIDIイベントを生成

- [ ] `types.rs` にMidiEvent型を定義
- [ ] `pass3_events.rs` に `ast_to_events()` 関数を実装
  - note_on/note_offイベント生成
  - タイミング計算 (デフォルト: 480 ticks/beat)
- [ ] JSON出力機能を実装
- [ ] `test_pass3.rs` でユニットテストを作成
- [ ] テスト実行・確認

### Phase 5: Pass 4 実装 (SMFファイル作成)
**目標**: MIDIイベントをSMFファイルに書き出す

- [ ] `pass4_midi.rs` に `events_to_midi()` 関数を実装
  - midlyを使ったMIDIファイル生成
  - テンポ設定 (120 BPM)
  - デルタタイム計算
- [ ] `save_midi_file()` 関数を実装
- [ ] `test_pass4.rs` でユニットテストを作成
- [ ] テスト実行・確認

### Phase 6: メインCLI実装
**目標**: コマンドラインインターフェースを完成させる

- [ ] `main.rs` にCLI引数パーサーを実装 (clapを使用)
  - MML文字列の受け取り
  - `-o` オプションによる出力ファイル指定
- [ ] 各パスの順次実行
- [ ] 進捗表示メッセージの実装
- [ ] エラーハンドリング
- [ ] 動作確認 (`cargo run -- "cde"`)

### Phase 7: 統合テスト
**目標**: エンドツーエンドの動作を検証

- [ ] `tests/integration_test.rs` を作成
- [ ] 全パイプラインの統合テストを実装
  - MML → SMF変換
  - デバッグJSON出力の検証
  - MIDIファイル内容の検証
- [ ] テスト実行・確認

### Phase 8: ドキュメント整備とリリース準備
**目標**: プロジェクトを公開できる状態にする

- [ ] README.md を充実させる
  - インストール方法
  - 使用例
  - サポートされる機能
- [ ] README.en.md (英語版) を作成
- [ ] LICENSEファイルの確認
- [ ] CI設定 (GitHub Actions)
  - ビルド・テスト自動化
  - Lintチェック (clippy, rustfmt)
- [ ] リリースビルドのテスト (`cargo build --release`)

### Phase 9 (オプション): 機能拡張
段階的実装が完了後、以下の拡張を検討：

- [ ] オクターブ指定 (`o4`, `o5` 等)
- [ ] 音長指定 (`c4`, `d8` 等)
- [ ] 休符 (`r` 等)
- [ ] テンポ変更 (`t120` 等)
- [ ] 複数トラック対応
- [ ] より複雑なMML構文のサポート

## 実装時の留意点

### コーディングスタイル
- **Rustの慣習に従う**: `rustfmt` と `clippy` を使用
- **型安全性**: `Result<T, E>` と `Option<T>` を適切に使用
- **エラーハンドリング**: `anyhow` を使って明確なエラーメッセージを提供
- **ドキュメント**: 公開関数には `///` コメントを記載

### Python版との相違点
- **所有権とライフタイム**: Rustの所有権システムに適応
- **エラーハンドリング**: Pythonの例外ではなくResultを使用
- **型システム**: 静的型付けによる厳密な型チェック
- **パフォーマンス**: メモリ効率とパフォーマンスの最適化

### デバッグ方法
- 各パスのJSON出力を確認
- `cargo test -- --nocapture` で詳細なテスト出力
- `RUST_BACKTRACE=1` で詳細なスタックトレース

## 成功基準

実装完了の判断基準：

1. **機能要件**:
   - [ ] MML文字列 "cde" を正しくSMFファイルに変換できる
   - [ ] 4つのデバッグJSON (pass1-4) が出力される
   - [ ] 生成されたMIDIファイルをDAWで再生できる

2. **テスト要件**:
   - [ ] すべてのユニットテストがパスする
   - [ ] 統合テストがパスする
   - [ ] `cargo test` がエラーなく完了する

3. **品質要件**:
   - [ ] `cargo clippy` でwarningが出ない
   - [ ] `cargo fmt --check` でフォーマット違反がない
   - [ ] ドキュメントが整備されている (README, コード内コメント)

4. **ビルド要件**:
   - [ ] `cargo build` が成功する
   - [ ] `cargo build --release` で最適化ビルドが成功する

## まとめ

このプロジェクトは、Python版の実績あるアーキテクチャをRustで再実装するものです。4パス設計とデバッグ出力により、段階的かつ検証可能な開発が可能です。TDDアプローチで各パスを順次実装することで、高品質で保守性の高いコードベースを構築します。

Rustの強力な型システムと所有権モデルにより、Python版よりも安全で高性能な実装が期待できます。
