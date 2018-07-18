use super::Register;
use super::State;

impl State {
    pub fn cmp(&mut self, register: Register) -> u8 {
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
                panic!("sub doesn't support {:?}", unsupported);
            }
        };

        self.set_flags(result, borrow);

        match register {
            Register::M => 7,
            _ => 4,
        }
    }

    pub fn cpi(&mut self) -> u8 {
        let byte = self.read_byte().unwrap();

        let (result, borrow) = self.a.overflowing_sub(byte);

        self.set_flags(result, borrow);

        7
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cmp_b_with_smaller_b_compares_b_to_accumulator_and_sets_flags() {
        let mut state = State {
            a: 10,
            b: 9,
            ..State::default()
        };

        state.cmp(Register::B);

        assert_eq!(state.a, 10);
        assert_eq!(state.b, 9);

        assert_eq!(state.cc.carry, false);
        assert_eq!(state.cc.zero, false);
        assert_eq!(state.cc.sign, false);
        assert_eq!(state.cc.parity, false);
    }

    #[test]
    fn cmp_b_with_equal_b_compares_b_to_accumulator_and_sets_flags() {
        let mut state = State {
            a: 10,
            b: 10,
            ..State::default()
        };

        state.cmp(Register::B);

        assert_eq!(state.a, 10);
        assert_eq!(state.b, 10);

        assert_eq!(state.cc.carry, false);
        assert_eq!(state.cc.zero, true);
        assert_eq!(state.cc.sign, false);
        assert_eq!(state.cc.parity, true);
    }

    #[test]
    fn cmp_b_with_larger_b_compares_b_to_accumulator_and_sets_flags() {
        let mut state = State {
            a: 10,
            b: 11,
            ..State::default()
        };

        state.cmp(Register::B);

        assert_eq!(state.a, 10);
        assert_eq!(state.b, 11);

        assert_eq!(state.cc.carry, true);
        assert_eq!(state.cc.zero, false);
        assert_eq!(state.cc.sign, true);
        assert_eq!(state.cc.parity, true);
    }

    #[test]
    fn cpi_with_smaller_immediate_byte_compares_it_to_accumulator_and_sets_flags() {
        let mut state = State {
            memory: vec![9],
            a: 10,
            ..State::default()
        };

        state.cpi();

        assert_eq!(state.a, 10);

        assert_eq!(state.cc.carry, false);
        assert_eq!(state.cc.zero, false);
        assert_eq!(state.cc.sign, false);
        assert_eq!(state.cc.parity, false);
    }

    #[test]
    fn cpi_with_equal_immediate_byte_compares_it_to_accumulator_and_sets_flags() {
        let mut state = State {
            memory: vec![10],
            a: 10,
            ..State::default()
        };

        state.cpi();

        assert_eq!(state.a, 10);

        assert_eq!(state.cc.carry, false);
        assert_eq!(state.cc.zero, true);
        assert_eq!(state.cc.sign, false);
        assert_eq!(state.cc.parity, true);
    }

    #[test]
    fn cpi_with_larget_immediate_byte_compares_it_to_accumulator_and_sets_flags() {
        let mut state = State {
            memory: vec![11],
            a: 10,
            ..State::default()
        };

        state.cpi();

        assert_eq!(state.a, 10);

        assert_eq!(state.cc.carry, true);
        assert_eq!(state.cc.zero, false);
        assert_eq!(state.cc.sign, true);
        assert_eq!(state.cc.parity, true);
    }
}
