// `cargo test --benches` and `#[feature(test)]` work only in nightly
#![cfg(rustc_nightly)]
#![feature(test)]

extern crate test;

#[macro_use]
extern crate log;
extern crate log_ndc;

use std::sync::Once;
use test::Bencher;

fn init_once() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| log_ndc_env_logger::init());
}

#[bench]
fn log_ndc_empty(b: &mut Bencher) {
    init_once();

    b.iter(|| {
        debug!("fgfg");
    })
}

#[bench]
fn log_ndc_not_empty(b: &mut Bencher) {
    init_once();

    log_ndc::set("server");
    b.iter(|| {
        debug!("fgfg");
    })
}
