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
    if state.cc.carry {
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
    if !state.cc.carry {
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
    if state.cc.zero {
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
    if !state.cc.zero {
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
    if state.cc.sign {
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
    if !state.cc.sign {
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
    if state.cc.parity {
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
    if !state.cc.parity {
        state.pc = address;
    }

    10
}

#[cfg(test)]
mod test {
    use super::*;
    use state::ConditionCodes;

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
            cc: ConditionCodes {
                carry: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        jc(&mut state);

        assert_eq!(state.pc, 2);

        state.cc.carry = true;
        jc(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jnc_sets_pc_to_new_address_if_carry_flag_is_not_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        jnc(&mut state);

        assert_eq!(state.pc, 2);

        state.cc.carry = false;
        jnc(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jz_sets_pc_to_new_address_if_zero_flag_is_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                zero: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        jz(&mut state);

        assert_eq!(state.pc, 2);

        state.cc.zero = true;
        jz(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jnz_sets_pc_to_new_address_if_zero_flag_is_not_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                zero: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        jnz(&mut state);

        assert_eq!(state.pc, 2);

        state.cc.zero = false;
        jnz(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jm_sets_pc_to_new_address_if_sign_flag_is_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                sign: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        jm(&mut state);

        assert_eq!(state.pc, 2);

        state.cc.sign = true;
        jm(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jp_sets_pc_to_new_address_if_sign_flag_is_not_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                sign: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        jp(&mut state);

        assert_eq!(state.pc, 2);

        state.cc.sign = false;
        jp(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jpe_sets_pc_to_new_address_if_parity_flag_is_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                parity: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        jpe(&mut state);

        assert_eq!(state.pc, 2);

        state.cc.parity = true;
        jpe(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jpo_sets_pc_to_new_address_if_parity_flag_is_not_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                parity: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        jpo(&mut state);

        assert_eq!(state.pc, 2);

        state.cc.parity = false;
        jpo(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }
}
