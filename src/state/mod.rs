mod add;
mod add_with_carry;
mod and;
mod carry;
mod compare;
mod complement;
mod daa;
mod decrement;
mod exchange;
mod full_test;
mod increment;
mod load;
mod mov;
mod or;
mod pop;
mod push;
mod rotate;
mod store;
mod sub;
mod sub_with_borrow;
mod xor;

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

    pub fn write_byte(&mut self, byte: u8) -> () {
        if self.pc == 0 {
            panic!("Writing out of bounds!")
        }
        self.pc -= 1;
        self.memory[self.pc as usize] = byte;
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

            // POP ?
            Some(0xC1) => self.pop(Register::B),
            Some(0xD1) => self.pop(Register::D),
            Some(0xE1) => self.pop(Register::H),
            Some(0xF1) => self.pop(Register::PSW),

            // PUSH ?
            Some(0xC5) => self.push(Register::B),
            Some(0xD5) => self.push(Register::D),
            Some(0xE5) => self.push(Register::H),
            Some(0xF5) => self.push(Register::PSW),

            // DAD ?
            Some(0x09) => self.dad(Register::B),
            Some(0x19) => self.dad(Register::D),
            Some(0x29) => self.dad(Register::H),
            Some(0x39) => self.dad(Register::SP),

            // INX ?
            Some(0x03) => self.inx(Register::B),
            Some(0x13) => self.inx(Register::D),
            Some(0x23) => self.inx(Register::H),
            Some(0x33) => self.inx(Register::SP),

            // DCX ?
            Some(0x0B) => self.dcx(Register::B),
            Some(0x1B) => self.dcx(Register::D),
            Some(0x2B) => self.dcx(Register::H),
            Some(0x3B) => self.dcx(Register::SP),

            // STAX ?
            Some(0x02) => self.stax(Register::B),
            Some(0x12) => self.stax(Register::D),

            // LDAX ?
            Some(0x0A) => self.ldax(Register::B),
            Some(0x1A) => self.ldax(Register::D),

            // Instructions without registers
            // ADI d8
            Some(0xC6) => self.adi(),
            // SUI d8
            Some(0xD6) => self.sui(),
            // ANI d8
            Some(0xE6) => self.ani(),
            // ORI d8
            Some(0xF6) => self.ori(),
            // ACI d8
            Some(0xCE) => self.aci(),
            // SBI d8
            Some(0xDE) => self.sbi(),
            // XRI d8
            Some(0xEE) => self.xri(),
            // CPI d8
            Some(0xFE) => self.cpi(),

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

            // Exchange registers
            Some(0xEB) => self.xchg(),

            // Exchange stack
            Some(0xE3) => self.xthl(),

            // Load SP from H and L
            Some(0xF9) => self.sphl(),

            // Store accumulator direct a16
            Some(0x32) => self.sta(),

            // Load accumulator direct a16
            Some(0x3A) => self.lda(),

            // Store H and L direct a16
            Some(0x22) => self.shld(),

            // Load H and L direct a16
            Some(0x2A) => self.lhld(),

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
    fn write_byte_writes_byte_and_decrements_pc() {
        let mut state = State {
            memory: vec![0x01, 0x02, 0x03],
            pc: 3,
            ..State::default()
        };

        state.write_byte(0xFF);
        assert_eq!(state.memory, vec![0x01, 0x02, 0xFF]);
        assert_eq!(state.pc, 2);

        state.write_byte(0xFF);
        assert_eq!(state.memory, vec![0x01, 0xFF, 0xFF]);
        assert_eq!(state.pc, 1);

        state.write_byte(0xFF);
        assert_eq!(state.memory, vec![0xFF, 0xFF, 0xFF]);
        assert_eq!(state.pc, 0);
    }
}
