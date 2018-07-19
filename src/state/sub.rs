use super::Register;
use super::State;

impl State {
    pub fn sub(&mut self, register: Register) -> u8 {
        let (result, borrow) = match register {
            Register::A => self.a.overflowing_sub(self.a),
            Register::B => self.a.overflowing_sub(self.b),
            Register::C => self.a.overflowing_sub(self.c),
            Register::D => self.a.overflowing_sub(self.d),
            Register::E => self.a.overflowing_sub(self.e),
            Register::H => self.a.overflowing_sub(self.h),
            Register::L => self.a.overflowing_sub(self.l),
            Register::M => {
                let offset = (u16::from(self.h) << 8) + u16::from(self.l);
                self.a.overflowing_sub(self.memory[offset as usize])
            }
            unsupported => {
                panic!("sub doesn't support {:?}", unsupported);
            }
        };

        self.a = result;
        self.set_flags(result, borrow);

        match register {
            Register::M => 7,
            _ => 4,
        }
    }

    pub fn sui(&mut self) -> u8 {
        let byte = self.read_byte().unwrap();
        let (result, carry) = self.a.overflowing_sub(byte);

        self.a = result;
        self.set_flags(result, carry);

        7
    }

    pub fn sbb(&mut self, register: Register) -> u8 {
        let byte = match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            Register::M => {
                let offset = (u16::from(self.h) << 8) + u16::from(self.l);
                self.memory[offset as usize]
            }
            unsupported => {
                panic!("sbb doesn't support {:?}", unsupported);
            }
        };

        let (byte, byte_carry) = if self.cc.carry {
            byte.overflowing_add(1)
        } else {
            (byte, false)
        };

        let (result, carry) = self.a.overflowing_sub(byte);

        self.a = result;
        self.set_flags(result, carry || byte_carry);

        match register {
            Register::M => 7,
            _ => 4,
        }
    }

    pub fn sbi(&mut self) -> u8 {
        let byte = self.read_byte().unwrap();

        let (byte, byte_carry) = if self.cc.carry {
            byte.overflowing_add(1)
        } else {
            (byte, false)
        };

        let (result, carry) = self.a.overflowing_sub(byte);

        self.a = result;
        self.set_flags(result, carry || byte_carry);

        7
    }
}

#[cfg(test)]
mod test {
    use super::super::ConditionCodes;
    use super::*;

    #[test]
    fn sub_a_subs_a_from_accumulator() {
        let mut state = State {
            a: 10,
            ..State::default()
        };

        state.sub(Register::A);

        assert_eq!(state.a, 0);
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
    fn sub_c_subs_c_from_accumulator() {
        let mut state = State {
            a: 10,
            c: 3,
            ..State::default()
        };

        state.sub(Register::C);

        assert_eq!(state.a, 7);
    }

    #[test]
    fn sub_d_subs_d_from_accumulator() {
        let mut state = State {
            a: 10,
            d: 3,
            ..State::default()
        };

        state.sub(Register::D);

        assert_eq!(state.a, 7);
    }

    #[test]
    fn sub_e_subs_e_from_accumulator() {
        let mut state = State {
            a: 10,
            e: 3,
            ..State::default()
        };

        state.sub(Register::E);

        assert_eq!(state.a, 7);
    }

    #[test]
    fn sub_h_subs_h_from_accumulator() {
        let mut state = State {
            a: 10,
            h: 3,
            ..State::default()
        };

        state.sub(Register::H);

        assert_eq!(state.a, 7);
    }

    #[test]
    fn sub_l_subs_l_from_accumulator() {
        let mut state = State {
            a: 10,
            l: 3,
            ..State::default()
        };

        state.sub(Register::L);

        assert_eq!(state.a, 7);
    }

    #[test]
    fn sub_m_subs_byte_at_hl_address_to_accumulator() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 3],
            a: 10,
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        state.sub(Register::M);

        assert_eq!(state.a, 7);
    }

    #[test]
    fn sub_resets_the_carry_bit_if_no_borrow() {
        let mut state = State {
            a: 10,
            b: 3,
            ..State::default()
        };

        state.sub(Register::B);

        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn sub_sets_the_carry_bit_if_borrow() {
        let mut state = State {
            a: 1,
            b: 3,
            ..State::default()
        };

        state.sub(Register::B);

        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sub_a_resets_the_carry_and_zeros_the_accumulator() {
        let mut state = State {
            a: 0x3e,
            ..State::default()
        };

        state.sub(Register::A);

        assert_eq!(state.cc.carry, false);
        assert_eq!(state.a, 0);
    }

    #[test]
    fn sui_removes_immediate_byte_from_accumulator() {
        let mut state = State {
            memory: vec![1, 5],
            a: 0,
            ..State::default()
        };

        state.sui();
        assert_eq!(state.a, 255);
        assert_eq!(state.cc.carry, true);

        state.sui();
        assert_eq!(state.a, 250);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn sbb_b_subs_b_from_accumulator_with_carry_flag_and_borrowing() {
        let mut state = State {
            a: 4,
            b: 10,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.sbb(Register::B);

        assert_eq!(state.a, 249);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sbb_b_subs_b_from_accumulator_without_carry_flag_and_borrowing() {
        let mut state = State {
            a: 4,
            b: 10,
            cc: ConditionCodes {
                carry: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.sbb(Register::B);

        assert_eq!(state.a, 250);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sbb_b_subs_b_from_accumulator_with_carry_flag_and_not_borrowing() {
        let mut state = State {
            a: 4,
            b: 1,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.sbb(Register::B);

        assert_eq!(state.a, 2);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn sbb_b_subs_b_from_accumulator_without_carry_flag_and_not_borrowing() {
        let mut state = State {
            a: 4,
            b: 1,
            cc: ConditionCodes {
                carry: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.sbb(Register::B);

        assert_eq!(state.a, 3);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn sbb_c_subs_c_from_accumulator_with_carry_flag_set() {
        let mut state = State {
            a: 5,
            c: 10,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.sbb(Register::C);

        assert_eq!(state.a, 250);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sbb_d_subs_d_from_accumulator_with_carry_flag_set() {
        let mut state = State {
            a: 5,
            d: 10,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.sbb(Register::D);

        assert_eq!(state.a, 250);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sbb_e_subs_e_from_accumulator_with_carry_flag_set() {
        let mut state = State {
            a: 5,
            e: 10,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.sbb(Register::E);

        assert_eq!(state.a, 250);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sbb_h_subs_h_from_accumulator_with_carry_flag_set() {
        let mut state = State {
            a: 5,
            h: 10,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.sbb(Register::H);

        assert_eq!(state.a, 250);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sbb_m_subs_byte_at_hl_address_to_accumulator_with_carry_flag_set() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 10],
            a: 5,
            h: 0x00,
            l: 0x05,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.sbb(Register::M);

        assert_eq!(state.a, 250);
    }

    #[test]
    fn sbb_sub_with_carry_bit() {
        let mut state = State {
            a: 4,
            l: 2,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.sbb(Register::L);

        assert_eq!(state.a, 1);
        assert_eq!(state.cc.carry, false);
        assert_eq!(state.cc.zero, false);
    }

    #[test]
    fn sbb_sub_with_max_values() {
        let mut state = State {
            a: u8::max_value(),
            b: u8::max_value(),
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.sbb(Register::B);

        assert_eq!(state.a, 255);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sbi_removes_immediate_byte_from_accumulator_with_borrow() {
        let mut state = State {
            memory: vec![0xFF, 0xFF, 0x00, 0x01],
            a: 0x00,
            ..State::default()
        };

        state.sbi();
        assert_eq!(state.a, 0x01);
        assert_eq!(state.cc.carry, true);

        state.sbi();
        assert_eq!(state.a, 0x01);
        assert_eq!(state.cc.carry, true);

        state.sbi();
        assert_eq!(state.a, 0x00);
        assert_eq!(state.cc.carry, false);

        state.sbi();
        assert_eq!(state.a, 0xFF);
        assert_eq!(state.cc.carry, true);
    }
}
