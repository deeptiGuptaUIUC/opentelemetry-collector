use libloading::{Library, Symbol};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

fn main() {
    println!("=== Rust Loader: Starting ===");
    
    // Load the Go shared library
    let lib_path = "./go-main/libmain.so";
    println!("Loading Go shared library: {}", lib_path);
    
    let lib = match unsafe { Library::new(lib_path) } {
        Ok(lib) => {
            println!("✓ Go shared library loaded successfully");
            lib
        }
        Err(e) => {
            eprintln!("✗ Failed to load Go shared library: {}", e);
            std::process::exit(1);
        }
    };
    
    // Get runtime info first
    let get_runtime_info: Symbol<unsafe extern "C" fn() -> *mut c_char> = unsafe {
        match lib.get(b"GetRuntimeInfo") {
            Ok(func) => func,
            Err(e) => {
                eprintln!("✗ Failed to find GetRuntimeInfo function: {}", e);
                std::process::exit(1);
            }
        }
    };
    
    let info_ptr = unsafe { get_runtime_info() };
    let info_cstr = unsafe { CStr::from_ptr(info_ptr) };
    println!("Go runtime info: {}", info_cstr.to_string_lossy());
    
    // Load and call the plugin through the Go library
    let load_plugin: Symbol<unsafe extern "C" fn(*const c_char) -> c_int> = unsafe {
        match lib.get(b"LoadAndCallPlugin") {
            Ok(func) => func,
            Err(e) => {
                eprintln!("✗ Failed to find LoadAndCallPlugin function: {}", e);
                std::process::exit(1);
            }
        }
    };
    
    let plugin_path = "./go-plugin/plugin.so";
    println!("Calling Go library to load plugin: {}", plugin_path);
    
    let plugin_path_cstring = CString::new(plugin_path).unwrap();
    let result = unsafe { load_plugin(plugin_path_cstring.as_ptr()) };
    
    if result == 0 {
        println!("✓ Plugin loading and execution successful!");
        println!("=== Rust Loader: Success! ===");
    } else {
        eprintln!("✗ Plugin loading failed with code: {}", result);
        println!("=== Rust Loader: Failed ===");
        std::process::exit(1);
    }
}
