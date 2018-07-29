use cpu::register::Register;
use state::State;

/// Load register pair from the next two immediate bytes
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `state` - The state to perform the load in
/// * `register` - The register pair to load into
///
pub fn lxi(state: &mut State, register: Register) -> u8 {
    let least = state.read_byte().unwrap();
    let most = state.read_byte().unwrap();

    match register {
        Register::B => {
            state.c = least;
            state.b = most;
        }
        Register::D => {
            state.e = least;
            state.d = most;
        }
        Register::H => {
            state.l = least;
            state.h = most;
        }
        Register::SP => {
            state.sp = (u16::from(most) << 8) + u16::from(least);
        }
        unsupported => {
            panic!("lxi doesn't support {:?}", unsupported);
        }
    };

    10
}

/// Load SP from H and L
///
/// # Cycles
///
/// 5
///
/// # Arguments
///
/// * `state` - The state to load SP in
///
pub fn sphl(state: &mut State) -> u8 {
    state.sp = (u16::from(state.h) << 8) + u16::from(state.l);

    5
}

/// Load the accumulator directly from an address
///
/// # Cycles
///
/// 13
///
/// # Arguments
///
/// * `state` - The state to load the accumulator in
///
pub fn lda(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();

    state.a = state.memory[address as usize];

    13
}

/// Load H and L directrly from an address
///
/// # Cycles
///
/// 16
///
/// # Arguments
///
/// * `state` - The state to load H and L in
///
pub fn lhld(state: &mut State) -> u8 {
    let address = state.read_address().unwrap();

    state.l = state.memory[address as usize];
    state.h = state.memory[(address + 1) as usize];

    16
}

/// Load the accumulator from a memory address
///
/// # Cycles
///
/// 7
///
/// # Arguments
///
/// * `state` - The state to load the accumulator in
/// * `register` - The register pair containing the address to load the accumulator from
///
pub fn ldax(state: &mut State, register: Register) -> u8 {
    let address = match register {
        Register::B => (u16::from(state.b) << 8) + u16::from(state.c),
        Register::D => (u16::from(state.d) << 8) + u16::from(state.e),
        unsupported => {
            panic!("stax doesn't support {:?}", unsupported);
        }
    };

    state.a = state.memory[address as usize];

    7
}

/// Load program counter
///
/// # Cycles
///
/// 5
///
/// # Arguments
///
/// * `state` - The state to load the program counter in
///
pub fn pchl(state: &mut State) -> u8 {
    state.pc = (u16::from(state.h) << 8) + u16::from(state.l);

    5
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

        lxi(&mut state, Register::B);

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

        lxi(&mut state, Register::D);

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

        lxi(&mut state, Register::H);

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

        lxi(&mut state, Register::SP);

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

        sphl(&mut state);

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

        lda(&mut state);

        assert_eq!(state.pc, 4);
        assert_eq!(state.memory, vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAA]);
        assert_eq!(state.a, 0xAA);
    }

    #[test]
    fn lhld_loads_h_and_l_from_address() {
        let mut state = State {
            memory: vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAD, 0xDE],
            pc: 2,
            ..State::default()
        };

        lhld(&mut state);

        assert_eq!(state.pc, 4);
        assert_eq!(
            state.memory,
            vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAD, 0xDE]
        );
        assert_eq!(state.h, 0xDE);
        assert_eq!(state.l, 0xAD);
    }

    #[test]
    fn ldax_b_accumulator_from_b_c_address() {
        let mut state = State {
            memory: vec![0, 0, 0xFF],
            a: 0x00,
            b: 0x00,
            c: 0x02,
            ..State::default()
        };

        ldax(&mut state, Register::B);

        assert_eq!(state.a, 0xFF);
    }

    #[test]
    fn ldax_d_accumulator_from_d_e_address() {
        let mut state = State {
            memory: vec![0, 0, 0xFF],
            a: 0x00,
            d: 0x00,
            e: 0x02,
            ..State::default()
        };

        ldax(&mut state, Register::D);

        assert_eq!(state.a, 0xFF);
    }

    #[test]
    fn pchl_loads_pc_from_h_l_pair() {
        let mut state = State {
            pc: 0x1234,
            h: 0xDE,
            l: 0xAD,
            ..State::default()
        };

        pchl(&mut state);

        assert_eq!(state.pc, 0xDEAD);
    }
}
