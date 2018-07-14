use super::State;

impl State {
    pub fn call(&mut self) -> () {
        let address = self.read_address().unwrap();

        let least = self.pc as u8;
        let most = (self.pc >> 8) as u8;

        self.write_byte_to_stack(most);
        self.write_byte_to_stack(least);

        self.pc = address;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn call_pushes_the_address_after_to_the_stack_and_jumps() {
        let mut state = State {
            memory: vec![0, 0, 0, 0 /* SP */, 0, 0, 0xAD /* PC */, 0xDE],
            sp: 3,
            pc: 6,
            ..State::default()
        };

        state.call();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(
            state.memory,
            vec![0, 0x08 /* SP */, 0x00, 0, 0, 0, 0xAD, 0xDE]
        )
    }
}
