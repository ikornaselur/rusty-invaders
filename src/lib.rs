use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use state::State;

pub mod state;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    buffer.resize(0x8000, 0);

    let state = State::new(buffer, true);

    let _state = match emulate(state) {
        Ok(state) => state,
        Err(e) => {
            panic!("Error from emulate: {}", e);
        }
    };
    Ok(())
}

fn emulate(mut state: State) -> Result<State, Box<Error>> {
    loop {
        match state.step() {
            None => break,
            _ => (),
        }
    }

    Ok(state)
}

pub struct Config {
    pub filename: String,
    pub debug: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Filename missing"),
        };

        Ok(Config {
            filename,
            debug: false,
        })
    }
}
