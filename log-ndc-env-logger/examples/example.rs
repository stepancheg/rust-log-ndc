#[macro_use]
extern crate log;

use std::thread;

// Try it with command: `RUST_LOG=debug cargo run --example example`
fn main() {
    // Initialize logger
    // (could be as simple as `ndc_log_env_logger::init()` for default settings)
    log_ndc_env_logger::init();

    // No NDC here
    info!("app started");

    thread::spawn(|| {
        log_ndc::set("server");
        // Should print `[server] started`
        info!("started");
    }).join().unwrap();

    thread::spawn(|| {
        log_ndc::set(format!("client {}", 17));
        error!("ah oh");
    }).join().unwrap();
}
