use state::State;

/// Write the next byte address to the stack and jump to a predefined RST address at the start
///
/// # Cycles
///
/// 11
///
/// # Arguments
///
/// * `state` - The state to perform the restart in
/// * `rst` - Which restart to perform, from 0 to 7 (inclusive)
///
pub fn rst(state: &mut State, rst: usize) -> u8 {
    if rst > 7 {
        panic!("rst doesn't support {}", rst);
    }

    let most = (state.pc >> 8) as u8;
    let least = state.pc as u8;

    state.write_byte_to_stack(most);
    state.write_byte_to_stack(least);

    state.pc = (8 * rst) as u16;

    11
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rst_0_writes_pc_to_stack_and_sets_pc_to_0() {
        let mut state = State {
            memory: vec![0; 4],
            pc: 0xDEAD,
            sp: 4,
            ..State::default()
        };

        rst(&mut state, 0);

        assert_eq!(state.pc, 0x00);
        assert_eq!(state.memory, vec![0, 0, 0xAD, 0xDE]);
    }
}
