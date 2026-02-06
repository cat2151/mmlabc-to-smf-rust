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

# Step 2: Copy web-tree-sitter files to demo
echo ""
echo "Step 2/4: Copying web-tree-sitter files..."
cp node_modules/web-tree-sitter/web-tree-sitter.js web-tree-sitter.js
cp node_modules/web-tree-sitter/web-tree-sitter.wasm web-tree-sitter.wasm

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

# Install tree-sitter-cli if not already installed
if [ ! -d "node_modules" ]; then
    echo "Installing tree-sitter-cli..."
    npm install
fi

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
echo "  - demo/ (HTML, JS, copied web-tree-sitter files)"
echo "  - mmlabc-to-smf-wasm/pkg/ (Rust WASM module)"
echo "  - tree-sitter-mml/ (tree-sitter-mml.wasm)"
echo ""
echo "To test locally, run from repository root:"
echo "  python3 -m http.server 8000"
echo "Then open: http://localhost:8000/"
