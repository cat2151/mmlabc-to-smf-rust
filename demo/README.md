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

**Important**: This step is required whenever `tree-sitter-mml/grammar.js` is modified. The tree-sitter WASM is separate from the Rust tree-sitter integration used by the CLI, so it must be explicitly rebuilt.

```bash
cd ../tree-sitter-mml
npm install tree-sitter-cli  # If not already installed
npx tree-sitter build-wasm
```

This creates `tree-sitter-mml.wasm` for web-tree-sitter.

**Automated build**: The `scripts/build-demo.sh` script now automatically builds the tree-sitter WASM, ensuring it stays in sync with the grammar.

### 5. Run the Demo

Start a local web server from the repository root:

```bash
cd ..
python3 -m http.server 8000
```

Then open http://localhost:8000/ (or http://localhost:8000/index.html) in your browser.

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
- `c;e;g` - Multi-channel (notes on separate MIDI channels)

## Multi-Channel Support

✅ **Multi-channel MML is now supported!** The tree-sitter WASM grammar has been rebuilt to include multi-channel support. The demo can now parse MML with semicolons (e.g., `c;e;g`) correctly, assigning each channel group to a separate MIDI channel.

### Root Cause and Prevention

**What happened**: The `tree-sitter-mml.wasm` file was out of sync with `grammar.js`. While the grammar defined multi-channel support (`channel_groups`), the WASM binary hadn't been rebuilt, causing parse errors in the browser demo.

**Why it happened**: 
1. The build process (`scripts/build-demo.sh`) only verified the WASM file existed but didn't rebuild it
2. The CI/CD pipeline (`deploy-github-pages.yml`) copied the existing WASM without rebuilding
3. The CLI worked correctly because it uses the Rust tree-sitter integration, which auto-regenerates from `grammar.js`

**Prevention measures implemented**:
1. ✅ Updated `scripts/build-demo.sh` to automatically build tree-sitter WASM (Step 4/4)
2. ✅ Added clear documentation that grammar changes require WASM rebuild
3. 📝 TODO: Consider adding a build check to CI that compares grammar.js timestamp with WASM timestamp
4. 📝 TODO: Consider adding tree-sitter WASM build to CI/CD pipeline before deployment

**Note**: If you modify `tree-sitter-mml/grammar.js`, always rebuild the WASM with `scripts/build-demo.sh` or manually run `npx tree-sitter build-wasm` in the `tree-sitter-mml/` directory.

## Implementation Notes

The key difference from the original approach in PR 47:
- **PR 47 (Rejected)**: JavaScript had `extractTokens()` function duplicating Rust logic
- **This Implementation (Option A)**: JavaScript only converts parse tree to JSON, all token extraction is in Rust

This ensures that the token extraction logic is maintained in a single place (Rust), making it easier to maintain and avoiding synchronization issues.
