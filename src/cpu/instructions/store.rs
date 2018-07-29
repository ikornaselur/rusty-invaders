use state::{Register, State};

/// Store the accumulator at the immediate address
///
/// # Cycles
///
/// 13
///
/// # Argumnets
///
/// * `state` - The state to perform the storing in
///
pub fn sta(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();

    state.memory[address as usize] = state.a;

    13
}

/// Store H and L at the immediate address
///
/// # Cycles
///
/// 16
///
/// # Arguments
///
/// * `state` - The state to perform the storing in
///
pub fn shld(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();

    state.memory[address as usize] = state.l;
    state.memory[(address + 1) as usize] = state.h;

    16
}

/// Store the accumulator in the address from the given register pair
///
/// # Cycles
///
/// 13
///
/// # Arguments
///
/// * `state` - The state to perform the storing in
/// * `register` - The register pair to read the address from
///
pub fn stax(state: &mut State, register: Register) -> u8 {
    let address = match register {
        Register::B => (u16::from(state.b) << 8) + u16::from(state.c),
        Register::D => (u16::from(state.d) << 8) + u16::from(state.e),
        unsupported => {
            panic!("stax doesn't support {:?}", unsupported);
        }
    };

    state.memory[address as usize] = state.a;

    13
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sta_stores_accumulator_at_address() {
        let mut state = State {
            memory: vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAA],
            a: 0xFF,
            pc: 2,
            ..State::default()
        };

        sta(&mut state);

        assert_eq!(state.pc, 4);
        assert_eq!(state.memory, vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xFF]);
    }

    #[test]
    fn shld_stores_h_and_l_at_address() {
        let mut state = State {
            memory: vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0x22, 0x21],
            h: 0xDE,
            l: 0xAD,
            pc: 2,
            ..State::default()
        };

        shld(&mut state);

        assert_eq!(state.pc, 4);
        assert_eq!(
            state.memory,
            vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAD, 0xDE]
        );
    }

    #[test]
    fn stax_b_stores_accumulator_at_address_from_b_c() {
        let mut state = State {
            memory: vec![0, 0, 0],
            a: 0xFF,
            b: 0x00,
            c: 0x02,
            ..State::default()
        };

        stax(&mut state, Register::B);

        assert_eq!(state.memory, vec![0, 0, 0xFF]);
    }

    #[test]
    fn stax_d_stores_accumulator_at_address_from_d_e() {
        let mut state = State {
            memory: vec![0, 0, 0],
            a: 0xFF,
            d: 0x00,
            e: 0x02,
            ..State::default()
        };

        stax(&mut state, Register::D);

        assert_eq!(state.memory, vec![0, 0, 0xFF]);
    }
}
