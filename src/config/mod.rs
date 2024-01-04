use crate::error::Result;
use std::{net::Ipv4Addr, sync::OnceLock};

use self::helper::ConfigEnvKey;

pub mod helper;
pub mod tracing;

pub const APP_NAME: &str = "cook-book";
pub const DEFAULT_SERVICE_PORT: u16 = 8080;
pub const DEFAULT_SERVICE_IP: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
pub const DEFAULT_OTEL_COLLECTOR_URL: &str = "https://localhost:4317";
pub const DEFAULT_LOG_FILTER: &str = "INFO";
pub const DEFAULT_TRACE_FILTER: &str = "INFO";

pub fn get_config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONFIG - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub SERVICE_IP: Ipv4Addr,
    pub SERVICE_PORT: u16,
    pub OTEL_COLLECTOR_URL: String,
}

impl Config {
    // TODO: load all of these values and then handle all errors at once.
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            SERVICE_IP: Ipv4Addr::from(ConfigEnvKey::ServiceIp),
            SERVICE_PORT: u16::from(ConfigEnvKey::ServicePort),
            OTEL_COLLECTOR_URL: String::from(ConfigEnvKey::OtelCollectorUrl),
        })
    }
}
