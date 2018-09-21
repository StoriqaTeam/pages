use std::env;
use std::net::IpAddr;
use stq_logging;

use config_crate::{Config as RawConfig, ConfigError, Environment, File};

use sentry_integration::SentryConfig;

/// Service configuration
#[derive(Clone, Debug, Deserialize)]
pub struct Server {
    pub host: IpAddr,
    pub port: u16,
    pub thread_count: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Database {
    pub dsn: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    /// Server settings
    pub server: Server,
    /// Database settings
    pub db: Database,
    /// GrayLog settings
    pub graylog: Option<stq_logging::GrayLogConfig>,
    /// Sentry settings
    pub sentry: Option<SentryConfig>,
}

const ENV_PREFIX: &str = "STQ_PAGES";

/// Creates new app config struct
/// #Examples
/// ```
/// use pages_lib::*;
///
/// let config = Config::new();
/// ```
impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = RawConfig::new();
        s.merge(File::with_name("config/base"))?;

        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in settings from the environment (with a prefix)
        s.merge(Environment::with_prefix(ENV_PREFIX))?;

        s.try_into()
    }
}
