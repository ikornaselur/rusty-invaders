use state::State;

/// Complement the accumulator
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `state` - The state to perform the complement in
///
pub fn cma(state: &mut State) -> u8 {
    state.a = !state.a;

    4
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cma_complements_accumulator() {
        let mut state = State {
            a: 0b1100_0101,
            ..State::default()
        };

        cma(&mut state);

        assert_eq!(state.a, 0b0011_1010);

        cma(&mut state);

        assert_eq!(state.a, 0b1100_0101);
    }
}
