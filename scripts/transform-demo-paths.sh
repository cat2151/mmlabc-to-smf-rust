#!/bin/bash
# Transform demo/index.html paths for deployment to project root
# This script adjusts relative paths from demo/ context to root context

set -e

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <input-file> <output-file>"
    exit 1
fi

INPUT_FILE="$1"
OUTPUT_FILE="$2"

# Transform paths using sed
sed \
  -e 's|from '"'"'./web-tree-sitter.js'"'"'|from '"'"'./demo/web-tree-sitter.js'"'"'|g' \
  -e 's|from '"'"'../mmlabc-to-smf-wasm/|from '"'"'./mmlabc-to-smf-wasm/|g' \
  -e 's|load('"'"'../tree-sitter-mml/|load('"'"'./tree-sitter-mml/|g' \
  "$INPUT_FILE" > "$OUTPUT_FILE"

echo "✓ Transformed $INPUT_FILE -> $OUTPUT_FILE"
