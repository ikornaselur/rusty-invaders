use super::Register;
use super::State;

impl State {
    pub fn sta(&mut self) -> u8 {
        let address = self.read_address().unwrap();

        self.memory[address as usize] = self.a;

        13
    }

    pub fn shld(&mut self) -> u8 {
        let address = self.read_address().unwrap();

        self.memory[address as usize] = self.l;
        self.memory[(address + 1) as usize] = self.h;

        16
    }

    pub fn stax(&mut self, register: Register) -> u8 {
        let address = match register {
            Register::B => (u16::from(self.b) << 8) + u16::from(self.c),
            Register::D => (u16::from(self.d) << 8) + u16::from(self.e),
            unsupported => {
                panic!("stax doesn't support {:?}", unsupported);
            }
        };

        self.memory[address as usize] = self.a;

        13
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

    #[test]
    fn stax_b_stores_accumulator_at_address_from_b_c() {
        let mut state = State {
            memory: vec![0, 0, 0],
            a: 0xFF,
            b: 0x00,
            c: 0x02,
            ..State::default()
        };

        state.stax(Register::B);

        assert_eq!(state.memory, vec![0, 0, 0xFF]);
    }

    #[test]
    fn stax_d_stores_accumulator_at_address_from_d_e() {
        let mut state = State {
            memory: vec![0, 0, 0],
            a: 0xFF,
            d: 0x00,
            e: 0x02,
            ..State::default()
        };

        state.stax(Register::D);

        assert_eq!(state.memory, vec![0, 0, 0xFF]);
    }
}
