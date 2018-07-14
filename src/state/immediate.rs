use super::State;

impl State {
    pub fn adi(&mut self) -> () {
        let byte = self.read_byte().unwrap();
        let (result, carry) = self.a.overflowing_add(byte);

        self.a = result;
        self.set_flags(result, carry);
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

    pub fn sui(&mut self) -> () {
        let byte = self.read_byte().unwrap();
        let (result, carry) = self.a.overflowing_sub(byte);

        self.a = result;
        self.set_flags(result, carry);
    }

    pub fn sbi(&mut self) -> () {
        let byte = self.read_byte().unwrap();

        let (byte, byte_carry) = match self.cc.carry {
            true => byte.overflowing_add(1),
            false => (byte, false),
        };

        let (result, carry) = self.a.overflowing_sub(byte);

        self.a = result;
        self.set_flags(result, carry || byte_carry);
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn sui_removes_immediate_byte_from_accumulator() {
        let mut state = State {
            memory: vec![1, 5],
            a: 0,
            ..State::default()
        };

        state.sui();
        assert_eq!(state.a, 255);
        assert_eq!(state.cc.carry, true);

        state.sui();
        assert_eq!(state.a, 250);
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

    #[test]
    fn sbi_removes_immediate_byte_from_accumulator_with_borrow() {
        let mut state = State {
            memory: vec![0xFF, 0xFF, 0x00, 0x01],
            a: 0x00,
            ..State::default()
        };

        state.sbi();
        assert_eq!(state.a, 0x01);
        assert_eq!(state.cc.carry, true);

        state.sbi();
        assert_eq!(state.a, 0x01);
        assert_eq!(state.cc.carry, true);

        state.sbi();
        assert_eq!(state.a, 0x00);
        assert_eq!(state.cc.carry, false);

        state.sbi();
        assert_eq!(state.a, 0xFF);
        assert_eq!(state.cc.carry, true);
    }
}
