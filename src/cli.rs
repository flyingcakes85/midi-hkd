use std::{error::Error, path::PathBuf};

fn default_config_path() -> PathBuf {
    let mut config_directory = match home::home_dir() {
        Some(path) => path,
        None => panic!("Can't determine home directory"),
    };

    config_directory.push(".config");
    config_directory.push("midi-hkd");
    config_directory.push("config.toml");

    config_directory
}

pub fn config_path(config_arg: Option<&String>) -> Result<PathBuf, Box<dyn Error>> {
    let config_path = if config_arg.is_none() {
        default_config_path()
    } else {
        PathBuf::from(config_arg.unwrap())
    };

    Ok(config_path)
}
