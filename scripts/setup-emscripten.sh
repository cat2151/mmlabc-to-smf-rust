#!/bin/bash
# Script to setup Emscripten environment for building WASM

set -e

EMSDK_DIR="${EMSDK_DIR:-/tmp/emsdk}"
EMSDK_VERSION="${EMSDK_VERSION:-latest}"

echo "Setting up Emscripten SDK..."

# Clone emsdk if not already present
if [ ! -d "$EMSDK_DIR" ]; then
    echo "Cloning emsdk..."
    git clone https://github.com/emscripten-core/emsdk.git "$EMSDK_DIR"
fi

cd "$EMSDK_DIR"

# Install and activate Emscripten
echo "Installing Emscripten $EMSDK_VERSION..."
./emsdk install "$EMSDK_VERSION"

echo "Activating Emscripten $EMSDK_VERSION..."
./emsdk activate "$EMSDK_VERSION"

# Source the environment
echo "Sourcing Emscripten environment..."
source ./emsdk_env.sh

# Verify installation
echo "Verifying Emscripten installation..."
emcc --version

echo ""
echo "Emscripten setup complete!"
echo "To use Emscripten in your current shell, run:"
echo "  source $EMSDK_DIR/emsdk_env.sh"
