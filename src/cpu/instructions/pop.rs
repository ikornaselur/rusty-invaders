use cpu::register::Register;
use state::State;

/// Pop data off the stack into the specified register pair
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to perform the pop in
/// * `register` - The register pair to pop the data into
///
pub fn pop(state: &mut State, register: Register) -> u8 {
    let least = state.read_byte_from_stack().unwrap();
    let most = state.read_byte_from_stack().unwrap();

    match register {
        Register::B => {
            state.c = least;
            state.b = most;
        }
        Register::D => {
            state.e = least;
            state.d = most;
        }
        Register::H => {
            state.l = least;
            state.h = most;
        }
        Register::PSW => {
            state.set_flags_from_bits(least);
            state.a = most;
        }
        unsupported => {
            panic!("pop doesn't support {:?}", unsupported);
        }
    };

    10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pop_into_register_b_pops_two_bytes_off_the_stack_into_b_and_c() {
        let mut state = State {
            memory: vec![0, 0, 0, 0x15, 0x26, 0x37],
            sp: 3,
            ..State::default()
        };

        pop(&mut state, Register::B);

        assert_eq!(state.c, 0x15);
        assert_eq!(state.b, 0x26);
        assert_eq!(state.sp, 5);
    }

    #[test]
    fn pop_into_register_d_pops_two_bytes_off_the_stack_into_d_and_e() {
        let mut state = State {
            memory: vec![0, 0, 0, 0x15, 0x26, 0x37],
            sp: 3,
            ..State::default()
        };

        pop(&mut state, Register::D);

        assert_eq!(state.e, 0x15);
        assert_eq!(state.d, 0x26);
        assert_eq!(state.sp, 5);
    }

    #[test]
    fn pop_into_register_h_pops_two_bytes_off_the_stack_into_h_and_l() {
        let mut state = State {
            memory: vec![0, 0, 0, 0x15, 0x26, 0x37],
            sp: 3,
            ..State::default()
        };

        pop(&mut state, Register::H);

        assert_eq!(state.l, 0x15);
        assert_eq!(state.h, 0x26);
        assert_eq!(state.sp, 5);
    }

    #[test]
    fn pop_into_psq_pops_two_bytes_off_the_stack_into_accumulator_and_flags() {
        let mut state = State {
            memory: vec![0, 0, 0, 0b0100_0100, 0x26, 0b1000_0001, 0x37],
            sp: 3,
            ..State::default()
        };

        pop(&mut state, Register::PSW);

        assert_eq!(state.a, 0x26);
        assert_eq!(state.sp, 5);
        assert_eq!(state.cc.sign, false);
        assert_eq!(state.cc.zero, true);
        assert_eq!(state.cc.parity, true);
        assert_eq!(state.cc.carry, false);

        pop(&mut state, Register::PSW);
        assert_eq!(state.a, 0x37);
        assert_eq!(state.sp, 7);
        assert_eq!(state.cc.sign, true);
        assert_eq!(state.cc.zero, false);
        assert_eq!(state.cc.parity, false);
        assert_eq!(state.cc.carry, true);
    }
}
