mod cli;

fn main() {
    let app_args = cli::AppArgs::gather();
    println!("{:#?}", app_args);
}
