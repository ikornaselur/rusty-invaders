use super::State;

impl State {
    pub fn input(&mut self) -> () {
        let port = self.read_byte().unwrap();
        self.a = self.io.read(port as usize);
    }

    pub fn output(&mut self) -> () {
        let port = self.read_byte().unwrap();
        self.io.write(port as usize, self.a);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_reads_from_input_port_into_accumulator() {
        let mut state = State {
            memory: vec![0x1, 0x2],
            ..State::default()
        };

        state.io.set(1, 0xDE);
        state.io.set(2, 0xAD);

        state.input();
        assert_eq!(state.a, 0xDE);

        state.input();
        assert_eq!(state.a, 0xAD);
    }

    #[test]
    fn output_writes_into_output_from_accumulator() {
        let mut state = State {
            memory: vec![0x1, 0x1],
            ..State::default()
        };

        state.a = 0xDE;
        state.output();
        assert_eq!(state.io.read(1), 0xDE);

        state.a = 0xAD;
        state.output();
        assert_eq!(state.io.read(1), 0xAD);
    }
}
