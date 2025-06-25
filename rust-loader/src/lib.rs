use libloading::{Library, Symbol};
use std::ffi::c_void;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::signal;

// Define the function signature for the exported Main function from Go
type MainFunc = unsafe extern "C" fn();

pub struct CollectorLoader {
    library: Option<Library>,
    is_running: Arc<Mutex<bool>>,
}

impl CollectorLoader {
    pub fn new() -> Self {
        Self {
            library: None,
            is_running: Arc::new(Mutex::new(false)),
        }
    }
    
    pub fn load_library(&mut self, lib_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(lib_path).exists() {
            return Err(format!("Shared library not found at {}", lib_path).into());
        }
        
        println!("Loading shared library: {}", lib_path);
        let lib = unsafe { Library::new(lib_path)? };
        self.library = Some(lib);
        
        Ok(())
    }
    
    pub fn run_collector(&mut self) -> Result<thread::JoinHandle<()>, Box<dyn std::error::Error>> {
        let lib = self.library.as_ref()
            .ok_or("Library not loaded. Call load_library() first.")?;
            
        // Get the Main function symbol
        let main_func: Symbol<MainFunc> = unsafe { lib.get(b"Main")? };
        
        // Clone the function for the thread
        let main_func_ptr = *main_func as *const ();
        let is_running = Arc::clone(&self.is_running);
        
        println!("Starting OpenTelemetry Collector in thread...");
        
        let handle = thread::spawn(move || {
            {
                let mut running = is_running.lock().unwrap();
                *running = true;
            }
            
            // Call the Main function
            unsafe {
                let func: MainFunc = std::mem::transmute(main_func_ptr);
                func();
            }
            
            {
                let mut running = is_running.lock().unwrap();
                *running = false;
            }
            
            println!("OpenTelemetry Collector Main function completed");
        });
        
        Ok(handle)
    }
    
    pub fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rust OpenTelemetry Collector Loader (Async Version)");
    
    let mut loader = CollectorLoader::new();
    
    // Path to the shared library
    let lib_path = "./libotelcorecol.so";
    
    // Load the library
    if let Err(e) = loader.load_library(lib_path) {
        eprintln!("Error loading library: {}", e);
        eprintln!("Please build the Go shared library first with:");
        eprintln!("cd /home/deeguptwsl/repos/opentelemetry-collector");
        eprintln!("CGO_ENABLED=1 go build -buildmode=c-shared -o rust-loader/libotelcorecol.so cmd/otelcorecol/main.go");
        return Err(e);
    }
    
    // Start the collector
    let handle = loader.run_collector()?;
    
    // Wait for either the collector to finish or a shutdown signal
    tokio::select! {
        result = tokio::task::spawn_blocking(move || handle.join()) => {
            match result {
                Ok(Ok(_)) => println!("Collector thread completed successfully"),
                Ok(Err(e)) => eprintln!("Collector thread panicked: {:?}", e),
                Err(e) => eprintln!("Failed to join collector thread: {:?}", e),
            }
        }
        _ = signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down...");
            // Note: The Go Main function might not respond to signals immediately
            // You might need to implement proper shutdown signaling
        }
    }
    
    Ok(())
}
