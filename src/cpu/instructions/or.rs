use cpu::register::Register;
use cpu::state::State;

/// Perform and or between accumulator and register and put results into the accumulator
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
/// * `state` - The state to perform the or in
/// * `register` - The register to perform the or with
///
pub fn ora(state: &mut State, register: Register) -> u8 {
    let result = state.a | match register {
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
            panic!("ora doesn't support {:?}", unsupported);
        }
    };

    state.a = result;
    state.flags.set(result, false);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Perform and or between accumulator and the next immediate byte and put results into the accumulator
///
/// Sets condition flags
///
/// # Cycles
///
/// 7
///
/// # Arguments
///
/// * `state` - The state to perform the or in
///
pub fn ori(state: &mut State) -> u8 {
    let byte = state.read_byte().unwrap();

    let result = state.a | byte;

    state.a = result;
    state.flags.set(result, false);

    7
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn ora_resets_carry_bit() {
        let mut state = State {
            a: 123,
            b: 123,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        ora(&mut state, Register::B);

        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn ora_b_ors_b_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            b: 0b0111_1000,
            ..State::default()
        };

        ora(&mut state, Register::B);

        assert_eq!(state.a, 0b0111_1100);
    }

    #[test]
    fn ora_c_ors_c_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            c: 0b0111_1000,
            ..State::default()
        };

        ora(&mut state, Register::C);

        assert_eq!(state.a, 0b0111_1100);
    }

    #[test]
    fn ora_d_ors_d_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            d: 0b0111_1000,
            ..State::default()
        };

        ora(&mut state, Register::D);

        assert_eq!(state.a, 0b0111_1100);
    }

    #[test]
    fn ora_e_ors_e_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            e: 0b0111_1000,
            ..State::default()
        };

        ora(&mut state, Register::E);

        assert_eq!(state.a, 0b0111_1100);
    }

    #[test]
    fn ora_h_ors_h_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            h: 0b0111_1000,
            ..State::default()
        };

        ora(&mut state, Register::H);

        assert_eq!(state.a, 0b0111_1100);
    }

    #[test]
    fn ora_l_ors_l_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            l: 0b0111_1000,
            ..State::default()
        };

        ora(&mut state, Register::L);

        assert_eq!(state.a, 0b0111_1100);
    }

    #[test]
    fn ora_a_ors_a_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            ..State::default()
        };

        ora(&mut state, Register::A);

        assert_eq!(state.a, 0b1111_1100);
    }

    #[test]
    fn ora_m_ors_byte_at_hl_address_to_accumulator() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0b0111_1000],
            a: 0b0101_1100,
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        ora(&mut state, Register::M);

        assert_eq!(state.a, 0b0111_1100);
    }

    #[test]
    fn ori_ors_immediate_byte_with_accumulator() {
        let mut state = State {
            memory: vec![0b0011_0101, 0b0010_0110],
            a: 0b0111_0000,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        ori(&mut state);
        assert_eq!(state.a, 0b0111_0101);
        assert_eq!(state.flags.carry, false);

        ori(&mut state);
        assert_eq!(state.a, 0b0111_0111);
        assert_eq!(state.flags.carry, false);
    }
}
