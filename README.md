# mmlabc-to-smf-rust

A tool for converting Music Macro Language (MML) to Standard MIDI File (SMF) (Rust version)

## Overview

This project is a Rust implementation of [mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf).
It converts Music Macro Language strings into Standard MIDI Files using a 4-pass architecture with comprehensive debug output.

## WIP

Currently under development. At present, only `c` to `b` notes are recognized; other MML features will be implemented going forward.

### Implemented Features ✅
- **Basic Note Conversion**: `cdefgab` → MIDI note conversion
- **4-Pass Architecture**: Fully implemented
  - Pass 1: Tokenization of MML string (simple parser)
  - Pass 2: Conversion from tokens to AST (Abstract Syntax Tree)
  - Pass 3: Generation of MIDI events from AST
  - Pass 4: Creation of Standard MIDI File from MIDI events
- **Channel Functionality**: Multi-channel support using semicolons (`;`)
- **JSON Debug Output**: Intermediate results of each pass outputted in JSON format
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
- **tree-sitter Integration**: For parsing more complex MML syntax
- **Repository Setup**: Configuration of formatter, linter, etc.
- **Error Handling**: More detailed error messages

### Long-term Goals 🎯
- **mmlabc Command Implementation**: Full support for mmlabc format
  - Note length specification (quarter note, eighth note, etc.)
  - Octave specification (`>`, `<`)
  - Control commands for tempo, volume, etc.
  - Chord functionality extension
- **Performance Optimization**: Fast processing of large MML files

### References
- For mmlabc, refer to the [mml2abc](https://github.com/cat2151/mml2abc) repository.

## Features

- **4-Pass Architecture**:
  - **Pass 1**: Parses MML string into tokens (Current: simple parser, Future: tree-sitter)
  - **Pass 2**: Converts tokens into an Abstract Syntax Tree (AST)
  - **Pass 3**: Generates MIDI events from the AST
  - **Pass 4**: Creates the Standard MIDI File
- **Multi-channel Support**: Semicolons (`;`) separate simultaneous playback channels
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

By default, after generating a MIDI file, it is automatically played using the `cat-play-mml` command.
This allows for immediate sound confirmation during MML development.

- Use the `--no-play` option to disable auto-playback.
- If `cat-play-mml` is not installed, a warning message will appear, but the MIDI file will still be generated correctly.

#### Custom Player Configuration

You can configure a custom MIDI player by creating an `mmlabc-to-smf-rust.toml` file in the directory where the tool is executed.

Example configuration file:
```toml
# mmlabc-to-smf-rust.toml
external_smf_player = "timidity"
```

Configurable common MIDI players:
- `timidity` - TiMidity++ MIDI player
- `fluidsynth` - FluidSynth software synthesizer
- `vlc` - VLC media player
- `cat-play-mml` (default)

If no configuration file exists, `cat-play-mml` will be used by default.

Refer to `mmlabc-to-smf-rust.toml.example` for a sample configuration file.

### Output Files

Upon execution, the following files will be generated:
- `pass1_tokens.json` - Pass 1 token information (for debugging)
- `pass2_ast.json` - Pass 2 AST information (for debugging)
- `pass3_events.json` - Pass 3 MIDI event information (for debugging)
- `output.mid` - The final MIDI file

### Supported MML Notation

Currently supported notation:
- **Basic Notes**: `c`, `d`, `e`, `f`, `g`, `a`, `b` (case-insensitive)
- **Multi-channel**: `;` for channel separation (simultaneous playback)

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

tree-sitter parser files (under `tree-sitter-mml/src/`) are **git-tracked** following tree-sitter best practices for reliable distribution on crates.io.

**Development Workflow:**
- The C source files (`parser.c`, `grammar.json`, `node-types.json`, and the `tree_sitter/` directory) are automatically regenerated when `grammar.js` is modified.
- The build script checks file modification times and regenerates only when necessary.
- **Prerequisite**: You need Node.js and `npx` installed on your system if you intend to update the grammar.
- Regular builds (without grammar changes) will use the committed C files and work without Node.js.

**Why commit generated files:**
This follows best practices in the tree-sitter ecosystem:
- Users installing from crates.io do not need Node.js or `tree-sitter-cli`.
- Ensures that the grammar and parser versions precisely match.
- Simplifies CI/CD and cross-platform builds.
- It's a standard practice for all tree-sitter language crates.

**Updating the Grammar:**
If you modify `tree-sitter-mml/grammar.js`:
1. Run `cargo build` - the build script will detect changes and regenerate the parser files.
2. Commit both `grammar.js` and the regenerated C files together.
3. This keeps the grammar and parser synchronized.

To manually regenerate parser files:
```bash
cd tree-sitter-mml
npm install  # If tree-sitter-cli is not already installed
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