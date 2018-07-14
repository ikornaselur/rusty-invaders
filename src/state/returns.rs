use super::State;

impl State {
    pub fn ret(&mut self) -> () {
        self.pc = self.read_address_from_stack().unwrap();
    }

    pub fn rc(&mut self) -> () {
        if self.cc.carry {
            self.ret();
        }
    }

    pub fn rnc(&mut self) -> () {
        if !self.cc.carry {
            self.ret();
        }
    }

    pub fn rz(&mut self) -> () {
        if self.cc.zero {
            self.ret();
        }
    }

    pub fn rnz(&mut self) -> () {
        if !self.cc.zero {
            self.ret();
        }
    }

    pub fn rm(&mut self) -> () {
        if self.cc.sign {
            self.ret();
        }
    }

    pub fn rp(&mut self) -> () {
        if !self.cc.sign {
            self.ret();
        }
    }

    pub fn rpe(&mut self) -> () {
        if self.cc.parity {
            self.ret();
        }
    }

    pub fn rpo(&mut self) -> () {
        if !self.cc.parity {
            self.ret();
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::ConditionCodes;
    use super::*;

    #[test]
    fn ret_pops_the_address_off_the_stack_and_jumps_back() {
        let mut state = State {
            memory: vec![0, 0x08 /* SP */, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            ..State::default()
        };

        state.ret();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rc_pops_the_address_off_the_stack_and_jumps_back_if_carry_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: ConditionCodes {
                carry: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.rc();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.carry = true;
        state.rc();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rnc_pops_the_address_off_the_stack_and_jumps_back_if_carry_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: ConditionCodes {
                carry: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.rnc();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.carry = false;
        state.rnc();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rz_pops_the_address_off_the_stack_and_jumps_back_if_zero_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: ConditionCodes {
                zero: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.rz();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.zero = true;
        state.rz();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rnz_pops_the_address_off_the_stack_and_jumps_back_if_zero_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: ConditionCodes {
                zero: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.rnz();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.zero = false;
        state.rnz();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rm_pops_the_address_off_the_stack_and_jumps_back_if_sign_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: ConditionCodes {
                sign: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.rm();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.sign = true;
        state.rm();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rp_pops_the_address_off_the_stack_and_jumps_back_if_sign_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: ConditionCodes {
                sign: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.rp();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.sign = false;
        state.rp();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rpe_pops_the_address_off_the_stack_and_jumps_back_if_parity_flag_is_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: ConditionCodes {
                parity: false,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.rpe();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.parity = true;
        state.rpe();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }

    #[test]
    fn rpo_pops_the_address_off_the_stack_and_jumps_back_if_parity_flag_is_not_set() {
        let mut state = State {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            cc: ConditionCodes {
                parity: true,
                ..ConditionCodes::default()
            },
            ..State::default()
        };

        state.rpo();

        assert_eq!(state.sp, 1);
        assert_eq!(state.pc, 0xDEAD);
        assert_eq!(state.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        state.cc.parity = false;
        state.rpo();

        assert_eq!(state.sp, 3);
        assert_eq!(state.pc, 0x0008);
    }
}
