use cpu::register::Register;
use state::State;

/// Push data onto the stack from the specified register pair
///
/// # Cycles
///
/// 11
///
/// # Arguments
///
/// * `state` - The state to perform the pop in
/// * `register` - The register pair to push the data from
///
pub fn push(state: &mut State, register: Register) -> u8 {
    let (most, least) = match register {
        Register::B => (state.b, state.c),
        Register::D => (state.d, state.e),
        Register::H => (state.h, state.l),
        Register::PSW => (state.a, state.get_flags_as_bits()),
        unsupported => {
            panic!("pop doesn't support {:?}", unsupported);
        }
    };
    state.write_byte_to_stack(most);
    state.write_byte_to_stack(least);

    11
}

#[cfg(test)]
mod test {
    use super::*;
    use state::ConditionCodes;

    #[test]
    fn push_from_register_b_pushed_bytes_onto_the_stack_from_b_and_c() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0],
            b: 0xBB,
            c: 0xCC,
            sp: 0x0004,
            ..State::default()
        };

        push(&mut state, Register::B);

        assert_eq!(state.sp, 0x0002);
        assert_eq!(state.memory, vec![0, 0, 0xCC, 0xBB, 0, 0]);
    }

    #[test]
    fn push_from_register_d_pushed_bytes_onto_the_stack_from_d_and_e() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0],
            d: 0x8F,
            e: 0x9D,
            sp: 0x0004,
            ..State::default()
        };

        push(&mut state, Register::D);

        assert_eq!(state.sp, 0x0002);
        assert_eq!(state.memory, vec![0, 0, 0x9D, 0x8F, 0, 0]);
    }

    #[test]
    fn push_from_register_h_pushed_bytes_onto_the_stack_from_h_and_l() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0],
            h: 0xFF,
            l: 0x11,
            sp: 0x0004,
            ..State::default()
        };

        push(&mut state, Register::H);

        assert_eq!(state.sp, 2);
        assert_eq!(state.memory, vec![0, 0, 0x11, 0xFF, 0, 0]);
    }

    #[test]
    fn push_from_psw_pushed_bytes_onto_the_stack_from_accumulator_and_flags() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0],
            a: 0xAA,
            sp: 0x0004,
            cc: ConditionCodes {
                carry: true,
                sign: true,
                zero: true,
                parity: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        push(&mut state, Register::PSW);

        assert_eq!(state.sp, 2);
        assert_eq!(state.memory, vec![0, 0, 0b1100_0101, 0xAA, 0, 0]);
    }
}
