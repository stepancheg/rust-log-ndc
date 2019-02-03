/// The standard logging macro.
///
/// This macro will generically log with the specified `Level` and `format!`
/// based argument list.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate log;
/// use log::Level;
///
/// # fn main() {
/// let data = (42, "Forty-two");
/// let private_data = "private";
///
/// log!(Level::Error, "Received errors: {}, {}", data.0, data.1);
/// log!(target: "app_events", Level::Warn, "App warning: {}, {}, {}",
///     data.0, data.1, private_data);
/// # }
/// ```
#[macro_export]
macro_rules! ndc_log {
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= log::STATIC_MAX_LEVEL && lvl <= log::max_level() {
            $crate::get(|s| {
                if s.is_empty() {
                    // TODO: figure out how to delegate to `log!` macro
                    log::__private_api_log(
                        format_args!($($arg)+),
                        lvl,
                        &($target, module_path!(), file!(), line!()),
                    );
                } else {
                    log::__private_api_log(
                        // Format twice since `concat_idents!` is not stable
                        format_args!("[{}] {}", s, format_args!($($arg)+)),
                        lvl,
                        &($target, module_path!(), file!(), line!()),
                    );
                };
            });
        }
    });
    ($lvl:expr, $($arg:tt)+) => (ndc_log!(target: module_path!(), $lvl, $($arg)+))
}

/// Logs a message at the error level.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate log;
/// # fn main() {
/// let (err_info, port) = ("No connection", 22);
///
/// error!("Error: {} on port {}", err_info, port);
/// error!(target: "app_events", "App Error: {}, Port: {}", err_info, 22);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! ndc_error {
    (target: $target:expr, $($arg:tt)*) => (
        ndc_log!(target: $target, log::Level::Error, $($arg)*);
    );
    ($($arg:tt)*) => (
        ndc_log!(log::Level::Error, $($arg)*);
    )
}

/// Logs a message at the warn level.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate log;
/// # fn main() {
/// let warn_description = "Invalid Input";
///
/// warn!("Warning! {}!", warn_description);
/// warn!(target: "input_events", "App received warning: {}", warn_description);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! ndc_warn {
    (target: $target:expr, $($arg:tt)*) => (
        ndc_log!(target: $target, log::Level::Warn, $($arg)*);
    );
    ($($arg:tt)*) => (
        ndc_log!(log::Level::Warn, $($arg)*);
    )
}

/// Logs a message at the info level.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate log;
/// # fn main() {
/// # struct Connection { port: u32, speed: f32 }
/// let conn_info = Connection { port: 40, speed: 3.20 };
///
/// info!("Connected to port {} at {} Mb/s", conn_info.port, conn_info.speed);
/// info!(target: "connection_events", "Successfull connection, port: {}, speed: {}",
///       conn_info.port, conn_info.speed);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! ndc_info {
    (target: $target:expr, $($arg:tt)*) => (
        ndc_log!(target: $target, log::Level::Info, $($arg)*);
    );
    ($($arg:tt)*) => (
        ndc_log!(::log::Level::Info, $($arg)*);
    )
}

/// Logs a message at the debug level.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate log;
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// debug!("New position: x: {}, y: {}", pos.x, pos.y);
/// debug!(target: "app_events", "New position: x: {}, y: {}", pos.x, pos.y);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! ndc_debug {
    (target: $target:expr, $($arg:tt)*) => (
        ndc_log!(target: $target, log::Level::Debug, $($arg)*);
    );
    ($($arg:tt)*) => (
        ndc_log!(log::Level::Debug, $($arg)*);
    )
}

/// Logs a message at the trace level.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate log;
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// trace!("Position is: x: {}, y: {}", pos.x, pos.y);
/// trace!(target: "app_events", "x is {} and y is {}",
///        if pos.x >= 0.0 { "positive" } else { "negative" },
///        if pos.y >= 0.0 { "positive" } else { "negative" });
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! ndc_trace {
    (target: $target:expr, $($arg:tt)*) => (
        ndc_log!(target: $target, log::Level::Trace, $($arg)*);
    );
    ($($arg:tt)*) => (
        ndc_log!(log::Level::Trace, $($arg)*);
    )
}
