use super::State;

impl State {
    pub fn jmp(&mut self) -> () {
        self.pc = self.read_address().unwrap();
    }
    pub fn jc(&mut self) -> () {
        let address = self.read_address().unwrap();
        if self.cc.carry {
            self.pc = address;
        }
    }
    pub fn jnc(&mut self) -> () {
        let address = self.read_address().unwrap();
        if !self.cc.carry {
            self.pc = address;
        }
    }
    pub fn jz(&mut self) -> () {
        let address = self.read_address().unwrap();
        if self.cc.zero {
            self.pc = address;
        }
    }
    pub fn jnz(&mut self) -> () {
        let address = self.read_address().unwrap();
        if !self.cc.zero {
            self.pc = address;
        }
    }
    pub fn jm(&mut self) -> () {
        let address = self.read_address().unwrap();
        if self.cc.sign {
            self.pc = address;
        }
    }
    pub fn jp(&mut self) -> () {
        let address = self.read_address().unwrap();
        if !self.cc.sign {
            self.pc = address;
        }
    }
    pub fn jpe(&mut self) -> () {
        let address = self.read_address().unwrap();
        if self.cc.parity {
            self.pc = address;
        }
    }
    pub fn jpo(&mut self) -> () {
        let address = self.read_address().unwrap();
        if !self.cc.parity {
            self.pc = address;
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::ConditionCodes;
    use super::*;

    #[test]
    fn jmp_sets_pc_to_new_address() {
        let mut state = State {
            memory: vec![0xAD, 0xDE],
            ..State::default()
        };

        state.jmp();

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jc_sets_pc_to_new_address_if_carry_bit_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                carry: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.jc();

        assert_eq!(state.pc, 2);

        state.cc.carry = true;
        state.jc();

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jnc_sets_pc_to_new_address_if_carry_bit_is_not_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.jnc();

        assert_eq!(state.pc, 2);

        state.cc.carry = false;
        state.jnc();

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jz_sets_pc_to_new_address_if_zero_bit_is_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                zero: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.jz();

        assert_eq!(state.pc, 2);

        state.cc.zero = true;
        state.jz();

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jnz_sets_pc_to_new_address_if_zero_bit_is_not_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                zero: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.jnz();

        assert_eq!(state.pc, 2);

        state.cc.zero = false;
        state.jnz();

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jm_sets_pc_to_new_address_if_sign_bit_is_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                sign: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.jm();

        assert_eq!(state.pc, 2);

        state.cc.sign = true;
        state.jm();

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jp_sets_pc_to_new_address_if_sign_bit_is_not_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                sign: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.jp();

        assert_eq!(state.pc, 2);

        state.cc.sign = false;
        state.jp();

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jpe_sets_pc_to_new_address_if_parity_bit_is_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                parity: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.jpe();

        assert_eq!(state.pc, 2);

        state.cc.parity = true;
        state.jpe();

        assert_eq!(state.pc, 0xDEAD);
    }

    #[test]
    fn jpo_sets_pc_to_new_address_if_parity_bit_is_not_set() {
        let mut state = State {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            cc: ConditionCodes {
                parity: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.jpo();

        assert_eq!(state.pc, 2);

        state.cc.parity = false;
        state.jpo();

        assert_eq!(state.pc, 0xDEAD);
    }
}
