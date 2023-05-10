use midir::{Ignore, MidiInput};
use std::{error::Error, io::stdin};
use toml::Table;

fn id_to_key(key_id: u8) -> String {
    let notes = vec![
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    format!(
        "{}{}",
        notes.get((key_id % 12) as usize).unwrap(),
        key_id / 12 - 2
    )
}

// TODO : refactor
pub fn daemon(keymap: Table, midi_device: u8) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port
    let in_ports = midi_in.ports();
    let in_port = &in_ports[midi_device as usize];

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |_, message, _| {
            // TODO : call commands
            println!("{:#?}", message);
            let key_name = id_to_key(*message.get(1).unwrap());
            println!("{} pressed", key_name);

            if keymap.contains_key(&key_name) {
                println!("{}", keymap[&key_name]);
            }
        },
        (),
    )?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}
