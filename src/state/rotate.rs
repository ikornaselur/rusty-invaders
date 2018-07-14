use super::State;

impl State {
    pub fn rlc(&mut self) -> () {
        // 4 cycles
        let carry = self.a >> 7 == 1;
        let result = self.a.rotate_left(1);

        self.a = result;
        self.set_flags(result, carry);
    }

    pub fn ral(&mut self) -> () {
        // 4 cycles
        let carry = self.a >> 7 == 1;
        let mut result = self.a << 1;

        if self.cc.carry {
            result |= 0x01;
        }

        self.a = result;
        self.set_flags(result, carry);
    }

    pub fn rrc(&mut self) -> () {
        // 4 cycles
        let carry = self.a & 0x01 == 1;
        let result = self.a.rotate_right(1);

        self.a = result;
        self.set_flags(result, carry);
    }

    pub fn rar(&mut self) -> () {
        // 4 cycles
        let carry = self.a & 0x01 == 1;
        let mut result = self.a >> 1;

        if self.cc.carry {
            result |= 0x01 << 7;
        }

        self.a = result;
        self.set_flags(result, carry);
    }
}

#[cfg(test)]
mod test {
    use super::super::ConditionCodes;
    use super::*;

    #[test]
    fn rlc_rotates_accumulator_left() {
        let mut state = State {
            a: 0b0111_0010,
            cc: ConditionCodes {
                carry: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.rlc();

        assert_eq!(state.a, 0b1110_0100);
        assert_eq!(state.cc.carry, false);

        state.rlc();

        assert_eq!(state.a, 0b1100_1001);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn ral_rotates_accumulator_left_through_carry() {
        let mut state = State {
            a: 0b0111_0010,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.ral();

        assert_eq!(state.a, 0b1110_0101);
        assert_eq!(state.cc.carry, false);

        state.ral();

        assert_eq!(state.a, 0b1100_1010);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn rrc_rotates_accumulator_right() {
        let mut state = State {
            a: 0b1111_0010,
            cc: ConditionCodes {
                carry: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.rrc();

        assert_eq!(state.a, 0b0111_1001);
        assert_eq!(state.cc.carry, false);

        state.rrc();

        assert_eq!(state.a, 0b1011_1100);
        assert_eq!(state.cc.carry, true);
    }

    #[test]
    fn rar_rotates_accumulator_right_through_carry() {
        let mut state = State {
            a: 0b1111_0011,
            cc: ConditionCodes {
                carry: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.rar();

        assert_eq!(state.a, 0b0111_1001);
        assert_eq!(state.cc.carry, true);

        state.rar();

        assert_eq!(state.a, 0b1011_1100);
        assert_eq!(state.cc.carry, true);

        state.rar();

        assert_eq!(state.a, 0b1101_1110);
        assert_eq!(state.cc.carry, false);
    }
}
