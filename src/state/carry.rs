use super::State;

impl State {
    pub fn stc(&mut self) -> u8 {
        self.cc.carry = true;
        4
    }

    pub fn cmc(&mut self) -> u8 {
        self.cc.carry = !self.cc.carry;
        4
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
