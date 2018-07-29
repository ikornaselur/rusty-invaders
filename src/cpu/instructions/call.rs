use cpu::state::State;

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

/// Call a subroutine at a specified address, storing the address of the next instruction on the
/// stack
///
/// # Cycles
///
/// 17
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

/// Call a subroutine at a specified address, if the carry bit is set, storing the address of the
/// next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cc(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.flags.carry {
        process_call(state, address);
        17
    } else {
        11
    }
}

/// Call a subroutine at a specified address, if the carry bit is not set, storing the address of
/// the next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cnc(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.flags.carry {
        11
    } else {
        process_call(state, address);
        17
    }
}

/// Call a subroutine at a specified address, if the zero bit is set, storing the address of the
/// next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cz(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.flags.zero {
        process_call(state, address);
        17
    } else {
        11
    }
}

/// Call a subroutine at a specified address, if the zero bit is not set, storing the address of
/// the next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cnz(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.flags.zero {
        11
    } else {
        process_call(state, address);
        17
    }
}

/// Call a subroutine at a specified address, if the sign bit is set, storing the address of the
/// next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cm(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.flags.sign {
        process_call(state, address);
        17
    } else {
        11
    }
}

/// Call a subroutine at a specified address, if the sign bit is not set, storing the address of
/// the next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cp(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.flags.sign {
        11
    } else {
        process_call(state, address);
        17
    }
}

/// Call a subroutine at a specified address, if the parity bit is set, storing the address of the
/// next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cpe(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.flags.parity {
        process_call(state, address);
        17
    } else {
        11
    }
}

/// Call a subroutine at a specified address, if the parity bit is not set, storing the address of
/// the next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `state` - The state to perform the addition in
///
pub fn cpo(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();
    if state.flags.parity {
        11
    } else {
        process_call(state, address);
        17
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

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
            flags: Flags {
                carry: false,
                ..Flags::default()
            },
            ..State::default()
        };

        cc(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.flags.carry = true;
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
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        cnc(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.flags.carry = false;
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
            flags: Flags {
                zero: false,
                ..Flags::default()
            },
            ..State::default()
        };

        cz(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.flags.zero = true;
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
            flags: Flags {
                zero: true,
                ..Flags::default()
            },
            ..State::default()
        };

        cnz(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.flags.zero = false;
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
            flags: Flags {
                sign: false,
                ..Flags::default()
            },
            ..State::default()
        };

        cm(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.flags.sign = true;
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
            flags: Flags {
                sign: true,
                ..Flags::default()
            },
            ..State::default()
        };

        cp(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.flags.sign = false;
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
            flags: Flags {
                parity: false,
                ..Flags::default()
            },
            ..State::default()
        };

        cpe(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.flags.parity = true;
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
            flags: Flags {
                parity: true,
                ..Flags::default()
            },
            ..State::default()
        };

        cpo(&mut state);

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.flags.parity = false;
        cpo(&mut state);

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }
}
