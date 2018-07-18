use super::State;

impl State {
    pub fn rst(&mut self, rst: usize) -> u8 {
        if rst > 7 {
            panic!("rst doesn't support {}", rst);
        }

        let most = (self.pc >> 8) as u8;
        let least = self.pc as u8;

        self.write_byte_to_stack(most);
        self.write_byte_to_stack(least);

        self.pc = (8 * rst) as u16;

        11
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rst_0_writes_pc_to_stack_and_sets_pc_to_0() {
        let mut state = State {
            memory: vec![0; 4],
            pc: 0xDEAD,
            sp: 4,
            ..State::default()
        };

        state.rst(0);

        assert_eq!(state.pc, 0x00);
        assert_eq!(state.memory, vec![0, 0, 0xAD, 0xDE]);
    }
}
