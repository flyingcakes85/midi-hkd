use toml::Table;

#[derive(Debug)]
pub struct Config {
    pub midi_device: u64,
    pub velocity_threshold: u8,
    pub hotkeys: Table,
}

pub fn parse_config(config_text: String) -> Config {
    let value = config_text.parse::<Table>().unwrap();
    let config = Config {
        midi_device: value["midi_device"].as_integer().unwrap() as u64,
        velocity_threshold: value["velocity_threshold"].as_integer().unwrap() as u8,
        hotkeys: value["hotkeys"].as_table().unwrap().clone(),
    };

    config
}
