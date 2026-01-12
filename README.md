# mmlabc-to-smf-rust

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
  <a href="https://deepwiki.com/cat2151/mmlabc-to-smf-rust"><img src="https://img.shields.io/badge/📖-DeepWiki-blue.svg" alt="DeepWiki"></a>
</p>

A library for converting Music Macro Language (MML) to Standard MIDI File (SMF)

## Overview

This library converts Music Macro Language (MML) strings into Standard MIDI Files. It is written in Rust.

## Usage

It is used as a library by `cat-play-mml`.

## Status

Frequent breaking changes are being made.

The README is currently not well-maintained. Many more MML commands are actually implemented. The README will be updated later.

To see which MML commands are implemented, please refer to `tree-sitter-mml/grammar.js` (note that this file will also undergo breaking changes in the future).

### Implemented Features ✅
- **Basic Note Conversion**: Converts `cdefgab` to MIDI notes.
- **4-Pass Architecture**: Fully implemented.
  - Pass 1: Tokenization of MML strings (simple parser)
  - Pass 2: Conversion of tokens to AST (Abstract Syntax Tree)
  - Pass 3: Generation of MIDI events from AST
  - Pass 4: Creation of Standard MIDI Files from MIDI events
- **Channel Functionality**: Multi-channel support using semicolons (`;`).
- **JSON Debug Output**: Outputs intermediate results of each pass in JSON format.
- **CLI**: Basic operations via command-line arguments.
- **Comprehensive Testing**: All 35 test cases pass.

### Usage Examples
```bash
# Basic scale conversion
cargo run -- "cdefgab"

# Multi-channel
cargo run -- "c;e;g"

# Custom output file
cargo run -- "cde" -o my_song.mid
```

## Future Prospects

### Short-Term Goals 🚧
- **tree-sitter Integration**: For parsing more complex MML syntax.
- **Repository Configuration**: Setting up formatters, linters, etc.
- **Error Handling**: More detailed error messages.

### Long-Term Goals 🎯
- **mmlabc Command Implementation**: Full support for the mmlabc format.
  - Note length specification (quarter notes, eighth notes, etc.)
  - Octave specification (`>`, `<`)
  - Control commands for tempo, volume, etc.
  - Extension of chord functionality
- **Performance Optimization**: High-speed processing of large MML files.

### References
- For mmlabc, refer to the [mml2abc](https://github.com/cat2151/mml2abc) repository.

## Features

- **4-Pass Architecture**:
  - **Pass 1**: Parses MML strings into tokens (currently a simple parser, future: tree-sitter).
  - **Pass 2**: Converts tokens into an Abstract Syntax Tree (AST).
  - **Pass 3**: Generates MIDI events from the AST.
  - **Pass 4**: Creates a Standard MIDI File.
- **Multi-channel Support**: Separation of simultaneous voice channels using semicolons (`;`).
- **JSON Debug Output**: Intermediate results of each pass can be saved and inspected in JSON format.
- **Comprehensive Testing**: A total of 35 unit and integration test cases.
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
# Basic scale conversion (automatically played by cat-play-mml by default)
cargo run -- "cdefgab"

# Multi-channel (simultaneous voices)
cargo run -- "c;e;g"  # C major chord

# Custom output file
cargo run -- "cde" -o my_song.mid

# Disable auto-playback
cargo run -- "cde" --no-play
```

### Auto-Playback Feature

By default, after a MIDI file is generated, it will be automatically played using the `cat-play-mml` command.
This allows for immediate sound confirmation during MML development.

- To disable auto-playback, use the `--no-play` option.
- If `cat-play-mml` is not installed, a warning message will be displayed, but the MIDI file will still be generated successfully.

#### Custom Player Configuration

You can configure a custom MIDI player by creating an `mmlabc-to-smf-rust.toml` file in the directory where the tool is executed.

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

The following files are generated upon execution:
- `pass1_tokens.json` - Token information from Pass 1 (for debugging)
- `pass2_ast.json` - AST information from Pass 2 (for debugging)
- `pass3_events.json` - MIDI event information from Pass 3 (for debugging)
- `output.mid` - The final MIDI file

### Supported MML Notation

Currently supported notation:
- **Basic Notes**: `c`, `d`, `e`, `f`, `g`, `a`, `b` (case-insensitive)
- **Multi-channel**: `;` for channel separation (simultaneous voices)

Example:
```
cdefgab     → Continuous playback of C-D-E-F-G-A-B
c;e;g       → Simultaneous playback of C, E, G notes (C major chord)
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

### Format and Lint

```bash
cargo clippy       # Code quality check
cargo fmt --check  # Format check
cargo fmt          # Apply format
```

### tree-sitter Parser Files

The tree-sitter parser files (located under `tree-sitter-mml/src/`) are **git-tracked** following tree-sitter best practices for reliable distribution on crates.io.

**Development Workflow:**
- The C source files (`parser.c`, `grammar.json`, `node-types.json`, and the `tree_sitter/` directory) are automatically regenerated when `grammar.js` is modified.
- The build script checks file modification times and regenerates only when necessary.
- **Prerequisite**: If you update the grammar, Node.js and npx must be installed on your system.
- Normal builds (without grammar changes) use the committed C files and therefore work without Node.js.

**Why Commit Generated Files**
This follows tree-sitter ecosystem best practices:
- Users installing from crates.io do not need Node.js or tree-sitter-cli.
- Ensures that the grammar and parser versions precisely match.
- Simplifies CI/CD and cross-platform builds.
- This is standard practice for all tree-sitter language crates.

**Updating the Grammar:**
If you modify `tree-sitter-mml/grammar.js`:
1. Run `cargo build` - the build script will detect changes and regenerate the parser files.
2. Commit both `grammar.js` and the regenerated C files together.
3. This ensures that the grammar and parser remain synchronized.

To manually regenerate the parser files:
```bash
cd tree-sitter-mml
npm install  # If tree-sitter-cli is not yet installed
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