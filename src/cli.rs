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
