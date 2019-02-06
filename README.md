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

warn!("something bad happened"); // this line does not output NDC information
```

This crate depends on `log` crate, not replaces it.

This crate is fully compatible with log-formatting crates like `env_logger`.

Regular `trace!(...)`...`error!(...)` macros still work but do not output NDC information.

The word "NDC" is [taken from log4j](https://logging.apache.org/log4j/1.2/apidocs/org/apache/log4j/NDC.html),
it means "nested dianostics context".

## FAQ

(I think these are FAQ, nobody asked me anything yet)

### Is it compatible with `log` crate?

Partially. When `ndc_*!` macros are used, thread-local information is logged to regular logger.

When macros like `info!` from `log` crate are used, thread-local information is lost.

### Is it compatible with my favorite log backend?

Yes! In particular, I've used it with `env_logger`.

## See also `log-mdc`

* [log-mdc](https://github.com/sfackler/rust-log-mdc) crate allows storing thread-local data by key.
However, log-mdc is compatible only with certain loggers like [log4rs](https://github.com/sfackler/log4rs).
It is not possible to obtain thread-local information set in `log-mdc` in when e. g. `env-logger` is used.

## Where should we go?

I think it would be right thing if `log` crate supported basic
thread-local context like provided in this crate.
