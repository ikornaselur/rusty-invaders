use state::State;

/// Read the input from a port
///
/// Goes through the io interface on the state
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to read the input in
///
pub fn input(state: &mut State) -> u8 {
    let port = state.read_byte().unwrap();
    state.a = state.io.read(port as usize);

    10
}

/// Write the output to a port
///
/// Goes through the io interface on the state
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to write the output in
///
pub fn output(state: &mut State) -> u8 {
    let port = state.read_byte().unwrap();
    state.io.write(port as usize, state.a);

    10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_reads_from_input_port_into_accumulator() {
        let mut state = State {
            memory: vec![0x1, 0x2],
            ..State::default()
        };

        state.io.set(1, 0xDE);
        state.io.set(2, 0xAD);

        input(&mut state);
        assert_eq!(state.a, 0xDE);

        input(&mut state);
        assert_eq!(state.a, 0xAD);
    }

    #[test]
    fn output_writes_into_output_from_accumulator() {
        let mut state = State {
            memory: vec![0x1, 0x1],
            ..State::default()
        };

        state.a = 0xDE;
        output(&mut state);
        assert_eq!(state.io.read(1), 0xDE);

        state.a = 0xAD;
        output(&mut state);
        assert_eq!(state.io.read(1), 0xAD);
    }
}
