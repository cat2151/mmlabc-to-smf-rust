#!/bin/bash
# Build script for WASM module
# This script builds the WASM module and copies it to the demo directory

set -e  # Exit on error

echo "Building WASM module..."
cargo build --target wasm32-wasip1 --lib --release

echo "Copying WASM file to demo directory..."
mkdir -p demo
cp target/wasm32-wasip1/release/mmlabc_to_smf.wasm demo/

WASM_SIZE=$(du -h demo/mmlabc_to_smf.wasm | cut -f1)
echo "✓ Build complete! WASM module size: $WASM_SIZE"
echo ""
echo "To test the demo:"
echo "  1. Run: python3 demo/serve.py"
echo "  2. Open: http://localhost:8000"
