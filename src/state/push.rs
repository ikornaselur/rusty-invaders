use super::Register;
use super::State;

impl State {
    pub fn push(&mut self, register: Register) -> () {
        let (most, least) = match register {
            Register::B => (self.b, self.c),
            Register::D => (self.d, self.e),
            Register::H => (self.h, self.l),
            Register::PSW => (self.a, self.get_flags_as_bits()),
            unsupported => {
                panic!("pop doesn't support {:?}", unsupported);
            }
        };
        self.write_byte(most);
        self.write_byte(least);
    }
}

#[cfg(test)]
use super::ConditionCodes;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_from_register_b_pushed_bytes_onto_the_stack_from_b_and_c() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0],
            b: 0xBB,
            c: 0xCC,
            pc: 4,
            ..State::default()
        };

        state.push(Register::B);

        assert_eq!(state.pc, 2);
        assert_eq!(state.memory, vec![0, 0, 0xCC, 0xBB, 0, 0]);
    }

    #[test]
    fn push_from_register_d_pushed_bytes_onto_the_stack_from_d_and_e() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0],
            d: 0xDD,
            e: 0xEE,
            pc: 4,
            ..State::default()
        };

        state.push(Register::D);

        assert_eq!(state.pc, 2);
        assert_eq!(state.memory, vec![0, 0, 0xEE, 0xDD, 0, 0]);
    }

    #[test]
    fn push_from_register_h_pushed_bytes_onto_the_stack_from_h_and_l() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0],
            h: 0xFF,
            l: 0x11,
            pc: 4,
            ..State::default()
        };

        state.push(Register::H);

        assert_eq!(state.pc, 2);
        assert_eq!(state.memory, vec![0, 0, 0x11, 0xFF, 0, 0]);
    }

    #[test]
    fn push_from_psw_pushed_bytes_onto_the_stack_from_accumulator_and_flags() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0],
            a: 0xAA,
            pc: 4,
            cc: ConditionCodes {
                carry: true,
                sign: true,
                zero: true,
                parity: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.push(Register::PSW);

        assert_eq!(state.pc, 2);
        assert_eq!(state.memory, vec![0, 0, 0b1100_0101, 0xAA, 0, 0]);
    }
}
