# mmlabc-to-smf-rust

A tool for converting Music Macro Language (MML) to Standard MIDI File (SMF) (Rust version)

## Overview

This project is a Rust implementation of [mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf).
It converts Music Macro Language format strings to Standard MIDI Files using a 4-pass architecture with comprehensive debug output.

## Status

### Implemented Features ✅
- **Basic Note Conversion**: `cdefgab` → MIDI note conversion
- **4-Pass Architecture**: Fully implemented
  - Pass 1: Tokenization of MML string (simple parser)
  - Pass 2: Conversion from tokens to AST (Abstract Syntax Tree)
  - Pass 3: Generation of MIDI events from AST
  - Pass 4: Creation of Standard MIDI File from MIDI events
- **Channel Functionality**: Multi-channel support using semicolons (`;`)
- **JSON Debug Output**: Intermediate results of each pass outputted in JSON
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

## Future Outlook

### Short-term Goals 🚧
- **tree-sitter integration**: For parsing more complex MML syntax
- **Repository configuration**: Setting up formatters, linters, etc.
- **Error handling**: More detailed error messages

### Long-term Goals 🎯
- **mmlabc command implementation**: Full support for mmlabc format
  - Note length specification (quarter note, eighth note, etc.)
  - Octave specification (`>`, `<`)
  - Control commands like tempo, volume
  - Extension of chord functionality
- **Performance optimization**: Fast processing of large MML files

### References
- For mmlabc, refer to the [mml2abc](https://github.com/cat2151/mml2abc) repository.

## Features

- **4-Pass Architecture**:
  - **Pass 1**: Parses MML string into tokens (currently: simple parser, future: tree-sitter)
  - **Pass 2**: Converts tokens into an Abstract Syntax Tree (AST)
  - **Pass 3**: Generates MIDI events from the AST
  - **Pass 4**: Creates a Standard MIDI File
- **Multi-channel Support**: Simultaneous sounding channel separation using semicolons (`;`)
- **JSON Debug Output**: Intermediate results of each pass can be saved and reviewed in JSON format
- **Comprehensive Tests**: Total of 35 unit and integration test cases
- **Safe Design**: Memory safety ensured by Rust's type system and ownership model

## Requirements

- Rust 1.70.0 or higher
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

# Multi-channel (simultaneous notes)
cargo run -- "c;e;g"  # C major chord

# Custom output file
cargo run -- "cde" -o my_song.mid

# Disable auto-play
cargo run -- "cde" --no-play
```

### Auto-play Functionality

By default, after generating a MIDI file, it is automatically played using the `cat-play-mml` command.
This allows immediate sound confirmation during MML development.

- To disable auto-play, use the `--no-play` option.
- If `cat-play-mml` is not installed, a warning message will be displayed, but the MIDI file will still be generated successfully.

### Output Files

The following files are generated upon execution:
- `pass1_tokens.json` - Token information from Pass 1 (for debugging)
- `pass2_ast.json` - AST information from Pass 2 (for debugging)
- `pass3_events.json` - MIDI event information from Pass 3 (for debugging)
- `output.mid` - The final MIDI file

### Supported MML Notation

Currently supported notation:
- **Basic Notes**: `c`, `d`, `e`, `f`, `g`, `a`, `b` (case-insensitive)
- **Multi-channel**: Channel separation with `;` (simultaneous notes)

Examples:
```
cdefgab     → Consecutive playback of CDEFGAB
c;e;g       → Simultaneous playback of C, E, and G (C major chord)
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
cargo fmt          # Apply format
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