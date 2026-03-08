#!/bin/bash
set -e

# Build script for GitHub Pages demo
# This script can be run standalone or called by GitHub Actions

echo "=== Building MML to SMF WASM Demo ==="

# Get the root directory (parent of scripts/)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

echo "Root directory: ${ROOT_DIR}"

# Step 1: Install npm dependencies for demo
echo ""
echo "Step 1/4: Installing demo npm dependencies..."
cd "${ROOT_DIR}/demo"
npm install

# Step 2: Copy web-tree-sitter, bundle Tone.js, copy web-ym2151 and smf-to-ym2151log-rust, compile TypeScript
echo ""
echo "Step 2/4: Copying web-tree-sitter, bundling Tone.js, copying library files, and compiling TypeScript..."
cp node_modules/web-tree-sitter/web-tree-sitter.js web-tree-sitter.js
cp node_modules/web-tree-sitter/web-tree-sitter.wasm web-tree-sitter.wasm
# Bundle Tone.js and its dependencies into a single ESM file for browsers
rm -rf tone
mkdir -p tone
npx esbuild node_modules/tone/build/esm/index.js --bundle --format=esm --platform=browser --outfile=tone/index.js
# Copy web-ym2151 WASM files (Emscripten-compiled C code for YM2151 emulation)
cp node_modules/web-ym2151/ym2151.js ym2151.js
cp node_modules/web-ym2151/ym2151.wasm ym2151.wasm
echo "✓ Copied web-ym2151 files (ym2151.js, ym2151.wasm)"
# Copy smf-to-ym2151log-rust WASM pkg (downloaded by postinstall from GitHub Pages)
rm -rf "${ROOT_DIR}/demo/smf-to-ym2151log-wasm"
mkdir -p "${ROOT_DIR}/demo/smf-to-ym2151log-wasm"
cp node_modules/smf-to-ym2151log-rust/pkg/smf_to_ym2151log.js "${ROOT_DIR}/demo/smf-to-ym2151log-wasm/"
cp node_modules/smf-to-ym2151log-rust/pkg/smf_to_ym2151log_bg.wasm "${ROOT_DIR}/demo/smf-to-ym2151log-wasm/"
echo "✓ Copied smf-to-ym2151log-rust pkg files"
# Compile TypeScript source modules into a single bundled app.js
npx esbuild src/main.ts --bundle --format=esm --platform=browser \
    --external:./web-tree-sitter.js \
    --external:./tone/index.js \
    '--external:../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js' \
    '--external:./smf-to-ym2151log-wasm/smf_to_ym2151log.js' \
    --outfile=app.js
echo "✓ Copied web-tree-sitter files, bundled Tone.js, and compiled TypeScript"

# Step 3: Build the WASM module
echo ""
echo "Step 3/4: Building Rust WASM module..."
cd "${ROOT_DIR}/mmlabc-to-smf-wasm"

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed"
    echo "Please install it with: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

wasm-pack build --target web --release

# Step 4: Build tree-sitter-mml.wasm
echo ""
echo "Step 4/4: Building tree-sitter-mml.wasm..."
cd "${ROOT_DIR}/tree-sitter-mml"

# Ensure tree-sitter-cli is installed
echo "Installing/updating tree-sitter-cli..."
npm install

# Build the WASM grammar
echo "Running tree-sitter build-wasm..."
npx tree-sitter build-wasm

if [ -f "${ROOT_DIR}/tree-sitter-mml/tree-sitter-mml.wasm" ]; then
    echo "✓ tree-sitter-mml.wasm built successfully"
else
    echo "Error: Failed to build tree-sitter-mml.wasm"
    exit 1
fi

echo ""
echo "=== Build completed successfully! ==="
echo ""
echo "Demo files are ready in:"
echo "  - demo/ (HTML, JS, copied web-tree-sitter and Tone.js files)"
echo "  - mmlabc-to-smf-wasm/pkg/ (Rust WASM module)"
echo "  - tree-sitter-mml/ (tree-sitter-mml.wasm)"
echo ""
echo "To test locally, run from repository root:"
echo "  python3 -m http.server 8000"
echo "Then open: http://localhost:8000/"
