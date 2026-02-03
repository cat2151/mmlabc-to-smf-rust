# mmlabc-to-smf-rust

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
  <a href="https://deepwiki.com/cat2151/mmlabc-to-smf-rust"><img src="https://img.shields.io/badge/📖-DeepWiki-blue.svg" alt="DeepWiki"></a>
</p>

A conversion library from Music Macro Language (MML) to Standard MIDI File (SMF)

## Overview

This library converts Music Macro Language (MML) format strings into Standard MIDI Files. It is written in Rust.

## Usage

Used as a library by `cat-play-mml`.

## Status

Under frequent breaking changes.

The README is currently out of date. Many more MML commands have been implemented than are documented here. The README will be updated later.

To see the implemented MML, please refer to [tree-sitter-mml/grammar.js](https://github.com/cat2151/mmlabc-to-smf-rust/blob/main/tree-sitter-mml/grammar.js) first (note that this may be subject to future breaking changes).

### Implemented Features ✅
- **Basic Note Conversion**: Converts `cdefgab` to MIDI notes.
- **4-Pass Architecture**: Fully implemented.
  - Pass 1: Tokenization of MML string (using tree-sitter parser).
  - Pass 2: Conversion from tokens to AST (Abstract Syntax Tree).
  - Pass 3: Generation of MIDI events from AST.
  - Pass 4: Creation of Standard MIDI File from MIDI events.
- **tree-sitter Integration**: Full tree-sitter parser integration for MML syntax analysis.
- **Channel Support**: Multi-channel support using semicolons (`;`).
- **JSON Debug Output**: Intermediate results of each pass are output in JSON format.
- **CLI**: Basic operations via command-line arguments.
- **Comprehensive Tests**: All 35 test cases pass.

### Usage Examples
```bash
# 基本音階変換
cargo run -- "cdefgab"

# 多チャンネル
cargo run -- "c;e;g"

# カスタム出力ファイル
cargo run -- "cde" -o my_song.mid
```

## Future Outlook

### Short-Term Goals 🚧
- **Repository Setup**: Set up formatter, linter, and other configurations.
- **Error Handling**: More detailed error messages.

### Long-Term Goals 🎯
- **mmlabc Command Implementation**: Full support for mmlabc format.
  - Note length specification (quarter note, eighth note, etc.).
  - Octave specification (`>`, `<`).
  - Control commands for tempo, volume, etc.
  - Chord functionality extension.
- **Performance Optimization**: Fast processing of large MML files.

### References
- For mmlabc, refer to the [mml2abc](https://github.com/cat2151/mml2abc) repository.

## Features

- **4-Pass Architecture**:
  - **Pass 1**: Parses MML strings into tokens (using tree-sitter parser).
  - **Pass 2**: Converts tokens into an Abstract Syntax Tree (AST).
  - **Pass 3**: Generates MIDI events from the AST.
  - **Pass 4**: Creates a Standard MIDI File.
- **Multi-channel Support**: Separates simultaneous voice channels using semicolons (`;`).
- **JSON Debug Output**: Intermediate results of each pass can be saved and viewed in JSON format.
- **Comprehensive Tests**: A total of 35 unit and integration test cases.
- **Safe Design**: Memory safety ensured by Rust's type system and ownership model.

## Requirements

- Rust 1.70.0 or later
- Cargo

## Installation

### Development Version (Current State)

```bash
git clone https://github.com/cat2151/mmlabc-to-smf-rust
cd mmlabc-to-smf-rust
cargo build --release
```

### Direct Execution (via Cargo)

```bash
cargo run -- "cdefgab"
```

## Usage

### Basic Usage

```bash
# 基本音階の変換（デフォルトでcat-play-mmlで自動再生されます）
cargo run -- "cdefgab"

# 多チャンネル（同時発音）
cargo run -- "c;e;g"  # Cメジャーコード

# カスタム出力ファイル
cargo run -- "cde" -o my_song.mid

# 自動再生を無効化
cargo run -- "cde" --no-play
```

### Auto-Play Functionality

By default, generated MIDI files are automatically played using the `cat-play-mml` command.
This allows for immediate sound confirmation during MML development.

- Use the `--no-play` option to disable auto-play.
- If `cat-play-mml` is not installed, a warning message will be displayed, but the MIDI file will still be generated correctly.

#### Custom Player Configuration

You can configure a custom MIDI player by creating a `mmlabc-to-smf-rust.toml` file in the directory where the tool is executed.

Example configuration file:
```toml
# mmlabc-to-smf-rust.toml
external_smf_player = "timidity"
```

Common configurable MIDI players:
- `timidity` - TiMidity++ MIDI player
- `fluidsynth` - FluidSynth software synthesizer
- `vlc` - VLC Media Player
- `cat-play-mml` (default)

If no configuration file exists, `cat-play-mml` is used by default.

Refer to `mmlabc-to-smf-rust.toml.example` for a sample configuration file.

### Output Files

The following files are generated upon execution:
- `pass1_tokens.json` - Pass 1 token information (for debugging).
- `pass2_ast.json` - Pass 2 AST information (for debugging).
- `pass3_events.json` - Pass 3 MIDI event information (for debugging).
- `output.mid` - The final MIDI file.

### Supported MML Notations

Currently supported notations:
- **Basic Notes**: `c`, `d`, `e`, `f`, `g`, `a`, `b` (case-insensitive).
- **Multi-channel**: Channel separation with `;` (simultaneous voicing).

Examples:
```
cdefgab     → ドレミファソラシの連続再生
c;e;g       → C・E・G音の同時再生（Cメジャーコード）
```

## Development

### Build

```bash
cargo build        # デバッグビルド
cargo build --release  # リリースビルド
```

### Tests

```bash
cargo test         # 全テスト実行（35個のテストケース）
```

### Formatting & Linting

```bash
cargo clippy       # コード品質チェック
cargo fmt --check  # フォーマットチェック
cargo fmt          # フォーマット適用
```

### tree-sitter Parser Files

tree-sitter parser files (located under `tree-sitter-mml/src/`) are **git-tracked** in accordance with tree-sitter best practices for reliable distribution on crates.io.

**Development Workflow:**
- C source files (`parser.c`, `grammar.json`, `node-types.json`, and the `tree_sitter/` directory) are automatically regenerated when [grammar.js](https://github.com/cat2151/mmlabc-to-smf-rust/blob/main/tree-sitter-mml/grammar.js) is modified.
- The build script checks the modification time of files and regenerates them only when necessary.
- **Prerequisite**: If you need to update the grammar, Node.js and npx must be installed on your system.
- Normal builds (without grammar changes) work without Node.js, as they use the committed C language files.

**Why commit generated files?**
This follows the best practices of the tree-sitter ecosystem:
- Users installing from crates.io do not need Node.js or tree-sitter-cli.
- Ensures that the grammar and parser versions precisely match.
- Simplifies CI/CD and cross-platform builds.
- This is standard practice for all tree-sitter language crates.

**Updating the Grammar:**
If you modify [tree-sitter-mml/grammar.js](https://github.com/cat2151/mmlabc-to-smf-rust/blob/main/tree-sitter-mml/grammar.js):
1. Run `cargo build` - the build script will detect changes and regenerate the parser files.
2. Commit both [grammar.js](https://github.com/cat2151/mmlabc-to-smf-rust/blob/main/tree-sitter-mml/grammar.js) and the regenerated C source files together.
3. This ensures the grammar and parser remain synchronized.

To manually regenerate the parser files:
```bash
cd tree-sitter-mml
npm install  # tree-sitter-cli がまだインストールされていない場合
npx tree-sitter generate
```

### Project Structure

```
src/
├── main.rs              # CLI エントリーポイント
├── lib.rs               # ライブラリルート
├── pass1_parser.rs      # パス1: トークン解析
├── pass2_ast.rs         # パス2: AST変換
├── pass3_events.rs      # パス3: MIDIイベント生成
├── pass4_midi.rs        # パス4: MIDI ファイル作成
├── tree_sitter_mml.rs   # tree-sitter MML統合
└── types.rs             # 共通型定義

tests/
├── integration_test.rs  # 統合テスト
├── test_channel.rs      # チャンネル機能テスト
├── test_pass1.rs        # パス1テスト
├── test_pass2.rs        # パス2テスト
├── test_pass3.rs        # パス3テスト
└── test_pass4.rs        # パス4テスト
```

## License

MIT License - See the [LICENSE](LICENSE) file for details.

## Reference

- Original Python implementation: [cat2151/mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf)