use super::State;

impl State {
    pub fn cma(&mut self) -> Option<u8> {
        // 4 cycles
        self.a = !self.a;
        None
    }
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

        state.cma();

        assert_eq!(state.a, 0b0011_1010);

        state.cma();

        assert_eq!(state.a, 0b1100_0101);
    }
}
