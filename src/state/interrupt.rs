use super::State;

impl State {
    pub fn ei(&mut self) -> u8 {
        self.int_enabled = true;

        4
    }

    pub fn di(&mut self) -> u8 {
        self.int_enabled = false;

        4
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ei_enables_interrupts() {
        let mut state = State {
            int_enabled: false,
            ..State::default()
        };

        state.ei();

        assert_eq!(state.int_enabled, true);
    }

    #[test]
    fn di_enables_interrupts() {
        let mut state = State {
            int_enabled: true,
            ..State::default()
        };

        state.di();

        assert_eq!(state.int_enabled, false);
    }
}
