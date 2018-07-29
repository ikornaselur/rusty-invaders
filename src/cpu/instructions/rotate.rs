use state::State;

/// Rotate the accumulator left
///
/// Sets the carry flag if the left most bit is set before the rotation
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `state` - The state to perform the rotation in
///
pub fn rlc(state: &mut State) -> u8 {
    let carry = state.a >> 7 == 1;
    let result = state.a.rotate_left(1);

    state.a = result;
    state.set_flags(result, carry);

    4
}

/// Rotate the accumulator left, through the carry bit
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `state` - The state to perform the rotation in
///
pub fn ral(state: &mut State) -> u8 {
    let carry = state.a >> 7 == 1;
    let mut result = state.a << 1;

    if state.flags.carry {
        result |= 0x01;
    }

    state.a = result;
    state.set_flags(result, carry);

    4
}

/// Rotate the accumulator right
///
/// Sets the carry flag if the right most bit is set before the rotation
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `state` - The state to perform the rotation in
///
pub fn rrc(state: &mut State) -> u8 {
    let carry = state.a & 0x01 == 1;
    let result = state.a.rotate_right(1);

    state.a = result;
    state.set_flags(result, carry);

    4
}

/// Rotate the accumulator right, through the carry bit
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `state` - The state to perform the rotation in
///
pub fn rar(state: &mut State) -> u8 {
    let carry = state.a & 0x01 == 1;
    let mut result = state.a >> 1;

    if state.flags.carry {
        result |= 0x01 << 7;
    }

    state.a = result;
    state.set_flags(result, carry);

    4
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn rlc_rotates_accumulator_left() {
        let mut state = State {
            a: 0b0111_0010,
            flags: Flags {
                carry: false,
                ..Flags::default()
            },
            ..State::default()
        };

        rlc(&mut state);

        assert_eq!(state.a, 0b1110_0100);
        assert_eq!(state.flags.carry, false);

        rlc(&mut state);

        assert_eq!(state.a, 0b1100_1001);
        assert_eq!(state.flags.carry, true);
    }

    #[test]
    fn ral_rotates_accumulator_left_through_carry() {
        let mut state = State {
            a: 0b0111_0010,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..State::default()
        };

        ral(&mut state);

        assert_eq!(state.a, 0b1110_0101);
        assert_eq!(state.flags.carry, false);

        ral(&mut state);

        assert_eq!(state.a, 0b1100_1010);
        assert_eq!(state.flags.carry, true);
    }

    #[test]
    fn rrc_rotates_accumulator_right() {
        let mut state = State {
            a: 0b1111_0010,
            flags: Flags {
                carry: false,
                ..Flags::default()
            },
            ..State::default()
        };

        rrc(&mut state);

        assert_eq!(state.a, 0b0111_1001);
        assert_eq!(state.flags.carry, false);

        rrc(&mut state);

        assert_eq!(state.a, 0b1011_1100);
        assert_eq!(state.flags.carry, true);
    }

    #[test]
    fn rar_rotates_accumulator_right_through_carry() {
        let mut state = State {
            a: 0b1111_0011,
            flags: Flags {
                carry: false,
                ..Flags::default()
            },
            ..State::default()
        };

        rar(&mut state);

        assert_eq!(state.a, 0b0111_1001);
        assert_eq!(state.flags.carry, true);

        rar(&mut state);

        assert_eq!(state.a, 0b1011_1100);
        assert_eq!(state.flags.carry, true);

        rar(&mut state);

        assert_eq!(state.a, 0b1101_1110);
        assert_eq!(state.flags.carry, false);
    }
}
