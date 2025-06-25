// Example of using the CollectorLoader programmatically

use otel_collector_loader::CollectorLoader;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example: Programmatic OpenTelemetry Collector Loader");
    
    let mut loader = CollectorLoader::new();
    
    // Load the shared library
    loader.load_library("./libotelcorecol.so")?;
    
    // Start the collector in a thread
    let handle = loader.run_collector()?;
    
    // Do other work while the collector runs
    for i in 1..=5 {
        thread::sleep(Duration::from_secs(1));
        println!("Doing other work... step {}/5", i);
        
        if loader.is_running() {
            println!("Collector is still running");
        } else {
            println!("Collector has stopped");
            break;
        }
    }
    
    // Wait for the collector to finish
    match handle.join() {
        Ok(_) => println!("Collector completed successfully"),
        Err(e) => eprintln!("Collector thread panicked: {:?}", e),
    }
    
    Ok(())
}
