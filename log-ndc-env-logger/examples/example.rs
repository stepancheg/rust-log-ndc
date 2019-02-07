#[macro_use]
extern crate log;

use std::thread;

// Try it with command: `RUST_LOG=debug cargo run --example example`
fn main() {
    // Initialize logger
    log_ndc_env_logger::init();

    // Outputs:
    // INFO 2019-02-07T01:08:07Z: example: app started
    info!("app started");

    thread::spawn(|| {
        log_ndc::set("server");
        // Outputs:
        // INFO 2019-02-07T01:08:07Z: example: [server] started
        info!("started");
    }).join().unwrap();

    thread::spawn(|| {
        log_ndc::set(format!("client {}", 17));
        // Outputs:
        // ERROR 2019-02-07T01:08:07Z: example: [client 17] ah oh
        error!("ah oh");
    }).join().unwrap();
}
