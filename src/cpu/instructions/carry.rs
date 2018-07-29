use cpu::state::State;

/// Set the carry flag
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `state` - The state to set the flag in
///
pub fn stc(state: &mut State) -> u8 {
    state.flags.carry = true;
    4
}

/// Complement the carry flag
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `state` - The state to complement the flag in
///
pub fn cmc(state: &mut State) -> u8 {
    state.flags.carry = !state.flags.carry;
    4
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stc_sets_carry_bit() {
        let mut state = State { ..State::default() };

        stc(&mut state);

        assert_eq!(state.flags.carry, true);
    }

    #[test]
    fn cmc_reverses_carry_bit() {
        let mut state = State { ..State::default() };

        cmc(&mut state);
        assert_eq!(state.flags.carry, true);

        cmc(&mut state);
        assert_eq!(state.flags.carry, false);

        cmc(&mut state);
        assert_eq!(state.flags.carry, true);
    }
}
