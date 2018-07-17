use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;

use clock::Clock;
use state::State;

pub mod clock;
pub mod io;
pub mod state;

const FRAME_TIME: Duration = Duration::from_micros(1_000_000 / 60);

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
    _clock: Clock,
    interrupt_timer: Clock,
}

impl Machine {
    fn new(mut buffer: Vec<u8>) -> Machine {
        buffer.resize(0x10000, 0);

        Machine {
            state: State::new(buffer, true),
            _clock: Clock::new(),
            interrupt_timer: Clock::new(),
        }
    }

    fn emulate(&mut self) -> () {
        loop {
            if self.state.int_enabled && self.interrupt_timer.elapsed() > FRAME_TIME {
                self.interrupt_timer.reset_last_time();
                self.state.rst(2);
                self.state.di();
            }
            match self.state.step() {
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
