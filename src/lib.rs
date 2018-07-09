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

            // Instructions with registers

            // LXI ?,d16
            Some(0x01) => state.lxi(Register::B),
            Some(0x11) => state.lxi(Register::D),
            Some(0x21) => state.lxi(Register::H),
            Some(0x31) => state.lxi(Register::SP),

            // INR ?
            Some(0x04) => state.inr(Register::B),
            Some(0x14) => state.inr(Register::D),
            Some(0x24) => state.inr(Register::H),
            Some(0x34) => state.inr(Register::M),
            Some(0x0C) => state.inr(Register::C),
            Some(0x1C) => state.inr(Register::E),
            Some(0x2C) => state.inr(Register::L),
            Some(0x3C) => state.inr(Register::A),

            // DCR ?
            Some(0x05) => state.dcr(Register::B),
            Some(0x15) => state.dcr(Register::D),
            Some(0x25) => state.dcr(Register::H),
            Some(0x35) => state.dcr(Register::M),
            Some(0x0D) => state.dcr(Register::C),
            Some(0x1D) => state.dcr(Register::E),
            Some(0x2D) => state.dcr(Register::L),
            Some(0x3D) => state.dcr(Register::A),

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

            // SUB ?
            Some(0x90) => state.sub(Register::B),
            Some(0x91) => state.sub(Register::C),
            Some(0x92) => state.sub(Register::D),
            Some(0x93) => state.sub(Register::E),
            Some(0x94) => state.sub(Register::H),
            Some(0x95) => state.sub(Register::L),
            Some(0x96) => state.sub(Register::M),
            Some(0x97) => state.sub(Register::A),

            // SBB ?
            Some(0x98) => state.sbb(Register::B),
            Some(0x99) => state.sbb(Register::C),
            Some(0x9A) => state.sbb(Register::D),
            Some(0x9B) => state.sbb(Register::E),
            Some(0x9C) => state.sbb(Register::H),
            Some(0x9D) => state.sbb(Register::L),
            Some(0x9E) => state.sbb(Register::M),
            Some(0x9F) => state.sbb(Register::A),

            // ANA ?
            Some(0xA0) => state.ana(Register::B),
            Some(0xA1) => state.ana(Register::C),
            Some(0xA2) => state.ana(Register::D),
            Some(0xA3) => state.ana(Register::E),
            Some(0xA4) => state.ana(Register::H),
            Some(0xA5) => state.ana(Register::L),
            Some(0xA6) => state.ana(Register::M),
            Some(0xA7) => state.ana(Register::A),

            // XRA ?
            Some(0xA8) => state.xra(Register::B),
            Some(0xA9) => state.xra(Register::C),
            Some(0xAA) => state.xra(Register::D),
            Some(0xAB) => state.xra(Register::E),
            Some(0xAC) => state.xra(Register::H),
            Some(0xAD) => state.xra(Register::L),
            Some(0xAE) => state.xra(Register::M),
            Some(0xAF) => state.xra(Register::A),

            // ORA ?
            Some(0xB0) => state.ora(Register::B),
            Some(0xB1) => state.ora(Register::C),
            Some(0xB2) => state.ora(Register::D),
            Some(0xB3) => state.ora(Register::E),
            Some(0xB4) => state.ora(Register::H),
            Some(0xB5) => state.ora(Register::L),
            Some(0xB6) => state.ora(Register::M),
            Some(0xB7) => state.ora(Register::A),

            // CMP ?
            Some(0xB8) => state.cmp(Register::B),
            Some(0xB9) => state.cmp(Register::C),
            Some(0xBA) => state.cmp(Register::D),
            Some(0xBB) => state.cmp(Register::E),
            Some(0xBC) => state.cmp(Register::H),
            Some(0xBD) => state.cmp(Register::L),
            Some(0xBE) => state.cmp(Register::M),
            Some(0xBF) => state.cmp(Register::A),

            // Instructions without registers
            // Some(0x07) => state.rlc(),
            // Some(0x17) => state.ral(),
            // Some(0x27) => state.daa(),
            // Some(0x37) => state.stc(),
            // Some(0x0F) => state.rrc(),
            // Some(0x1F) => state.rar(),
            // Some(0x2F) => state.cma(),
            // Some(0x3F) => state.cmc(),
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
