use super::Register;
use super::State;

impl State {
    pub fn mov(&mut self, to: Register, from: Register) -> () {
        // 4 cycles
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
        }
        ()
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
}
