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
}
