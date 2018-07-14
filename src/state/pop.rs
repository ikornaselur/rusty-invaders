use super::Register;
use super::State;

impl State {
    pub fn pop(&mut self, register: Register) -> () {
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
            Register::PSW => {
                self.set_flags_from_bits(least);
                self.a = most;
            }
            unsupported => {
                panic!("lxi doesn't support {:?}", unsupported);
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pop_into_register_b_pops_two_bytes_off_the_stack_into_b_and_c() {
        let mut state = State {
            memory: vec![0, 0, 0, 0x15, 0x26, 0x37],
            pc: 3,
            ..State::default()
        };

        state.pop(Register::B);

        assert_eq!(state.c, 0x15);
        assert_eq!(state.b, 0x26);
        assert_eq!(state.pc, 5);
    }

    #[test]
    fn pop_into_register_d_pops_two_bytes_off_the_stack_into_d_and_e() {
        let mut state = State {
            memory: vec![0, 0, 0, 0x15, 0x26, 0x37],
            pc: 3,
            ..State::default()
        };

        state.pop(Register::D);

        assert_eq!(state.e, 0x15);
        assert_eq!(state.d, 0x26);
        assert_eq!(state.pc, 5);
    }

    #[test]
    fn pop_into_register_h_pops_two_bytes_off_the_stack_into_h_and_l() {
        let mut state = State {
            memory: vec![0, 0, 0, 0x15, 0x26, 0x37],
            pc: 3,
            ..State::default()
        };

        state.pop(Register::H);

        assert_eq!(state.l, 0x15);
        assert_eq!(state.h, 0x26);
        assert_eq!(state.pc, 5);
    }

    #[test]
    fn pop_into_psq_pops_two_bytes_off_the_stack_into_accumulator_and_flags() {
        let mut state = State {
            memory: vec![0, 0, 0, 0b0100_0100, 0x26, 0b1000_0001, 0x37],
            pc: 3,
            ..State::default()
        };

        state.pop(Register::PSW);

        assert_eq!(state.a, 0x26);
        assert_eq!(state.pc, 5);
        assert_eq!(state.cc.sign, false);
        assert_eq!(state.cc.zero, true);
        assert_eq!(state.cc.parity, true);
        assert_eq!(state.cc.carry, false);

        state.pop(Register::PSW);
        assert_eq!(state.a, 0x37);
        assert_eq!(state.pc, 7);
        assert_eq!(state.cc.sign, true);
        assert_eq!(state.cc.zero, false);
        assert_eq!(state.cc.parity, false);
        assert_eq!(state.cc.carry, true);
    }
}
