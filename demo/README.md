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

- **Pass 1 (Parsing)**: Done in JavaScript using a simplified parser
  - In production, this could use tree-sitter compiled to WASM separately, or be done server-side
- **Passes 2-4 (AST → Events → MIDI)**: Done in WebAssembly
  - The JavaScript code sends parsed tokens as JSON to the WASM module
  - The WASM module processes the tokens and returns binary MIDI data

## Notes

- The current demo uses a simplified MML parser in JavaScript for demonstration purposes
- For full MML support (including all mmlabc commands), integrate a proper MML parser
- The WASM module is ~339KB in release mode
- The module requires WASI support, which is provided by minimal WASI stubs in the JavaScript

## Example MML Inputs

- Simple notes: `cde`
- Scale: `cdefgab`
- Chord: `[ceg]`
- With rests: `c r e r g`
