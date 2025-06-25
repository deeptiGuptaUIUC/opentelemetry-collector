# OpenTelemetry Collector Rust Loader

This Rust program loads the OpenTelemetry Collector Go shared library and runs it in a separate thread using FFI (Foreign Function Interface).

## Prerequisites

- Rust (latest stable version)
- Go with CGO enabled
- Linux environment (for shared library support)

## Building

### Option 1: Use the build script (Recommended)

```bash
# From the rust-loader directory
./build.sh
```

### Option 2: Manual build

1. Build the Go shared library:
```bash
# From the opentelemetry-collector root directory
cd /home/deeguptwsl/repos/opentelemetry-collector
CGO_ENABLED=1 go build -buildmode=c-shared -o rust-loader/libotelcorecol.so cmd/otelcorecol/main.go
```

2. Build the Rust loader:
```bash
cd rust-loader
cargo build --release
```

## Running

### Development mode:
```bash
cd rust-loader
cargo run
```

### Release mode:
```bash
cd rust-loader
./target/release/otel-collector-loader
```

## Architecture

The Rust loader consists of two main components:

1. **Simple version (`main.rs`)**: A straightforward implementation that loads the shared library and calls the Main function in a thread.

2. **Advanced version (`lib.rs`)**: An async implementation with better error handling and control features.

## Key Features

- **Thread Safety**: Runs the Go collector in a separate thread to avoid blocking
- **Error Handling**: Comprehensive error handling for library loading and function calls
- **Async Support**: Tokio-based async runtime for better control
- **Signal Handling**: Supports Ctrl+C for graceful shutdown
- **Memory Safety**: Proper management of shared library lifetime

## Configuration

The Go collector is configured through the `test.yaml` file (as set in the Go code). Make sure this file exists in the working directory when running the loader.

## Environment Variables

The Go collector supports the following environment variable:
- `OTEL_COLLECTOR_SHARED_LIBRARY`: Comma-separated list of additional .so files to load as processors

## Troubleshooting

### Library not found error
Make sure the shared library is built and located at `./libotelcorecol.so` relative to the Rust binary.

### Symbol not found error
Ensure the Go code is compiled with the `//export Main` directive and CGO is enabled.

### Runtime errors
Check that all dependencies and configuration files are properly set up for the OpenTelemetry Collector.

## Files

- `Cargo.toml`: Rust project configuration
- `src/main.rs`: Simple synchronous implementation
- `src/lib.rs`: Advanced async implementation with better control
- `build.sh`: Automated build script
- `README.md`: This documentation

## Dependencies

- `libloading`: For dynamic library loading
- `tokio`: For async runtime and signal handling
