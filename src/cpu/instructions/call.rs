use state::State;

fn process_call(state: &mut State, address: u16) -> () {
    // A specific hack for full cpu test
    if state.debug && address == 5 && state.c == 9 {
        let offset = (u16::from(state.d) << 8) + u16::from(state.e);
        if offset == 0x018B {
            panic!("CPU HAS FAILED");
        } else if offset == 0x0174 {
            println!("*** CPU IS OPERATIONAL ***");
            state.exit = true;
        } else {
            panic!("UNKNOWN PRINT");
        }
    }
    // End of said hack

    let least = state.pc as u8;
    let most = (state.pc >> 8) as u8;

    state.write_byte_to_stack(most);
    state.write_byte_to_stack(least);

    state.pc = address;
}

/// Jump to the specified address
///
/// Cycles: 17
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn call(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();

    process_call(state, address);

    17
}

/// Jump to the specified address if carry bit is set
///
/// Cycles: 17 if jump taken, else 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cc(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.cc.carry {
        process_call(state, address);
        17
    } else {
        11
    }
}

/// Jump to the specified address if carry bit is not set
///
/// Cycles: 17 if jump taken, else 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cnc(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.cc.carry {
        11
    } else {
        process_call(state, address);
        17
    }
}

/// Jump to the specified address if zero bit is set
///
/// Cycles: 17 if jump taken, else 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cz(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.cc.zero {
        process_call(state, address);
        17
    } else {
        11
    }
}

/// Jump to the specified address if zero bit is not set
///
/// Cycles: 17 if jump taken, else 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cnz(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.cc.zero {
        11
    } else {
        process_call(state, address);
        17
    }
}

/// Jump to the specified address if sign bit is set
///
/// Cycles: 17 if jump taken, else 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cm(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.cc.sign {
        process_call(state, address);
        17
    } else {
        11
    }
}

/// Jump to the specified address if sign bit is not set
///
/// Cycles: 17 if jump taken, else 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cp(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.cc.sign {
        11
    } else {
        process_call(state, address);
        17
    }
}

/// Jump to the specified address if parity bit is set
///
/// Cycles: 17 if jump taken, else 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cpe(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.cc.parity {
        process_call(state, address);
        17
    } else {
        11
    }
}

/// Jump to the specified address if parity bit is not set
///
/// Cycles: 17 if jump taken, else 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cpo(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.cc.parity {
        11
    } else {
        process_call(state, address);
        17
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use state::ConditionCodes;

    #[test]
    fn call_pushes_the_address_after_to_the_stack_and_jumps() {
        let mut state = State {
            memory: vec![0, 0, 0, 0 /* SP */, 0, 0, 0xAD /* PC */, 0xDE],
            sp: 3,
            pc: 6,
            ..State::default()
        };

        call(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(
            state.memory,
            vec![0, 0x08 /* SP */, 0x00, 0, 0, 0, 0xAD, 0xDE]
        )
    }

    #[test]
    fn cc_pushes_the_address_after_to_the_stack_and_jumps_if_carry_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                carry: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        cc(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.carry = true;
        cc(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cnc_pushes_the_address_after_to_the_stack_and_jumps_if_carry_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        cnc(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.carry = false;
        cnc(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cz_pushes_the_address_after_to_the_stack_and_jumps_if_zero_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                zero: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        cz(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.zero = true;
        cz(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cnz_pushes_the_address_after_to_the_stack_and_jumps_if_zero_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                zero: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        cnz(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.zero = false;
        cnz(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cm_pushes_the_address_after_to_the_stack_and_jumps_if_sign_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                sign: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        cm(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.sign = true;
        cm(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cp_pushes_the_address_after_to_the_stack_and_jumps_if_sign_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                sign: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        cp(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.sign = false;
        cp(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cpe_pushes_the_address_after_to_the_stack_and_jumps_if_parity_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                parity: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        cpe(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.parity = true;
        cpe(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cpo_pushes_the_address_after_to_the_stack_and_jumps_if_parity_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                parity: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        cpo(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.parity = false;
        cpo(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }
}
