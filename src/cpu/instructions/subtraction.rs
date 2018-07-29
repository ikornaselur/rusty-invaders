use cpu::register::Register;
use state::State;

/// Perform accumulator subtraction from a register
///
/// Sets condition flags
///
/// # Cycles
///
/// * Register M: 7
/// * Other: 4
///
/// # Arguments
///
/// * `state` - The state to perform the subtraction in
/// * `register` - The register to subtract from the accumulator
///
pub fn sub(state: &mut State, register: Register) -> u8 {
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

    state.a = result;
    state.set_flags(result, borrow);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Perform accumulator subtraction with the next immediate byte
///
/// Sets the condition flags
///
/// # Cycles
///
/// 7
///
/// # Arguments
///
/// * `state` - The state to perform the subtraction in
///
pub fn sui(state: &mut State) -> u8 {
    let byte = state.read_byte().unwrap();
    let (result, carry) = state.a.overflowing_sub(byte);

    state.a = result;
    state.set_flags(result, carry);

    7
}

/// Perform accumulator subtraction from a register with the carry bit
///
/// Sets condition codes
///
/// # Cycles
///
/// * Register M: 7
/// * Other: 4
///
/// # Arguments
///
/// * `state` - The state to perform the subtraction in
/// * `register` - The register to subtract from the accumulator
///
pub fn sbb(state: &mut State, register: Register) -> u8 {
    let byte = match register {
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
            panic!("sbb doesn't support {:?}", unsupported);
        }
    };

    let (byte, byte_carry) = if state.cc.carry {
        byte.overflowing_add(1)
    } else {
        (byte, false)
    };

    let (result, carry) = state.a.overflowing_sub(byte);

    state.a = result;
    state.set_flags(result, carry || byte_carry);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Perform accumulator subtraction with the next immediate byte and carry bit
///
/// Sets condition codes
///
/// # Cycles
///
/// 7
///
/// # Arguments
///
/// * `state` - The state to perform the subtraction in
///
pub fn sbi(state: &mut State) -> u8 {
    let byte = state.read_byte().unwrap();

    let (byte, byte_carry) = if state.cc.carry {
        byte.overflowing_add(1)
    } else {
        (byte, false)
    };

    let (result, carry) = state.a.overflowing_sub(byte);

    state.a = result;
    state.set_flags(result, carry || byte_carry);

    7
}

#[cfg(test)]
mod test {
    use super::*;
    use state::ConditionCodes;

    #[test]
    fn sub_a_subs_a_from_accumulator() {
        let mut state = State {
            a: 10,
            ..State::default()
        };

        sub(&mut state, Register::A);

        assert_eq!(state.a, 0);
    }

    #[test]
    fn sub_b_subs_b_from_accumulator() {
        let mut state = State {
            a: 10,
            b: 3,
            ..State::default()
        };

        sub(&mut state, Register::B);

        assert_eq!(state.a, 7);
    }

    #[test]
    fn sub_c_subs_c_from_accumulator() {
        let mut state = State {
            a: 10,
            c: 3,
            ..State::default()
        };

        sub(&mut state, Register::C);

        assert_eq!(state.a, 7);
    }

    #[test]
    fn sub_d_subs_d_from_accumulator() {
        let mut state = State {
            a: 10,
            d: 3,
            ..State::default()
        };

        sub(&mut state, Register::D);

        assert_eq!(state.a, 7);
    }

    #[test]
    fn sub_e_subs_e_from_accumulator() {
        let mut state = State {
            a: 10,
            e: 3,
            ..State::default()
        };

        sub(&mut state, Register::E);

        assert_eq!(state.a, 7);
    }

    #[test]
    fn sub_h_subs_h_from_accumulator() {
        let mut state = State {
            a: 10,
            h: 3,
            ..State::default()
        };

        sub(&mut state, Register::H);

        assert_eq!(state.a, 7);
    }

    #[test]
    fn sub_l_subs_l_from_accumulator() {
        let mut state = State {
            a: 10,
            l: 3,
            ..State::default()
        };

        sub(&mut state, Register::L);

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

        sub(&mut state, Register::M);

        assert_eq!(state.a, 7);
    }

    #[test]
    fn sub_resets_the_carry_bit_if_no_borrow() {
        let mut state = State {
            a: 10,
            b: 3,
            ..State::default()
        };

        sub(&mut state, Register::B);

        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn sub_sets_the_carry_bit_if_borrow() {
        let mut state = State {
            a: 1,
            b: 3,
            ..State::default()
        };

        sub(&mut state, Register::B);

        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sub_a_resets_the_carry_and_zeros_the_accumulator() {
        let mut state = State {
            a: 0x3e,
            ..State::default()
        };

        sub(&mut state, Register::A);

        assert_eq!(state.cc.carry, false);
        assert_eq!(state.a, 0);
    }

    #[test]
    fn sui_removes_immediate_byte_from_accumulator() {
        let mut state = State {
            memory: vec![1, 5],
            a: 0,
            ..State::default()
        };

        sui(&mut state);
        assert_eq!(state.a, 255);
        assert_eq!(state.cc.carry, true);

        sui(&mut state);
        assert_eq!(state.a, 250);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn sbb_b_subs_b_from_accumulator_with_carry_flag_and_borrowing() {
        let mut state = State {
            a: 4,
            b: 10,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        sbb(&mut state, Register::B);

        assert_eq!(state.a, 249);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sbb_b_subs_b_from_accumulator_without_carry_flag_and_borrowing() {
        let mut state = State {
            a: 4,
            b: 10,
            cc: ConditionCodes {
                carry: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        sbb(&mut state, Register::B);

        assert_eq!(state.a, 250);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sbb_b_subs_b_from_accumulator_with_carry_flag_and_not_borrowing() {
        let mut state = State {
            a: 4,
            b: 1,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        sbb(&mut state, Register::B);

        assert_eq!(state.a, 2);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn sbb_b_subs_b_from_accumulator_without_carry_flag_and_not_borrowing() {
        let mut state = State {
            a: 4,
            b: 1,
            cc: ConditionCodes {
                carry: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        sbb(&mut state, Register::B);

        assert_eq!(state.a, 3);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn sbb_c_subs_c_from_accumulator_with_carry_flag_set() {
        let mut state = State {
            a: 5,
            c: 10,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        sbb(&mut state, Register::C);

        assert_eq!(state.a, 250);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sbb_d_subs_d_from_accumulator_with_carry_flag_set() {
        let mut state = State {
            a: 5,
            d: 10,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        sbb(&mut state, Register::D);

        assert_eq!(state.a, 250);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sbb_e_subs_e_from_accumulator_with_carry_flag_set() {
        let mut state = State {
            a: 5,
            e: 10,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        sbb(&mut state, Register::E);

        assert_eq!(state.a, 250);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sbb_h_subs_h_from_accumulator_with_carry_flag_set() {
        let mut state = State {
            a: 5,
            h: 10,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        sbb(&mut state, Register::H);

        assert_eq!(state.a, 250);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sbb_m_subs_byte_at_hl_address_to_accumulator_with_carry_flag_set() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 10],
            a: 5,
            h: 0x00,
            l: 0x05,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        sbb(&mut state, Register::M);

        assert_eq!(state.a, 250);
    }

    #[test]
    fn sbb_sub_with_carry_bit() {
        let mut state = State {
            a: 4,
            l: 2,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        sbb(&mut state, Register::L);

        assert_eq!(state.a, 1);
        assert_eq!(state.cc.carry, false);
        assert_eq!(state.cc.zero, false);
    }

    #[test]
    fn sbb_sub_with_max_values() {
        let mut state = State {
            a: u8::max_value(),
            b: u8::max_value(),
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        sbb(&mut state, Register::B);

        assert_eq!(state.a, 255);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn sbi_removes_immediate_byte_from_accumulator_with_borrow() {
        let mut state = State {
            memory: vec![0xFF, 0xFF, 0x00, 0x01],
            a: 0x00,
            ..State::default()
        };

        sbi(&mut state);
        assert_eq!(state.a, 0x01);
        assert_eq!(state.cc.carry, true);

        sbi(&mut state);
        assert_eq!(state.a, 0x01);
        assert_eq!(state.cc.carry, true);

        sbi(&mut state);
        assert_eq!(state.a, 0x00);
        assert_eq!(state.cc.carry, false);

        sbi(&mut state);
        assert_eq!(state.a, 0xFF);
        assert_eq!(state.cc.carry, true);
    }
}
