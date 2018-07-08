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
}

impl State {
    pub fn nop(&mut self) -> () {
        // 4 instructions
        ()
    }

    pub fn lxi(&mut self, register: Register) -> () {
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

    pub fn add(&mut self, register: Register) -> () {
        // 4 instructions
        let (result, carry) = match register {
            Register::A => self.a.overflowing_add(self.a),
            Register::B => self.a.overflowing_add(self.b),
            Register::C => self.a.overflowing_add(self.c),
            Register::D => self.a.overflowing_add(self.d),
            Register::E => self.a.overflowing_add(self.e),
            Register::H => self.a.overflowing_add(self.h),
            Register::L => self.a.overflowing_add(self.l),
            Register::M => {
                let offset: u16 = ((self.h as u16) << 8) + self.l as u16;
                let byte = self.memory.get(offset as usize).unwrap();
                self.a.overflowing_add(*byte)
            }
            unsupported => {
                panic!("add doesn't support {:?}", unsupported);
            }
        };

        self.a = result;
        self.set_flags(result, carry);
    }

    pub fn adc(&mut self, register: Register) -> () {
        if !self.cc.carry {
            self.add(register)
        } else {
            // 4 instructions
            let (result, carry) = match register {
                Register::A => self.a.overflowing_add(self.a),
                Register::B => self.a.overflowing_add(self.b),
                Register::C => self.a.overflowing_add(self.c),
                Register::D => self.a.overflowing_add(self.d),
                Register::E => self.a.overflowing_add(self.e),
                Register::H => self.a.overflowing_add(self.h),
                Register::L => self.a.overflowing_add(self.l),
                Register::M => {
                    let offset: u16 = ((self.h as u16) << 8) + self.l as u16;
                    let byte = self.memory.get(offset as usize).unwrap();
                    self.a.overflowing_add(*byte)
                }
                unsupported => {
                    panic!("add doesn't support {:?}", unsupported);
                }
            };

            if !carry {
                let (result, carry) = result.overflowing_add(1);
                self.a = result;
                self.set_flags(result, carry);
            } else {
                self.a = result.wrapping_add(1);
                self.set_flags(result, carry);
            }
        }
    }

    pub fn sub(&mut self, register: Register) -> () {
        // 4 instructions
        let (result, borrow) = match register {
            Register::A => self.a.overflowing_sub(self.a),
            Register::B => self.a.overflowing_sub(self.b),
            Register::C => self.a.overflowing_sub(self.c),
            Register::D => self.a.overflowing_sub(self.d),
            Register::E => self.a.overflowing_sub(self.e),
            Register::H => self.a.overflowing_sub(self.h),
            Register::L => self.a.overflowing_sub(self.l),
            Register::M => {
                let offset: u16 = ((self.h as u16) << 8) + self.l as u16;
                let byte = self.memory.get(offset as usize).unwrap();
                self.a.overflowing_sub(*byte)
            }
            unsupported => {
                panic!("add doesn't support {:?}", unsupported);
            }
        };

        self.a = result;
        self.set_flags(result, !borrow);
    }

    pub fn sbb(&mut self, register: Register) -> () {
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

    #[test]
    fn lxi_b_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0xDE, 0xAD],
            pc: 0,
            ..State::default()
        };

        state.lxi(Register::B);

        assert_eq!(state.c, 0xDE);
        assert_eq!(state.b, 0xAD);
    }

    #[test]
    fn lxi_d_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0xDE, 0xAD],
            pc: 0,
            ..State::default()
        };

        state.lxi(Register::D);

        assert_eq!(state.e, 0xDE);
        assert_eq!(state.d, 0xAD);
    }

    #[test]
    fn lxi_h_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0xDE, 0xAD],
            pc: 0,
            ..State::default()
        };

        state.lxi(Register::H);

        assert_eq!(state.l, 0xDE);
        assert_eq!(state.h, 0xAD);
    }

    #[test]
    fn lxi_sp_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0xAD, 0xDE],
            pc: 0,
            ..State::default()
        };

        state.lxi(Register::SP);

        assert_eq!(state.sp, 0xDEAD);
    }

    #[test]
    fn add_b_adds_b_to_accumulator() {
        let mut state = State {
            a: 1,
            b: 2,
            ..State::default()
        };

        state.add(Register::B);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_c_adds_c_to_accumulator() {
        let mut state = State {
            a: 1,
            c: 2,
            ..State::default()
        };

        state.add(Register::C);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_d_adds_d_to_accumulator() {
        let mut state = State {
            a: 1,
            d: 2,
            ..State::default()
        };

        state.add(Register::D);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_e_adds_e_to_accumulator() {
        let mut state = State {
            a: 1,
            e: 2,
            ..State::default()
        };

        state.add(Register::E);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_h_adds_h_to_accumulator() {
        let mut state = State {
            a: 1,
            h: 2,
            ..State::default()
        };

        state.add(Register::H);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_l_adds_l_to_accumulator() {
        let mut state = State {
            a: 1,
            l: 2,
            ..State::default()
        };

        state.add(Register::L);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_a_adds_a_to_accumulator() {
        let mut state = State {
            a: 1,
            ..State::default()
        };

        state.add(Register::A);

        assert_eq!(state.a, 2);
    }

    #[test]
    fn add_m_adds_byte_at_hl_address_to_accumulator() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 5],
            a: 1,
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        state.add(Register::M);

        assert_eq!(state.a, 6);
    }

    #[test]
    fn adc_b_adds_b_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            b: 2,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::B);

        assert_eq!(state.a, 4);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_with_max_values() {
        let mut state = State {
            a: u8::max_value(),
            b: u8::max_value(),
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::B);

        assert_eq!(state.a, 255u8);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn adc_where_carry_causes_carry() {
        let mut state = State {
            a: u8::max_value(),
            b: 0,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::B);

        assert_eq!(state.a, 0);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn adc_c_adds_c_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            c: 2,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::C);

        assert_eq!(state.a, 4);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_d_adds_d_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            d: 2,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::D);

        assert_eq!(state.a, 4);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_e_adds_e_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            e: 2,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::E);

        assert_eq!(state.a, 4);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_h_adds_h_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            h: 2,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::H);

        assert_eq!(state.a, 4);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_l_adds_l_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            l: 2,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::L);

        assert_eq!(state.a, 4);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_m_adds_m_with_carry_to_accumulator() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 5],
            a: 1,
            h: 0x00,
            l: 0x05,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };
        state.adc(Register::M);

        assert_eq!(state.a, 7);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_a_adds_a_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::A);

        assert_eq!(state.a, 3);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn sub_b_subs_b_from_accumulator() {
        let mut state = State {
            a: 10,
            b: 3,
            ..State::default()
        };

        state.sub(Register::B);

        assert_eq!(state.a, 7);
    }

    #[test]
    fn sub_sets_the_carry_bit_if_no_borrow() {
        let mut state = State {
            a: 10,
            b: 3,
            ..State::default()
        };

        state.sub(Register::B);

        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sub_resets_the_carry_bit_if_borrow() {
        let mut state = State {
            a: 1,
            b: 3,
            ..State::default()
        };

        state.sub(Register::B);

        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn sub_a_resets_the_carry_and_zeros_the_accumulator() {
        let mut state = State {
            a: 0x3e,
            ..State::default()
        };

        state.sub(Register::A);

        assert_eq!(state.cc.carry, true);
        assert_eq!(state.a, 0);
    }
}
