mod adc;
mod add;
mod lxi;
mod sbb;
mod sub;

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

    pub fn nop(&mut self) -> () {
        // 4 instructions
        ()
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
}
