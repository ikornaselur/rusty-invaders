use state::State;

/// Perform an unconditional return to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to perform the return in
///
pub fn ret(state: &mut State) -> u8 {
    state.pc = state.read_address_from_stack().unwrap();

    10
}

/// Perform a return, if the carry bit is set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `state` - The state to perform the return in
///
pub fn rc(state: &mut State) -> u8 {
    if state.cc.carry {
        ret(state);
        11
    } else {
        5
    }
}

/// Perform a return, if the carry bit is not set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `state` - The state to perform the return in
///
pub fn rnc(state: &mut State) -> u8 {
    if state.cc.carry {
        5
    } else {
        ret(state);
        11
    }
}

/// Perform a return, if the zero bit is set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `state` - The state to perform the return in
///
pub fn rz(state: &mut State) -> u8 {
    if state.cc.zero {
        ret(state);
        11
    } else {
        5
    }
}

/// Perform a return, if the zero bit is not set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `state` - The state to perform the return in
///
pub fn rnz(state: &mut State) -> u8 {
    if state.cc.zero {
        5
    } else {
        ret(state);
        11
    }
}

/// Perform a return, if the sign bit is set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `state` - The state to perform the return in
///
pub fn rm(state: &mut State) -> u8 {
    if state.cc.sign {
        ret(state);
        11
    } else {
        5
    }
}

/// Perform a return, if the sign bit is not set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `state` - The state to perform the return in
///
pub fn rp(state: &mut State) -> u8 {
    if state.cc.sign {
        5
    } else {
        ret(state);
        11
    }
}

/// Perform a return, if the parity bit is set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `state` - The state to perform the return in
///
pub fn rpe(state: &mut State) -> u8 {
    if state.cc.parity {
        ret(state);
        11
    } else {
        5
    }
}

/// Perform a return, if the parity bit is not set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `state` - The state to perform the return in
///
pub fn rpo(state: &mut State) -> u8 {
    if state.cc.parity {
        5
    } else {
        ret(state);
        11
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn ret_pops_the_address_off_the_stack_and_jumps_back() {
        let mut state = State {
            memory: vec![0, 0x08 /* SP */, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            ..State::default()
        };

        ret(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rc_pops_the_address_off_the_stack_and_jumps_back_if_carry_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: Flags {
                carry: false,
                ..Flags::default()
            },
            ..State::default()
        };

        rc(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.carry = true;
        rc(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rnc_pops_the_address_off_the_stack_and_jumps_back_if_carry_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        rnc(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.carry = false;
        rnc(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rz_pops_the_address_off_the_stack_and_jumps_back_if_zero_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: Flags {
                zero: false,
                ..Flags::default()
            },
            ..State::default()
        };

        rz(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.zero = true;
        rz(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rnz_pops_the_address_off_the_stack_and_jumps_back_if_zero_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: Flags {
                zero: true,
                ..Flags::default()
            },
            ..State::default()
        };

        rnz(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.zero = false;
        rnz(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rm_pops_the_address_off_the_stack_and_jumps_back_if_sign_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: Flags {
                sign: false,
                ..Flags::default()
            },
            ..State::default()
        };

        rm(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.sign = true;
        rm(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rp_pops_the_address_off_the_stack_and_jumps_back_if_sign_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: Flags {
                sign: true,
                ..Flags::default()
            },
            ..State::default()
        };

        rp(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.sign = false;
        rp(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rpe_pops_the_address_off_the_stack_and_jumps_back_if_parity_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: Flags {
                parity: false,
                ..Flags::default()
            },
            ..State::default()
        };

        rpe(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.parity = true;
        rpe(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rpo_pops_the_address_off_the_stack_and_jumps_back_if_parity_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: Flags {
                parity: true,
                ..Flags::default()
            },
            ..State::default()
        };

        rpo(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.parity = false;
        rpo(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }
}
