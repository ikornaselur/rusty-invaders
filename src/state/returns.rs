use super::State;

impl State {
    pub fn ret(&mut self) -> () {
        self.pc = self.read_address_from_stack().unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ret_pops_the_address_off_the_stack_and_jumps_back() {
        let mut state = State {
            memory: vec![0, 0x08 /* SP */, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            ..State::default()
        };

        state.ret();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }
}
