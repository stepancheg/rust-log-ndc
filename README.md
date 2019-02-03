# log crate extension with NDC-like functionality

[![Build Status](https://img.shields.io/travis/stepancheg/rust-log-ndc.svg)](https://travis-ci.org/stepancheg/rust-http2)
[![License](https://img.shields.io/crates/l/httpbis.svg)](https://github.com/stepancheg/rust-log-ndc/blob/master/LICENSE.txt)
[![crates.io](https://img.shields.io/crates/v/httpbis.svg)](https://crates.io/crates/log-ndc)

This crate allows settings a thread-local variable and crate prepends in in `ndc_*` macros. E. g.

```
ndc_warn!("something happened"); // works exactly like `log!` macro

log_ndc::set(format!("reqid={}", 10));

ndc_info!("starting request");
// outputs
// INFO 2019-02-03T23:51:26Z: mycrate: [reqid=10] starting request
```
