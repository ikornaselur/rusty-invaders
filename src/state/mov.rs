use super::Register;
use super::State;

impl State {
    pub fn mov(&mut self, to: Register, from: Register) -> () {
        // 7 cycles if either is M, else 5
        let val = match from {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            Register::M => {
                let offset: u16 = ((self.h as u16) << 8) + self.l as u16;
                self.memory[offset as usize]
            }
            unsupported => {
                panic!("mov doesn't support moving from {:?}", unsupported);
            }
        };

        match to {
            Register::A => self.a = val,
            Register::B => self.b = val,
            Register::C => self.c = val,
            Register::D => self.d = val,
            Register::E => self.e = val,
            Register::H => self.h = val,
            Register::L => self.l = val,
            Register::M => {
                let offset: u16 = ((self.h as u16) << 8) + self.l as u16;
                self.memory[offset as usize] = val;
            }
            unsupported => {
                panic!("mov doesn't support moving to {:?}", unsupported);
            }
        };
    }

    pub fn mvi(&mut self, to: Register) -> () {
        // 10 cycles if either is M, else 7
        let byte = self.read_byte().unwrap();

        match to {
            Register::A => self.a = byte,
            Register::B => self.b = byte,
            Register::C => self.c = byte,
            Register::D => self.d = byte,
            Register::E => self.e = byte,
            Register::H => self.h = byte,
            Register::L => self.l = byte,
            Register::M => {
                let offset: u16 = ((self.h as u16) << 8) + self.l as u16;
                self.memory[offset as usize] = byte;
            }
            unsupported => {
                panic!("mov doesn't support moving to {:?}", unsupported);
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mov_moves_between_registers() {
        let mut state = State {
            a: 2,
            b: 3,
            c: 4,
            ..State::default()
        };

        state.mov(Register::A, Register::B);

        assert_eq!(state.a, 3);

        state.mov(Register::A, Register::C);

        assert_eq!(state.a, 4);

        state.mov(Register::A, Register::A);

        assert_eq!(state.a, 4);
    }

    #[test]
    fn mov_moves_from_memory_address_if_from_m() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 5],
            a: 2,
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        state.mov(Register::A, Register::M);

        assert_eq!(state.a, 5);
    }

    #[test]
    fn mov_moves_to_memory_address_if_to_m() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 5],
            a: 2,
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        state.mov(Register::M, Register::A);

        assert_eq!(state.memory[5], 2);
    }

    #[test]
    fn mvi_sets_register_to_byte() {
        let mut state = State {
            memory: vec![0x11, 0x12],
            ..State::default()
        };

        state.mvi(Register::A);

        assert_eq!(state.a, 0x11);

        state.mvi(Register::B);

        assert_eq!(state.b, 0x12);
    }

    #[test]
    fn mvi_sets_byte_in_memory_to_byte_for_register_m() {
        let mut state = State {
            memory: vec![0x11, 0x00, 0x00, 0x00, 0x00, 0x00],
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        state.mvi(Register::M);

        assert_eq!(state.memory[5], 0x11);
    }
}
