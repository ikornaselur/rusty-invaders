use state::{Register, State};

/// Perform an and between accumulator and register and put the results into the accumulator
///
/// Sets condition flags
///
/// Cycles: 7 for register M, else 4
///
/// # Arguments
///
/// * `state` - The state to perform the and in
/// * `register` - The register to perform the and with
///
pub fn ana(state: &mut State, register: Register) -> u8 {
    let result = state.a & match register {
        Register::A => state.a,
        Register::B => state.b,
        Register::C => state.c,
        Register::D => state.d,
        Register::E => state.e,
        Register::H => state.h,
        Register::L => state.l,
        Register::M => {
            let offset = (u16::from(state.h) << 8) + u16::from(state.l);
            state.memory[offset as usize]
        }
        unsupported => {
            panic!("ana doesn't support {:?}", unsupported);
        }
    };

    state.a = result;
    state.set_flags(result, false);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Perform an and between accumulator and the next immediate byte and put the results into the
/// accumulator
///
/// Sets condition flags
///
/// Cycles: 7
///
/// # Arguments
///
/// * `state` - The state to perform the and in
///
pub fn ani(state: &mut State) -> u8 {
    let byte = state.read_byte().unwrap();

    let result = state.a & byte;

    state.a = result;
    state.set_flags(result, false);

    7
}

#[cfg(test)]
mod test {
    use super::*;
    use state::ConditionCodes;

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

        ana(&mut state, Register::B);

        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn ana_b_ands_b_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            b: 0b0000_1111,
            ..State::default()
        };

        ana(&mut state, Register::B);

        assert_eq!(state.a, 0b0000_1100);
    }

    #[test]
    fn ana_c_ands_c_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            c: 0b0000_1111,
            ..State::default()
        };

        ana(&mut state, Register::C);

        assert_eq!(state.a, 0b0000_1100);
    }

    #[test]
    fn ana_d_ands_d_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            d: 0b0000_1111,
            ..State::default()
        };

        ana(&mut state, Register::D);

        assert_eq!(state.a, 0b0000_1100);
    }

    #[test]
    fn ana_e_ands_e_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            e: 0b0000_1111,
            ..State::default()
        };

        ana(&mut state, Register::E);

        assert_eq!(state.a, 0b0000_1100);
    }

    #[test]
    fn ana_h_ands_h_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            h: 0b0000_1111,
            ..State::default()
        };

        ana(&mut state, Register::H);

        assert_eq!(state.a, 0b0000_1100);
    }

    #[test]
    fn ana_l_ands_l_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            l: 0b0000_1111,
            ..State::default()
        };

        ana(&mut state, Register::L);

        assert_eq!(state.a, 0b0000_1100);
    }

    #[test]
    fn ana_a_ands_a_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            ..State::default()
        };

        ana(&mut state, Register::A);

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

        ana(&mut state, Register::M);

        assert_eq!(state.a, 0b0000_1100);
    }

    #[test]
    fn ani_ands_immediate_byte_with_accumulator() {
        let mut state = State {
            memory: vec![0b0011_0101, 0b0010_0010],
            a: 0b1111_0000,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        ani(&mut state);
        assert_eq!(state.a, 0b0011_0000);
        assert_eq!(state.cc.carry, false);

        ani(&mut state);
        assert_eq!(state.a, 0b0010_0000);
        assert_eq!(state.cc.carry, false);
    }
}
