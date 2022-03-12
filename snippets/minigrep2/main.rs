use std::env;
use std::process;
use minigrep2;

fn main() {
    let config = minigrep2::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep2::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}