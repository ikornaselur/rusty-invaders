mod load;
mod mov;
mod or;
mod pop;
mod push;
mod restart;
mod returns;
mod rotate;
mod store;
mod sub;
mod xor;

use cpu::instructions::addition::{aci, adc, add, adi, dad};
use cpu::instructions::and::{ana, ani};
use cpu::instructions::call::{call, cc, cm, cnc, cnz, cp, cpe, cpo, cz};
use cpu::instructions::carry::{cmc, stc};
use cpu::instructions::compare::{cmp, cpi};
use cpu::instructions::complement::cma;
use cpu::instructions::daa::daa;
use cpu::instructions::decrement::{dcr, dcx};
use cpu::instructions::exchange::{xchg, xthl};
use cpu::instructions::increment::{inr, inx};
use cpu::instructions::interrupt::{di, ei};
use cpu::instructions::io::{input, output};
use cpu::instructions::jump::{jc, jm, jmp, jnc, jnz, jp, jpe, jpo, jz};

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
pub(crate) struct ConditionCodes {
    pub(crate) zero: bool,   // Zero - when arithmetic result is 0
    pub(crate) sign: bool,   // Sign - when the most significant bit is set
    pub(crate) parity: bool, // Parity - when the answer has even parity
    pub(crate) carry: bool,  // Carry - when the instruction resulted in carry
    pub(crate) zc: u8,
    pub(crate) pad: u8,
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
    pub(crate) a: u8,
    pub(crate) b: u8,
    pub(crate) c: u8,
    pub(crate) d: u8,
    pub(crate) e: u8,
    pub(crate) h: u8,
    pub(crate) l: u8,
    pub(crate) sp: u16,
    pub(crate) pc: u16,
    pub(crate) memory: Vec<u8>,
    pub(crate) cc: ConditionCodes,
    pub(crate) int_enabled: bool,
    pub(crate) exit: bool,
    pub(crate) debug: bool,
    pub(crate) io: IO,
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
            int_enabled: false,
            exit: false,
            debug: false,
            io: IO::new(7),
        }
    }
}

impl State {
    pub fn new(memory: Vec<u8>, debug: bool) -> State {
        State {
            memory,
            debug,
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
                return Some((u16::from(most) << 8) + u16::from(least));
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
                return Some((u16::from(most) << 8) + u16::from(least));
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

    pub fn get_frame(&mut self) -> &[u8] {
        &self.memory[0x2400..0x4000]
    }

    pub fn set_input(&mut self, port: usize, byte: u8) -> () {
        if port < 1 || port > 2 {
            panic!("Can only write to port 1 and 2");
        }
        self.io.set(port, byte);
    }

    pub fn set_flags(&mut self, byte: u8, carry: bool) -> () {
        self.cc.sign = (byte & 0x80) != 0;
        self.cc.zero = byte == 0u8;
        self.cc.parity = byte.count_ones() % 2 == 0;
        self.cc.carry = carry;
    }

    pub fn set_flags_from_bits(&mut self, bits: u8) -> () {
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

    pub fn nop(&mut self) -> u8 {
        4
    }

    pub fn hlt(&mut self) -> u8 {
        self.exit = true;
        7
    }

    pub fn step(&mut self) -> Option<u8> {
        if self.exit {
            return None;
        }
        let byte = match self.read_byte() {
            Some(byte) => byte,
            None => {
                panic!("Got no byte when reading");
            }
        };

        let cycles = match byte {
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
            0x04 => inr(self, Register::B),
            0x14 => inr(self, Register::D),
            0x24 => inr(self, Register::H),
            0x34 => inr(self, Register::M),
            0x0C => inr(self, Register::C),
            0x1C => inr(self, Register::E),
            0x2C => inr(self, Register::L),
            0x3C => inr(self, Register::A),

            // DCR ?
            0x05 => dcr(self, Register::B),
            0x15 => dcr(self, Register::D),
            0x25 => dcr(self, Register::H),
            0x35 => dcr(self, Register::M),
            0x0D => dcr(self, Register::C),
            0x1D => dcr(self, Register::E),
            0x2D => dcr(self, Register::L),
            0x3D => dcr(self, Register::A),

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
            0x80 => add(self, Register::B),
            0x81 => add(self, Register::C),
            0x82 => add(self, Register::D),
            0x83 => add(self, Register::E),
            0x84 => add(self, Register::H),
            0x85 => add(self, Register::L),
            0x86 => add(self, Register::M),
            0x87 => add(self, Register::A),

            // ADC ?
            0x88 => adc(self, Register::B),
            0x89 => adc(self, Register::C),
            0x8A => adc(self, Register::D),
            0x8B => adc(self, Register::E),
            0x8C => adc(self, Register::H),
            0x8D => adc(self, Register::L),
            0x8E => adc(self, Register::M),
            0x8F => adc(self, Register::A),

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
            0xA0 => ana(self, Register::B),
            0xA1 => ana(self, Register::C),
            0xA2 => ana(self, Register::D),
            0xA3 => ana(self, Register::E),
            0xA4 => ana(self, Register::H),
            0xA5 => ana(self, Register::L),
            0xA6 => ana(self, Register::M),
            0xA7 => ana(self, Register::A),

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
            0xB8 => cmp(self, Register::B),
            0xB9 => cmp(self, Register::C),
            0xBA => cmp(self, Register::D),
            0xBB => cmp(self, Register::E),
            0xBC => cmp(self, Register::H),
            0xBD => cmp(self, Register::L),
            0xBE => cmp(self, Register::M),
            0xBF => cmp(self, Register::A),

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
            0x09 => dad(self, Register::B),
            0x19 => dad(self, Register::D),
            0x29 => dad(self, Register::H),
            0x39 => dad(self, Register::SP),

            // INX ?
            0x03 => inx(self, Register::B),
            0x13 => inx(self, Register::D),
            0x23 => inx(self, Register::H),
            0x33 => inx(self, Register::SP),

            // DCX ?
            0x0B => dcx(self, Register::B),
            0x1B => dcx(self, Register::D),
            0x2B => dcx(self, Register::H),
            0x3B => dcx(self, Register::SP),

            // STAX ?
            0x02 => self.stax(Register::B),
            0x12 => self.stax(Register::D),

            // LDAX ?
            0x0A => self.ldax(Register::B),
            0x1A => self.ldax(Register::D),

            // Instructions without registers
            // ADI d8
            0xC6 => adi(self),
            // SUI d8
            0xD6 => self.sui(),
            // ANI d8
            0xE6 => ani(self),
            // ORI d8
            0xF6 => self.ori(),
            // ACI d8
            0xCE => aci(self),
            // SBI d8
            0xDE => self.sbi(),
            // XRI d8
            0xEE => self.xri(),
            // CPI d8
            0xFE => cpi(self),

            // Rotate accumulator
            0x07 => self.rlc(),
            0x0F => self.rrc(),
            0x17 => self.ral(),
            0x1F => self.rar(),

            // Decimal Adjustment Accumulator
            0x27 => daa(self),

            // Set carry
            0x37 => stc(self),

            // Complement accumulator
            0x2F => cma(self),

            // Complement carry
            0x3F => cmc(self),

            // Exchange registers
            0xEB => xchg(self),

            // Exchange stack
            0xE3 => xthl(self),

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
            0xC3 => jmp(self),
            0xCB => jmp(self),
            0xDA => jc(self),  // Jump if carry
            0xD2 => jnc(self), // Jump if no carry
            0xCA => jz(self),  // Jump if zero
            0xC2 => jnz(self), // Jump if not zero
            0xFA => jm(self),  // Jump if minus
            0xF2 => jp(self),  // Jump if positive
            0xEA => jpe(self), // Jump if parity even
            0xE2 => jpo(self), // Jump if parity odd

            // Calls
            0xCD => call(self),
            0xDD => call(self),
            0xED => call(self),
            0xFD => call(self),

            0xDC => cc(self),  // Call if carry
            0xD4 => cnc(self), // Call if no carry
            0xCC => cz(self),  // Call if zero
            0xC4 => cnz(self), // Call if not zero
            0xFC => cm(self),  // Call if minus
            0xF4 => cp(self),  // Call if plus
            0xEC => cpe(self), // Call if parity even
            0xE4 => cpo(self), // Call if parity odd

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

            // Halt
            0x76 => self.hlt(),

            // IO
            0xD3 => output(self),
            0xDB => input(self),

            // Interrupts
            0xF3 => di(self),
            0xFB => ei(self),

            // Restarts
            0xC7 => self.rst(0),
            0xCF => self.rst(1),
            0xD7 => self.rst(2),
            0xDF => self.rst(3),
            0xE7 => self.rst(4),
            0xEF => self.rst(5),
            0xF7 => self.rst(6),
            0xFF => self.rst(7),

            byte => {
                panic!("Unknown OP: 0x{:02X?}", byte);
            }
        };
        Some(cycles)
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

    #[test]
    fn get_frame_returns_whole_frame_buffer() {
        let mut state = State {
            memory: vec![0x00; 0x4000],
            ..State::default()
        };

        let frame = state.get_frame();

        assert_eq!(frame.len(), 224 * 256 / 8);
    }
}
