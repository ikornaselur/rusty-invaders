use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    SP,
    PC,
}

struct ConditionCodes {
    z: u8,
    s: u8,
    p: u8,
    cy: u8,
    zc: u8,
    pad: u8,
}

impl Default for ConditionCodes {
    fn default() -> ConditionCodes {
        ConditionCodes {
            z: 0,
            s: 0,
            p: 0,
            cy: 0,
            zc: 0,
            pad: 0,
        }
    }
}

struct State {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    memory: Vec<u8>,
    cc: ConditionCodes,
    int_enable: u8,
}

impl Default for State {
    fn default() -> State {
        State {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory: Vec::new(),
            cc: Default::default(),
            int_enable: 0,
        }
    }
}

impl State {
    fn read_byte(&mut self) -> Option<u8> {
        let byte = self.memory.get(self.pc as usize);
        match byte.cloned() {
            Some(byte) => {
                self.pc += 1;
                Some(byte)
            }
            None => None,
        }
    }
    fn nop(&mut self) -> () {
        // 4 instructions
        ()
    }

    fn lxi(&mut self, register: Register) -> () {
        let least = self.read_byte().unwrap();
        let most = self.read_byte().unwrap();

        match register {
            Register::B => {
                self.c = least;
                self.b = most;
            }
            Register::D => {
                self.e = least;
                self.d = most;
            }
            Register::H => {
                self.l = least;
                self.h = most;
            }
            Register::SP => {
                self.sp = ((most as u16) << 8) + least as u16;
            }
            unsupported => {
                panic!("lxi doesn't support {:?}", unsupported);
            }
        }
        ()
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let state = State {
        memory: buffer,
        ..State::default()
    };
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nop_advances_pc_by_one() {
        let mut state = State {
            memory: vec![0],
            ..State::default()
        };
        assert_eq!(state.pc, 0);

        state = emulate(state).unwrap();

        assert_eq!(state.pc, 1);
    }

    #[test]
    fn lxi_b_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0x01, 0xDE, 0xAD],
            ..State::default()
        };

        assert_eq!(state.pc, 0);

        state = emulate(state).unwrap();

        assert_eq!(state.pc, 3);
        assert_eq!(state.c, 0xDE);
        assert_eq!(state.b, 0xAD);
    }

    #[test]
    fn lxi_d_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0x11, 0xDE, 0xAD],
            ..State::default()
        };

        assert_eq!(state.pc, 0);

        state = emulate(state).unwrap();

        assert_eq!(state.pc, 3);
        assert_eq!(state.e, 0xDE);
        assert_eq!(state.d, 0xAD);
    }

    #[test]
    fn lxi_h_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0x21, 0xDE, 0xAD],
            ..State::default()
        };

        assert_eq!(state.pc, 0);

        state = emulate(state).unwrap();

        assert_eq!(state.pc, 3);
        assert_eq!(state.l, 0xDE);
        assert_eq!(state.h, 0xAD);
    }

    #[test]
    fn lxi_sp_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0x31, 0xAD, 0xDE],
            ..State::default()
        };

        assert_eq!(state.pc, 0);

        state = emulate(state).unwrap();

        assert_eq!(state.pc, 3);
        assert_eq!(state.sp, 0xDEAD);
    }
}
