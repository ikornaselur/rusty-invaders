use super::Register;
use super::State;

impl State {
    pub fn inr(&mut self, register: Register) -> () {
        // 4 instructions
        match register {
            Register::A => {
                let (result, carry) = self.a.overflowing_add(1);
                self.a = result;
                self.set_flags(result, carry);
            }
            Register::B => {
                let (result, carry) = self.b.overflowing_add(1);
                self.b = result;
                self.set_flags(result, carry);
            }
            Register::C => {
                let (result, carry) = self.c.overflowing_add(1);
                self.c = result;
                self.set_flags(result, carry);
            }
            Register::D => {
                let (result, carry) = self.d.overflowing_add(1);
                self.d = result;
                self.set_flags(result, carry);
            }
            Register::E => {
                let (result, carry) = self.e.overflowing_add(1);
                self.e = result;
                self.set_flags(result, carry);
            }
            Register::H => {
                let (result, carry) = self.h.overflowing_add(1);
                self.h = result;
                self.set_flags(result, carry);
            }
            Register::L => {
                let (result, carry) = self.l.overflowing_add(1);
                self.l = result;
                self.set_flags(result, carry);
            }
            Register::M => {
                let offset: u16 = ((self.h as u16) << 8) + self.l as u16;
                let byte = self.memory[offset as usize];

                let (result, carry) = byte.overflowing_add(1);
                self.memory[offset as usize] = result;
                self.set_flags(result, carry);
            }
            unsupported => {
                panic!("add doesn't support {:?}", unsupported);
            }
        }
        ()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inr_b_increases_b_by_one() {
        let mut state = State {
            b: 0x10,
            ..State::default()
        };

        state.inr(Register::B);

        assert_eq!(state.b, 0x11);
    }

    #[test]
    fn inr_wraps_and_sets_carry_flag() {
        let mut state = State {
            b: 0xff,
            ..State::default()
        };

        state.inr(Register::B);

        assert_eq!(state.b, 0x00);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn inr_c_increases_c_by_one() {
        let mut state = State {
            c: 0x10,
            ..State::default()
        };

        state.inr(Register::C);

        assert_eq!(state.c, 0x11);
    }

    #[test]
    fn inr_d_increases_d_by_one() {
        let mut state = State {
            d: 0x10,
            ..State::default()
        };

        state.inr(Register::D);

        assert_eq!(state.d, 0x11);
    }

    #[test]
    fn inr_e_increases_e_by_one() {
        let mut state = State {
            e: 0x10,
            ..State::default()
        };

        state.inr(Register::E);

        assert_eq!(state.e, 0x11);
    }

    #[test]
    fn inr_h_increases_h_by_one() {
        let mut state = State {
            h: 0x10,
            ..State::default()
        };

        state.inr(Register::H);

        assert_eq!(state.h, 0x11);
    }

    #[test]
    fn inr_l_increases_l_by_one() {
        let mut state = State {
            l: 0x10,
            ..State::default()
        };

        state.inr(Register::L);

        assert_eq!(state.l, 0x11);
    }

    #[test]
    fn inr_a_increases_a_by_one() {
        let mut state = State {
            a: 0x10,
            ..State::default()
        };

        state.inr(Register::A);

        assert_eq!(state.a, 0x11);
    }

    #[test]
    fn inr_m_increases_byte_at_hl_address() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        state.inr(Register::M);

        assert_eq!(state.memory[0x05], 0x02);
    }
}
