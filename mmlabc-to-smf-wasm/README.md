# mmlabc-to-smf-wasm

WebAssembly bindings for the MML to SMF converter.

## Overview

This crate provides WASM bindings that enable browser-based conversion of Music Macro Language (MML) to Standard MIDI Files (SMF).

## Architecture

The WASM implementation uses a **hybrid approach**:

1. **JavaScript (web-tree-sitter)**: Handles Pass 1 - parsing MML text into tokens using `tree-sitter-mml.wasm` grammar
2. **Rust WASM (this crate)**: Handles Pass 2-4 - converting tokens → AST → MIDI events → SMF binary

This design is necessary because the Rust `tree-sitter` crate cannot be compiled to WASM for browser use (it depends on Wasmtime which is not browser-compatible).

## Building

Build the WASM module using wasm-pack:

```bash
wasm-pack build --target web
```

This generates files in the `pkg/` directory:
- `mmlabc_to_smf_wasm_bg.wasm` - The compiled WASM binary
- `mmlabc_to_smf_wasm.js` - JavaScript glue code
- `mmlabc_to_smf_wasm.d.ts` - TypeScript type definitions

## Usage

See the `../demo/` directory for a complete working example.

### Basic Usage

```javascript
import init, { mml_to_smf } from './pkg/mmlabc_to_smf_wasm.js';
import TreeSitter from 'web-tree-sitter';

// Initialize
await TreeSitter.init();
await init();

// Create parser and load MML grammar
const parser = new TreeSitter();
const mmlLanguage = await TreeSitter.Language.load('tree-sitter-mml.wasm');
parser.setLanguage(mmlLanguage);

// Parse MML to tokens (Pass 1)
const tree = parser.parse('cde');
// ... extract tokens from tree (see demo for full implementation)

// Convert tokens to SMF (Pass 2-4)
const tokensJson = JSON.stringify(tokens);
const smfData = mml_to_smf(tokensJson);

// Download the MIDI file
const blob = new Blob([smfData], { type: 'audio/midi' });
const url = URL.createObjectURL(blob);
// ... trigger download
```

## API

### `mml_to_smf(tokens_json: string) -> Uint8Array`

Converts MML tokens (as JSON string) to Standard MIDI File binary data.

**Parameters:**
- `tokens_json`: JSON string containing array of token objects from tree-sitter parsing

**Returns:**
- `Uint8Array`: SMF binary data that can be downloaded as a `.mid` file

**Token Format:**
Each token should have the following structure:
```javascript
{
  "type": string,                // e.g. "note", "rest", "octave_set", etc.
  "value": string,               // token value (e.g. "c", "r", "o5", "@1", "t120")
  "channel_group": number | null, // channel group index for multi-channel (omitted if null)
  "chord_id": number | null,     // chord identifier for notes in a chord (omitted if null)
  "modifier": string | null,     // note modifier (e.g. "+", "-") (omitted if null)
  "note_length": number | null,  // parsed note length value (omitted if null)
  "dots": number | null          // number of dots for dotted notes (omitted if null)
}
```

Note: Null/undefined fields should be omitted from the JSON to match Rust's serialization format.

## Dependencies

- `wasm-bindgen`: WebAssembly bindings
- `serde`, `serde_json`: JSON serialization
- `midly`: MIDI file generation
- `anyhow`, `thiserror`: Error handling

The crate includes copies of the pass2-4 modules from the main crate, adapted to work without file I/O and tree-sitter dependencies.

## Testing

Run tests with:

```bash
cargo test
```

## License

MIT
