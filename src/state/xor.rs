use super::Register;
use super::State;

impl State {
    pub fn xra(&mut self, register: Register) -> u8 {
        let result = self.a ^ match register {
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
                panic!("xra doesn't support {:?}", unsupported);
            }
        };

        self.a = result;
        self.set_flags(result, false);

        match register {
            Register::M => 7,
            _ => 4,
        }
    }

    pub fn xri(&mut self) -> u8 {
        let byte = self.read_byte().unwrap();

        let result = self.a ^ byte;

        self.a = result;
        self.set_flags(result, false);

        7
    }
}

#[cfg(test)]
mod test {
    use super::super::ConditionCodes;
    use super::*;

    #[test]
    fn xra_resets_carry_bit() {
        let mut state = State {
            a: 123,
            b: 123,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.xra(Register::B);

        assert_eq!(state.cc.carry, false);
    }

    #[test]
    fn xra_b_xors_b_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            b: 0b0111_1000,
            ..State::default()
        };

        state.xra(Register::B);

        assert_eq!(state.a, 0b0010_0100);
    }

    #[test]
    fn xra_c_xors_c_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            c: 0b0111_1000,
            ..State::default()
        };

        state.xra(Register::C);

        assert_eq!(state.a, 0b0010_0100);
    }

    #[test]
    fn xra_d_xors_d_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            d: 0b0111_1000,
            ..State::default()
        };

        state.xra(Register::D);

        assert_eq!(state.a, 0b0010_0100);
    }

    #[test]
    fn xra_e_xors_e_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            e: 0b0111_1000,
            ..State::default()
        };

        state.xra(Register::E);

        assert_eq!(state.a, 0b0010_0100);
    }

    #[test]
    fn xra_h_xors_h_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            h: 0b0111_1000,
            ..State::default()
        };

        state.xra(Register::H);

        assert_eq!(state.a, 0b0010_0100);
    }

    #[test]
    fn xra_l_xors_l_with_accumulator() {
        let mut state = State {
            a: 0b0101_1100,
            l: 0b0111_1000,
            ..State::default()
        };

        state.xra(Register::L);

        assert_eq!(state.a, 0b0010_0100);
    }

    #[test]
    fn xra_a_xors_a_with_accumulator() {
        let mut state = State {
            a: 0b1111_1100,
            ..State::default()
        };

        state.xra(Register::A);

        assert_eq!(state.a, 0b0000_0000);
    }

    #[test]
    fn xra_m_xors_byte_at_hl_address_to_accumulator() {
        let mut state = State {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0b0111_1000],
            a: 0b0101_1100,
            h: 0x00,
            l: 0x05,
            ..State::default()
        };

        state.xra(Register::M);

        assert_eq!(state.a, 0b0010_0100);
    }

    #[test]
    fn xri_xors_immediate_byte_with_accumulator() {
        let mut state = State {
            memory: vec![0b0011_0101, 0b0010_0110],
            a: 0b0111_0000,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.xri();
        assert_eq!(state.a, 0b0100_0101);
        assert_eq!(state.cc.carry, false);

        state.xri();
        assert_eq!(state.a, 0b0110_0011);
        assert_eq!(state.cc.carry, false);
    }
}
