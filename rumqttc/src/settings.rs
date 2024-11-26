use std::path::Path;

use config::{builder::DefaultState, ConfigBuilder, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct HealthCheckSettings {
    pub health_check_username: String,
    pub health_check_password: String,
    pub health_check_client_id: String,
    pub health_check_topic: String,
    pub health_check_port: u16,
    pub health_check_host: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Config {
    pub health_check: HealthCheckSettings,
}

fn update_settings(
    builder: ConfigBuilder<DefaultState>,
    file_path: &str,
) -> ConfigBuilder<DefaultState> {
    if Path::new(file_path).exists() {
        return builder.add_source(File::with_name(file_path));
    }
    builder
}

pub fn get_settings() -> Config {
    let mut config_builder = config::Config::builder();
    config_builder =
        config_builder.add_source(Environment::with_prefix("DYNACONF").separator("__"));
    config_builder = update_settings(config_builder, "config/default.toml");
    config_builder = update_settings(config_builder, "config/local.toml");
    let result: Config = config_builder.build().unwrap().try_deserialize().unwrap();
    result
}
