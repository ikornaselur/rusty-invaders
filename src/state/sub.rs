use super::Register;
use super::State;

impl State {
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
                panic!("sub doesn't support {:?}", unsupported);
            }
        };

        self.a = result;
        self.set_flags(result, borrow);
    }
}

#[cfg(test)]
mod test {
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
}
