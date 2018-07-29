use cpu::state::State;
use std::mem::swap;

/// Swap the contents of register pairs HL with DE
///
/// # Cycles
///
/// 5
///
/// # Arguments
///
/// * `state` - The state to perform the swap in
///
pub fn xchg(state: &mut State) -> u8 {
    swap(&mut state.h, &mut state.d);
    swap(&mut state.l, &mut state.e);

    5
}

/// Swap the contents of register pairs HL with the top of the stack
///
/// # Cycles
///
/// 18
///
/// # Arguments
///
/// * `state` - The state to perform the swap in
///
pub fn xthl(state: &mut State) -> u8 {
    swap(&mut state.h, &mut state.memory[(state.sp + 1) as usize]);
    swap(&mut state.l, &mut state.memory[state.sp as usize]);

    18
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn xchg_exchanges_h_l_pair_with_d_e_pair() {
        let mut state = State {
            d: 0xDE,
            e: 0xAD,
            h: 0xBE,
            l: 0xEF,
            ..State::default()
        };

        xchg(&mut state);

        assert_eq!(state.d, 0xBE);
        assert_eq!(state.e, 0xEF);
        assert_eq!(state.h, 0xDE);
        assert_eq!(state.l, 0xAD);
    }

    #[test]
    fn xthl_exchanges_h_l_pair_with_bytes_on_stack() {
        let mut state = State {
            memory: vec![0, 0xDE, 0xAD],
            h: 0xBE,
            l: 0xEF,
            sp: 1,
            ..State::default()
        };

        xthl(&mut state);

        assert_eq!(state.h, 0xAD);
        assert_eq!(state.l, 0xDE);
        assert_eq!(state.memory, vec![0, 0xEF, 0xBE]);
    }
}
