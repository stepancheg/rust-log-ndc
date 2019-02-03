#[macro_use]
extern crate log_ndc;

use std::thread;


fn main() {
    let mut builder = env_logger::Builder::new();
    builder.parse("debug");
    builder.init();

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
