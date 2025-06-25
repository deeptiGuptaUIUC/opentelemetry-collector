use libloading::{Library, Symbol};
use std::ffi::{CString, CStr};
use std::path::Path;
use std::os::raw::c_char;

// Define the function signature for the exported Main function from Go
type MainFunc = unsafe extern "C" fn();

// Function to set command line arguments in Go
// This mimics what the Go main() function does
fn set_go_args() -> Result<(), Box<dyn std::error::Error>> {
    // We need to simulate os.Args being set to: ["otelcorecol", "--config=./test.yaml"]
    // Since we can't directly modify os.Args from Rust, we'll need to modify the Go code
    // or use a different approach
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rust OpenTelemetry Collector Loader");
    
    // Path to the shared library
    let lib_path = "./libotelcorecol.so";
    
    if !Path::new(lib_path).exists() {
        eprintln!("Error: Shared library not found at {}", lib_path);
        eprintln!("Please build the Go shared library first with:");
        eprintln!("cd /home/deeguptwsl/repos/opentelemetry-collector");
        eprintln!("cd cmd/otelcorecol && CGO_ENABLED=1 go build -buildmode=c-shared -o ../../rust-loader/libotelcorecol.so .");
        return Err("Shared library not found".into());
    }
    
    // Check if config file exists
    if !Path::new("./test.yaml").exists() {
        eprintln!("Error: Configuration file './test.yaml' not found");
        eprintln!("Please ensure test.yaml exists in the current directory");
        return Err("Configuration file not found".into());
    }
    
    println!("Loading shared library: {}", lib_path);
    
    // Load the shared library
    let lib = unsafe { Library::new(lib_path)? };
    
    // Get the Main function symbol
    let main_func: Symbol<MainFunc> = unsafe { lib.get(b"Main")? };
    
    println!("Successfully loaded Main function from shared library");
    println!("Starting OpenTelemetry Collector...");
    
    // Set up arguments (this will be handled in the modified Go main function)
    set_go_args()?;
    
    // Call the Main function directly
    unsafe {
        main_func();
    }
    
    println!("OpenTelemetry Collector Main function completed");
    
    // Keep the library loaded until the end
    std::mem::forget(lib);
    
    Ok(())
}
