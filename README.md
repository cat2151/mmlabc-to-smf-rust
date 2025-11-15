# mmlabc-to-smf-rust

Music Macro Language (MML) to Standard MIDI File (SMF) conversion tool (Rust version)

## Overview

This project is a Rust implementation of [mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf).
It converts Music Macro Language strings to Standard MIDI Files using a 4-pass architecture with comprehensive debug output.

## WIP

Under development. Currently, only `c` through `b` are recognized; other MML features will be implemented later.

### Implemented Features ✅
- **Basic Note Conversion**: ``cdefgab` → Conversion to MIDI notes`
- **4-Pass Architecture**: Fully implemented
  - Pass 1: Tokenization of MML strings (simple parser)
  - Pass 2: Conversion from tokens to AST (Abstract Syntax Tree)
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

## Future Outlook

### Short-Term Goals 🚧
- **tree-sitter Integration**: For parsing more complex MML syntax
- **Repository Configuration**: Setting up formatters, linters, etc.
- **Error Handling**: More detailed error messages

### Long-Term Goals 🎯
- **mmlabc Command Implementation**: Full support for mmlabc format
  - Note length specification (quarter notes, eighth notes, etc.)
  - Octave specification (``>`, `<``)
  - Control commands for tempo, volume, etc.
  - Extension of chord functionality
- **Performance Optimization**: Fast processing of large MML files

### References
- For mmlabc, refer to the [mml2abc](https://github.com/cat2151/mml2abc) repository.

## Features

- **4-Pass Architecture**:
  - **Pass 1**: Parses MML strings into tokens (currently: simple parser, future: tree-sitter)
  - **Pass 2**: Converts tokens into an Abstract Syntax Tree (AST)
  - **Pass 3**: Generates MIDI events from the AST
  - **Pass 4**: Creates a Standard MIDI File
- **Multi-Channel Support**: Separation of concurrent sound channels using semicolons (`;`)
- **JSON Debug Output**: Intermediate results of each pass can be saved and viewed in JSON format
- **Comprehensive Testing**: A total of 35 test cases including unit and integration tests
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
# Basic scale conversion (automatically played by cat-play-mml by default)
cargo run -- "cdefgab"

# Multi-channel (concurrent notes)
cargo run -- "c;e;g"  # C major chord

# Custom output file
cargo run -- "cde" -o my_song.mid

# Disable auto-playback
cargo run -- "cde" --no-play
```

### Auto-Playback Feature

By default, after generating a MIDI file, it is automatically played using the `cat-play-mml` command.
This allows immediate sound verification during MML development.

- Use the `--no-play` option to disable auto-playback
- If `cat-play-mml` is not installed, a warning message will be displayed, but the MIDI file will still be generated successfully.

#### Custom Player Configuration

You can configure a custom MIDI player by creating a `mmlabc-to-smf-rust.toml` file in the directory where you run the tool.

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

If the configuration file does not exist, `cat-play-mml` will be used by default.

Refer to `mmlabc-to-smf-rust.toml.example` for a sample configuration file.

### Output Files

The following files are generated upon execution:
- `pass1_tokens.json` - Pass 1 token information (for debugging)
- `pass2_ast.json` - Pass 2 AST information (for debugging)
- `pass3_events.json` - Pass 3 MIDI event information (for debugging)
- `output.mid` - Final MIDI file

### Supported MML Notation

Currently supported notation:
- **Basic Notes**: ``c`, `d`, `e`, `f`, `g`, `a`, `b` (case-insensitive)`
- **Multi-Channel**: `;` for channel separation (concurrent notes)

Examples:
```
cdefgab     → Consecutive playback of C-D-E-F-G-A-B
c;e;g       → Simultaneous playback of C, E, G (C major chord)
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

### Tree-sitter Parser Files

The tree-sitter parser files (located in `tree-sitter-mml/src/`) are auto-generated during the build process and are **not tracked in git**. 

**Important Notes:**
- The C source files (`parser.c`, `grammar.json`, `node-types.json`, and `tree_sitter/` directory) are automatically generated when you run `cargo build`
- If the parser files don't exist, the build script will automatically run `npx tree-sitter generate` to create them
- **Requirements**: Node.js and npx must be installed on your system for the auto-generation to work
- You should never manually edit the generated parser files - instead, modify `tree-sitter-mml/grammar.js` and rebuild

To manually regenerate the parser files:
```bash
cd tree-sitter-mml
npm install  # Install tree-sitter-cli if not already installed
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