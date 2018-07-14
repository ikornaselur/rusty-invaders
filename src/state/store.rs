use super::Register;
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

    pub fn stax(&mut self, register: Register) -> () {
        let address = match register {
            Register::B => ((self.b as u16) << 8) + self.c as u16,
            Register::D => ((self.d as u16) << 8) + self.e as u16,
            unsupported => {
                panic!("stax doesn't support {:?}", unsupported);
            }
        };

        self.memory[address as usize] = self.a;
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
