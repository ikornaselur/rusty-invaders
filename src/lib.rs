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
    M,
    SP,
    PC,
}

struct ConditionCodes {
    zero: bool,   // Zero - when arithmetic result is 0
    sign: bool,   // Sign - when the most significant bit is set
    parity: bool, // Parity - when the answer has even parity
    carry: bool,  // Carry - when the instruction resulted in carry
    zc: u8,
    pad: u8,
}

impl Default for ConditionCodes {
    fn default() -> ConditionCodes {
        ConditionCodes {
            zero: false,
            sign: false,
            parity: false,
            carry: false,
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

    fn set_flags(&mut self, byte: u8, carry: bool) -> () {
        self.cc.carry = carry;
        self.cc.zero = byte == 0u8;
        self.cc.sign = (byte & 0x80) != 0;
        self.cc.parity = byte.count_ones() % 2 == 0;
    }

    /*
     * Opcodes
     */
    fn nop(&mut self) -> () {
        // 4 instructions
        ()
    }

    fn lxi(&mut self, register: Register) -> () {
        // 10 instrucitons
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
        };
    }

    fn add(&mut self, register: Register) -> () {
        // 4 instructions
        let (result, carry) = match register {
            Register::A => self.a.overflowing_add(self.a),
            Register::B => self.a.overflowing_add(self.b),
            Register::C => self.a.overflowing_add(self.c),
            Register::D => self.a.overflowing_add(self.d),
            Register::E => self.a.overflowing_add(self.e),
            Register::H => self.a.overflowing_add(self.h),
            Register::L => self.a.overflowing_add(self.l),
            unsupported => {
                panic!("add doesn't support {:?}", unsupported);
            }
        };

        self.a = result;
        self.set_flags(result, carry);
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

            // ADD ?
            Some(0x80) => state.add(Register::B),
            Some(0x81) => state.add(Register::C),
            Some(0x82) => state.add(Register::D),
            Some(0x83) => state.add(Register::E),
            Some(0x84) => state.add(Register::H),
            Some(0x85) => state.add(Register::L),
            Some(0x86) => state.add(Register::M),
            Some(0x87) => state.add(Register::A),

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
    fn set_flags_sets_sign_flag() {
        let mut state = State::default();

        let signed: u8 = 0b1000_0000;
        state.set_flags(signed, false);
        assert_eq!(state.cc.sign, true);

        let unsigned: u8 = 0b0111_1111;
        state.set_flags(unsigned, false);
        assert_eq!(state.cc.sign, false);
    }

    #[test]
    fn set_flags_sets_carry_flag() {
        let mut state = State::default();

        state.set_flags(0, true);
        assert_eq!(state.cc.carry, true);

        state.set_flags(0, false);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn set_flags_sets_parity_flag() {
        let mut state = State::default();

        let even1: u8 = 0b0000_0000;
        let even2: u8 = 0b0110_0000;
        let even3: u8 = 0b0001_1011;

        state.set_flags(even1, false);
        assert_eq!(state.cc.parity, true);

        state.set_flags(even2, false);
        assert_eq!(state.cc.parity, true);

        state.set_flags(even3, false);
        assert_eq!(state.cc.parity, true);

        let odd1: u8 = 0b0000_0001;
        let odd2: u8 = 0b0101_0001;
        let odd3: u8 = 0b1011_0101;

        let hehe = "whoa there";

        state.set_flags(odd1, false);
        assert_eq!(state.cc.parity, false);

        state.set_flags(odd2, false);
        assert_eq!(state.cc.parity, false);

        state.set_flags(odd3, false);
        assert_eq!(state.cc.parity, false);
    }

    #[test]
    fn nop_advances_pc_by_one() {
        let mut state = State {
            memory: vec![0],
            pc: 0,
            ..State::default()
        };

        state = emulate(state).unwrap();

        assert_eq!(state.pc, 1);
    }

    #[test]
    fn lxi_b_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0x01, 0xDE, 0xAD],
            pc: 0,
            ..State::default()
        };

        state = emulate(state).unwrap();

        assert_eq!(state.pc, 3);
        assert_eq!(state.c, 0xDE);
        assert_eq!(state.b, 0xAD);
    }

    #[test]
    fn lxi_d_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0x11, 0xDE, 0xAD],
            pc: 0,
            ..State::default()
        };

        state = emulate(state).unwrap();

        assert_eq!(state.pc, 3);
        assert_eq!(state.e, 0xDE);
        assert_eq!(state.d, 0xAD);
    }

    #[test]
    fn lxi_h_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0x21, 0xDE, 0xAD],
            pc: 0,
            ..State::default()
        };

        state = emulate(state).unwrap();

        assert_eq!(state.pc, 3);
        assert_eq!(state.l, 0xDE);
        assert_eq!(state.h, 0xAD);
    }

    #[test]
    fn lxi_sp_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0x31, 0xAD, 0xDE],
            pc: 0,
            ..State::default()
        };

        state = emulate(state).unwrap();

        assert_eq!(state.pc, 3);
        assert_eq!(state.sp, 0xDEAD);
    }

    #[test]
    fn add_b_adds_b_to_accumulator() {
        let state = emulate(State {
            memory: vec![0x80],
            a: 1,
            b: 2,
            ..State::default()
        }).unwrap();

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_c_adds_c_to_accumulator() {
        let state = emulate(State {
            memory: vec![0x81],
            a: 1,
            c: 2,
            ..State::default()
        }).unwrap();

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_d_adds_d_to_accumulator() {
        let state = emulate(State {
            memory: vec![0x82],
            a: 1,
            d: 2,
            ..State::default()
        }).unwrap();

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_e_adds_e_to_accumulator() {
        let state = emulate(State {
            memory: vec![0x83],
            a: 1,
            e: 2,
            ..State::default()
        }).unwrap();

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_h_adds_h_to_accumulator() {
        let state = emulate(State {
            memory: vec![0x84],
            a: 1,
            h: 2,
            ..State::default()
        }).unwrap();

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_l_adds_l_to_accumulator() {
        let state = emulate(State {
            memory: vec![0x85],
            a: 1,
            l: 2,
            ..State::default()
        }).unwrap();

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_a_adds_a_to_accumulator() {
        let state = emulate(State {
            memory: vec![0x87],
            a: 1,
            ..State::default()
        }).unwrap();

        assert_eq!(state.a, 2);
    }
}
