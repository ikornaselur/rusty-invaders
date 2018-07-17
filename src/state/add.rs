use super::Register;
use super::State;

impl State {
    pub fn add(&mut self, register: Register) -> () {
        // 4 cycles
        let (result, carry) = match register {
            Register::A => self.a.overflowing_add(self.a),
            Register::B => self.a.overflowing_add(self.b),
            Register::C => self.a.overflowing_add(self.c),
            Register::D => self.a.overflowing_add(self.d),
            Register::E => self.a.overflowing_add(self.e),
            Register::H => self.a.overflowing_add(self.h),
            Register::L => self.a.overflowing_add(self.l),
            Register::M => {
                let offset: u16 = ((self.h as u16) << 8) + self.l as u16;
                self.a.overflowing_add(self.memory[offset as usize])
            }
            unsupported => {
                panic!("add doesn't support {:?}", unsupported);
            }
        };

        self.a = result;
        self.set_flags(result, carry);
    }

    pub fn adi(&mut self) -> () {
        let byte = self.read_byte().unwrap();
        let (result, carry) = self.a.overflowing_add(byte);

        self.a = result;
        self.set_flags(result, carry);
    }

    pub fn dad(&mut self, register: Register) -> () {
        let current: u16 = ((self.h as u16) << 8) + self.l as u16;
        let (result, carry) = match register {
            Register::B => current.overflowing_add(((self.b as u16) << 8) + self.c as u16),
            Register::D => current.overflowing_add(((self.d as u16) << 8) + self.e as u16),
            Register::H => current.overflowing_add(((self.h as u16) << 8) + self.l as u16),
            Register::SP => current.overflowing_add(self.sp),
            unsupported => {
                panic!("dad doesn't support {:?}", unsupported);
            }
        };

        self.l = result as u8;
        self.h = (result >> 8) as u8;
        self.cc.carry = carry;
    }

    pub fn adc(&mut self, register: Register) -> () {
        // 4 cycles
        let byte = match register {
            //let (result, carry) = match register {
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
                panic!("adc doesn't support {:?}", unsupported);
            }
        };

        let (byte, byte_carry) = match self.cc.carry {
            true => byte.overflowing_add(1),
            false => (byte, false),
        };

        let (result, carry) = self.a.overflowing_add(byte);

        self.a = result;
        self.set_flags(result, carry || byte_carry);
    }

    pub fn aci(&mut self) -> () {
        let byte = self.read_byte().unwrap();

        let (byte, byte_carry) = match self.cc.carry {
            true => byte.overflowing_add(1),
            false => (byte, false),
        };

        let (result, carry) = self.a.overflowing_add(byte);

        self.a = result;
        self.set_flags(result, carry || byte_carry);
    }
}

#[cfg(test)]
mod test {
    use super::super::ConditionCodes;
    use super::*;

    #[test]
    fn add_b_adds_b_to_accumulator() {
        let mut state = State {
            a: 1,
            b: 2,
            ..State::default()
        };

        state.add(Register::B);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_c_adds_c_to_accumulator() {
        let mut state = State {
            a: 1,
            c: 2,
            ..State::default()
        };

        state.add(Register::C);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_d_adds_d_to_accumulator() {
        let mut state = State {
            a: 1,
            d: 2,
            ..State::default()
        };

        state.add(Register::D);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_e_adds_e_to_accumulator() {
        let mut state = State {
            a: 1,
            e: 2,
            ..State::default()
        };

        state.add(Register::E);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_h_adds_h_to_accumulator() {
        let mut state = State {
            a: 1,
            h: 2,
            ..State::default()
        };

        state.add(Register::H);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_l_adds_l_to_accumulator() {
        let mut state = State {
            a: 1,
            l: 2,
            ..State::default()
        };

        state.add(Register::L);

        assert_eq!(state.a, 3);
    }

    #[test]
    fn add_a_adds_a_to_accumulator() {
        let mut state = State {
            a: 1,
            ..State::default()
        };

        state.add(Register::A);

        assert_eq!(state.a, 2);
    }

    #[test]
    fn add_m_adds_byte_at_hl_address_to_accumulator() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 5],
            a: 1,
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        state.add(Register::M);

        assert_eq!(state.a, 6);
    }

    #[test]
    fn adi_adds_immediate_byte_to_accumulator() {
        let mut state = State {
            memory: vec![1, 5],
            a: 0xFF,
            ..State::default()
        };

        state.adi();
        assert_eq!(state.a, 0);
        assert_eq!(state.cc.carry, true);

        state.adi();
        assert_eq!(state.a, 5);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn dad_b_double_adds_b_c_to_h_l() {
        let mut state = State {
            b: 0x33,
            c: 0x9F,
            h: 0xA1,
            l: 0x7B,
            ..State::default()
        };

        state.dad(Register::B);

        assert_eq!(state.h, 0xD5);
        assert_eq!(state.l, 0x1A);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn dad_d_double_adds_d_e_to_h_l() {
        let mut state = State {
            d: 0x33,
            e: 0x9F,
            h: 0xA1,
            l: 0x7B,
            ..State::default()
        };

        state.dad(Register::D);

        assert_eq!(state.h, 0xD5);
        assert_eq!(state.l, 0x1A);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn dad_h_double_adds_h_l_to_h_l() {
        let mut state = State {
            h: 0x11,
            l: 0x22,
            ..State::default()
        };

        state.dad(Register::H);

        assert_eq!(state.h, 0x22);
        assert_eq!(state.l, 0x44);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn dad_sp_double_adds_sp_to_h_l() {
        let mut state = State {
            h: 0x11,
            l: 0x22,
            sp: 0x1111,
            ..State::default()
        };

        state.dad(Register::SP);

        assert_eq!(state.h, 0x22);
        assert_eq!(state.l, 0x33);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_b_adds_b_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            b: 2,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::B);

        assert_eq!(state.a, 4);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_with_max_values() {
        let mut state = State {
            a: u8::max_value(),
            b: u8::max_value(),
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::B);

        assert_eq!(state.a, 255u8);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn adc_where_carry_causes_carry() {
        let mut state = State {
            a: u8::max_value(),
            b: 0,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::B);

        assert_eq!(state.a, 0);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn adc_c_adds_c_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            c: 2,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::C);

        assert_eq!(state.a, 4);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_d_adds_d_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            d: 2,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::D);

        assert_eq!(state.a, 4);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_e_adds_e_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            e: 2,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::E);

        assert_eq!(state.a, 4);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_h_adds_h_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            h: 2,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::H);

        assert_eq!(state.a, 4);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_l_adds_l_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            l: 2,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::L);

        assert_eq!(state.a, 4);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_m_adds_m_with_carry_to_accumulator() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 5],
            a: 1,
            h: 0x00,
            l: 0x05,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };
        state.adc(Register::M);

        assert_eq!(state.a, 7);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn adc_a_adds_a_with_carry_to_accumulator() {
        let mut state = State {
            a: 1,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.adc(Register::A);

        assert_eq!(state.a, 3);
        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn aci_adds_immediate_byte_to_accumulator_with_carry() {
        let mut state = State {
            memory: vec![0xFF, 0xFF, 0x00, 0x01],
            a: 0xFF,
            ..State::default()
        };

        state.aci();
        assert_eq!(state.a, 0xFE);
        assert_eq!(state.cc.carry, true);

        state.aci();
        assert_eq!(state.a, 0xFE);
        assert_eq!(state.cc.carry, true);

        state.aci();
        assert_eq!(state.a, 0xFF);
        assert_eq!(state.cc.carry, false);

        state.aci();
        assert_eq!(state.a, 0x00);
        assert_eq!(state.cc.carry, true);
    }
}
