use state::State;

/// Perform decimal adjustment, ignoring the Auxiliary Carry
///
/// Sets conditions flags
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `state` - The state to perform the DDA in
///
pub fn daa(state: &mut State) -> u8 {
    if state.a & 0x0f > 9 {
        state.a += 6;
    }
    if state.a & 0xf0 > 0x90 {
        let (result, carry) = state.a.overflowing_add(0x60);
        state.a = result;
        state.set_flags(result, carry);
    }

    4
}
