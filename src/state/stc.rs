use super::State;

impl State {
    pub fn stc(&mut self) -> () {
        // 4 cycles
        self.cc.carry = true;
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
}
