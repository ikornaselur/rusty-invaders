use super::State;

impl State {
    pub fn input(&mut self) -> Option<u8> {
        let port = self.read_byte().unwrap();
        self.a = self.input.read(port as usize);
        None
    }

    pub fn output(&mut self) -> Option<u8> {
        let port = self.read_byte().unwrap();
        self.output.write(port as usize, self.a);
        None
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

        state.input.write(1, 0xDE);
        state.input.write(2, 0xAD);

        state.input();
        assert_eq!(state.a, 0xDE);

        state.input();
        assert_eq!(state.a, 0xAD);
    }

    #[test]
    fn output_writes_into_ouput_from_accumulator() {
        let mut state = State {
            memory: vec![0x1, 0x2],
            ..State::default()
        };

        state.a = 0xDE;
        state.output();
        state.a = 0xAD;
        state.output();

        assert_eq!(state.output.read(1), 0xDE);
        assert_eq!(state.output.read(2), 0xAD);
    }
}
