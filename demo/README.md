# MML to SMF WASM Demo

This demo demonstrates browser-based MML to SMF conversion using Option A architecture:
- **JavaScript (web-tree-sitter)**: Parses MML text and generates parse tree
- **Rust WASM**: Receives parse tree JSON, extracts tokens, and generates SMF binary

This achieves SSOT (Single Source of Truth) by keeping all token extraction logic in Rust.

## Setup

### 1. Install Dependencies

```bash
cd demo
npm install
```

This installs `web-tree-sitter` which is needed for parsing.

### 2. Copy web-tree-sitter Files

```bash
cp node_modules/web-tree-sitter/web-tree-sitter.js .
cp node_modules/web-tree-sitter/web-tree-sitter.wasm .
```

These files need to be in the demo directory for the browser to load them.

### 3. Build the WASM Module

```bash
cd ../mmlabc-to-smf-wasm
wasm-pack build --target web
```

This creates the `pkg/` directory with the WASM binary and JavaScript bindings.

### 4. Build the Tree-sitter Grammar WASM

```bash
cd ../tree-sitter-mml
npm install tree-sitter-cli  # If not already installed
npx tree-sitter build-wasm
```

This creates `tree-sitter-mml.wasm` for web-tree-sitter.

### 5. Run the Demo

Start a local web server from the repository root:

```bash
cd ..
python3 -m http.server 8000
```

Then open http://localhost:8000/demo/ in your browser.

## Architecture

```
User Input (MML text)
     |
     v
JavaScript: web-tree-sitter.parse(mml)
     |
     v
Parse Tree JSON
     |
     v
Rust WASM: parse_tree_json_to_smf(json, mml)
     |
     +-> Extract tokens from parse tree (SSOT in Rust!)
     +-> Pass 2: Tokens → AST
     +-> Pass 3: AST → MIDI Events
     +-> Pass 4: Events → SMF Binary
     |
     v
SMF Binary (downloadable .mid file)
```

## Key Features

- **SSOT Achieved**: All token extraction logic is in Rust only (not duplicated in JavaScript)
- **No C stdlib Issues**: Uses web-tree-sitter (already compiled to WASM) for parsing
- **Clean Architecture**: JavaScript only handles UI and parse tree serialization
- **Full MML Support**: Uses the same grammar and conversion logic as the CLI

## Examples

Try these in the demo:

- `cde` - Simple notes
- `c+d-e` - Notes with modifiers (sharp/flat)
- `c4d8e16` - Notes with different lengths
- `o5cde` - Octave set command
- `t120@1v100cde` - Tempo, program, and velocity settings
- `'ceg'` - Chord (notes played simultaneously)

## Current Limitations

- **Single-channel MML only**: The current demo does not support multi-channel MML (MML with semicolons like `cde;fga`). Multi-channel support would require splitting the MML by semicolons in JavaScript and calling the WASM function separately for each channel. This is a known limitation and may be addressed in future updates.

## Implementation Notes

The key difference from the original approach in PR 47:
- **PR 47 (Rejected)**: JavaScript had `extractTokens()` function duplicating Rust logic
- **This Implementation (Option A)**: JavaScript only converts parse tree to JSON, all token extraction is in Rust

This ensures that the token extraction logic is maintained in a single place (Rust), making it easier to maintain and avoiding synchronization issues.
