#[macro_use]
extern crate log_ndc;

use std::thread;


fn main() {
    let mut builder = env_logger::Builder::new();
    builder.parse("debug");
    builder.init();

    ndc_info!("app started");

    thread::spawn(|| {
        log_ndc::set_static_str("server");
        ndc_info!("started");
    }).join().unwrap();

    thread::spawn(|| {
        log_ndc::set_static_str("client");
        ndc_error!("ah oh");
    }).join().unwrap();
}
