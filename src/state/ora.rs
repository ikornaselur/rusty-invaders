use super::Register;
use super::State;

impl State {
    pub fn ora(&mut self, register: Register) -> () {
        // 4 instructions
        let result = self.a | match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            Register::M => {
                let offset: u16 = ((self.h as u16) << 8) + self.l as u16;
                *self.memory.get(offset as usize).unwrap()
            }
            unsupported => {
                panic!("add doesn't support {:?}", unsupported);
            }
        };

        self.a = result;
        self.set_flags(result, false);
    }
}

#[cfg(test)]
use super::ConditionCodes;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ora_resets_carry_bit() {
        let mut state = State {
            a: 123,
            b: 123,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.ora(Register::B);

        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn ora_b_ors_b_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            b: 0b0111_1000,
            ..State::default()
        };

        state.ora(Register::B);

        assert_eq!(state.a, 0b0111_1100);
    }

    #[test]
    fn ora_c_ors_c_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            c: 0b0111_1000,
            ..State::default()
        };

        state.ora(Register::C);

        assert_eq!(state.a, 0b0111_1100);
    }

    #[test]
    fn ora_d_ors_d_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            d: 0b0111_1000,
            ..State::default()
        };

        state.ora(Register::D);

        assert_eq!(state.a, 0b0111_1100);
    }

    #[test]
    fn ora_e_ors_e_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            e: 0b0111_1000,
            ..State::default()
        };

        state.ora(Register::E);

        assert_eq!(state.a, 0b0111_1100);
    }

    #[test]
    fn ora_h_ors_h_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            h: 0b0111_1000,
            ..State::default()
        };

        state.ora(Register::H);

        assert_eq!(state.a, 0b0111_1100);
    }

    #[test]
    fn ora_l_ors_l_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            l: 0b0111_1000,
            ..State::default()
        };

        state.ora(Register::L);

        assert_eq!(state.a, 0b0111_1100);
    }

    #[test]
    fn ora_a_ors_a_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            ..State::default()
        };

        state.ora(Register::A);

        assert_eq!(state.a, 0b1111_1100);
    }

    #[test]
    fn ora_m_ors_byte_at_hl_address_to_accumulator() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0b0111_1000],
            a: 0b0101_1100,
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        state.ora(Register::M);

        assert_eq!(state.a, 0b0111_1100);
    }
}
