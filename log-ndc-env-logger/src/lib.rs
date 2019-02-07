extern crate env_logger;
extern crate log;
extern crate log_ndc;

/// Wrap configured `env_logger::Logger` with NDC logger
/// and register it as `log` logger.
pub fn set_env_logger(logger: env_logger::Logger) -> Result<(), log::SetLoggerError> {
    log::set_max_level(logger.filter());
    log_ndc::set_boxed_logger(Box::new(logger))
}

/// Initialize logger wrapped with `log_ndc` logger.
/// This is similar to `env_logger::init()` call.
pub fn init() {
    let env_logger = env_logger::Builder::from_default_env().build();
    set_env_logger(env_logger).expect("failed to set logger");
}

/// Initialize logger wrapped with `log_ndc` logger.
/// This is similar to `env_logger::init_from_env()` call.
pub fn init_from_env<'e, E>(env: E)
    where E: Into<env_logger::Env<'e>>
{
    let env_logger = env_logger::Builder::from_env(env).build();
    set_env_logger(env_logger).expect("failed to set logger");
}
