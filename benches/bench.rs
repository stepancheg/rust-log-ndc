// `cargo test --benches` and `#[feature(test)]` work only in nightly
#![cfg(rustc_nightly)]
#![feature(test)]

extern crate test;

#[macro_use]
extern crate log;
#[macro_use]
extern crate log_ndc;

extern crate env_logger;

use test::Bencher;
use std::sync::Once;

fn init_once() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| env_logger::init());
}

#[bench]
fn log(b: &mut Bencher) {
    init_once();

    b.iter(|| {
        debug!("fgfg");
    })
}

#[bench]
fn log_ndc_empty(b: &mut Bencher) {
    init_once();

    b.iter(|| {
        ndc_debug!("fgfg");
    })
}

#[bench]
fn log_ndc_not_empty(b: &mut Bencher) {
    init_once();

    log_ndc::set("server");
    b.iter(|| {
        ndc_debug!("fgfg");
    })
}
