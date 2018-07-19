use super::State;

impl State {
    fn process_call(&mut self, address: u16) -> () {
        // A specific hack for full cpu test
        if self.debug && address == 5 && self.c == 9 {
            let offset = (u16::from(self.d) << 8) + u16::from(self.e);
            if offset == 0x018B {
                panic!("CPU HAS FAILED");
            } else if offset == 0x0174 {
                println!("*** CPU IS OPERATIONAL ***");
                self.exit = true;
            } else {
                panic!("UNKNOWN PRINT");
            }
        }
        // End of said hack

        let least = self.pc as u8;
        let most = (self.pc >> 8) as u8;

        self.write_byte_to_stack(most);
        self.write_byte_to_stack(least);

        self.pc = address;
    }

    pub fn call(&mut self) -> u8 {
        let address = self.read_address().unwrap();

        self.process_call(address);

        17
    }

    pub fn cc(&mut self) -> u8 {
        let address = self.read_address().unwrap();
        if self.cc.carry {
            self.process_call(address);
            17
        } else {
            11
        }
    }

    pub fn cnc(&mut self) -> u8 {
        let address = self.read_address().unwrap();
        if self.cc.carry {
            11
        } else {
            self.process_call(address);
            17
        }
    }

    pub fn cz(&mut self) -> u8 {
        let address = self.read_address().unwrap();
        if self.cc.zero {
            self.process_call(address);
            17
        } else {
            11
        }
    }

    pub fn cnz(&mut self) -> u8 {
        let address = self.read_address().unwrap();
        if self.cc.zero {
            11
        } else {
            self.process_call(address);
            17
        }
    }

    pub fn cm(&mut self) -> u8 {
        let address = self.read_address().unwrap();
        if self.cc.sign {
            self.process_call(address);
            17
        } else {
            11
        }
    }

    pub fn cp(&mut self) -> u8 {
        let address = self.read_address().unwrap();
        if self.cc.sign {
            11
        } else {
            self.process_call(address);
            17
        }
    }

    pub fn cpe(&mut self) -> u8 {
        let address = self.read_address().unwrap();
        if self.cc.parity {
            self.process_call(address);
            17
        } else {
            11
        }
    }

    pub fn cpo(&mut self) -> u8 {
        let address = self.read_address().unwrap();
        if self.cc.parity {
            11
        } else {
            self.process_call(address);
            17
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::ConditionCodes;
    use super::*;

    #[test]
    fn call_pushes_the_address_after_to_the_stack_and_jumps() {
        let mut state = State {
            memory: vec![0, 0, 0, 0 /* SP */, 0, 0, 0xAD /* PC */, 0xDE],
            sp: 3,
            pc: 6,
            ..State::default()
        };

        state.call();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(
            state.memory,
            vec![0, 0x08 /* SP */, 0x00, 0, 0, 0, 0xAD, 0xDE]
        )
    }

    #[test]
    fn cc_pushes_the_address_after_to_the_stack_and_jumps_if_carry_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                carry: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.cc();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.carry = true;
        state.cc();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cnc_pushes_the_address_after_to_the_stack_and_jumps_if_carry_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.cnc();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.carry = false;
        state.cnc();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cz_pushes_the_address_after_to_the_stack_and_jumps_if_zero_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                zero: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.cz();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.zero = true;
        state.cz();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cnz_pushes_the_address_after_to_the_stack_and_jumps_if_zero_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                zero: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.cnz();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.zero = false;
        state.cnz();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cm_pushes_the_address_after_to_the_stack_and_jumps_if_sign_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                sign: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.cm();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.sign = true;
        state.cm();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cp_pushes_the_address_after_to_the_stack_and_jumps_if_sign_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                sign: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.cp();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.sign = false;
        state.cp();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cpe_pushes_the_address_after_to_the_stack_and_jumps_if_parity_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                parity: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.cpe();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.parity = true;
        state.cpe();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cpo_pushes_the_address_after_to_the_stack_and_jumps_if_parity_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            cc: ConditionCodes {
                parity: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.cpo();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 8);
        assert_eq!(state.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        state.pc = 6;
        state.cc.parity = false;
        state.cpo();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }
}
