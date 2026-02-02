# Build Scripts

This directory contains scripts for building and deploying the project.

## build-demo.sh

Builds the MML to SMF WASM demo for GitHub Pages deployment.

### Prerequisites

- Rust toolchain (with cargo)
- wasm-pack
- Node.js and npm

### Installation

Install wasm-pack if not already installed:

```bash
# Using cargo
cargo install wasm-pack

# Or using the installer (if network access available)
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### Usage

Run from the repository root:

```bash
./scripts/build-demo.sh
```

Or from anywhere:

```bash
cd /path/to/mmlabc-to-smf-rust
./scripts/build-demo.sh
```

### What It Does

1. **Installs demo npm dependencies**: Runs `npm install` in the `demo/` directory to get web-tree-sitter
2. **Copies web-tree-sitter files**: Copies `web-tree-sitter.js` and `web-tree-sitter.wasm` from node_modules to the demo directory
3. **Builds Rust WASM module**: Uses wasm-pack to compile the Rust code to WASM (creates `mmlabc-to-smf-wasm/pkg/`)
4. **Verifies tree-sitter grammar**: Checks that `tree-sitter-mml.wasm` exists

### Output

After successful build:
- `demo/web-tree-sitter.js` - Web Tree-sitter parser JavaScript
- `demo/web-tree-sitter.wasm` - Web Tree-sitter parser WASM
- `mmlabc-to-smf-wasm/pkg/` - Rust WASM module and JavaScript bindings
- `tree-sitter-mml/tree-sitter-mml.wasm` - MML grammar for tree-sitter (pre-existing)

### Testing Locally

After building, test the demo:

```bash
# Start a local web server from repository root
python3 -m http.server 8000

# Open in browser
# http://localhost:8000/demo/
```

### Troubleshooting

**Error: wasm-pack is not installed**
- Install wasm-pack using cargo: `cargo install wasm-pack`

**Error: tree-sitter-mml.wasm not found**
- This is a warning, not an error. The file should already exist in the repository.
- If needed, rebuild it: `cd tree-sitter-mml && npx tree-sitter build-wasm`

### GitHub Actions

This script is automatically called by the `.github/workflows/deploy-github-pages.yml` workflow on push to main branch.
