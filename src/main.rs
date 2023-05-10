use std::fs;

mod cli;
mod config;

fn main() {
    let app_args = cli::AppArgs::gather();

    let config = config::parse_config(fs::read_to_string(app_args.config_path).unwrap());

    println!("{:#?}", config);
}
