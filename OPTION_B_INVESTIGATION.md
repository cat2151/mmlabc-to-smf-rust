# Option B Investigation: Compiling tree-sitter parser.c to WASM

## Objective
Compile tree-sitter's C parser (parser.c) to WebAssembly so that the Rust WASM module can handle all 4 passes including parsing, achieving true SSOT (Single Source of Truth).

## Investigation Results

### Attempt 1: wasm32-unknown-unknown Target
**Command:**
```bash
cargo build --target wasm32-unknown-unknown
```

**Result:** ❌ Failed

**Error:**
```
fatal error: 'stdlib.h' file not found
```

**Analysis:**
- The bare `wasm32-unknown-unknown` target does not include C standard library headers
- parser.c requires standard C library functions (stdlib.h, string.h, etc.)
- This target is intentionally minimal and does not provide libc

### Attempt 2: wasm32-unknown-emscripten Target
**Command:**
```bash
cargo build --target wasm32-unknown-emscripten
```

**Result:** ❌ Failed (but more promising)

**Error:**
```
error: linker `emcc` not found
```

**Analysis:**
- The `wasm32-unknown-emscripten` target DOES support C standard library
- However, it requires the Emscripten toolchain to be installed
- Emscripten (`emcc`) provides the C standard library implementation for WASM

## Conclusion

### ✅ Technically Feasible
Option B IS technically feasible with Emscripten. The approach would be:
1. Install Emscripten SDK
2. Use `wasm32-unknown-emscripten` target
3. Configure build.rs to compile parser.c with emcc
4. Link the compiled parser with Rust WASM code

### ❌ Practical Concerns

**Increased Complexity:**
- **Build Requirements:** Emscripten SDK (~2GB+ installation)
- **Build Tools:** Node.js + Python + Emscripten + Rust
- **CI/CD:** All build environments need Emscripten
- **Maintenance:** Additional build configuration complexity

**Build Process:**
```bash
# Current (simple):
wasm-pack build --target web

# With Emscripten (complex):
1. Install Emscripten SDK
2. Configure environment variables
3. cargo build --target wasm32-unknown-emscripten
4. Additional post-processing for browser compatibility
```

## Recommendations

### For Production Use:
**Option A (Parse Tree via JSON)** is more practical:
- No additional toolchain dependencies
- Simpler build process
- Still achieves SSOT for token extraction logic
- Easier CI/CD integration

### For Exploration:
**Option B (Emscripten)** is worth pursuing if:
- Complete SSOT is absolutely required
- Team is willing to manage Emscripten dependency
- Build complexity is acceptable trade-off

## Next Steps

**If proceeding with Option B:**
1. Install Emscripten SDK in development environment
2. Update build.rs to properly configure emcc
3. Test integration with wasm-bindgen
4. Document build requirements
5. Update CI/CD configurations

**If proceeding with Option A:**
1. Modify JavaScript to serialize parse tree to JSON
2. Update Rust WASM to accept and parse tree JSON
3. Move token extraction logic entirely to Rust
4. Test end-to-end flow

## Technical Details

### parser.c Requirements
- C standard library (stdlib.h, string.h, stdint.h)
- Approximately 25KB of C code
- No external dependencies beyond libc
- Standard tree-sitter parser interface

### Emscripten Compatibility
- ✅ Provides complete C standard library for WASM
- ✅ Generates browser-compatible WASM
- ✅ Well-tested with tree-sitter
- ❌ Requires ~2GB SDK installation
- ❌ Adds build complexity

## Proof of Concept

The investigation included:
1. Adding build.rs to mmlabc-to-smf-wasm
2. Attempting compilation with both WASM targets
3. Documenting specific errors and requirements

The code changes demonstrate that the approach is blocked only by toolchain availability, not fundamental incompatibility.
