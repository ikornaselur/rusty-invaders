use super::Register;
use super::State;

impl State {
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
}

#[cfg(test)]
use super::ConditionCodes;

#[cfg(test)]
mod test {
    use super::*;

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
}
