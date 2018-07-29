use cpu::register::Register;
use state::State;

/// Perform accumulator addition from a register
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
/// * `state` - The state to perform the addition in
/// * `register` - The register to add to the accumulator
///
pub fn add(state: &mut State, register: Register) -> u8 {
    let (result, carry) = match register {
        Register::A => state.a.overflowing_add(state.a),
        Register::B => state.a.overflowing_add(state.b),
        Register::C => state.a.overflowing_add(state.c),
        Register::D => state.a.overflowing_add(state.d),
        Register::E => state.a.overflowing_add(state.e),
        Register::H => state.a.overflowing_add(state.h),
        Register::L => state.a.overflowing_add(state.l),
        Register::M => {
            let offset = (u16::from(state.h) << 8) + u16::from(state.l);
            state.a.overflowing_add(state.memory[offset as usize])
        }
        unsupported => {
            panic!("add doesn't support {:?}", unsupported);
        }
    };

    state.a = result;
    state.set_flags(result, carry);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Perform accumulator addition with the next immediate byte
///
/// Sets condition flags
///
/// # Cycles
///
/// 7
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn adi(state: &mut State) -> u8 {
    let byte = state.read_byte().unwrap();
    let (result, carry) = state.a.overflowing_add(byte);

    state.a = result;
    state.set_flags(result, carry);

    7
}

/// Perform double addition to the pseudo register M
///
/// Sets carry flag
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
/// * `register` - The double register pair to add to M
///
pub fn dad(state: &mut State, register: Register) -> u8 {
    let current: u16 = (u16::from(state.h) << 8) + u16::from(state.l);
    let (result, carry) = match register {
        Register::B => current.overflowing_add((u16::from(state.b) << 8) + u16::from(state.c)),
        Register::D => current.overflowing_add((u16::from(state.d) << 8) + u16::from(state.e)),
        Register::H => current.overflowing_add((u16::from(state.h) << 8) + u16::from(state.l)),
        Register::SP => current.overflowing_add(state.sp),
        unsupported => {
            panic!("dad doesn't support {:?}", unsupported);
        }
    };

    state.l = result as u8;
    state.h = (result >> 8) as u8;
    state.flags.carry = carry;

    10
}

/// Perform accumulator addition from a register with the carry bit
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
/// * `state` - The state to perform the addition in
/// * `register` - The register to add to the accumulator
///
pub fn adc(state: &mut State, register: Register) -> u8 {
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
            panic!("adc doesn't support {:?}", unsupported);
        }
    };

    let (byte, byte_carry) = if state.flags.carry {
        byte.overflowing_add(1)
    } else {
        (byte, false)
    };

    let (result, carry) = state.a.overflowing_add(byte);

    state.a = result;
    state.set_flags(result, carry || byte_carry);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Perform accumulator addition with the next immediate byte and carry bit
///
/// Sets condition codes
///
/// # Cycles
///
/// 7
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn aci(state: &mut State) -> u8 {
    let byte = state.read_byte().unwrap();

    let (byte, byte_carry) = if state.flags.carry {
        byte.overflowing_add(1)
    } else {
        (byte, false)
    };

    let (result, carry) = state.a.overflowing_add(byte);

    state.a = result;
    state.set_flags(result, carry || byte_carry);

    7
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn add_b_adds_b_to_accumulator() {
        let mut state = State {
            a: 1,
            b: 2,
            ..State::default()
        };

        add(&mut state, Register::B);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_c_adds_c_to_accumulator() {
        let mut state = State {
            a: 1,
            c: 2,
            ..State::default()
        };

        add(&mut state, Register::C);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_d_adds_d_to_accumulator() {
        let mut state = State {
            a: 1,
            d: 2,
            ..State::default()
        };

        add(&mut state, Register::D);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_e_adds_e_to_accumulator() {
        let mut state = State {
            a: 1,
            e: 2,
            ..State::default()
        };

        add(&mut state, Register::E);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_h_adds_h_to_accumulator() {
        let mut state = State {
            a: 1,
            h: 2,
            ..State::default()
        };

        add(&mut state, Register::H);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_l_adds_l_to_accumulator() {
        let mut state = State {
            a: 1,
            l: 2,
            ..State::default()
        };

        add(&mut state, Register::L);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_a_adds_a_to_accumulator() {
        let mut state = State {
            a: 1,
            ..State::default()
        };

        add(&mut state, Register::A);

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

        add(&mut state, Register::M);

        assert_eq!(state.a, 6);
    }

    #[test]
    fn adi_adds_immediate_byte_to_accumulator() {
        let mut state = State {
            memory: vec![1, 5],
            a: 0xFF,
            ..State::default()
        };

        adi(&mut state);
        assert_eq!(state.a, 0);
        assert_eq!(state.flags.carry, true);

        adi(&mut state);
        assert_eq!(state.a, 5);
        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn dad_b_double_adds_b_c_to_h_l() {
        let mut state = State {
            b: 0x33,
            c: 0x9F,
            h: 0xA1,
            l: 0x7B,
            ..State::default()
        };

        dad(&mut state, Register::B);

        assert_eq!(state.h, 0xD5);
        assert_eq!(state.l, 0x1A);
        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn dad_d_double_adds_d_e_to_h_l() {
        let mut state = State {
            d: 0x33,
            e: 0x9F,
            h: 0xA1,
            l: 0x7B,
            ..State::default()
        };

        dad(&mut state, Register::D);

        assert_eq!(state.h, 0xD5);
        assert_eq!(state.l, 0x1A);
        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn dad_h_double_adds_h_l_to_h_l() {
        let mut state = State {
            h: 0x11,
            l: 0x22,
            ..State::default()
        };

        dad(&mut state, Register::H);

        assert_eq!(state.h, 0x22);
        assert_eq!(state.l, 0x44);
        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn dad_sp_double_adds_sp_to_h_l() {
        let mut state = State {
            h: 0x11,
            l: 0x22,
            sp: 0x1111,
            ..State::default()
        };

        dad(&mut state, Register::SP);

        assert_eq!(state.h, 0x22);
        assert_eq!(state.l, 0x33);
        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn adc_b_adds_b_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            b: 2,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        adc(&mut state, Register::B);

        assert_eq!(state.a, 4);
        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn adc_with_max_values() {
        let mut state = State {
            a: u8::max_value(),
            b: u8::max_value(),
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        adc(&mut state, Register::B);

        assert_eq!(state.a, 255u8);
        assert_eq!(state.flags.carry, true);
    }

    #[test]
    fn adc_where_carry_causes_carry() {
        let mut state = State {
            a: u8::max_value(),
            b: 0,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        adc(&mut state, Register::B);

        assert_eq!(state.a, 0);
        assert_eq!(state.flags.carry, true);
    }

    #[test]
    fn adc_c_adds_c_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            c: 2,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        adc(&mut state, Register::C);

        assert_eq!(state.a, 4);
        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn adc_d_adds_d_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            d: 2,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        adc(&mut state, Register::D);

        assert_eq!(state.a, 4);
        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn adc_e_adds_e_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            e: 2,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        adc(&mut state, Register::E);

        assert_eq!(state.a, 4);
        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn adc_h_adds_h_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            h: 2,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        adc(&mut state, Register::H);

        assert_eq!(state.a, 4);
        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn adc_l_adds_l_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            l: 2,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        adc(&mut state, Register::L);

        assert_eq!(state.a, 4);
        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn adc_m_adds_m_with_carry_to_accumulator() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 5],
            a: 1,
            h: 0x00,
            l: 0x05,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        adc(&mut state, Register::M);

        assert_eq!(state.a, 7);
        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn adc_a_adds_a_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        adc(&mut state, Register::A);

        assert_eq!(state.a, 3);
        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn aci_adds_immediate_byte_to_accumulator_with_carry() {
        let mut state = State {
            memory: vec![0xFF, 0xFF, 0x00, 0x01],
            a: 0xFF,
            ..State::default()
        };

        aci(&mut state);
        assert_eq!(state.a, 0xFE);
        assert_eq!(state.flags.carry, true);

        aci(&mut state);
        assert_eq!(state.a, 0xFE);
        assert_eq!(state.flags.carry, true);

        aci(&mut state);
        assert_eq!(state.a, 0xFF);
        assert_eq!(state.flags.carry, false);

        aci(&mut state);
        assert_eq!(state.a, 0x00);
        assert_eq!(state.flags.carry, true);
    }
}
