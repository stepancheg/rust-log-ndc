// `cargo test --benches` and `#[feature(test)]` work only in nightly
#![cfg(rustc_nightly)]
#![feature(test)]

extern crate test;

#[macro_use]
extern crate log;

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
