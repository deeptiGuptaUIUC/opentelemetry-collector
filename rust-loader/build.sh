#!/bin/bash

# Build script for the OpenTelemetry Collector Rust Loader

set -e  # Exit on any error

echo "Building OpenTelemetry Collector Rust Loader"
echo "============================================"

# Navigate to the OpenTelemetry Collector root directory
cd "$(dirname "$0")/.."

# Build the Go shared library
echo "Step 1: Updating Go dependencies..."
echo "Running: go mod download"
go mod download

echo ""
echo "Step 2: Building Go shared library..."
echo "Running: cd cmd/otelcorecol && CGO_ENABLED=1 go build -buildmode=c-shared -o ../../rust-loader/libotelcorecol.so ."

cd cmd/otelcorecol && CGO_ENABLED=1 go build -buildmode=c-shared -o ../../rust-loader/libotelcorecol.so . && cd ../..

if [ $? -eq 0 ]; then
    echo "✓ Go shared library built successfully"
else
    echo "✗ Failed to build Go shared library"
    exit 1
fi

# Build the Rust loader
echo ""
echo "Step 3: Building Rust loader..."
cd rust-loader

echo "Running: cargo build --release"
cargo build --release

if [ $? -eq 0 ]; then
    echo "✓ Rust loader built successfully"
else
    echo "✗ Failed to build Rust loader"
    exit 1
fi

echo ""
echo "Build completed successfully!"
echo "To run the loader:"
echo "  cd rust-loader"
echo "  cargo run"
echo ""
echo "Or run the release binary:"
echo "  ./target/release/otel-collector-loader"
