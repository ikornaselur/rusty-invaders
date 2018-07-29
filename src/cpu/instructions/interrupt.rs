use cpu::state::State;

/// Enable interrupts
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `state` - The state to enable interrupts in
///
pub fn ei(state: &mut State) -> u8 {
    state.int_enabled = true;

    4
}

/// Disable interrupts
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `state` - The state to disable interrupts in
///
pub fn di(state: &mut State) -> u8 {
    state.int_enabled = false;

    4
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ei_enables_interrupts() {
        let mut state = State {
            int_enabled: false,
            ..State::default()
        };

        ei(&mut state);

        assert_eq!(state.int_enabled, true);
    }

    #[test]
    fn di_enables_interrupts() {
        let mut state = State {
            int_enabled: true,
            ..State::default()
        };

        di(&mut state);

        assert_eq!(state.int_enabled, false);
    }
}
