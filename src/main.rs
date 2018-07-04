extern crate rusty_invaders;

use std::env;
use std::process;

use rusty_invaders::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = rusty_invaders::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(2);
    }
}
