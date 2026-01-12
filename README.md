# mmlabc-to-smf-rust

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
  <a href="https://deepwiki.jp/cat2151/mmlabc-to-smf-rust"><img src="https://img.shields.io/badge/📖-DeepWiki-blue.svg" alt="DeepWiki"></a>
</p>

A library for converting Music Macro Language (MML) to Standard MIDI File (SMF)

## Overview

This library converts Music Macro Language (MML) format strings into Standard MIDI Files. It is written in Rust.

## Usage

This library is used by `cat-play-mml`.

## Status

This project undergoes frequent breaking changes.

The README is out of date. Many more MML commands have actually been implemented. The README will be updated later.

To understand the implemented MML, please refer to `tree-sitter-mml/grammer.js` (note that this file will also undergo breaking changes in the future).

### Implemented Features ✅
- **Basic Note Conversion**: `cdefgab` → MIDI note conversion
- **4-Pass Architecture**: Fully implemented
  - Pass 1: Tokenization of MML string (simple parser)
  - Pass 2: Conversion of tokens to AST (Abstract Syntax Tree)
  - Pass 3: Generation of MIDI events from AST
  - Pass 4: Creation of Standard MIDI File from MIDI events
- **Channel Functionality**: Multi-channel support using semicolons (`;`)
- **JSON Debug Output**: Output intermediate results of each pass in JSON format
- **CLI**: Basic operations via command-line arguments
- **Comprehensive Tests**: All 35 test cases pass

### Demonstration
```bash
# Basic scale conversion
cargo run -- "cdefgab"

# Multi-channel
cargo run -- "c;e;g"

# Custom output file
cargo run -- "cde" -o my_song.mid
```

## Future Outlook

### Short-term Goals 🚧
- **tree-sitter Integration**: For parsing more complex MML syntax
- **Repository Setup**: Setting up formatter, linter, etc.
- **Error Handling**: More detailed error messages

### Long-term Goals 🎯
- **mmlabc Command Implementation**: Full support for mmlabc format
  - Note length specification (quarter note, eighth note, etc.)
  - Octave specification (`>`, `<`)
  - Control commands for tempo, volume, etc.
  - Expansion of chord functionality
- **Performance Optimization**: Fast processing of large MML files

### References
- For mmlabc, refer to the [mml2abc](https://github.com/cat2151/mml2abc) repository.

## Features

- **4-Pass Architecture**:
  - **Pass 1**: Parses MML string into tokens (currently: simple parser, future: tree-sitter)
  - **Pass 2**: Converts tokens into an Abstract Syntax Tree (AST)
  - **Pass 3**: Generates MIDI events from the AST
  - **Pass 4**: Creates a Standard MIDI File
- **Multi-channel Support**: Semicolons (`;`) separate channels for concurrent playback
- **JSON Debug Output**: Intermediate results of each pass can be saved and reviewed in JSON format
- **Comprehensive Tests**: A total of 35 unit and integration test cases
- **Safe Design**: Memory safety ensured by Rust's type system and ownership model

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
# Convert basic scales (automatically played by cat-play-mml by default)
cargo run -- "cdefgab"

# Multi-channel (concurrent playback)
cargo run -- "c;e;g"  # C major chord

# Custom output file
cargo run -- "cde" -o my_song.mid

# Disable autoplay
cargo run -- "cde" --no-play
```

### Autoplay Feature

By default, after generating a MIDI file, it is automatically played using the `cat-play-mml` command.
This allows you to immediately check the sound during MML development.

- Use the `--no-play` option to disable autoplay.
- If `cat-play-mml` is not installed, a warning message will be displayed, but the MIDI file will still be generated correctly.

#### Custom Player Configuration

You can configure a custom MIDI player by creating an `mmlabc-to-smf-rust.toml` file in the directory where the tool is executed.

Example configuration file:
```toml
# mmlabc-to-smf-rust.toml
external_smf_player = "timidity"
```

Common MIDI players that can be configured:
- `timidity` - TiMidity++ MIDI player
- `fluidsynth` - FluidSynth software synthesizer
- `vlc` - VLC media player
- `cat-play-mml` (default)

If no configuration file exists, `cat-play-mml` is used by default.

Refer to `mmlabc-to-smf-rust.toml.example` for a sample configuration file.

### Output Files

The following files are generated upon execution:
- `pass1_tokens.json` - Pass 1 token information (for debugging)
- `pass2_ast.json` - Pass 2 AST information (for debugging)
- `pass3_events.json` - Pass 3 MIDI event information (for debugging)
- `output.mid` - The final MIDI file

### Supported MML Notation

Currently supported notations:
- **Basic Notes**: `c`, `d`, `e`, `f`, `g`, `a`, `b` (case-insensitive)
- **Multi-channel**: `;` for channel separation (concurrent playback)

Examples:
```
cdefgab     → Continuous playback of CDEFGAB
c;e;g       → Simultaneous playback of C, E, and G notes (C major chord)
```

## Development

### Build

```bash
cargo build        # Debug build
cargo build --release  # Release build
```

### Test

```bash
cargo test         # Run all tests (35 test cases)
```

### Format & Lint

```bash
cargo clippy       # Code quality check
cargo fmt --check  # Format check
cargo fmt          # Apply formatting
```

### tree-sitter Parser Files

For reliable distribution on crates.io, tree-sitter parser files (under `tree-sitter-mml/src/`) are **git-tracked** following tree-sitter best practices.

**Development Workflow:**
- The C source files (`parser.c`, `grammar.json`, `node-types.json`, and the `tree_sitter/` directory) are automatically regenerated when `grammar.js` is changed.
- The build script checks the modification times of these files and regenerates them only when necessary.
- **Prerequisite**: If you modify the grammar, Node.js and npx must be installed on your system.
- Regular builds (without grammar changes) will use the committed C files and will work without Node.js.

**Why commit generated files?**
This follows best practices in the tree-sitter ecosystem:
- Users installing from crates.io do not need Node.js or `tree-sitter-cli`.
- It ensures that the grammar and parser versions precisely match.
- It simplifies CI/CD and cross-platform builds.
- It is standard practice for all tree-sitter language crates.

**Updating the grammar:**
If you modify `tree-sitter-mml/grammar.js`:
1. Run `cargo build` - the build script will detect the change and regenerate the parser files.
2. Commit both `grammar.js` and the regenerated C files together.
3. This ensures the grammar and parser remain in sync.

To manually regenerate the parser files:
```bash
cd tree-sitter-mml
npm install  # if tree-sitter-cli is not already installed
npx tree-sitter generate
```

### Project Structure

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library root
├── pass1_parser.rs      # Pass 1: Token parsing
├── pass2_ast.rs         # Pass 2: AST conversion
├── pass3_events.rs      # Pass 3: MIDI event generation
├── pass4_midi.rs        # Pass 4: MIDI file creation
├── tree_sitter_mml.rs   # tree-sitter MML integration
└── types.rs             # Common type definitions

tests/
├── integration_test.rs  # Integration tests
├── test_channel.rs      # Channel functionality tests
├── test_pass1.rs        # Pass 1 tests
├── test_pass2.rs        # Pass 2 tests
├── test_pass3.rs        # Pass 3 tests
└── test_pass4.rs        # Pass 4 tests
```

## License

MIT License - See the [LICENSE](LICENSE) file for details.

## References

- Original Python implementation: [cat2151/mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf)