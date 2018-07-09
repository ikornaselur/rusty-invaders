mod adc;
mod add;
mod ana;
mod carry;
mod cma;
mod cmp;
mod daa;
mod dcr;
mod inr;
mod lxi;
mod mov;
mod mvi;
mod ora;
mod rotate;
mod sbb;
mod sub;
mod xra;

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
    pub fn new(memory: Vec<u8>) -> State {
        State {
            memory: memory,
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

    fn set_flags(&mut self, byte: u8, carry: bool) -> () {
        self.cc.carry = carry;
        self.cc.zero = byte == 0u8;
        self.cc.sign = (byte & 0x80) != 0;
        self.cc.parity = byte.count_ones() % 2 == 0;
    }

    pub fn nop(&mut self) -> () {
        // 4 cycles
        ()
    }

    pub fn step(&mut self) -> Option<()> {
        match self.read_byte() {
            // NOPs
            Some(0x00) => self.nop(),
            Some(0x08) => self.nop(),
            Some(0x10) => self.nop(),
            Some(0x18) => self.nop(),
            Some(0x20) => self.nop(),
            Some(0x28) => self.nop(),
            Some(0x30) => self.nop(),
            Some(0x38) => self.nop(),

            // Instructions with registers

            // LXI ?,d16
            Some(0x01) => self.lxi(Register::B),
            Some(0x11) => self.lxi(Register::D),
            Some(0x21) => self.lxi(Register::H),
            Some(0x31) => self.lxi(Register::SP),

            // INR ?
            Some(0x04) => self.inr(Register::B),
            Some(0x14) => self.inr(Register::D),
            Some(0x24) => self.inr(Register::H),
            Some(0x34) => self.inr(Register::M),
            Some(0x0C) => self.inr(Register::C),
            Some(0x1C) => self.inr(Register::E),
            Some(0x2C) => self.inr(Register::L),
            Some(0x3C) => self.inr(Register::A),

            // DCR ?
            Some(0x05) => self.dcr(Register::B),
            Some(0x15) => self.dcr(Register::D),
            Some(0x25) => self.dcr(Register::H),
            Some(0x35) => self.dcr(Register::M),
            Some(0x0D) => self.dcr(Register::C),
            Some(0x1D) => self.dcr(Register::E),
            Some(0x2D) => self.dcr(Register::L),
            Some(0x3D) => self.dcr(Register::A),

            // MVI ?, d8
            Some(0x06) => self.mvi(Register::B),
            Some(0x0E) => self.mvi(Register::C),
            Some(0x16) => self.mvi(Register::D),
            Some(0x1E) => self.mvi(Register::E),
            Some(0x26) => self.mvi(Register::H),
            Some(0x2E) => self.mvi(Register::L),
            Some(0x36) => self.mvi(Register::M),
            Some(0x3E) => self.mvi(Register::A),

            // MOV ?, ?
            Some(0x40) => self.mov(Register::B, Register::B),
            Some(0x41) => self.mov(Register::B, Register::C),
            Some(0x42) => self.mov(Register::B, Register::D),
            Some(0x43) => self.mov(Register::B, Register::E),
            Some(0x44) => self.mov(Register::B, Register::H),
            Some(0x45) => self.mov(Register::B, Register::L),
            Some(0x46) => self.mov(Register::B, Register::M),
            Some(0x47) => self.mov(Register::B, Register::A),
            Some(0x48) => self.mov(Register::C, Register::B),
            Some(0x49) => self.mov(Register::C, Register::C),
            Some(0x4A) => self.mov(Register::C, Register::D),
            Some(0x4B) => self.mov(Register::C, Register::E),
            Some(0x4C) => self.mov(Register::C, Register::H),
            Some(0x4D) => self.mov(Register::C, Register::L),
            Some(0x4E) => self.mov(Register::C, Register::M),
            Some(0x4F) => self.mov(Register::C, Register::A),

            Some(0x50) => self.mov(Register::D, Register::B),
            Some(0x51) => self.mov(Register::D, Register::C),
            Some(0x52) => self.mov(Register::D, Register::D),
            Some(0x53) => self.mov(Register::D, Register::E),
            Some(0x54) => self.mov(Register::D, Register::H),
            Some(0x55) => self.mov(Register::D, Register::L),
            Some(0x56) => self.mov(Register::D, Register::M),
            Some(0x57) => self.mov(Register::D, Register::A),
            Some(0x58) => self.mov(Register::E, Register::B),
            Some(0x59) => self.mov(Register::E, Register::C),
            Some(0x5A) => self.mov(Register::E, Register::D),
            Some(0x5B) => self.mov(Register::E, Register::E),
            Some(0x5C) => self.mov(Register::E, Register::H),
            Some(0x5D) => self.mov(Register::E, Register::L),
            Some(0x5E) => self.mov(Register::E, Register::M),
            Some(0x5F) => self.mov(Register::E, Register::A),

            Some(0x60) => self.mov(Register::H, Register::B),
            Some(0x61) => self.mov(Register::H, Register::C),
            Some(0x62) => self.mov(Register::H, Register::D),
            Some(0x63) => self.mov(Register::H, Register::E),
            Some(0x64) => self.mov(Register::H, Register::H),
            Some(0x65) => self.mov(Register::H, Register::L),
            Some(0x66) => self.mov(Register::H, Register::M),
            Some(0x67) => self.mov(Register::H, Register::A),
            Some(0x68) => self.mov(Register::L, Register::B),
            Some(0x69) => self.mov(Register::L, Register::C),
            Some(0x6A) => self.mov(Register::L, Register::D),
            Some(0x6B) => self.mov(Register::L, Register::E),
            Some(0x6C) => self.mov(Register::L, Register::H),
            Some(0x6D) => self.mov(Register::L, Register::L),
            Some(0x6E) => self.mov(Register::L, Register::M),
            Some(0x6F) => self.mov(Register::L, Register::A),

            Some(0x70) => self.mov(Register::M, Register::B),
            Some(0x71) => self.mov(Register::M, Register::C),
            Some(0x72) => self.mov(Register::M, Register::D),
            Some(0x73) => self.mov(Register::M, Register::E),
            Some(0x74) => self.mov(Register::M, Register::H),
            Some(0x75) => self.mov(Register::M, Register::L),
            // Some(0x76) => self.hlt(),
            Some(0x77) => self.mov(Register::M, Register::A),
            Some(0x78) => self.mov(Register::A, Register::B),
            Some(0x79) => self.mov(Register::A, Register::C),
            Some(0x7A) => self.mov(Register::A, Register::D),
            Some(0x7B) => self.mov(Register::A, Register::E),
            Some(0x7C) => self.mov(Register::A, Register::H),
            Some(0x7D) => self.mov(Register::A, Register::L),
            Some(0x7E) => self.mov(Register::A, Register::M),
            Some(0x7F) => self.mov(Register::A, Register::A),

            // ADD ?
            Some(0x80) => self.add(Register::B),
            Some(0x81) => self.add(Register::C),
            Some(0x82) => self.add(Register::D),
            Some(0x83) => self.add(Register::E),
            Some(0x84) => self.add(Register::H),
            Some(0x85) => self.add(Register::L),
            Some(0x86) => self.add(Register::M),
            Some(0x87) => self.add(Register::A),

            // ADC ?
            Some(0x88) => self.adc(Register::B),
            Some(0x89) => self.adc(Register::C),
            Some(0x8A) => self.adc(Register::D),
            Some(0x8B) => self.adc(Register::E),
            Some(0x8C) => self.adc(Register::H),
            Some(0x8D) => self.adc(Register::L),
            Some(0x8E) => self.adc(Register::M),
            Some(0x8F) => self.adc(Register::A),

            // SUB ?
            Some(0x90) => self.sub(Register::B),
            Some(0x91) => self.sub(Register::C),
            Some(0x92) => self.sub(Register::D),
            Some(0x93) => self.sub(Register::E),
            Some(0x94) => self.sub(Register::H),
            Some(0x95) => self.sub(Register::L),
            Some(0x96) => self.sub(Register::M),
            Some(0x97) => self.sub(Register::A),

            // SBB ?
            Some(0x98) => self.sbb(Register::B),
            Some(0x99) => self.sbb(Register::C),
            Some(0x9A) => self.sbb(Register::D),
            Some(0x9B) => self.sbb(Register::E),
            Some(0x9C) => self.sbb(Register::H),
            Some(0x9D) => self.sbb(Register::L),
            Some(0x9E) => self.sbb(Register::M),
            Some(0x9F) => self.sbb(Register::A),

            // ANA ?
            Some(0xA0) => self.ana(Register::B),
            Some(0xA1) => self.ana(Register::C),
            Some(0xA2) => self.ana(Register::D),
            Some(0xA3) => self.ana(Register::E),
            Some(0xA4) => self.ana(Register::H),
            Some(0xA5) => self.ana(Register::L),
            Some(0xA6) => self.ana(Register::M),
            Some(0xA7) => self.ana(Register::A),

            // XRA ?
            Some(0xA8) => self.xra(Register::B),
            Some(0xA9) => self.xra(Register::C),
            Some(0xAA) => self.xra(Register::D),
            Some(0xAB) => self.xra(Register::E),
            Some(0xAC) => self.xra(Register::H),
            Some(0xAD) => self.xra(Register::L),
            Some(0xAE) => self.xra(Register::M),
            Some(0xAF) => self.xra(Register::A),

            // ORA ?
            Some(0xB0) => self.ora(Register::B),
            Some(0xB1) => self.ora(Register::C),
            Some(0xB2) => self.ora(Register::D),
            Some(0xB3) => self.ora(Register::E),
            Some(0xB4) => self.ora(Register::H),
            Some(0xB5) => self.ora(Register::L),
            Some(0xB6) => self.ora(Register::M),
            Some(0xB7) => self.ora(Register::A),

            // CMP ?
            Some(0xB8) => self.cmp(Register::B),
            Some(0xB9) => self.cmp(Register::C),
            Some(0xBA) => self.cmp(Register::D),
            Some(0xBB) => self.cmp(Register::E),
            Some(0xBC) => self.cmp(Register::H),
            Some(0xBD) => self.cmp(Register::L),
            Some(0xBE) => self.cmp(Register::M),
            Some(0xBF) => self.cmp(Register::A),

            // Instructions without registers

            // Rotate accumulator
            Some(0x07) => self.rlc(),
            Some(0x0F) => self.rrc(),
            Some(0x17) => self.ral(),
            Some(0x1F) => self.rar(),

            // Decimal Adjustment Accumulator
            Some(0x27) => self.daa(),

            // Set carry
            Some(0x37) => self.stc(),

            // Complement accumulator
            Some(0x2F) => self.cma(),

            // Complement carry
            Some(0x3F) => self.cmc(),

            Some(byte) => {
                panic!("Unknown OP: 0x{:02X?}", byte);
            }
            None => {
                return None;
            }
        };
        Some(())
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
    fn read_bytes_increases_pc() {
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
    fn full_step_test() {
        let mut state = State { ..State::default() };
        let mut manual_state = State { ..State::default() };

        // NOP
        state.memory.push(0x00);
        manual_state.memory.push(0x00);
        state.step();

        manual_state.pc = 1;
        assert_eq!(state, manual_state);

        state.memory.push(0x08);
        manual_state.memory.push(0x08);
        state.step();

        manual_state.pc = 2;
        assert_eq!(state, manual_state);

        state.memory.push(0x10);
        manual_state.memory.push(0x10);
        state.step();

        manual_state.pc = 3;
        assert_eq!(state, manual_state);

        state.memory.push(0x18);
        manual_state.memory.push(0x18);
        state.step();

        manual_state.pc = 4;
        assert_eq!(state, manual_state);

        state.memory.push(0x20);
        manual_state.memory.push(0x20);
        state.step();

        manual_state.pc = 5;
        assert_eq!(state, manual_state);

        state.memory.push(0x28);
        manual_state.memory.push(0x28);
        state.step();

        manual_state.pc = 6;
        assert_eq!(state, manual_state);

        state.memory.push(0x30);
        manual_state.memory.push(0x30);
        state.step();

        manual_state.pc = 7;
        assert_eq!(state, manual_state);

        state.memory.push(0x38);
        manual_state.memory.push(0x38);
        state.step();

        manual_state.pc = 8;
        assert_eq!(state, manual_state);

        // LXI B,d16
        state.memory.push(0x01);
        manual_state.memory.push(0x01);
        state.memory.push(0xAD);
        manual_state.memory.push(0xAD);
        state.memory.push(0xDE);
        manual_state.memory.push(0xDE);
        state.step();

        manual_state.pc = 11;
        manual_state.b = 0xDE;
        manual_state.c = 0xAD;
        assert_eq!(state, manual_state);

        // LXI D,d16
        state.memory.push(0x11);
        manual_state.memory.push(0x11);
        state.memory.push(0xAD);
        manual_state.memory.push(0xAD);
        state.memory.push(0xDE);
        manual_state.memory.push(0xDE);
        state.step();

        manual_state.pc = 14;
        manual_state.d = 0xDE;
        manual_state.e = 0xAD;
        assert_eq!(state, manual_state);

        // LXI H,d16
        state.memory.push(0x21);
        manual_state.memory.push(0x21);
        state.memory.push(0xAD);
        manual_state.memory.push(0xAD);
        state.memory.push(0xDE);
        manual_state.memory.push(0xDE);
        state.step();

        manual_state.pc = 17;
        manual_state.h = 0xDE;
        manual_state.l = 0xAD;
        assert_eq!(state, manual_state);

        // LXI SP,d16
        state.memory.push(0x31);
        manual_state.memory.push(0x31);
        state.memory.push(0xAD);
        manual_state.memory.push(0xAD);
        state.memory.push(0xDE);
        manual_state.memory.push(0xDE);
        state.step();

        manual_state.pc = 20;
        manual_state.sp = 0xDEAD;
        assert_eq!(state, manual_state);

        // INR B
        state.memory.push(0x04);
        manual_state.memory.push(0x04);
        state.step();

        manual_state.pc = 21;
        manual_state.b = 0xDF;
        manual_state.cc.sign = true;
        assert_eq!(state, manual_state);

        // INR D
        state.memory.push(0x14);
        manual_state.memory.push(0x14);
        state.step();

        manual_state.pc = 22;
        manual_state.d = 0xDF;
        assert_eq!(state, manual_state);

        // INR H
        state.memory.push(0x24);
        manual_state.memory.push(0x24);
        state.step();

        manual_state.pc = 23;
        manual_state.h = 0xDF;
        assert_eq!(state, manual_state);

        // INR M
        state.memory.push(0x34);
        manual_state.memory.push(0x34);
        state.h = 0x00;
        manual_state.h = 0x00;
        state.l = 0x05;
        manual_state.l = 0x05;
        state.memory[0x05] = 0x01;
        manual_state.memory[0x05] = 0x01;
        state.step();

        manual_state.pc = 24;
        manual_state.cc.sign = false;
        manual_state.memory[0x05] = 0x02;
        assert_eq!(state, manual_state);

        // INR C
        state.memory.push(0x0C);
        manual_state.memory.push(0x0C);
        state.step();

        manual_state.pc = 25;
        manual_state.c = 0xAE;
        manual_state.cc.sign = true;
        assert_eq!(state, manual_state);

        // INR E
        state.memory.push(0x1C);
        manual_state.memory.push(0x1C);
        state.step();

        manual_state.pc = 26;
        manual_state.e = 0xAE;
        assert_eq!(state, manual_state);

        // INR L
        state.memory.push(0x2C);
        manual_state.memory.push(0x2C);
        state.step();

        manual_state.pc = 27;
        manual_state.l = 0x06; // Because of INR M
        manual_state.cc.sign = false;
        manual_state.cc.parity = true;
        assert_eq!(state, manual_state);

        // INR A
        state.memory.push(0x3C);
        manual_state.memory.push(0x3C);
        state.step();

        manual_state.pc = 28;
        manual_state.a = 0x01;
        manual_state.cc.sign = false;
        manual_state.cc.parity = false;
        assert_eq!(state, manual_state);
    }
}
