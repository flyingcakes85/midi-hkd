use std::fs;

mod cli;
mod config;
mod midi;

fn main() {
    let app_args = cli::AppArgs::gather();

    if app_args.list_devices == true {
        midi::list_devices().unwrap();
    } else {
        let config = config::parse_config(fs::read_to_string(app_args.config_path).unwrap());
        midi::daemon(config.hotkeys, config.midi_device.try_into().unwrap()).unwrap();
    }
}
