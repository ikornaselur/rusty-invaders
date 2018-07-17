use super::Register;
use super::State;

impl State {
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
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
