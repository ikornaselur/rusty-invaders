use super::State;
use std::mem::swap;

impl State {
    pub fn xchg(&mut self) -> () {
        swap(&mut self.h, &mut self.d);
        swap(&mut self.l, &mut self.e);
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
}
