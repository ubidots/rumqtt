use std::thread;

use rumqttd::requests::utils_requests;
use rumqttd::requests::utils_requests::MetricsPayload;
use rumqttd::requests::utils_settings;
use rumqttd::Broker;

use clap::Parser;
use rumqttd::Meter;
use tracing::trace;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(version)]
#[command(name = "rumqttd")]
#[command(about = "A high performance, lightweight and embeddable MQTT broker written in Rust.")]
#[command(author = "tekjar <raviteja@bytebeam.io>")]
struct CommandLine {
    /// path to config file
    #[arg(short, long)]
    config: Option<String>,
    #[command(subcommand)]
    command: Option<Command>,
    /// log level (v: info, vv: debug, vvv: trace)
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    verbose: u8,
    /// launch without printing banner
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Parser)]
enum Command {
    /// Write default configuration file to stdout
    GenerateConfig,
}

fn main() {
    let commandline: CommandLine = CommandLine::parse();
    if !commandline.quiet {
        banner();
    }

    // tracing syntax ->
    let builder = tracing_subscriber::fmt()
        .pretty()
        .with_line_number(false)
        .with_file(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_env_filter(EnvFilter::from_env("RUST_LOG"))
        .with_filter_reloading();

    let reload_handle = builder.reload_handle();

    builder
        .try_init()
        .expect("initialized subscriber successfully");

    let mut configs = utils_settings::get_settings();
    let server_id = configs.id.to_owned();
    let webhook_url = utils_settings::get_webhook_url(&configs);
    let metrics_url = utils_settings::get_metrics_url(&configs);
    let authentication_url = utils_settings::get_authentication_url(&configs);
    let authorization_url = utils_settings::get_authorization_url(&configs);
    let retained_url = utils_settings::get_retained_url(&configs);
    if let Some(console_config) = configs.console.as_mut() {
        console_config.set_filter_reload_handle(reload_handle)
    }

    validate_config(&configs);

    // println!("{:#?}", configs);

    let server = configs.v4.as_mut().and_then(|v4| v4.get_mut("1")).unwrap();
    server.set_webhook_auth_handler(
        auth,
        authentication_url.to_string(),
        webhook_url.to_string(),
        authorization_url.to_string(),
        retained_url.to_string(),
    );
    let mut broker = Broker::new(configs);
    let meters = broker.meters().unwrap();
    thread::spawn(move || loop {
        if let Ok(meters_vector) = meters.recv() {
            let metrics_payloads: Vec<MetricsPayload> = meters_vector
                .iter()
                .map(|meter| match meter {
                    Meter::Router(router_id, router_meter) => MetricsPayload {
                        server_id,
                        router_id: Some(router_id.to_owned()),
                        router_meter: Some(router_meter.to_owned()),
                        subscription_meter: None,
                        topic: None,
                    },
                    Meter::Subscription(topic, subscription_meter) => MetricsPayload {
                        server_id,
                        router_id: None,
                        router_meter: None,
                        subscription_meter: Some(subscription_meter.to_owned()),
                        topic: Some(topic.to_owned()),
                    },
                })
                .collect();
            utils_requests::metrics(&metrics_url, metrics_payloads);
        }
    });
    broker.start().unwrap();
}

async fn auth(webhook_url: String, _client_id: String, username: String, password: String) -> bool {
    // users can fetch data from DB or tokens and use them!
    // do the verification and return true if verified, else false
    let result = utils_requests::authenticate_user(&webhook_url, &username, &password).await;
    match result.auth_response {
        Some(response) => response.result == "allow",
        None => false,
    }
}

// Do any extra validation that needs to be done before starting the broker here.
fn validate_config(configs: &rumqttd::Config) {
    if let Some(v4) = &configs.v4 {
        for (name, server_setting) in v4 {
            if let Some(tls_config) = &server_setting.tls {
                if !tls_config.validate_paths() {
                    panic!("Certificate path not valid for server v4.{name}.")
                }
                trace!("Validated certificate paths for server v4.{name}.");
            }
        }
    }

    if let Some(v5) = &configs.v5 {
        for (name, server_setting) in v5 {
            if let Some(tls_config) = &server_setting.tls {
                if !tls_config.validate_paths() {
                    panic!("Certificate path not valid for server v5.{name}.")
                }
                trace!("Validated certificate paths for server v5.{name}.");
            }
        }
    }

    if let Some(ws) = &configs.ws {
        for (name, server_setting) in ws {
            if let Some(tls_config) = &server_setting.tls {
                if !tls_config.validate_paths() {
                    panic!("Certificate path not valid for server ws.{name}.")
                }
                trace!("Validated certificate paths for server ws.{name}.");
            }
        }
    }
}

fn banner() {
    const B: &str = r"                                              
         ___ _   _ __  __  ___ _____ _____ ___  
        | _ \ | | |  \/  |/ _ \_   _|_   _|   \ 
        |   / |_| | |\/| | (_) || |   | | | |) |
        |_|_\\___/|_|  |_|\__\_\|_|   |_| |___/ 
    ";

    println!("{B}\n");
}
