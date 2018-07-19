use super::Register;
use super::State;

impl State {
    pub fn dcr(&mut self, register: Register) -> u8 {
        match register {
            Register::A => {
                let (result, carry) = self.a.overflowing_sub(1);
                self.a = result;
                self.set_flags(result, carry);
            }
            Register::B => {
                let (result, carry) = self.b.overflowing_sub(1);
                self.b = result;
                self.set_flags(result, carry);
            }
            Register::C => {
                let (result, carry) = self.c.overflowing_sub(1);
                self.c = result;
                self.set_flags(result, carry);
            }
            Register::D => {
                let (result, carry) = self.d.overflowing_sub(1);
                self.d = result;
                self.set_flags(result, carry);
            }
            Register::E => {
                let (result, carry) = self.e.overflowing_sub(1);
                self.e = result;
                self.set_flags(result, carry);
            }
            Register::H => {
                let (result, carry) = self.h.overflowing_sub(1);
                self.h = result;
                self.set_flags(result, carry);
            }
            Register::L => {
                let (result, carry) = self.l.overflowing_sub(1);
                self.l = result;
                self.set_flags(result, carry);
            }
            Register::M => {
                let offset = (u16::from(self.h) << 8) + u16::from(self.l);
                let byte = self.memory[offset as usize];

                let (result, carry) = byte.overflowing_sub(1);
                self.memory[offset as usize] = result;
                self.set_flags(result, carry);
            }
            unsupported => {
                panic!("add doesn't support {:?}", unsupported);
            }
        };

        match register {
            Register::M => 10,
            _ => 5,
        }
    }

    pub fn dcx(&mut self, register: Register) -> u8 {
        match register {
            Register::B => {
                let result = ((u16::from(self.b) << 8) + u16::from(self.c)).wrapping_sub(1);
                self.b = (result >> 8) as u8;
                self.c = result as u8;
            }
            Register::D => {
                let result = ((u16::from(self.d) << 8) + u16::from(self.e)).wrapping_sub(1);
                self.d = (result >> 8) as u8;
                self.e = result as u8;
            }
            Register::H => {
                let result = ((u16::from(self.h) << 8) + u16::from(self.l)).wrapping_sub(1);
                self.h = (result >> 8) as u8;
                self.l = result as u8;
            }
            Register::SP => {
                self.sp = self.sp.wrapping_sub(1);
            }
            unsupported => {
                panic!("inx doesn't support {:?}", unsupported);
            }
        }

        5
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dcr_b_decreases_b_by_one() {
        let mut state = State {
            b: 0x10,
            ..State::default()
        };

        state.dcr(Register::B);

        assert_eq!(state.b, 0x0F);
    }

    #[test]
    fn dcr_wraps_and_sets_carry_flag() {
        let mut state = State {
            b: 0x00,
            ..State::default()
        };

        state.dcr(Register::B);

        assert_eq!(state.b, 0xff);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn dcr_c_decreases_c_by_one() {
        let mut state = State {
            c: 0x10,
            ..State::default()
        };

        state.dcr(Register::C);

        assert_eq!(state.c, 0x0F);
    }

    #[test]
    fn dcr_d_decreases_d_by_one() {
        let mut state = State {
            d: 0x10,
            ..State::default()
        };

        state.dcr(Register::D);

        assert_eq!(state.d, 0x0F);
    }

    #[test]
    fn dcr_e_decreases_e_by_one() {
        let mut state = State {
            e: 0x10,
            ..State::default()
        };

        state.dcr(Register::E);

        assert_eq!(state.e, 0x0F);
    }

    #[test]
    fn dcr_h_decreases_h_by_one() {
        let mut state = State {
            h: 0x10,
            ..State::default()
        };

        state.dcr(Register::H);

        assert_eq!(state.h, 0x0F);
    }

    #[test]
    fn dcr_l_decreases_l_by_one() {
        let mut state = State {
            l: 0x10,
            ..State::default()
        };

        state.dcr(Register::L);

        assert_eq!(state.l, 0x0F);
    }

    #[test]
    fn dcr_a_decreases_a_by_one() {
        let mut state = State {
            a: 0x10,
            ..State::default()
        };

        state.dcr(Register::A);

        assert_eq!(state.a, 0x0F);
    }

    #[test]
    fn dcr_m_decreases_byte_at_hl_address() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x03],
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        state.dcr(Register::M);

        assert_eq!(state.memory[0x05], 0x02);
    }

    #[test]
    fn dcx_b_increments_b_c_pair() {
        let mut state = State {
            b: 0x98,
            c: 0x00,
            ..State::default()
        };

        state.dcx(Register::B);

        assert_eq!(state.b, 0x97);
        assert_eq!(state.c, 0xFF);
    }

    #[test]
    fn dcx_d_increments_d_e_pair() {
        let mut state = State {
            d: 0x98,
            e: 0x00,
            ..State::default()
        };

        state.dcx(Register::D);

        assert_eq!(state.d, 0x97);
        assert_eq!(state.e, 0xFF);
    }

    #[test]
    fn dcx_h_increments_h_l_pair() {
        let mut state = State {
            h: 0x98,
            l: 0x00,
            ..State::default()
        };

        state.dcx(Register::H);

        assert_eq!(state.h, 0x97);
        assert_eq!(state.l, 0xFF);
    }

    #[test]
    fn dcx_sp_increments_sp() {
        let mut state = State {
            sp: 0x0001,
            ..State::default()
        };

        state.dcx(Register::SP);

        assert_eq!(state.sp, 0x0000);

        state.dcx(Register::SP);

        assert_eq!(state.sp, 0xFFFF);
    }
}
