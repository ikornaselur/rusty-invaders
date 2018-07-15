use super::State;
use std::mem::swap;

impl State {
    pub fn xchg(&mut self) -> Option<u8> {
        swap(&mut self.h, &mut self.d);
        swap(&mut self.l, &mut self.e);
        None
    }

    pub fn xthl(&mut self) -> Option<u8> {
        swap(&mut self.h, &mut self.memory[(self.sp + 1) as usize]);
        swap(&mut self.l, &mut self.memory[self.sp as usize]);
        None
    }
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

        state.xchg();

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

        state.xthl();

        assert_eq!(state.h, 0xAD);
        assert_eq!(state.l, 0xDE);
        assert_eq!(state.memory, vec![0, 0xEF, 0xBE]);
    }
}
