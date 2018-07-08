use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use state::Register;
use state::State;

pub mod state;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let state = State::new(buffer);

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
        match state.read_byte() {
            // NOPs
            Some(0x00) => state.nop(),
            Some(0x08) => state.nop(),
            Some(0x10) => state.nop(),
            Some(0x18) => state.nop(),
            Some(0x20) => state.nop(),
            Some(0x28) => state.nop(),
            Some(0x30) => state.nop(),
            Some(0x38) => state.nop(),

            // LXI ?,d16
            Some(0x01) => state.lxi(Register::B),
            Some(0x11) => state.lxi(Register::D),
            Some(0x21) => state.lxi(Register::H),
            Some(0x31) => state.lxi(Register::SP),

            // ADD ?
            Some(0x80) => state.add(Register::B),
            Some(0x81) => state.add(Register::C),
            Some(0x82) => state.add(Register::D),
            Some(0x83) => state.add(Register::E),
            Some(0x84) => state.add(Register::H),
            Some(0x85) => state.add(Register::L),
            Some(0x86) => state.add(Register::M),
            Some(0x87) => state.add(Register::A),

            // ADC ?
            Some(0x88) => state.adc(Register::B),
            Some(0x89) => state.adc(Register::C),
            Some(0x8A) => state.adc(Register::D),
            Some(0x8B) => state.adc(Register::E),
            Some(0x8C) => state.adc(Register::H),
            Some(0x8D) => state.adc(Register::L),
            Some(0x8E) => state.adc(Register::M),
            Some(0x8F) => state.adc(Register::A),

            Some(byte) => {
                panic!("Unknown OP: 0x{:02X?}", byte);
            }
            None => {
                break;
            }
        }
    }

    Ok(state)
}

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Filename missing"),
        };

        Ok(Config { filename })
    }
}
