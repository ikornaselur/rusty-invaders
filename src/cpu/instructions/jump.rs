use state::State;

/// Perform a unconditional jump to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to perform the jump in
///
pub fn jmp(state: &mut State) -> u8 {
    state.pc = state.read_address().unwrap();

    10
}

/// Perform a jump, if the carry bit is set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to perform the jump in
///
pub fn jc(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.flags.carry {
        state.pc = address;
    }

    10
}

/// Perform a jump, if the carry bit is not set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to perform the jump in
///
pub fn jnc(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if !state.flags.carry {
        state.pc = address;
    }

    10
}

/// Perform a jump, if the zero bit is set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to perform the jump in
///
pub fn jz(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.flags.zero {
        state.pc = address;
    }

    10
}

/// Perform a jump, if the zero bit is not set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to perform the jump in
///
pub fn jnz(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if !state.flags.zero {
        state.pc = address;
    }

    10
}

/// Perform a jump, if the sign bit is set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to perform the jump in
///
pub fn jm(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.flags.sign {
        state.pc = address;
    }

    10
}

/// Perform a jump, if the sign bit is not set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to perform the jump in
///
pub fn jp(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if !state.flags.sign {
        state.pc = address;
    }

    10
}

/// Perform a jump, if the parity bit is set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to perform the jump in
///
pub fn jpe(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.flags.parity {
        state.pc = address;
    }

    10
}

/// Perform a jump, if the parity bit is not set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to perform the jump in
///
pub fn jpo(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if !state.flags.parity {
        state.pc = address;
    }

    10
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn jmp_sets_pc_to_new_address() {
        let mut state = State {
            memory: vec![0xAD, 0xDE],
            ..State::default()
        };

        jmp(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jc_sets_pc_to_new_address_if_carry_flag_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                carry: false,
                ..Flags::default()
            },
            ..State::default()
        };

        jc(&mut state);

        assert_eq!(state.pc, 2);

        state.flags.carry = true;
        jc(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jnc_sets_pc_to_new_address_if_carry_flag_is_not_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        jnc(&mut state);

        assert_eq!(state.pc, 2);

        state.flags.carry = false;
        jnc(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jz_sets_pc_to_new_address_if_zero_flag_is_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                zero: false,
                ..Flags::default()
            },
            ..State::default()
        };

        jz(&mut state);

        assert_eq!(state.pc, 2);

        state.flags.zero = true;
        jz(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jnz_sets_pc_to_new_address_if_zero_flag_is_not_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                zero: true,
                ..Flags::default()
            },
            ..State::default()
        };

        jnz(&mut state);

        assert_eq!(state.pc, 2);

        state.flags.zero = false;
        jnz(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jm_sets_pc_to_new_address_if_sign_flag_is_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                sign: false,
                ..Flags::default()
            },
            ..State::default()
        };

        jm(&mut state);

        assert_eq!(state.pc, 2);

        state.flags.sign = true;
        jm(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jp_sets_pc_to_new_address_if_sign_flag_is_not_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                sign: true,
                ..Flags::default()
            },
            ..State::default()
        };

        jp(&mut state);

        assert_eq!(state.pc, 2);

        state.flags.sign = false;
        jp(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jpe_sets_pc_to_new_address_if_parity_flag_is_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                parity: false,
                ..Flags::default()
            },
            ..State::default()
        };

        jpe(&mut state);

        assert_eq!(state.pc, 2);

        state.flags.parity = true;
        jpe(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jpo_sets_pc_to_new_address_if_parity_flag_is_not_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                parity: true,
                ..Flags::default()
            },
            ..State::default()
        };

        jpo(&mut state);

        assert_eq!(state.pc, 2);

        state.flags.parity = false;
        jpo(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }
}
