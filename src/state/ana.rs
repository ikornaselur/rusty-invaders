use super::Register;
use super::State;

impl State {
    pub fn ana(&mut self, register: Register) -> () {
        // 4 cycles
        let result = self.a & match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            Register::M => {
                let offset: u16 = ((self.h as u16) << 8) + self.l as u16;
                self.memory[offset as usize]
            }
            unsupported => {
                panic!("ana doesn't support {:?}", unsupported);
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
    fn ana_resets_carry_bit() {
        let mut state = State {
            a: 123,
            b: 123,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.ana(Register::B);

        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn ana_b_ands_b_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            b: 0b0000_1111,
            ..State::default()
        };

        state.ana(Register::B);

        assert_eq!(state.a, 0b0000_1100);
    }

    #[test]
    fn ana_c_ands_c_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            c: 0b0000_1111,
            ..State::default()
        };

        state.ana(Register::C);

        assert_eq!(state.a, 0b0000_1100);
    }

    #[test]
    fn ana_d_ands_d_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            d: 0b0000_1111,
            ..State::default()
        };

        state.ana(Register::D);

        assert_eq!(state.a, 0b0000_1100);
    }

    #[test]
    fn ana_e_ands_e_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            e: 0b0000_1111,
            ..State::default()
        };

        state.ana(Register::E);

        assert_eq!(state.a, 0b0000_1100);
    }

    #[test]
    fn ana_h_ands_h_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            h: 0b0000_1111,
            ..State::default()
        };

        state.ana(Register::H);

        assert_eq!(state.a, 0b0000_1100);
    }

    #[test]
    fn ana_l_ands_l_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            l: 0b0000_1111,
            ..State::default()
        };

        state.ana(Register::L);

        assert_eq!(state.a, 0b0000_1100);
    }

    #[test]
    fn ana_a_ands_a_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            ..State::default()
        };

        state.ana(Register::A);

        assert_eq!(state.a, 0b1111_1100);
    }

    #[test]
    fn ana_m_ands_byte_at_hl_address_to_accumulator() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0b0000_1111],
            a: 0b1111_1100,
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        state.ana(Register::M);

        assert_eq!(state.a, 0b0000_1100);
    }
}
