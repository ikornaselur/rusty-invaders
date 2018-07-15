use super::State;

impl State {
    pub fn stc(&mut self) -> Option<u8> {
        // 4 cycles
        self.cc.carry = true;
        None
    }

    pub fn cmc(&mut self) -> Option<u8> {
        // 4 cycles
        self.cc.carry = !self.cc.carry;
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stc_sets_carry_bit() {
        let mut state = State { ..State::default() };

        state.stc();

        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn cmc_reverses_carry_bit() {
        let mut state = State { ..State::default() };

        state.cmc();
        assert_eq!(state.cc.carry, true);

        state.cmc();
        assert_eq!(state.cc.carry, false);

        state.cmc();
        assert_eq!(state.cc.carry, true);
    }
}
