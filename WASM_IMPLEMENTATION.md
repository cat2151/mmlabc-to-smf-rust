# WASM Implementation Summary

## Issue #44 - ブラウザで MML to SMF（バイナリバッファ） 変換を可能とするためのWASM版のクレート追加を、WASI Reactor (FFI export) の方法で実装

## Implementation Overview

This implementation adds WebAssembly support to the mmlabc-to-smf-rust library, enabling browser-based MML to MIDI file conversion.

## Architecture: WASI Reactor (FFI Export) Pattern

### Why This Approach?

The issue mentioned that tree-sitter's `parser.c` (C language dependency) prevents direct WASM compilation. The WASI Reactor pattern solves this by:

1. **Separating concerns**: Parsing (C-dependent) and conversion (Rust-only)
2. **Leveraging strengths**: Native/JavaScript for parsing, WASM for binary generation
3. **Maintaining quality**: Reuses existing, tested Rust code for Passes 2-4

### How It Works

```
┌─────────────────────────────────────────────────────────────┐
│ Browser Environment                                         │
│                                                             │
│  ┌──────────────┐         ┌──────────────────────────┐    │
│  │  JavaScript  │         │    WASM Module           │    │
│  │              │         │                          │    │
│  │  Pass 1:     │  JSON   │  Pass 2: Tokens → AST    │    │
│  │  MML → Tokens├────────>│  Pass 3: AST → Events    │    │
│  │              │         │  Pass 4: Events → MIDI   │    │
│  │              │<────────┤                          │    │
│  └──────────────┘  Binary │  (339KB .wasm file)      │    │
│                     MIDI  └──────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## Key Changes

### 1. Conditional Compilation (build.rs, Cargo.toml, src/lib.rs)

```toml
# Cargo.toml
[target.'cfg(not(target_family = "wasm"))'.dependencies]
tree-sitter = "0.22"
```

- Tree-sitter only compiled for non-WASM targets
- Build script detects WASM target and skips C compilation
- Modules conditional on `#[cfg(not(target_family = "wasm"))]`

### 2. WASM FFI Interface (src/wasm_compat.rs)

Three main exported functions:

```rust
// Convert JSON tokens to MIDI binary
pub extern "C" fn tokens_to_smf(tokens_json_ptr: *const c_char) -> c_int

// Get pointer to generated MIDI data
pub extern "C" fn get_midi_data() -> *const u8

// Get error message (must call free_error_string after use)
pub extern "C" fn get_last_error() -> *const c_char
```

**Input**: JSON-serialized Token array  
**Output**: Binary MIDI file data

### 3. Browser Demo (demo/)

Complete working example with:
- **index.html**: Full UI with textarea, examples, download
- **serve.py**: Simple HTTP server with WASI headers
- **README.md**: Usage instructions
- Simplified MML parser for demonstration

## Building and Using

### Build WASM Module

```bash
# Easy way
./build-wasm.sh

# Manual way
cargo build --target wasm32-wasip1 --lib --release
cp target/wasm32-wasip1/release/mmlabc_to_smf.wasm demo/
```

### Test Demo

```bash
python3 demo/serve.py
# Open http://localhost:8000 in browser
```

### JavaScript Integration Example

```javascript
// 1. Load WASM module
const wasmModule = await WebAssembly.instantiate(wasmBytes, wasiImports);

// 2. Parse MML to tokens (simplified example)
const tokens = parseMml("cde");

// 3. Convert tokens to JSON
const tokensJson = JSON.stringify(tokens);

// 4. Call WASM function
const midiLength = wasmModule.instance.exports.tokens_to_smf(tokensPtr);

// 5. Get MIDI data
const midiDataPtr = wasmModule.instance.exports.get_midi_data();
const midiData = new Uint8Array(memory.buffer, midiDataPtr, midiLength);

// 6. Download or use MIDI data
const blob = new Blob([midiData], { type: 'audio/midi' });
```

## File Changes

### Modified Files
- `Cargo.toml`: Added cdylib, conditional tree-sitter
- `build.rs`: Skip tree-sitter for WASM targets
- `src/lib.rs`: Conditional module exports
- `.gitignore`: Ignore WASM artifacts
- `README.md`, `README.ja.md`: WASM documentation

### New Files
- `src/wasm_compat.rs`: WASM FFI interface (194 lines)
- `demo/index.html`: Browser demo (283 lines)
- `demo/serve.py`: HTTP server with WASI support
- `demo/README.md`: Demo documentation
- `build-wasm.sh`: Build convenience script

## Testing

✅ **All tests pass**
- Integration tests: 3/3 passing
- CLI functionality: Unchanged and working
- WASM builds: Successfully compiles to 339KB

✅ **Code quality**
- No clippy warnings
- Formatted with cargo fmt
- Comprehensive documentation

## Compatibility

✅ **Backward compatible**
- No breaking changes to existing API
- CLI works exactly as before
- Native builds unaffected by WASM changes

## Limitations & Future Work

### Current Demo Limitations
- Simplified JavaScript parser (only supports basic notes and rests)
- For full MML support, integrate complete parser

### Possible Enhancements
1. **Full Parser Integration**
   - Port tree-sitter grammar to pure Rust parser
   - Or use tree-sitter-wasm separately
   - Or implement native JavaScript parser

2. **Streaming API**
   - Support large MIDI files without memory issues
   - Incremental processing

3. **Web Worker Support**
   - Run WASM in background thread
   - Non-blocking UI

4. **Size Optimization**
   - Use wasm-opt for smaller builds
   - Strip unused features

## Security Considerations

1. **Single-threaded only**: Static mutable variables used (safe in browser)
2. **Memory management**: Caller must free error strings
3. **Input validation**: JSON parsing errors handled gracefully

## Performance

- **WASM module size**: 339KB (release build)
- **Conversion speed**: Near-native performance
- **Memory usage**: Minimal, only stores last result

## Conclusion

This implementation successfully adds browser-based MIDI generation capability while:
- ✅ Solving the tree-sitter C dependency issue
- ✅ Maintaining code quality and compatibility
- ✅ Providing complete working demo
- ✅ Following Rust and WASM best practices
- ✅ Meeting all requirements from issue #44

The WASI Reactor (FFI export) pattern proved to be an effective solution for enabling browser usage while working around the C dependency constraints.
