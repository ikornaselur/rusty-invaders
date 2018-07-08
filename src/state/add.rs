use super::Register;
use super::State;

impl State {
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
}

#[cfg(test)]
mod test {
    use super::*;

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
}
