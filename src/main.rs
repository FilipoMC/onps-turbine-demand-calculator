mod cli;
mod constants;
mod logic;

use std::process;

fn main() {
    if let Err(e) = cli::run_cli() {
        eprintln!("There was an error: {e}");
        process::exit(-1);
    }
}
