use rumqttd::requests::utils_settings;

fn main() {
    let settings = utils_settings::get_settings();
    println!("{settings:#?}");
}
