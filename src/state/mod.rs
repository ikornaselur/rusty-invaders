mod add;
mod and;
mod call;
mod carry;
mod compare;
mod complement;
mod daa;
mod decrement;
mod exchange;
mod increment;
mod io;
mod jump;
mod load;
mod mov;
mod or;
mod pop;
mod push;
mod returns;
mod rotate;
mod store;
mod sub;
mod xor;

use io::IO;

#[derive(Debug)]
pub enum Register {
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
    PSW,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct State {
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
    exit: bool,
    debug: bool,
    input: IO,
    output: IO,
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
            cc: ConditionCodes::default(),
            int_enable: 0,
            exit: false,
            debug: false,
            input: IO::new(4),
            output: IO::new(7),
        }
    }
}

impl State {
    pub fn new(memory: Vec<u8>, debug: bool) -> State {
        State {
            memory: memory,
            debug: debug,
            ..State::default()
        }
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if self.pc as usize >= self.memory.len() {
            None
        } else {
            let byte = self.memory[self.pc as usize];
            self.pc += 1;
            Some(byte)
        }
    }

    pub fn read_address(&mut self) -> Option<u16> {
        if let Some(least) = self.read_byte() {
            if let Some(most) = self.read_byte() {
                return Some(((most as u16) << 8) + least as u16);
            }
        }
        None
    }

    pub fn read_byte_from_stack(&mut self) -> Option<u8> {
        if self.sp as usize >= self.memory.len() {
            None
        } else {
            let byte = self.memory[self.sp as usize];
            self.sp += 1;
            Some(byte)
        }
    }

    pub fn read_address_from_stack(&mut self) -> Option<u16> {
        if let Some(least) = self.read_byte_from_stack() {
            if let Some(most) = self.read_byte_from_stack() {
                return Some(((most as u16) << 8) + least as u16);
            }
        }
        None
    }

    pub fn write_byte_to_stack(&mut self, byte: u8) -> () {
        if self.sp == 0 {
            panic!("Stack pointer out of bounds!")
        }
        self.sp -= 1;
        self.memory[self.sp as usize] = byte;
    }

    fn set_flags(&mut self, byte: u8, carry: bool) -> () {
        self.cc.sign = (byte & 0x80) != 0;
        self.cc.zero = byte == 0u8;
        self.cc.parity = byte.count_ones() % 2 == 0;
        self.cc.carry = carry;
    }

    fn set_flags_from_bits(&mut self, bits: u8) -> () {
        self.cc.sign = bits & 0b1000_0000 != 0;
        self.cc.zero = bits & 0b0100_0000 != 0;
        self.cc.parity = bits & 0b0000_0100 != 0;
        self.cc.carry = bits & 0b0000_0001 != 0;
    }

    fn get_flags_as_bits(&self) -> u8 {
        let mut bits = 0;
        if self.cc.sign {
            bits |= 0b1000_0000
        }
        if self.cc.zero {
            bits |= 0b0100_0000
        }
        if self.cc.parity {
            bits |= 0b0000_0100
        }
        if self.cc.carry {
            bits |= 0b0000_0001
        }
        bits
    }

    pub fn nop(&mut self) -> Option<u8> {
        // 4 cycles
        None
    }

    pub fn step(&mut self) -> Option<(u8, Option<u8>)> {
        if self.exit {
            return None;
        }
        let byte = match self.read_byte() {
            Some(byte) => byte,
            None => {
                panic!("Got no byte when reading");
            }
        };

        let output = match byte {
            // NOPs
            0x00 => self.nop(),
            0x08 => self.nop(),
            0x10 => self.nop(),
            0x18 => self.nop(),
            0x20 => self.nop(),
            0x28 => self.nop(),
            0x30 => self.nop(),
            0x38 => self.nop(),

            // Instructions with registers

            // LXI ?,d16
            0x01 => self.lxi(Register::B),
            0x11 => self.lxi(Register::D),
            0x21 => self.lxi(Register::H),
            0x31 => self.lxi(Register::SP),

            // INR ?
            0x04 => self.inr(Register::B),
            0x14 => self.inr(Register::D),
            0x24 => self.inr(Register::H),
            0x34 => self.inr(Register::M),
            0x0C => self.inr(Register::C),
            0x1C => self.inr(Register::E),
            0x2C => self.inr(Register::L),
            0x3C => self.inr(Register::A),

            // DCR ?
            0x05 => self.dcr(Register::B),
            0x15 => self.dcr(Register::D),
            0x25 => self.dcr(Register::H),
            0x35 => self.dcr(Register::M),
            0x0D => self.dcr(Register::C),
            0x1D => self.dcr(Register::E),
            0x2D => self.dcr(Register::L),
            0x3D => self.dcr(Register::A),

            // MVI ?, d8
            0x06 => self.mvi(Register::B),
            0x0E => self.mvi(Register::C),
            0x16 => self.mvi(Register::D),
            0x1E => self.mvi(Register::E),
            0x26 => self.mvi(Register::H),
            0x2E => self.mvi(Register::L),
            0x36 => self.mvi(Register::M),
            0x3E => self.mvi(Register::A),

            // MOV ?, ?
            0x40 => self.mov(Register::B, Register::B),
            0x41 => self.mov(Register::B, Register::C),
            0x42 => self.mov(Register::B, Register::D),
            0x43 => self.mov(Register::B, Register::E),
            0x44 => self.mov(Register::B, Register::H),
            0x45 => self.mov(Register::B, Register::L),
            0x46 => self.mov(Register::B, Register::M),
            0x47 => self.mov(Register::B, Register::A),
            0x48 => self.mov(Register::C, Register::B),
            0x49 => self.mov(Register::C, Register::C),
            0x4A => self.mov(Register::C, Register::D),
            0x4B => self.mov(Register::C, Register::E),
            0x4C => self.mov(Register::C, Register::H),
            0x4D => self.mov(Register::C, Register::L),
            0x4E => self.mov(Register::C, Register::M),
            0x4F => self.mov(Register::C, Register::A),

            0x50 => self.mov(Register::D, Register::B),
            0x51 => self.mov(Register::D, Register::C),
            0x52 => self.mov(Register::D, Register::D),
            0x53 => self.mov(Register::D, Register::E),
            0x54 => self.mov(Register::D, Register::H),
            0x55 => self.mov(Register::D, Register::L),
            0x56 => self.mov(Register::D, Register::M),
            0x57 => self.mov(Register::D, Register::A),
            0x58 => self.mov(Register::E, Register::B),
            0x59 => self.mov(Register::E, Register::C),
            0x5A => self.mov(Register::E, Register::D),
            0x5B => self.mov(Register::E, Register::E),
            0x5C => self.mov(Register::E, Register::H),
            0x5D => self.mov(Register::E, Register::L),
            0x5E => self.mov(Register::E, Register::M),
            0x5F => self.mov(Register::E, Register::A),

            0x60 => self.mov(Register::H, Register::B),
            0x61 => self.mov(Register::H, Register::C),
            0x62 => self.mov(Register::H, Register::D),
            0x63 => self.mov(Register::H, Register::E),
            0x64 => self.mov(Register::H, Register::H),
            0x65 => self.mov(Register::H, Register::L),
            0x66 => self.mov(Register::H, Register::M),
            0x67 => self.mov(Register::H, Register::A),
            0x68 => self.mov(Register::L, Register::B),
            0x69 => self.mov(Register::L, Register::C),
            0x6A => self.mov(Register::L, Register::D),
            0x6B => self.mov(Register::L, Register::E),
            0x6C => self.mov(Register::L, Register::H),
            0x6D => self.mov(Register::L, Register::L),
            0x6E => self.mov(Register::L, Register::M),
            0x6F => self.mov(Register::L, Register::A),

            0x70 => self.mov(Register::M, Register::B),
            0x71 => self.mov(Register::M, Register::C),
            0x72 => self.mov(Register::M, Register::D),
            0x73 => self.mov(Register::M, Register::E),
            0x74 => self.mov(Register::M, Register::H),
            0x75 => self.mov(Register::M, Register::L),
            // Some(0x76) => self.hlt(),
            0x77 => self.mov(Register::M, Register::A),
            0x78 => self.mov(Register::A, Register::B),
            0x79 => self.mov(Register::A, Register::C),
            0x7A => self.mov(Register::A, Register::D),
            0x7B => self.mov(Register::A, Register::E),
            0x7C => self.mov(Register::A, Register::H),
            0x7D => self.mov(Register::A, Register::L),
            0x7E => self.mov(Register::A, Register::M),
            0x7F => self.mov(Register::A, Register::A),

            // ADD ?
            0x80 => self.add(Register::B),
            0x81 => self.add(Register::C),
            0x82 => self.add(Register::D),
            0x83 => self.add(Register::E),
            0x84 => self.add(Register::H),
            0x85 => self.add(Register::L),
            0x86 => self.add(Register::M),
            0x87 => self.add(Register::A),

            // ADC ?
            0x88 => self.adc(Register::B),
            0x89 => self.adc(Register::C),
            0x8A => self.adc(Register::D),
            0x8B => self.adc(Register::E),
            0x8C => self.adc(Register::H),
            0x8D => self.adc(Register::L),
            0x8E => self.adc(Register::M),
            0x8F => self.adc(Register::A),

            // SUB ?
            0x90 => self.sub(Register::B),
            0x91 => self.sub(Register::C),
            0x92 => self.sub(Register::D),
            0x93 => self.sub(Register::E),
            0x94 => self.sub(Register::H),
            0x95 => self.sub(Register::L),
            0x96 => self.sub(Register::M),
            0x97 => self.sub(Register::A),

            // SBB ?
            0x98 => self.sbb(Register::B),
            0x99 => self.sbb(Register::C),
            0x9A => self.sbb(Register::D),
            0x9B => self.sbb(Register::E),
            0x9C => self.sbb(Register::H),
            0x9D => self.sbb(Register::L),
            0x9E => self.sbb(Register::M),
            0x9F => self.sbb(Register::A),

            // ANA ?
            0xA0 => self.ana(Register::B),
            0xA1 => self.ana(Register::C),
            0xA2 => self.ana(Register::D),
            0xA3 => self.ana(Register::E),
            0xA4 => self.ana(Register::H),
            0xA5 => self.ana(Register::L),
            0xA6 => self.ana(Register::M),
            0xA7 => self.ana(Register::A),

            // XRA ?
            0xA8 => self.xra(Register::B),
            0xA9 => self.xra(Register::C),
            0xAA => self.xra(Register::D),
            0xAB => self.xra(Register::E),
            0xAC => self.xra(Register::H),
            0xAD => self.xra(Register::L),
            0xAE => self.xra(Register::M),
            0xAF => self.xra(Register::A),

            // ORA ?
            0xB0 => self.ora(Register::B),
            0xB1 => self.ora(Register::C),
            0xB2 => self.ora(Register::D),
            0xB3 => self.ora(Register::E),
            0xB4 => self.ora(Register::H),
            0xB5 => self.ora(Register::L),
            0xB6 => self.ora(Register::M),
            0xB7 => self.ora(Register::A),

            // CMP ?
            0xB8 => self.cmp(Register::B),
            0xB9 => self.cmp(Register::C),
            0xBA => self.cmp(Register::D),
            0xBB => self.cmp(Register::E),
            0xBC => self.cmp(Register::H),
            0xBD => self.cmp(Register::L),
            0xBE => self.cmp(Register::M),
            0xBF => self.cmp(Register::A),

            // POP ?
            0xC1 => self.pop(Register::B),
            0xD1 => self.pop(Register::D),
            0xE1 => self.pop(Register::H),
            0xF1 => self.pop(Register::PSW),

            // PUSH ?
            0xC5 => self.push(Register::B),
            0xD5 => self.push(Register::D),
            0xE5 => self.push(Register::H),
            0xF5 => self.push(Register::PSW),

            // DAD ?
            0x09 => self.dad(Register::B),
            0x19 => self.dad(Register::D),
            0x29 => self.dad(Register::H),
            0x39 => self.dad(Register::SP),

            // INX ?
            0x03 => self.inx(Register::B),
            0x13 => self.inx(Register::D),
            0x23 => self.inx(Register::H),
            0x33 => self.inx(Register::SP),

            // DCX ?
            0x0B => self.dcx(Register::B),
            0x1B => self.dcx(Register::D),
            0x2B => self.dcx(Register::H),
            0x3B => self.dcx(Register::SP),

            // STAX ?
            0x02 => self.stax(Register::B),
            0x12 => self.stax(Register::D),

            // LDAX ?
            0x0A => self.ldax(Register::B),
            0x1A => self.ldax(Register::D),

            // Instructions without registers
            // ADI d8
            0xC6 => self.adi(),
            // SUI d8
            0xD6 => self.sui(),
            // ANI d8
            0xE6 => self.ani(),
            // ORI d8
            0xF6 => self.ori(),
            // ACI d8
            0xCE => self.aci(),
            // SBI d8
            0xDE => self.sbi(),
            // XRI d8
            0xEE => self.xri(),
            // CPI d8
            0xFE => self.cpi(),

            // Rotate accumulator
            0x07 => self.rlc(),
            0x0F => self.rrc(),
            0x17 => self.ral(),
            0x1F => self.rar(),

            // Decimal Adjustment Accumulator
            0x27 => self.daa(),

            // Set carry
            0x37 => self.stc(),

            // Complement accumulator
            0x2F => self.cma(),

            // Complement carry
            0x3F => self.cmc(),

            // Exchange registers
            0xEB => self.xchg(),

            // Exchange stack
            0xE3 => self.xthl(),

            // Load SP from H and L
            0xF9 => self.sphl(),

            // Store accumulator direct a16
            0x32 => self.sta(),

            // Load accumulator direct a16
            0x3A => self.lda(),

            // Store H and L direct a16
            0x22 => self.shld(),

            // Load H and L direct a16
            0x2A => self.lhld(),

            // Load Program Counter
            0xE9 => self.pchl(),

            // Jumps
            0xC3 => self.jmp(),
            0xCB => self.jmp(),
            0xDA => self.jc(),  // Jump if carry
            0xD2 => self.jnc(), // Jump if no carry
            0xCA => self.jz(),  // Jump if zero
            0xC2 => self.jnz(), // Jump if not zero
            0xFA => self.jm(),  // Jump if minus
            0xF2 => self.jp(),  // Jump if positive
            0xEA => self.jpe(), // Jump if parity even
            0xE2 => self.jpo(), // Jump if parity odd

            // Calls
            0xCD => self.call(),
            0xDD => self.call(),
            0xED => self.call(),
            0xFD => self.call(),

            0xDC => self.cc(),  // Call if carry
            0xD4 => self.cnc(), // Call if no carry
            0xCC => self.cz(),  // Call if zero
            0xC4 => self.cnz(), // Call if not zero
            0xFC => self.cm(),  // Call if minus
            0xF4 => self.cp(),  // Call if plus
            0xEC => self.cpe(), // Call if parity even
            0xE4 => self.cpo(), // Call if parity odd

            // Returns
            0xC9 => self.ret(),
            0xD9 => self.ret(),

            0xD8 => self.rc(),  // Return if carry
            0xD0 => self.rnc(), // Return if no carry
            0xC8 => self.rz(),  // Return if zero
            0xC0 => self.rnz(), // Return if not zero
            0xF8 => self.rm(),  // Return if minus
            0xF0 => self.rp(),  // Return if plus
            0xE8 => self.rpe(), // Return if parity even
            0xE0 => self.rpo(), // Return if parity odd

            // IO
            0xD3 => self.output(),
            0xDB => self.input(),

            byte => {
                panic!("Unknown OP: 0x{:02X?}", byte);
            }
        };
        Some((byte, output))
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

        state.set_flags(odd1, false);
        assert_eq!(state.cc.parity, false);

        state.set_flags(odd2, false);
        assert_eq!(state.cc.parity, false);

        state.set_flags(odd3, false);
        assert_eq!(state.cc.parity, false);
    }

    #[test]
    fn set_flags_from_bits_sets_flags() {
        let mut state = State::default();

        let sign = 0b1000_0000;
        let zero = 0b0100_0000;
        let parity = 0b0000_0100;
        let carry = 0b0000_0001;

        state.set_flags_from_bits(sign);
        assert_eq!(state.cc.sign, true);
        assert_eq!(state.cc.zero, false);
        assert_eq!(state.cc.parity, false);
        assert_eq!(state.cc.carry, false);

        state.set_flags_from_bits(zero);
        assert_eq!(state.cc.sign, false);
        assert_eq!(state.cc.zero, true);
        assert_eq!(state.cc.parity, false);
        assert_eq!(state.cc.carry, false);

        state.set_flags_from_bits(parity);
        assert_eq!(state.cc.sign, false);
        assert_eq!(state.cc.zero, false);
        assert_eq!(state.cc.parity, true);
        assert_eq!(state.cc.carry, false);

        state.set_flags_from_bits(carry);
        assert_eq!(state.cc.sign, false);
        assert_eq!(state.cc.zero, false);
        assert_eq!(state.cc.parity, false);
        assert_eq!(state.cc.carry, true);

        state.set_flags_from_bits(0b1111_1111);
        assert_eq!(state.cc.sign, true);
        assert_eq!(state.cc.zero, true);
        assert_eq!(state.cc.parity, true);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn get_flags_as_bits_returns_correct_bits() {
        let mut state = State::default();

        assert_eq!(state.get_flags_as_bits(), 0b0000_0000);

        state.cc.carry = true;

        assert_eq!(state.get_flags_as_bits(), 0b0000_0001);

        state.cc.parity = true;

        assert_eq!(state.get_flags_as_bits(), 0b0000_0101);

        state.cc.zero = true;

        assert_eq!(state.get_flags_as_bits(), 0b0100_0101);

        state.cc.sign = true;

        assert_eq!(state.get_flags_as_bits(), 0b1100_0101);
    }

    #[test]
    fn read_byte_returns_byte_and_increases_pc() {
        let mut state = State {
            memory: vec![0x01, 0x02],
            pc: 0,
            ..State::default()
        };

        let byte = state.read_byte();
        assert_eq!(byte, Some(0x01));
        assert_eq!(state.pc, 1);

        let byte = state.read_byte();
        assert_eq!(byte, Some(0x02));
        assert_eq!(state.pc, 2);

        let byte = state.read_byte();
        assert_eq!(byte, None);
        assert_eq!(state.pc, 2);
    }

    #[test]
    fn read_byte_from_stack_returns_byte_and_increases_sp() {
        let mut state = State {
            memory: vec![0x01, 0x02],
            sp: 0,
            ..State::default()
        };

        let byte = state.read_byte_from_stack();
        assert_eq!(byte, Some(0x01));
        assert_eq!(state.sp, 1);

        let byte = state.read_byte_from_stack();
        assert_eq!(byte, Some(0x02));
        assert_eq!(state.sp, 2);

        let byte = state.read_byte_from_stack();
        assert_eq!(byte, None);
        assert_eq!(state.sp, 2);
    }

    #[test]
    fn read_address_returns_double_and_increases_pc() {
        let mut state = State {
            memory: vec![0x01, 0x02, 0x03, 0x04],
            pc: 0,
            ..State::default()
        };

        let address = state.read_address();
        assert_eq!(address, Some(0x0201));
        assert_eq!(state.pc, 2);

        let address = state.read_address();
        assert_eq!(address, Some(0x0403));
        assert_eq!(state.pc, 4);

        let address = state.read_address();
        assert_eq!(address, None);
    }

    #[test]
    fn write_byte_to_stack_writes_byte_and_decrements_pc() {
        let mut state = State {
            memory: vec![0x01, 0x02, 0x03],
            sp: 3,
            ..State::default()
        };

        state.write_byte_to_stack(0xFF);
        assert_eq!(state.memory, vec![0x01, 0x02, 0xFF]);
        assert_eq!(state.sp, 2);

        state.write_byte_to_stack(0xFF);
        assert_eq!(state.memory, vec![0x01, 0xFF, 0xFF]);
        assert_eq!(state.sp, 1);

        state.write_byte_to_stack(0xFF);
        assert_eq!(state.memory, vec![0xFF, 0xFF, 0xFF]);
        assert_eq!(state.sp, 0);
    }
}
