use toml::Table;

#[derive(Debug)]
pub struct Config {
    pub midi_device: u64,
    pub velocity_threshold: u8,
    pub hotkeys: Table,
    pub octave_shift: i8,
}

pub fn parse_config(config_text: String) -> Config {
    let value = config_text.parse::<Table>().unwrap();

    let config = Config {
        midi_device: get_midi_device(&value),
        velocity_threshold: get_velocity_threshold(&value),
        hotkeys: get_hotkeys(&value),
        octave_shift: get_octave_shift(&value),
    };

    config
}

fn get_midi_device(value: &Table) -> u64 {
    let x = value.get("midi_device");
    if x.is_none() {
        0
    } else {
        let device_id = x.unwrap().as_integer();
        if device_id.is_none() {
            eprintln!("MIDI device id is not positive integer. Defaulting to 0.");
            0
        } else {
            device_id.unwrap() as u64
        }
    }
}

fn get_velocity_threshold(value: &Table) -> u8 {
    let x = value.get("velocity_threshold");
    if x.is_none() {
        75
    } else {
        let velocity_threshold = x.unwrap().as_integer();
        if velocity_threshold.is_none() {
            eprintln!("Velocity threshold is not positive integer. Defaulting to 75.");
            75
        } else {
            velocity_threshold.unwrap() as u8
        }
    }
}

fn get_hotkeys(value: &Table) -> Table {
    let x = value.get("hotkeys");
    if x.is_none() {
        panic!("Config doesn't contain hotkey definitions. Quitting.");
    } else {
        let hotkeys = x.unwrap().as_table();

        if hotkeys.is_none() {
            panic!("Hotkey definitions are not correct. Quitting.");
        } else {
            hotkeys.unwrap().clone()
        }
    }
}

fn get_octave_shift(value: &Table) -> i8 {
    let x = value.get("octave_shift");
    if x.is_none() {
        0
    } else {
        let octave_shift = x.unwrap().as_integer();
        if octave_shift.is_none() {
            eprintln!("Octave shift is not an integer. Defaulting to 0.");
            0
        } else {
            octave_shift.unwrap() as i8
        }
    }
}

#[cfg(test)]
mod tests {
    use toml::{Table, Value};

    use crate::config::get_octave_shift;

    use super::{get_hotkeys, get_velocity_threshold};

    fn get_valid_hotkeys() -> Table {
        let mut x: toml::map::Map<String, Value> = Table::new();
        x.insert(
            String::from("C#5"),
            Value::String(String::from("command 1")),
        );
        x.insert(
            String::from("A-5"),
            Value::String(String::from("command -flag")),
        );
        x.insert(
            String::from("G3"),
            Value::String(String::from("command -flag --more-flag")),
        );

        x
    }

    // tests for hotkeys

    #[test]
    fn hotkeys_success() {
        let hotkeys = get_valid_hotkeys();
        let mut config = Table::new();

        config.insert(String::from("hotkeys"), Value::Table(hotkeys.clone()));
        config.insert(String::from("midi_device"), Value::Integer(6));

        let parsed_hotkeys = get_hotkeys(&config);

        assert_eq!(parsed_hotkeys, hotkeys);
    }

    #[test]
    #[should_panic]
    fn hotkeys_wrong_name() {
        let hotkeys = get_valid_hotkeys();
        let mut config = Table::new();

        config.insert(String::from("hotkey"), Value::Table(hotkeys.clone()));
        config.insert(String::from("midi_device"), Value::Integer(6));

        get_hotkeys(&config);
    }

    #[test]
    fn hotkeys_empty() {
        let mut config = Table::new();

        config.insert(String::from("hotkeys"), Value::Table(Table::new()));
        config.insert(String::from("midi_device"), Value::Integer(6));

        assert_eq!(Table::new(), get_hotkeys(&config))
    }

    #[test]
    #[should_panic]
    fn hotkeys_incorrect() {
        let mut config = Table::new();

        config.insert(String::from("hotkeys"), Value::Integer(1));
        config.insert(String::from("midi_device"), Value::Integer(6));

        get_hotkeys(&config);
    }

    // tests for velocity threshold

    #[test]
    fn velocity_threshold_success() {
        let hotkeys = get_valid_hotkeys();
        let mut config = Table::new();

        config.insert(String::from("hotkeys"), Value::Table(hotkeys.clone()));
        config.insert(String::from("midi_device"), Value::Integer(6));
        config.insert(String::from("velocity_threshold"), Value::Integer(46));

        let parsed_velocity_threshold = get_velocity_threshold(&config);

        assert_eq!(parsed_velocity_threshold, 46);
    }

    #[test]
    fn velocity_threshold_wrong_name() {
        let hotkeys = get_valid_hotkeys();
        let mut config = Table::new();

        config.insert(String::from("hotkey"), Value::Table(hotkeys.clone()));
        config.insert(String::from("midi_device"), Value::Integer(6));
        config.insert(String::from("velocity_trhshold"), Value::Integer(46));
        assert_eq!(75, get_velocity_threshold(&config));
    }

    #[test]
    fn default_velocity_threshold() {
        let mut config = Table::new();

        config.insert(String::from("hotkeys"), Value::Table(Table::new()));
        config.insert(String::from("midi_device"), Value::Integer(6));

        assert_eq!(75, get_velocity_threshold(&config));
    }

    #[test]
    fn velocity_threshold_incorrect() {
        let mut config = Table::new();

        config.insert(String::from("hotkeys"), Value::Table(get_valid_hotkeys()));
        config.insert(String::from("midi_device"), Value::Integer(6));
        config.insert(
            String::from("velocity_threshold"),
            Value::String(String::from("a46")),
        );

        assert_eq!(75, get_velocity_threshold(&config));
    }

    // tests for octave shift

    #[test]
    fn octave_shift_success() {
        let hotkeys = get_valid_hotkeys();
        let mut config = Table::new();

        config.insert(String::from("hotkeys"), Value::Table(hotkeys.clone()));
        config.insert(String::from("midi_device"), Value::Integer(6));
        config.insert(String::from("octave_shift"), Value::Integer(-2));

        assert_eq!(-2, get_octave_shift(&config));
    }

    #[test]
    fn octave_shift_wrong_name() {
        let hotkeys = get_valid_hotkeys();
        let mut config = Table::new();

        config.insert(String::from("hotkey"), Value::Table(hotkeys.clone()));
        config.insert(String::from("midi_device"), Value::Integer(6));
        config.insert(String::from("octave_shitf"), Value::Integer(2));

        assert_eq!(0, get_octave_shift(&config));
    }

    #[test]
    fn default_octave_shift() {
        let mut config = Table::new();

        config.insert(String::from("hotkeys"), Value::Table(Table::new()));
        config.insert(String::from("midi_device"), Value::Integer(6));

        assert_eq!(0, get_octave_shift(&config));
    }

    #[test]
    fn octave_shift_incorrect() {
        let mut config = Table::new();

        config.insert(String::from("hotkeys"), Value::Table(get_valid_hotkeys()));
        config.insert(String::from("midi_device"), Value::Integer(6));
        config.insert(
            String::from("octave_shift"),
            Value::String(String::from("-4b")),
        );

        assert_eq!(0, get_octave_shift(&config));
    }
}
