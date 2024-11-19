use rumqttd::requests::utils_settings;

static RUMQTTD_DEFAULT_CONFIG: &str = include_str!("rumqttd.toml");

fn main() {
    let settings = utils_settings::get_settings(RUMQTTD_DEFAULT_CONFIG.to_string());
    println!("{settings:#?}");
}
