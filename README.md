# log crate extension with NDC-like functionality

[![Build Status](https://img.shields.io/travis/stepancheg/rust-log-ndc.svg)](https://travis-ci.org/stepancheg/log-ndc)
[![License](https://img.shields.io/crates/l/log-ndc.svg)](https://github.com/stepancheg/rust-log-ndc/blob/master/LICENSE.txt)
[![crates.io](https://img.shields.io/crates/v/log-ndc.svg)](https://crates.io/crates/log-ndc)

* `log-ndc` crate provides a logger which wraps arbitrary logger
  which prepends thread-local information to each log message
* `log-ndc-env-logger` is a very simple (10 lines of code) drop-in wrapper/replacement of `env_logger` crate

## log-ndc

This crate allows settings a thread-local variable which will be prepended to log messages.

`log_ndc::set_boxed_logger(logger)` wraps passed `logger` object with `log_ndc::Logger` and calls
underlying `log::set_boxed_logger(..)`.

`log_ndc` functions like `log_ndc::set(..)` or `log_ndc::push(..)` replace thread-local text
which is later prepended to log messages in the wrapper logger.

```
// works exactly like regular `warn!` macro with any logger
// `warn!` is a macro from `log` library
warn!("something happened");

// set thread-local information like request id
log_ndc::set(format!("reqid={}", 10));

info!("starting request");
// outputs
// INFO 2019-02-03T23:51:26Z: mycrate: [reqid=10] starting request
```

## log-ndc-env-logger

Drop-in replacement/wrapper of `env_logger` crate.

It simply initialzes `env_logger` wrapper and wraps it with `log_ndc::Logger`.

The crate is dead simple, have a look at
[single file source](https://github.com/stepancheg/rust-log-ndc/blob/master/log-ndc-env-logger/src/lib.rs).

## FAQ

(I think these are FAQ, however nobody asked me anything yet.)

### What what NDC means?

The word "NDC" is [taken from log4j](https://logging.apache.org/log4j/1.2/apidocs/org/apache/log4j/NDC.html),
it means "nested dianostics context".

### Is it compatible with `log` crate or my favorite backend?

Yes, `log-ndc` wraps logging backend and delegates it to `log` crate.

So all macros like `warn!(..)` should work as before.

## See also `log-mdc`

* [log-mdc](https://github.com/sfackler/rust-log-mdc) is a similar project, which allows storing thread-local
data by key. However, log-mdc is compatible only with certain loggers like
[log4rs](https://github.com/sfackler/log4rs). It is not possible to obtain thread-local information set in
`log-mdc` in when e. g. `env-logger` is used.

## Where should we go?

I think it would be right thing if `log` crate supported basic
thread-local context like provided in this crate.
