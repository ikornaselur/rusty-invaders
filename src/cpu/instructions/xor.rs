use cpu::register::Register;
use state::State;

/// Logical xor accumulator to register
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
/// * `state` - The state to perform the xor in
/// * `register` - The register to perform the xor with
///
pub fn xra(state: &mut State, register: Register) -> u8 {
    let result = state.a ^ match register {
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
            panic!("xra doesn't support {:?}", unsupported);
        }
    };

    state.a = result;
    state.flags.set(result, false);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Logical xor accumulator to the immediate byte
///
/// Sets condition flags
///
/// # Cycles
///
/// 7
///
/// # Arguments
///
/// * `state` - The state to perform the xor in
///
pub fn xri(state: &mut State) -> u8 {
    let byte = state.read_byte().unwrap();

    let result = state.a ^ byte;

    state.a = result;
    state.flags.set(result, false);

    7
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn xra_resets_carry_bit() {
        let mut state = State {
            a: 123,
            b: 123,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        xra(&mut state, Register::B);

        assert_eq!(state.flags.carry, false);
    }

    #[test]
    fn xra_b_xors_b_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            b: 0b0111_1000,
            ..State::default()
        };

        xra(&mut state, Register::B);

        assert_eq!(state.a, 0b0010_0100);
    }

    #[test]
    fn xra_c_xors_c_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            c: 0b0111_1000,
            ..State::default()
        };

        xra(&mut state, Register::C);

        assert_eq!(state.a, 0b0010_0100);
    }

    #[test]
    fn xra_d_xors_d_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            d: 0b0111_1000,
            ..State::default()
        };

        xra(&mut state, Register::D);

        assert_eq!(state.a, 0b0010_0100);
    }

    #[test]
    fn xra_e_xors_e_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            e: 0b0111_1000,
            ..State::default()
        };

        xra(&mut state, Register::E);

        assert_eq!(state.a, 0b0010_0100);
    }

    #[test]
    fn xra_h_xors_h_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            h: 0b0111_1000,
            ..State::default()
        };

        xra(&mut state, Register::H);

        assert_eq!(state.a, 0b0010_0100);
    }

    #[test]
    fn xra_l_xors_l_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            l: 0b0111_1000,
            ..State::default()
        };

        xra(&mut state, Register::L);

        assert_eq!(state.a, 0b0010_0100);
    }

    #[test]
    fn xra_a_xors_a_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            ..State::default()
        };

        xra(&mut state, Register::A);

        assert_eq!(state.a, 0b0000_0000);
    }

    #[test]
    fn xra_m_xors_byte_at_hl_address_to_accumulator() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0b0111_1000],
            a: 0b0101_1100,
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        xra(&mut state, Register::M);

        assert_eq!(state.a, 0b0010_0100);
    }

    #[test]
    fn xri_xors_immediate_byte_with_accumulator() {
        let mut state = State {
            memory: vec![0b0011_0101, 0b0010_0110],
            a: 0b0111_0000,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        xri(&mut state);
        assert_eq!(state.a, 0b0100_0101);
        assert_eq!(state.flags.carry, false);

        xri(&mut state);
        assert_eq!(state.a, 0b0110_0011);
        assert_eq!(state.flags.carry, false);
    }
}
