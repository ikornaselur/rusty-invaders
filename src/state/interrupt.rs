use super::State;

impl State {
    pub fn ei(&mut self) -> Option<u8> {
        self.int_enable = true;
        None
    }

    pub fn di(&mut self) -> Option<u8> {
        self.int_enable = false;
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ei_enables_interrupts() {
        let mut state = State {
            int_enable: false,
            ..State::default()
        };

        state.ei();

        assert_eq!(state.int_enable, true);
    }

    #[test]
    fn di_enables_interrupts() {
        let mut state = State {
            int_enable: true,
            ..State::default()
        };

        state.di();

        assert_eq!(state.int_enable, false);
    }
}
