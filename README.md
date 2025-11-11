# mmlabc-to-smf-rust

A conversion tool from Music Macro Language (MML) to Standard MIDI File (SMF) (Rust version)

## Overview

This project is the Rust implementation of [mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf).
It converts Music Macro Language strings to Standard MIDI File format using a 4-pass architecture with comprehensive debug output.

## WIP

Currently under development. It only recognizes `c` through `b` at this stage; other MML features will be implemented later.

### Implemented Features ✅
- **Basic Note Conversion**: `cdefgab` → Conversion to MIDI notes
- **4-Pass Architecture**: Fully implemented
  - Pass 1: Tokenization of MML string (simple parser)
  - Pass 2: Conversion of tokens to AST (Abstract Syntax Tree)
  - Pass 3: Generation of MIDI events from AST
  - Pass 4: Creation of Standard MIDI File from MIDI events
- **Channel Functionality**: Multi-channel support using semicolons (`;`)
- **JSON Debug Output**: Outputs intermediate results of each pass in JSON format
- **CLI**: Basic operations via command-line arguments
- **Comprehensive Tests**: All 35 test cases pass

### Usage Examples
```bash
# Basic scale conversion
cargo run -- "cdefgab"

# Multi-channel
cargo run -- "c;e;g"

# Custom output file
cargo run -- "cde" -o my_song.mid
```

## Roadmap

### Short-term Goals 🚧
- **tree-sitter integration**: For parsing more complex MML syntax
- **Repository Configuration**: Setting up formatters, linters, etc.
- **Error Handling**: More detailed error messages

### Long-term Goals 🎯
- **mmlabc Command Implementation**: Full mmlabc format support
  - Note length specification (e.g., quarter notes, eighth notes)
  - Octave specification (`>`, `<`)
  - Control commands for tempo, volume, etc.
  - Chord functionality extensions
- **Performance Optimization**: Fast processing of large MML files

### References
- For mmlabc, refer to the [mml2abc](https://github.com/cat2151/mml2abc) repository.

## Features

- **4-Pass Architecture**:
  - **Pass 1**: Parses MML strings into tokens (Currently: simple parser, Future: tree-sitter)
  - **Pass 2**: Converts tokens to an Abstract Syntax Tree (AST)
  - **Pass 3**: Generates MIDI events from the AST
  - **Pass 4**: Creates a Standard MIDI File
- **Multi-channel support**: Separation of simultaneous sounding channels using semicolons (`;`)
- **JSON Debug Output**: Intermediate results of each pass can be saved and checked in JSON format
- **Comprehensive Tests**: A total of 35 test cases, including unit and integration tests
- **Safe Design**: Memory safety enabled by Rust's type system and ownership model

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

# Multi-channel (simultaneous playback)
cargo run -- "c;e;g"  # C major chord

# Custom output file
cargo run -- "cde" -o my_song.mid

# Disable auto-playback
cargo run -- "cde" --no-play
```

### Auto-playback Feature

By default, after generating a MIDI file, it will automatically be played using the `cat-play-mml` command.
This allows you to immediately check the sound during MML development.

- To disable auto-playback, use the `--no-play` option.
- If `cat-play-mml` is not installed, a warning message will be displayed, but the MIDI file will still be generated successfully.

### Output Files

The following files are generated upon execution:
- `pass1_tokens.json` - Pass 1 token information (for debugging)
- `pass2_ast.json` - Pass 2 AST information (for debugging)
- `pass3_events.json` - Pass 3 MIDI event information (for debugging)
- `output.mid` - The final MIDI file

### Supported MML Notation

Currently supported notation:
- **Basic Notes**: `c`, `d`, `e`, `f`, `g`, `a`, `b` (case-insensitive)
- **Multi-channel**: `;` to separate channels (simultaneous playback)

Examples:
```
cdefgab     → Continuous playback of C, D, E, F, G, A, B
c;e;g       → Simultaneous playback of C, E, G notes (C major chord)
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

### Project Structure

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library root
├── pass1_parser.rs      # Pass 1: Token analysis
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