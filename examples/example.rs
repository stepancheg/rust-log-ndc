#[macro_use]
extern crate log_ndc;

use std::thread;

// Try it with command: `cargo run --example example`
fn main() {
    // Initialize logger
    // (could be as simple as `env_logger::init`)
    let mut builder = env_logger::Builder::new();
    builder.parse("debug");
    builder.init();

    // There's not NDC here, this call is equivalent to `info!(...)` call.
    ndc_info!("app started");

    thread::spawn(|| {
        log_ndc::set("server");
        ndc_info!("started");
    }).join().unwrap();

    thread::spawn(|| {
        log_ndc::set(format!("client {}", 1));
        ndc_error!("ah oh");
    }).join().unwrap();
}
