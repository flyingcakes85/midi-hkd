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
        0
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
