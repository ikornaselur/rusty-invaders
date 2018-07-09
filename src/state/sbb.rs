use super::Register;
use super::State;

impl State {
    pub fn sbb(&mut self, register: Register) -> () {
        // 4 instructions
        if !self.cc.carry {
            self.sub(register)
        } else {
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
                    self.a.overflowing_sub(self.memory[offset as usize])
                }
                unsupported => {
                    panic!("sbb doesn't support {:?}", unsupported);
                }
            };

            if !borrow {
                let (result, borrow) = result.overflowing_sub(1);
                self.a = result;
                self.set_flags(result, borrow);
            } else {
                self.a = result.wrapping_sub(1);
                self.set_flags(result, borrow);
            }
        }
    }
}

#[cfg(test)]
use super::ConditionCodes;

#[cfg(test)]
mod test {
    use super::*;

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
}
