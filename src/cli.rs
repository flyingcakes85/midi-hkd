use easy_args::arg_spec;
use std::{error::Error, path::PathBuf};

#[derive(Debug)]
pub struct AppArgs {
    pub config_path: PathBuf,
    pub midi_device: u64,
    pub velocity_threshold: u8,
    pub list_devices: bool,
    pub octave_shift: i8,
}

impl AppArgs {
    pub fn gather() -> Self {
        let spec = arg_spec! {
            config: String,
            midi_device: u64,
            velocity_threshold: u64,
            list_devices: bool,
            octave_shift: String,
            help : bool,
            version: bool ,
        };
        let args = spec.parse().unwrap();
        // TODO : probably junky code;
        Self {
            config_path: config_path(args.string("config")).unwrap(),
            midi_device: *args.uinteger("midi_device").unwrap_or(&0),
            velocity_threshold: *args.uinteger("velocity_threshold").unwrap_or(&75) as u8,
            list_devices: *args.boolean("list_devices").unwrap_or(&false),
            octave_shift: {
                let x = args.string("octave_shift");
                if x.is_none() {
                    0
                } else {
                    x.unwrap().parse::<i8>().unwrap_or(0)
                }
            },
        }
    }
}

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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::cli::{config_path, default_config_path};

    #[test]
    fn default_config() {
        assert_eq!(config_path(None).unwrap(), default_config_path());
    }

    #[test]
    fn custom_config() {
        assert_eq!(
            config_path(Some(&String::from(
                "/home/user/.config/my-custom-config.toml"
            )))
            .unwrap(),
            PathBuf::from("/home/user/.config/my-custom-config.toml")
        );
    }

    #[test]
    fn custom_config_accented() {
        assert_eq!(
            config_path(Some(&String::from(
                "/home/àèìòùÀÈÌÒÙáéíóúýÁÉÍÓÚÝâêîôûÂÊÎÔÛãñõÃÑÕäëïöüÿÄËÏÖÜŸ/.config/my-custom-config.toml"
            )))
            .unwrap(),
            PathBuf::from("/home/àèìòùÀÈÌÒÙáéíóúýÁÉÍÓÚÝâêîôûÂÊÎÔÛãñõÃÑÕäëïöüÿÄËÏÖÜŸ/.config/my-custom-config.toml")
        );
    }
}
