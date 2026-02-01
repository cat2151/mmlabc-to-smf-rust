# WASM Demo

This directory contains a browser-based demo of the MML to SMF converter using WebAssembly.

## How to Use

1. Build the WASM module:
   ```bash
   cargo build --target wasm32-wasip1 --lib --release
   cp target/wasm32-wasip1/release/mmlabc_to_smf.wasm demo/
   ```

2. Start the local web server:
   ```bash
   python3 demo/serve.py
   ```
   
   Or use any other HTTP server, for example:
   ```bash
   cd demo
   python3 -m http.server 8000
   ```

3. Open your browser and navigate to:
   ```
   http://localhost:8000
   ```

4. Enter MML text in the textarea and click "Convert & Download" to generate a MIDI file.

## Architecture

This demo uses the WASI Reactor (FFI export) pattern:

- **Pass 1 (Parsing)**: Done outside WASM (JavaScript in this demo)
  - **IMPORTANT**: The demo's JavaScript parser is simplified for demonstration only
  - **The WASM module itself supports ALL MML features** - it accepts full Token structures
  - In production, use tree-sitter-wasm, server-side tree-sitter, or a complete JavaScript parser
- **Passes 2-4 (AST → Events → MIDI)**: Done in WebAssembly
  - **Full mmlabc-to-smf-rust functionality maintained**
  - Supports all MML commands: chords, octaves, lengths, modifiers, multi-channel, etc.
  - The JavaScript code sends parsed tokens as JSON to the WASM module
  - The WASM module processes the tokens and returns binary MIDI data

## Full Feature Support

**The WASM module maintains 100% of mmlabc-to-smf-rust functionality:**
- ✅ All MML commands supported by tree-sitter parser
- ✅ Chords, octave changes, note lengths, dots, modifiers
- ✅ Multi-channel support with semicolons
- ✅ All token types from Pass 1 (tree-sitter)

**The limitation is only in the demo's parser** - not the WASM module.

## Notes

- The current demo uses a simplified MML parser in JavaScript **for demonstration purposes only**
- The simplified parser supports only basic notes (`a`-`g`) and rests (`r`)
- **This is NOT a limitation of the WASM module** - it's only a limitation of the demo HTML
- The WASM module (`mmlabc_to_smf.wasm`) accepts full Token structures and supports all MML features
- For full MML support, integrate a proper MML parser (tree-sitter-wasm, server-side, or complete JS implementation)
- The WASM module is ~339KB in release mode
- The module requires WASI support, which is provided by minimal WASI stubs in the JavaScript

## Example MML Inputs

The simplified JavaScript parser in this demo currently supports:
- Simple notes: `cde`
- Scale: `cdefgab`
- With rests: `c r e r g`

For full MML features like chords (`[ceg]`), octaves (`>c<c`), lengths (`c4`), etc., you would need to integrate a complete MML parser.
