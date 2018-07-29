use cpu::register::Register;
use state::State;

/// Move a value from register to register
///
/// # Cycles
///
/// * To/from register M: 7
/// * Other: 5
///
/// # Arguments
/// * `state` - The state to perform the move in
/// * `to` - The register to move the value to
/// * `from` - The register to move the value from
///
pub fn mov(state: &mut State, to: Register, from: Register) -> u8 {
    let val = match from {
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
            panic!("mov doesn't support moving from {:?}", unsupported);
        }
    };

    match to {
        Register::A => state.a = val,
        Register::B => state.b = val,
        Register::C => state.c = val,
        Register::D => state.d = val,
        Register::E => state.e = val,
        Register::H => state.h = val,
        Register::L => state.l = val,
        Register::M => {
            let offset = (u16::from(state.h) << 8) + u16::from(state.l);
            state.memory[offset as usize] = val;
        }
        unsupported => {
            panic!("mov doesn't support moving to {:?}", unsupported);
        }
    };

    match (to, from) {
        (Register::M, _) | (_, Register::M) => 7,
        _ => 5,
    }
}

/// Move an immediate byte to a register
///
/// # Cycles
///
/// * Register M: 10
/// * Other: 7
///
/// # Arguments
/// * `state` - The state to perform the move in
/// * `to` - The register to move the value to
///
pub fn mvi(state: &mut State, to: Register) -> u8 {
    let byte = state.read_byte().unwrap();

    match to {
        Register::A => state.a = byte,
        Register::B => state.b = byte,
        Register::C => state.c = byte,
        Register::D => state.d = byte,
        Register::E => state.e = byte,
        Register::H => state.h = byte,
        Register::L => state.l = byte,
        Register::M => {
            let offset = (u16::from(state.h) << 8) + u16::from(state.l);
            state.memory[offset as usize] = byte;
        }
        unsupported => {
            panic!("mov doesn't support moving to {:?}", unsupported);
        }
    };

    match to {
        Register::M => 10,
        _ => 7,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mov_moves_between_registers() {
        let mut state = State {
            a: 2,
            b: 3,
            c: 4,
            ..State::default()
        };

        mov(&mut state, Register::A, Register::B);

        assert_eq!(state.a, 3);

        mov(&mut state, Register::A, Register::C);

        assert_eq!(state.a, 4);

        mov(&mut state, Register::A, Register::A);

        assert_eq!(state.a, 4);
    }

    #[test]
    fn mov_moves_from_memory_address_if_from_m() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 5],
            a: 2,
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        mov(&mut state, Register::A, Register::M);

        assert_eq!(state.a, 5);
    }

    #[test]
    fn mov_moves_to_memory_address_if_to_m() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 5],
            a: 2,
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        mov(&mut state, Register::M, Register::A);

        assert_eq!(state.memory[5], 2);
    }

    #[test]
    fn mvi_sets_register_to_byte() {
        let mut state = State {
            memory: vec![0x11, 0x12],
            ..State::default()
        };

        mvi(&mut state, Register::A);

        assert_eq!(state.a, 0x11);

        mvi(&mut state, Register::B);

        assert_eq!(state.b, 0x12);
    }

    #[test]
    fn mvi_sets_byte_in_memory_to_byte_for_register_m() {
        let mut state = State {
            memory: vec![0x11, 0x00, 0x00, 0x00, 0x00, 0x00],
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        mvi(&mut state, Register::M);

        assert_eq!(state.memory[5], 0x11);
    }
}
