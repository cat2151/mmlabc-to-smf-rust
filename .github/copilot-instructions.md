# copilot-instructions.md

## Overview

This repository implements a Music Macro Language (MML) to Standard MIDI File (SMF) converter in Rust. It is a Rust implementation of the original Python project [mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf). The converter uses a 4-pass architecture to transform MML strings into MIDI files with comprehensive debug output at each stage.

The project aims to:
- Convert MML format strings to Standard MIDI Files
- Provide detailed debug output (JSON) for each processing pass
- Eventually support the full mmlabc MML command set (see mml2abc repository)

## Contribution Guidelines

### Building
Build the project using:
```bash
cargo build
```

For release builds:
```bash
cargo build --release
```

### Testing
Run all tests using:
```bash
cargo test
```

All code changes must include passing tests. The project uses:
- Unit tests in individual modules
- Integration tests in the `tests/` directory
- Tests should validate behavior at each pass of the 4-pass architecture

### Linting and Formatting
Before committing, ensure code passes linting and formatting checks:

```bash
# Run clippy for linting
cargo clippy

# Check code formatting
cargo fmt --check

# Auto-format code
cargo fmt
```

All code must:
- Pass `cargo clippy` without warnings
- Be formatted with `cargo fmt`
- Follow Rust idioms and best practices

### Running the Application
After building, run the converter with:
```bash
cargo run -- "cde"
cargo run -- "cde" -o output.mid
```

## Project Structure

- `src/`: Primary source code
  - `main.rs`: CLI entry point
  - `lib.rs`: Library root
  - `pass1_parser.rs`: Pass 1 - Parse MML to tokens using tree-sitter
  - `pass2_ast.rs`: Pass 2 - Convert tokens to Abstract Syntax Tree
  - `pass3_events.rs`: Pass 3 - Generate MIDI events from AST
  - `pass4_midi.rs`: Pass 4 - Create Standard MIDI File
  - `tree_sitter_mml.rs`: Tree-sitter MML grammar integration
  - `types.rs`: Common type definitions
- `tests/`: Integration and unit tests
  - `integration_test.rs`: Full pipeline tests
  - `test_pass1.rs`: Pass 1 tests
  - `test_pass2.rs`: Pass 2 tests
  - `test_pass3.rs`: Pass 3 tests
  - `test_pass4.rs`: Pass 4 tests
- `tree-sitter-mml/`: Tree-sitter grammar for MML parsing
- `build.rs`: Build script for tree-sitter compilation

### Generated Files
The following files are generated during execution and should not be committed:
- `pass*.json`: Debug output JSON files from each pass
- `*.mid`: Generated MIDI files (except test fixtures in `tests/`)

## Technical Principles

### Language and Framework
- **Rust Edition**: 2021
- **Minimum Rust Version**: 1.70.0
- Use Rust's type system and ownership model for memory safety
- Prefer idiomatic Rust patterns

### Key Dependencies
- `midly`: MIDI file manipulation
- `serde` & `serde_json`: JSON serialization for debug output
- `clap`: Command-line argument parsing
- `tree-sitter`: Parsing MML syntax (fully integrated)
- `anyhow` & `thiserror`: Error handling

### Coding Standards
- Use meaningful variable and function names
- Add documentation comments (`///`) for public APIs
- Prefer explicit error handling with `Result` types
- Keep functions focused and single-purpose
- Write tests for new functionality

### Architecture
The project follows a **4-pass architecture**:
1. **Pass 1**: Parse MML string into tokens using tree-sitter
2. **Pass 2**: Transform tokens into Abstract Syntax Tree (AST)
3. **Pass 3**: Generate MIDI events from AST
4. **Pass 4**: Create Standard MIDI File from events

Each pass:
- Has its own module
- Produces JSON debug output
- Is independently testable
- Has clear input/output types

## Build, Test, and Validation Steps

### Complete Validation Workflow
Before submitting changes:
1. Build the project: `cargo build`
2. Run all tests: `cargo test`
3. Check formatting: `cargo fmt --check`
4. Run linter: `cargo clippy`
5. Test the CLI: `cargo run -- "cde" -o test.mid`

### Continuous Integration
The project should maintain:
- All tests passing
- Zero clippy warnings
- Proper code formatting
- Documentation for public APIs

## Pull Request Expectations

When submitting pull requests:
- Ensure all tests pass
- Add tests for new functionality
- Update documentation if adding/changing public APIs
- Keep changes focused and minimal
- Follow the existing code style and patterns
- Update README.md if adding user-facing features

### Making Changes
- Focus on one feature or fix per PR
- Avoid refactoring unrelated code
- Maintain backward compatibility when possible
- Consider the 4-pass architecture when making changes

### Debug Output
When adding new MML commands:
- Update the appropriate pass module
- Ensure JSON debug output reflects new functionality
- Add tests at each affected pass level

## Current Status and Roadmap

### Current Status
- ✅ Basic note conversion (c, d, e → MIDI)
- ✅ 4-pass architecture implemented
- ✅ Debug JSON output at each pass
- ✅ Tree-sitter integration (fully implemented)

### Future Work
- Repository configuration (formatter settings, etc.)
- Implement complete mmlabc MML command set
- Reference: See mml2abc repository for mmlabc format specification

## Additional Notes

- The project is Japanese-originated; both English and Japanese documentation exist
- Debug output files help understand processing at each stage
- When unsure about MML commands, refer to the original Python implementation or mml2abc repository
- Before committing, open the browser demo headlessly and confirm the console is error-free; if you cannot run this check, report that fact.
