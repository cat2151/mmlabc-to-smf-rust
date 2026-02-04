#!/usr/bin/env bash
# Transform demo/index.html paths for deployment to project root
# This script adjusts relative paths from demo/ context to root context

set -e

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <input-file> <output-file>"
    exit 1
fi

INPUT_FILE="$1"
OUTPUT_FILE="$2"

# Validate input file exists
if [ ! -f "$INPUT_FILE" ]; then
    echo "Error: Input file '$INPUT_FILE' not found"
    exit 1
fi

# Transform paths using sed with double quotes for clarity
sed \
  -e "s|from './web-tree-sitter.js'|from './demo/web-tree-sitter.js'|g" \
  -e "s|from '../mmlabc-to-smf-wasm/|from './mmlabc-to-smf-wasm/|g" \
  -e "s|load('../tree-sitter-mml/|load('./tree-sitter-mml/|g" \
  "$INPUT_FILE" > "$OUTPUT_FILE"

# Validate output file was created and is non-empty
if [ ! -s "$OUTPUT_FILE" ]; then
    echo "Error: Output file '$OUTPUT_FILE' is empty or was not created"
    exit 1
fi

# Verify all three transformations were applied
VALIDATION_FAILED=0
if ! grep -q "from './demo/web-tree-sitter.js'" "$OUTPUT_FILE"; then
    echo "Warning: Expected transformation 'from ./demo/web-tree-sitter.js' not found in output"
    VALIDATION_FAILED=1
fi
if ! grep -q "from './mmlabc-to-smf-wasm/" "$OUTPUT_FILE"; then
    echo "Warning: Expected transformation 'from ./mmlabc-to-smf-wasm/' not found in output"
    VALIDATION_FAILED=1
fi
if ! grep -q "load('./tree-sitter-mml/" "$OUTPUT_FILE"; then
    echo "Warning: Expected transformation 'load(./tree-sitter-mml/' not found in output"
    VALIDATION_FAILED=1
fi

if [ "$VALIDATION_FAILED" -eq 1 ]; then
    echo "Error: Some path transformations may not have been applied correctly"
    exit 1
fi

echo "✓ Transformed $INPUT_FILE -> $OUTPUT_FILE"
