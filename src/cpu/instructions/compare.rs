use state::{Register, State};

/// Compare a register to the accumulator and set the flags based on the comparison
///
/// Sets conditions flags
///
/// Cycles: 7 for register M, else 4
///
/// # Arguments
///
/// * `state` - The state to perform the comparison in
/// * `register` - The register to compare to the accumulator
///
pub fn cmp(state: &mut State, register: Register) -> u8 {
    let (result, borrow) = match register {
        Register::A => state.a.overflowing_sub(state.a),
        Register::B => state.a.overflowing_sub(state.b),
        Register::C => state.a.overflowing_sub(state.c),
        Register::D => state.a.overflowing_sub(state.d),
        Register::E => state.a.overflowing_sub(state.e),
        Register::H => state.a.overflowing_sub(state.h),
        Register::L => state.a.overflowing_sub(state.l),
        Register::M => {
            let offset = (u16::from(state.h) << 8) + u16::from(state.l);
            state.a.overflowing_sub(state.memory[offset as usize])
        }
        unsupported => {
            panic!("sub doesn't support {:?}", unsupported);
        }
    };

    state.set_flags(result, borrow);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Compare the accumulator to the next immediate byte and set the flags based on the comparison
///
/// Sets conditions flags
///
/// Cycles: 4
///
/// # Arguments
///
/// * `state` - The state to perform the comparison in
///
pub fn cpi(state: &mut State) -> u8 {
    let byte = state.read_byte().unwrap();

    let (result, borrow) = state.a.overflowing_sub(byte);

    state.set_flags(result, borrow);

    7
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

        cmp(&mut state, Register::B);

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

        cmp(&mut state, Register::B);

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

        cmp(&mut state, Register::B);

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

        cpi(&mut state);

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

        cpi(&mut state);

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

        cpi(&mut state);

        assert_eq!(state.a, 10);

        assert_eq!(state.cc.carry, true);
        assert_eq!(state.cc.zero, false);
        assert_eq!(state.cc.sign, true);
        assert_eq!(state.cc.parity, true);
    }
}
