# mmlabc-to-smf-rust

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

A library for converting Music Macro Language (MML) to Standard MIDI File (SMF).

## Overview

This library converts a Music Macro Language (MML) format string into a Standard MIDI File. It is written in Rust.

## Usage

Used as a library by `cat-play-mml`.

## Status

Frequent breaking changes are being made.

The README is outdated. Many more MML commands are actually implemented. The README will be updated later.

To understand the implemented MML, please refer to `tree-sitter-mml/grammar.js` first (though this will also be subject to breaking changes in the future).

### Implemented Features ✅
- **Basic Note Conversion**: `cdefgab` → Conversion to MIDI notes
- **4-Pass Architecture**: Fully implemented
  - Pass 1: Tokenization of MML string (simple parser)
  - Pass 2: Conversion of tokens to AST (Abstract Syntax Tree)
  - Pass 3: Generation of MIDI events from AST
  - Pass 4: Creation of Standard MIDI File from MIDI events
- **Channel Functionality**: Multi-channel support using semicolons (`;`)
- **JSON Debug Output**: Output intermediate results of each pass in JSON
- **CLI**: Basic operations via command-line arguments
- **Comprehensive Tests**: All 35 test cases are passing

### Verification
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
- **Repository Setup**: Set up formatter, linter, etc.
- **Error Handling**: More detailed error messages

### Long-term Goals 🎯
- **mmlabc Command Implementation**: Full mmlabc format support
  - Note length specification (quarter note, eighth note, etc.)
  - Octave specification (`>`, `<`)
  - Control commands for tempo, volume, etc.
  - Chord functionality expansion
- **Performance Optimization**: Fast processing of large MML files

### References
- For mmlabc, refer to the [mml2abc](https://github.com/cat2151/mml2abc) repository.

## Features

- **4-Pass Architecture**:
  - **Pass 1**: Parses MML string into tokens (Current: simple parser, Future: tree-sitter)
  - **Pass 2**: Converts tokens to an Abstract Syntax Tree (AST)
  - **Pass 3**: Generates MIDI events from the AST
  - **Pass 4**: Creates a Standard MIDI File
- **Multi-channel Support**: Semicolons (`;`) separate concurrent channels
- **JSON Debug Output**: Intermediate results of each pass can be saved and reviewed in JSON format
- **Comprehensive Tests**: A total of 35 test cases, including unit and integration tests
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
# Convert basic scale (automatically played by cat-play-mml by default)
cargo run -- "cdefgab"

# Multi-channel (concurrent notes)
cargo run -- "c;e;g"  # C major chord

# Custom output file
cargo run -- "cde" -o my_song.mid

# Disable auto-playback
cargo run -- "cde" --no-play
```

### Auto-playback Feature

By default, after generating a MIDI file, it will be automatically played using the `cat-play-mml` command.
This allows you to immediately check the sound during MML development.

- Use the `--no-play` option to disable auto-playback.
- If `cat-play-mml` is not installed, a warning message will be displayed, but the MIDI file will still be generated correctly.

#### Configuring a Custom Player

You can configure a custom MIDI player by creating a `mmlabc-to-smf-rust.toml` file in the directory where you run the tool.

Example configuration file:
```toml
# mmlabc-to-smf-rust.toml
external_smf_player = "timidity"
```

Common configurable MIDI players:
- `timidity` - TiMidity++ MIDI player
- `fluidsynth` - FluidSynth software synthesizer
- `vlc` - VLC media player
- `cat-play-mml` (default)

If no configuration file exists, `cat-play-mml` will be used by default.

Refer to `mmlabc-to-smf-rust.toml.example` for a sample configuration file.

### Output Files

Running the command will generate the following files:
- `pass1_tokens.json` - Pass 1 token information (for debugging)
- `pass2_ast.json` - Pass 2 AST information (for debugging)
- `pass3_events.json` - Pass 3 MIDI event information (for debugging)
- `output.mid` - The final MIDI file

### Supported MML Notation

Currently supported notation:
- **Basic Notes**: `c`, `d`, `e`, `f`, `g`, `a`, `b` (case-insensitive)
- **Multi-channel**: `;` for channel separation (concurrent notes)

Examples:
```
cdefgab     → Consecutive playback of C, D, E, F, G, A, B
c;e;g       → Simultaneous playback of C, E, G (C major chord)
```

## Development

### Build

```bash
cargo build        # Debug build
cargo build --release  # Release build
```

### Tests

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

The tree-sitter parser files (under `tree-sitter-mml/src/`) are **git-tracked** following tree-sitter best practices for reliable distribution on crates.io.

**Development Workflow:**
- The C source files (`parser.c`, `grammar.json`, `node-types.json`, and the `tree_sitter/` directory) are automatically regenerated when `grammar.js` is modified.
- The build script checks the file modification times and regenerates them only if necessary.
- **Prerequisite**: If you update the grammar, Node.js and npx must be installed on your system.
- Regular builds (without grammar changes) use the committed C files and therefore do not require Node.js.

**Why commit generated files?**
This follows best practices in the tree-sitter ecosystem:
- Users installing from crates.io do not need Node.js or `tree-sitter-cli`.
- It ensures that the grammar and parser versions precisely match.
- It simplifies CI/CD and cross-platform builds.
- It is standard practice for all tree-sitter language crates.

**Updating the Grammar:**
If you modify `tree-sitter-mml/grammar.js`:
1. Run `cargo build` - the build script will detect the change and regenerate the parser files.
2. Commit both `grammar.js` and the regenerated C files together.
3. This keeps the grammar and parser synchronized.

To manually regenerate the parser files:
```bash
cd tree-sitter-mml
npm install  # if tree-sitter-cli is not yet installed
npx tree-sitter generate
```

### Project Structure

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library root
├── pass1_parser.rs      # Pass 1: Token Parsing
├── pass2_ast.rs         # Pass 2: AST Transformation
├── pass3_events.rs      # Pass 3: MIDI Event Generation
├── pass4_midi.rs        # Pass 4: MIDI File Creation
├── tree_sitter_mml.rs   # tree-sitter MML Integration
└── types.rs             # Common Type Definitions

tests/
├── integration_test.rs  # Integration Tests
├── test_channel.rs      # Channel Feature Tests
├── test_pass1.rs        # Pass 1 Tests
├── test_pass2.rs        # Pass 2 Tests
├── test_pass3.rs        # Pass 3 Tests
└── test_pass4.rs        # Pass 4 Tests
```

## License

MIT License - See the [LICENSE](LICENSE) file for details.

## Reference

- Original Python implementation: [cat2151/mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf)