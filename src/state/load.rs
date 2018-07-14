use super::Register;
use super::State;

impl State {
    pub fn lxi(&mut self, register: Register) -> () {
        // 10 instrucitons
        let least = self.read_byte().unwrap();
        let most = self.read_byte().unwrap();

        match register {
            Register::B => {
                self.c = least;
                self.b = most;
            }
            Register::D => {
                self.e = least;
                self.d = most;
            }
            Register::H => {
                self.l = least;
                self.h = most;
            }
            Register::SP => {
                self.sp = ((most as u16) << 8) + least as u16;
            }
            unsupported => {
                panic!("lxi doesn't support {:?}", unsupported);
            }
        };
    }

    pub fn sphl(&mut self) -> () {
        self.sp = ((self.h as u16) << 8) + self.l as u16;
    }

    pub fn lda(&mut self) -> () {
        let least = self.read_byte().unwrap();
        let most = self.read_byte().unwrap();
        let address = ((most as u16) << 8) + least as u16;

        self.a = self.memory[address as usize];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lxi_b_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0xDE, 0xAD],
            pc: 0,
            ..State::default()
        };

        state.lxi(Register::B);

        assert_eq!(state.c, 0xDE);
        assert_eq!(state.b, 0xAD);
    }

    #[test]
    fn lxi_d_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0xDE, 0xAD],
            pc: 0,
            ..State::default()
        };

        state.lxi(Register::D);

        assert_eq!(state.e, 0xDE);
        assert_eq!(state.d, 0xAD);
    }

    #[test]
    fn lxi_h_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0xDE, 0xAD],
            pc: 0,
            ..State::default()
        };

        state.lxi(Register::H);

        assert_eq!(state.l, 0xDE);
        assert_eq!(state.h, 0xAD);
    }

    #[test]
    fn lxi_sp_reads_bytes_into_registers() {
        let mut state = State {
            memory: vec![0xAD, 0xDE],
            pc: 0,
            ..State::default()
        };

        state.lxi(Register::SP);

        assert_eq!(state.sp, 0xDEAD);
    }

    #[test]
    fn sphl_loads_sp_from_h_and_l() {
        let mut state = State {
            h: 0x50,
            l: 0x6C,
            sp: 0x1234,
            ..State::default()
        };

        state.sphl();

        assert_eq!(state.h, 0x50);
        assert_eq!(state.l, 0x6C);
        assert_eq!(state.sp, 0x506C);
    }

    #[test]
    fn lda_loads_accumulator_from_address() {
        let mut state = State {
            memory: vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAA],
            a: 0xFF,
            pc: 2,
            ..State::default()
        };

        state.lda();

        assert_eq!(state.pc, 4);
        assert_eq!(state.memory, vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAA]);
        assert_eq!(state.a, 0xAA);
    }
}
