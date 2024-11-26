use std::env;
use std::path::Path;

use config::builder::DefaultState;
use config::Environment;
use config::{ConfigBuilder, File};

use crate::Config;

fn update_settings(
    builder: ConfigBuilder<DefaultState>,
    file_path: &str,
) -> ConfigBuilder<DefaultState> {
    if Path::new(file_path).exists() {
        return builder.add_source(File::with_name(file_path));
    }
    builder
}

lazy_static! {
    pub static ref SETTINGS: Config = get_settings();
}

pub fn get_settings() -> Config {
    let mut config_builder = config::Config::builder();
    config_builder =
        config_builder.add_source(Environment::with_prefix("DYNACONF").separator("__"));
    config_builder = update_settings(config_builder, "config/default.toml");
    config_builder = update_settings(config_builder, "config/local.toml");
    config_builder = update_settings_from_environment(config_builder);
    let result: Config = config_builder.build().unwrap().try_deserialize().unwrap();
    result
}

pub fn get_authentication_url(config: &Config) -> String {
    if let Some(webhook_config) = &config.webhook {
        let protocol = webhook_config.authentication_protocol.to_string();
        let host = webhook_config.authentication_host.to_string();
        let port = webhook_config.authentication_port.to_string();
        let endpoint = webhook_config.authentication_endpoint.to_string();
        let result = format!("{protocol}://{host}:{port}{endpoint}");
        return result;
    }
    "".to_string()
}

pub fn get_authorization_url(config: &Config) -> String {
    if let Some(webhook_config) = &config.webhook {
        let protocol = webhook_config.authorization_protocol.to_string();
        let host = webhook_config.authorization_host.to_string();
        let port = webhook_config.authorization_port.to_string();
        let endpoint = webhook_config.authorization_endpoint.to_string();
        let result = format!("{protocol}://{host}:{port}{endpoint}");
        return result;
    }
    "".to_string()
}

pub fn get_webhook_url(config: &Config) -> String {
    if let Some(webhook_config) = &config.webhook {
        let protocol = webhook_config.webhook_protocol.to_string();
        let host = webhook_config.webhook_host.to_string();
        let port = webhook_config.webhook_port.to_string();
        let endpoint = webhook_config.webhook_endpoint.to_string();
        let result = format!("{protocol}://{host}:{port}{endpoint}");
        return result;
    }
    "".to_string()
}

pub fn get_retained_url(config: &Config) -> String {
    if let Some(webhook_config) = &config.webhook {
        let protocol = webhook_config.retained_protocol.to_string();
        let host = webhook_config.retained_host.to_string();
        let port = webhook_config.retained_port.to_string();
        let endpoint = webhook_config.retained_endpoint.to_string();
        let result = format!("{protocol}://{host}:{port}{endpoint}");
        return result;
    }
    "".to_string()
}

pub fn get_metrics_url(config: &Config) -> String {
    if let Some(webhook_config) = &config.webhook {
        let protocol = webhook_config.metrics_protocol.to_string();
        let host = webhook_config.metrics_host.to_string();
        let port = webhook_config.metrics_port.to_string();
        let endpoint = webhook_config.metrics_endpoint.to_string();
        let result = format!("{protocol}://{host}:{port}{endpoint}");
        return result;
    }
    "".to_string()
}

fn update_webhook_configuration(
    builder: ConfigBuilder<DefaultState>,
    host_env_variable: &str,
    port_env_variable: &str,
    default_host_env_variable: &str,
    default_port_env_variable: &str,
    host_setting: &str,
    port_setting: &str,
) -> ConfigBuilder<DefaultState> {
    let mut builder = builder;
    let host_env_variable = match env::var(host_env_variable) {
        Ok(host_env_variable) => host_env_variable,
        Err(_) => default_host_env_variable.to_string(),
    };
    let port_env_variable = match env::var(port_env_variable) {
        Ok(port_env_variable) => port_env_variable,
        Err(_) => default_port_env_variable.to_string(),
    };
    if let Ok(host) = env::var(host_env_variable) {
        if let Ok(port) = env::var(port_env_variable) {
            builder = builder.set_override(host_setting, host).unwrap();
            builder = builder.set_override(port_setting, port).unwrap();
        }
    }
    builder
}

pub fn update_settings_from_environment(
    builder: ConfigBuilder<DefaultState>,
) -> ConfigBuilder<DefaultState> {
    let mut builder = builder;
    builder = update_webhook_configuration(
        builder,
        "MQTT_AUTHENTICATION_HOST_ENV_VARIABLE",
        "MQTT_AUTHENTICATION_PORT_ENV_VARIABLE",
        "UBIDOTS_REACTOR_MQTT_SERVER_SERVICE_HOST",
        "UBIDOTS_REACTOR_MQTT_SERVER_SERVICE_PORT",
        "webhook.authentication_host",
        "webhook.authentication_port",
    );
    builder = update_webhook_configuration(
        builder,
        "MQTT_AUTHORIZATION_HOST_ENV_VARIABLE",
        "MQTT_AUTHORIZATION_PORT_ENV_VARIABLE",
        "UBIDOTS_REACTOR_MQTT_SERVER_SERVICE_HOST",
        "UBIDOTS_REACTOR_MQTT_SERVER_SERVICE_PORT",
        "webhook.authorization_host",
        "webhook.authorization_port",
    );
    builder = update_webhook_configuration(
        builder,
        "MQTT_WEBHOOK_HOST_ENV_VARIABLE",
        "MQTT_WEBHOOK_PORT_ENV_VARIABLE",
        "UBIDOTS_REACTOR_MQTT_SERVER_SERVICE_HOST",
        "UBIDOTS_REACTOR_MQTT_SERVER_SERVICE_PORT",
        "webhook.webhook_host",
        "webhook.webhook_port",
    );

    builder = update_webhook_configuration(
        builder,
        "MQTT_RETAINED_HOST_ENV_VARIABLE",
        "MQTT_RETAINED_PORT_ENV_VARIABLE",
        "UBIDOTS_REACTOR_MQTT_SERVER_SERVICE_HOST",
        "UBIDOTS_REACTOR_MQTT_SERVER_SERVICE_PORT",
        "webhook.retained_host",
        "webhook.retained_port",
    );
    builder = update_webhook_configuration(
        builder,
        "MQTT_METRICS_HOST_ENV_VARIABLE",
        "MQTT_METRICS_PORT_ENV_VARIABLE",
        "UBIDOTS_REACTOR_MQTT_SERVER_SERVICE_HOST",
        "UBIDOTS_REACTOR_MQTT_SERVER_SERVICE_PORT",
        "webhook.metrics_host",
        "webhook.metrics_port",
    );
    builder
}
