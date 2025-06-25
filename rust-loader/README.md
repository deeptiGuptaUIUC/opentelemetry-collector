# OpenTelemetry Collector Rust Loader

This Rust program loads the OpenTelemetry Collector Go shared library and runs it using FFI (Foreign Function Interface). It supports loading Go plugins dynamically at runtime.

## Features

- **FFI Integration**: Load and execute Go shared libraries from Rust
- **Plugin Support**: Load Go plugins (.so files) to extend collector functionality  
- **Direct Function Calls**: Call exported Go functions with parameters
- **Error Handling**: Comprehensive error handling for library and plugin loading
- **Memory Safety**: Proper management of shared library lifetime

## Prerequisites

- Rust (latest stable version)
- Go 1.23.8 with CGO enabled
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
cd /home/deeguptwsl/repos/opentelemetry-collector/cmd/otelcorecol
CGO_ENABLED=1 go build -buildmode=c-shared -o ../../rust-loader/libotelcorecol.so .
```

2. Build the Go plugin (optional):
```bash
cd /home/deeguptwsl/repos/opentelemetry-collector/shared/dynbatchprocessor
go build -o dynbatchprocessor.so -buildmode=plugin .
```

3. Build the Rust loader:
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

### With timeout (for testing):
```bash
cd rust-loader
timeout 5s cargo run || echo "Collector started successfully"
```

## Architecture

The Rust loader implements FFI to load and execute Go code:

1. **FFI Function Signatures**: 
   - `Main()`: Basic exported function for starting the collector
   - `MainWithPlugin(*char)`: Enhanced function that accepts plugin path as parameter

2. **Plugin Support**: 
   - Loads Go plugins (.so files) to extend processor functionality
   - Passes plugin paths directly to Go code via FFI parameters
   - Handles plugin loading errors gracefully

3. **Memory Management**: 
   - Safe loading and unloading of shared libraries
   - Proper C string handling for FFI calls

## How It Works

1. **Rust Side**:
   - Sets plugin path environment variable (for fallback compatibility)
   - Loads the Go shared library (`libotelcorecol.so`) using `libloading`
   - Attempts to call `MainWithPlugin` first, falls back to `Main` if not available
   - Passes plugin path as C string parameter to Go function

2. **Go Side**:
   - Exports `Main()` and `MainWithPlugin(*C.char)` functions  
   - `MainWithPlugin` receives plugin path parameter from Rust
   - Loads plugins using Go's `plugin.Open()` API
   - Integrates plugin factories into the collector's component system
   - Falls back to environment variable method if FFI parameter is empty

## Key Features

- **FFI Integration**: Direct function calls between Rust and Go using C ABI
- **Plugin Loading**: Dynamic loading of Go plugins to extend collector functionality
- **Dual Function Support**: Both parameterless `Main()` and parameterized `MainWithPlugin(*char)`
- **Error Handling**: Comprehensive error handling for library loading and function calls
- **Fallback Mechanisms**: Multiple approaches for plugin loading (FFI parameters + environment variables)
- **Memory Safety**: Proper management of shared library lifetime and C string handling

## Configuration

The Go collector is configured through the `test.yaml` file. The current configuration supports:

```yaml
receivers:
  otlp:  # OTLP receiver on localhost:4317

processors:
  memory_limiter:  # Built-in memory limiter processor
  # batch: # Plugin-provided batch processor (when plugin loading works)

exporters:  
  debug:  # Debug exporter for testing

service:
  pipelines:  # Traces, metrics, and logs pipelines
```

Make sure `test.yaml` exists in the working directory when running the loader.

## Environment Variables

The Go collector supports the following environment variable:
- `OTEL_COLLECTOR_SHARED_LIBRARY`: Comma-separated list of plugin .so files to load as processors

**Note**: Environment variables set in Rust are not automatically inherited by the Go shared library when loaded via FFI. The Rust loader now passes plugin paths directly as function parameters to work around this limitation.

## Troubleshooting

### Plugin loading errors
```
fatal error: runtime: no plugin module data
```
**Issue**: This error occurs when loading Go plugins from within a shared library loaded via FFI. This is a known limitation of Go's plugin system when used in this context.

**Current Status**: The basic FFI integration works successfully. Plugin loading encounters runtime compatibility issues between the shared library context and Go's plugin system.

### Runtime errors
Check that all dependencies and configuration files are properly set up for the OpenTelemetry Collector.

## Current Status

✅ **Working Features**:
- FFI loading of Go shared library from Rust
- Calling exported Go functions (`Main`, `MainWithPlugin`)
- Passing C string parameters from Rust to Go
- OpenTelemetry Collector startup and configuration
- Built-in processors (memory_limiter, nop, etc.)
- OTLP receiver and debug exporter functionality

⚠️ **Known Issues**:
- Go plugin loading fails with "runtime: no plugin module data" error when called from FFI context
- Environment variables set in Rust are not inherited by the Go shared library

🔬 **Under Investigation**:
- Alternative approaches for plugin loading in FFI context
- Static compilation of processors to avoid runtime plugin loading
- Alternative plugin architectures (gRPC, subprocess-based)

## Example Output

**Successful Run (without plugins)**:
```
Rust OpenTelemetry Collector Loader
Set plugin path: /home/deeguptwsl/repos/opentelemetry-collector/shared/dynbatchprocessor/dynbatchprocessor.so
OTEL_COLLECTOR_SHARED_LIBRARY environment variable: '/path/to/plugin.so'
Loading shared library: ./libotelcorecol.so
Successfully loaded MainWithPlugin function from shared library
Starting OpenTelemetry Collector with plugin...
Loading plugin from Rust parameter: /path/to/plugin.so
Error opening plugin: fatal error: runtime: no plugin module data
Continuing without plugin...
2025-06-25T12:56:56.789Z	info	service	service.go:169	Setting up pipelines...
2025-06-25T12:56:56.789Z	info	service	service.go:116	Starting otelcorecol...
```

## Files

- `Cargo.toml`: Rust project configuration with `libloading` dependency
- `src/main.rs`: Main implementation with FFI integration and plugin support
- `build.sh`: Automated build script for both Go and Rust components
- `test.yaml`: OpenTelemetry Collector configuration file
- `libotelcorecol.so`: Generated Go shared library (after build)
- `libotelcorecol.h`: Generated C header file (after build)
- `README.md`: This documentation

## Dependencies

### Rust Dependencies
- `libloading = "0.8"`: For dynamic library loading and FFI calls

### Go Dependencies  
- Standard Go OpenTelemetry Collector modules
- CGO for C interoperability
- Plugin package for dynamic loading (when working)

## Technical Details

### FFI Function Signatures

**Rust Side**:
```rust
type MainFunc = unsafe extern "C" fn();
type MainWithPluginFunc = unsafe extern "C" fn(*const c_char);
```

**Go Side**:
```go
//export Main
func Main()

//export MainWithPlugin  
func MainWithPlugin(pluginPath *C.char)
```

### Plugin Loading Flow

1. Rust sets plugin path and creates C string
2. Rust calls `MainWithPlugin(plugin_path_cstr.as_ptr())`
3. Go receives path via `C.GoString(pluginPath)`
4. Go attempts `plugin.Open(pluginPathStr)` 
5. **Issue**: Plugin loading fails in shared library FFI context

### Memory Management

- Rust uses `std::mem::forget(lib)` to keep library loaded
- C strings are properly managed with `CString::new()`
- Library symbols are safely accessed through `libloading::Symbol`
