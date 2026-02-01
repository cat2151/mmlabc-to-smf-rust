# MML to SMF Browser Demo

This demo showcases the MML to SMF converter running entirely in the browser using WebAssembly and tree-sitter.

## Architecture

1. **Tree-sitter Parser (JavaScript)**: Parses MML text using `tree-sitter-mml.wasm` grammar
2. **Rust WASM Module**: Converts parsed tokens to Standard MIDI File (SMF) format
3. **Browser Download**: Downloads the generated MIDI file

## Setup

Before running the demo, you need to build the WASM module and prepare the necessary files:

1. **Install dependencies:**
```bash
cd demo
npm install
```

2. **Copy web-tree-sitter WASM file:**
```bash
cp node_modules/web-tree-sitter/web-tree-sitter.wasm .
```

3. **Build the WASM module:**
```bash
cd ../mmlabc-to-smf-wasm
wasm-pack build --target web
```

4. **Copy WASM module to demo:**
```bash
cp -r pkg ../demo/
```

## Running the Demo

### Using Python HTTP Server

```bash
python3 -m http.server 8000
```

Then open: http://localhost:8000

### Using Node.js HTTP Server

```bash
npx http-server -p 8000
```

Then open: http://localhost:8000

## Files

- `index.html` - Main demo page
- `tree-sitter-mml.wasm` - MML grammar for tree-sitter parser
- `web-tree-sitter.wasm` - Tree-sitter runtime
- `pkg/` - Compiled Rust WASM module
  - `mmlabc_to_smf_wasm_bg.wasm` - WASM binary
  - `mmlabc_to_smf_wasm.js` - JavaScript glue code

## MML Examples

Try these examples in the demo:

- `cde` - Basic notes
- `c;e;g` - Multiple channels (semicolon-separated)
- `'ceg'` - Chord
- `c4 d8 e16` - Notes with different lengths
- `c+ d- e` - Sharp/flat modifiers
- `o5 cde` - Octave setting
- `l4 cde` - Default length setting
- `@1 cde` - Program change
- `t120 cde` - Tempo setting
- `v10 cde` - Velocity setting

## Technical Details

The implementation satisfies the requirement that tree-sitter parser functionality must work in the demo. This is achieved using:

- `web-tree-sitter`: Official JavaScript/WASM bindings for tree-sitter
- `tree-sitter-mml.wasm`: Pre-compiled MML grammar
- Rust WASM module: Handles AST → Events → MIDI conversion without tree-sitter dependency

This hybrid approach avoids the limitation that the Rust tree-sitter crate cannot be compiled to WASM for browser use.
