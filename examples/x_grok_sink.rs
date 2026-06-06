use log::{error, info, warn};
use prologger::ProLoggerBuilder;
use std::env;
use std::time::Duration;

fn main() {
    // Read the API key from the environment
    let api_key = match env::var("XAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: XAI_API_KEY environment variable is not set.");
            eprintln!("Run this example with:");
            eprintln!("XAI_API_KEY=your_key cargo run --example x_grok_sink --features=\"full\"");
            return;
        }
    };

    println!("Initializing Prologger with XGrokSink...");

    // Build the logger
    // We add a console sink so we can see the logs locally as well,
    // and we add the x_grok sink.
    // Crucially, we wrap it in `with_async` so the HTTP requests don't block.
    ProLoggerBuilder::new()
        .with_console_default()
        .with_x_grok(api_key)
        .with_async(10_000)
        .init()
        .unwrap();

    info!("This is a standard info log. It will be sent to the Grok API!");
    warn!("You can also send warnings to the analyzer.");
    error!("Critical errors can be ingested and analyzed silently.");

    // Because the sink runs in a background thread, we need to wait a moment 
    // for the HTTP requests to complete before the program exits and drops the channel.
    // (In a real long-running server, this isn't necessary, but for a short example it is).
    println!("Waiting a few seconds for logs to be sent to xAI...");
    std::thread::sleep(Duration::from_secs(3));
    
    println!("Done!");
}
