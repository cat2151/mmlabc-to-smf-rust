# MML to SMF Library Demo

This directory contains a minimal demonstration of how to use `mmlabc-to-smf-wasm` as a library in web applications.

## Purpose

This demo is designed for developers who want to integrate MML to MIDI conversion functionality into their own projects. It shows the basic API usage and provides example code.

## Contents

- `index.html` - Interactive documentation with code examples and a working demo

## Usage

The library provides a simple API:

1. Parse MML text with `web-tree-sitter` to get a parse tree
2. Convert the parse tree to MIDI with `parse_tree_json_to_smf()`

See `index.html` for detailed examples and API documentation.

## Install via npm (GitHub)

You can try this demo without cloning the repository by installing it from GitHub:

```bash
npm install github:cat2151/mmlabc-to-smf-rust
cd node_modules/mmlabc-to-smf-rust/demo-library
npm install
npm run build   # builds WASM and supporting assets (Rust + wasm-pack required)
npm run serve   # opens /demo-library/ via http-server
```

## Testing Locally

From the repository root:

```bash
# Build the WASM modules (same as npm run build inside demo-library)
npm --prefix demo-library install
npm --prefix demo-library run build

# Start a local server
python3 -m http.server 8000

# Open in browser
# http://localhost:8000/demo-library/
```

## Deployment

This demo is automatically deployed to GitHub Pages along with the main demo.
