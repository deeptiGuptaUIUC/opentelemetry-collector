#!/bin/bash

# Proof of Concept: Rust -> Go Shared Library -> Go Plugin
# This script demonstrates the plugin loading issue in isolation

set -e

# Use specific Go version
export GOROOT=/home/jmacd/go-1.24.1
GO_BIN=/home/jmacd/go-1.24.1/bin/go

echo "Go-Rust-Plugin Proof of Concept Build"
echo "====================================="

# Navigate to the PoC directory
cd "$(dirname "$0")"

# Clean previous builds
echo "Step 1: Cleaning previous builds..."
rm -f go-main/libmain.so go-main/libmain.h
rm -f go-plugin/plugin.so
rm -rf rust-loader/target
echo "✓ Cleaned"

echo ""
echo "Step 2: Building Go shared library..."
echo "Running: cd go-main && $GO_BIN build -buildmode=c-shared -o libmain.so ."
(cd go-main && $GO_BIN build -buildmode=c-shared -o libmain.so .)
echo "✓ Go shared library built: go-main/libmain.so"

echo ""
echo "Step 3: Building Go plugin..."
echo "Running: cd go-plugin && $GO_BIN build -buildmode=plugin -ldflags='-linkmode=external' -o plugin.so ."
(cd go-plugin && $GO_BIN build -buildmode=plugin -ldflags="-linkmode=external" -o plugin.so .)
echo "✓ Go plugin built: go-plugin/plugin.so"

echo ""
echo "Step 4: Building Rust loader..."
echo "Running: cd rust-loader && cargo build --release"
(cd rust-loader && cargo build --release)
echo "✓ Rust loader built: rust-loader/target/release/rust-loader"

echo ""
echo "Build completed successfully!"
echo "=========================="
echo ""
echo "Files created:"
ls -la go-main/libmain.so go-plugin/plugin.so rust-loader/target/release/rust-loader
echo ""
echo "Run the proof of concept with:"
echo "  ./rust-loader/target/release/rust-loader"
./rust-loader/target/release/rust-loader
