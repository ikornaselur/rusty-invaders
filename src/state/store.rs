use super::State;

impl State {
    pub fn sta(&mut self) -> () {
        let least = self.read_byte().unwrap();
        let most = self.read_byte().unwrap();
        let address = ((most as u16) << 8) + least as u16;

        self.memory[address as usize] = self.a;
    }

    pub fn shld(&mut self) -> () {
        let least = self.read_byte().unwrap();
        let most = self.read_byte().unwrap();
        let address = ((most as u16) << 8) + least as u16;

        self.memory[address as usize] = self.l;
        self.memory[(address + 1) as usize] = self.h;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sta_stores_accumulator_at_address() {
        let mut state = State {
            memory: vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAA],
            a: 0xFF,
            pc: 2,
            ..State::default()
        };

        state.sta();

        assert_eq!(state.pc, 4);
        assert_eq!(state.memory, vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xFF]);
    }

    #[test]
    fn shld_stores_h_and_l_at_address() {
        let mut state = State {
            memory: vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0x22, 0x21],
            h: 0xDE,
            l: 0xAD,
            pc: 2,
            ..State::default()
        };

        state.shld();

        assert_eq!(state.pc, 4);
        assert_eq!(
            state.memory,
            vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAD, 0xDE]
        );
    }
}
