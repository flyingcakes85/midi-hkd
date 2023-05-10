use easy_args::arg_spec;
mod cli;

fn main() {
    let spec = arg_spec! {
        config: String,
        midi_device: u64,
        help : bool,
        version: bool ,
    };

    let args = spec.parse().unwrap();
    let config_path = cli::config_path(args.string("config"));

    println!("{:#?}", config_path);
}
