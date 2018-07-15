use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;

use clock::Clock;
use state::State;

pub mod clock;
pub mod io;
pub mod state;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let mut machine = Machine::new(buffer);
    machine.emulate();

    Ok(())
}

struct Machine {
    state: State,
    clock: Clock,
    interrupt_timer: Clock,
}

impl Machine {
    fn new(mut buffer: Vec<u8>) -> Machine {
        buffer.resize(0x10000, 0);

        Machine {
            state: State::new(buffer, true),
            clock: Clock::new(),
            interrupt_timer: Clock::new(),
        }
    }

    fn emulate(&mut self) -> () {
        loop {
            if self.state.int_enabled && self.interrupt_timer.elapsed() > Duration::new(1 / 60, 0) {
                self.interrupt_timer.reset_last_time();
                self.state.rst(2);
                self.state.di();
            }
            match self.state.step() {
                Some((byte, None)) => {
                    println!("Read byte: {:02X?}", byte);
                    ()
                }
                None => break,
                _ => (),
            }
        }
    }
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
