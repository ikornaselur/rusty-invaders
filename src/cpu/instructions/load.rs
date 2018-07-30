use cpu::register::Register;
use cpu::CPU;

/// Load register pair from the next two immediate bytes
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the load in
/// * `register` - The register pair to load into
///
pub fn lxi(cpu: &mut CPU, register: Register) -> u8 {
    let least = cpu.read_byte().unwrap();
    let most = cpu.read_byte().unwrap();

    match register {
        Register::B => {
            cpu.c = least;
            cpu.b = most;
        }
        Register::D => {
            cpu.e = least;
            cpu.d = most;
        }
        Register::H => {
            cpu.l = least;
            cpu.h = most;
        }
        Register::SP => {
            cpu.sp = (u16::from(most) << 8) + u16::from(least);
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
/// * `cpu` - The cpu to load SP in
///
pub fn sphl(cpu: &mut CPU) -> u8 {
    cpu.sp = (u16::from(cpu.h) << 8) + u16::from(cpu.l);

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
/// * `cpu` - The cpu to load the accumulator in
///
pub fn lda(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();

    cpu.a = cpu.memory[address as usize];

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
/// * `cpu` - The cpu to load H and L in
///
pub fn lhld(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();

    cpu.l = cpu.memory[address as usize];
    cpu.h = cpu.memory[(address + 1) as usize];

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
/// * `cpu` - The cpu to load the accumulator in
/// * `register` - The register pair containing the address to load the accumulator from
///
pub fn ldax(cpu: &mut CPU, register: Register) -> u8 {
    let address = match register {
        Register::B => (u16::from(cpu.b) << 8) + u16::from(cpu.c),
        Register::D => (u16::from(cpu.d) << 8) + u16::from(cpu.e),
        unsupported => {
            panic!("stax doesn't support {:?}", unsupported);
        }
    };

    cpu.a = cpu.memory[address as usize];

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
/// * `cpu` - The cpu to load the program counter in
///
pub fn pchl(cpu: &mut CPU) -> u8 {
    cpu.pc = (u16::from(cpu.h) << 8) + u16::from(cpu.l);

    5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lxi_b_reads_bytes_into_registers() {
        let mut cpu = CPU {
            memory: vec![0xDE, 0xAD],
            pc: 0,
            ..CPU::default()
        };

        lxi(&mut cpu, Register::B);

        assert_eq!(cpu.c, 0xDE);
        assert_eq!(cpu.b, 0xAD);
    }

    #[test]
    fn lxi_d_reads_bytes_into_registers() {
        let mut cpu = CPU {
            memory: vec![0xDE, 0xAD],
            pc: 0,
            ..CPU::default()
        };

        lxi(&mut cpu, Register::D);

        assert_eq!(cpu.e, 0xDE);
        assert_eq!(cpu.d, 0xAD);
    }

    #[test]
    fn lxi_h_reads_bytes_into_registers() {
        let mut cpu = CPU {
            memory: vec![0xDE, 0xAD],
            pc: 0,
            ..CPU::default()
        };

        lxi(&mut cpu, Register::H);

        assert_eq!(cpu.l, 0xDE);
        assert_eq!(cpu.h, 0xAD);
    }

    #[test]
    fn lxi_sp_reads_bytes_into_registers() {
        let mut cpu = CPU {
            memory: vec![0xAD, 0xDE],
            pc: 0,
            ..CPU::default()
        };

        lxi(&mut cpu, Register::SP);

        assert_eq!(cpu.sp, 0xDEAD);
    }

    #[test]
    fn sphl_loads_sp_from_h_and_l() {
        let mut cpu = CPU {
            h: 0x50,
            l: 0x6C,
            sp: 0x1234,
            ..CPU::default()
        };

        sphl(&mut cpu);

        assert_eq!(cpu.h, 0x50);
        assert_eq!(cpu.l, 0x6C);
        assert_eq!(cpu.sp, 0x506C);
    }

    #[test]
    fn lda_loads_accumulator_from_address() {
        let mut cpu = CPU {
            memory: vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAA],
            a: 0xFF,
            pc: 2,
            ..CPU::default()
        };

        lda(&mut cpu);

        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.memory, vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAA]);
        assert_eq!(cpu.a, 0xAA);
    }

    #[test]
    fn lhld_loads_h_and_l_from_address() {
        let mut cpu = CPU {
            memory: vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAD, 0xDE],
            pc: 2,
            ..CPU::default()
        };

        lhld(&mut cpu);

        assert_eq!(cpu.pc, 4);
        assert_eq!(
            cpu.memory,
            vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAD, 0xDE]
        );
        assert_eq!(cpu.h, 0xDE);
        assert_eq!(cpu.l, 0xAD);
    }

    #[test]
    fn ldax_b_accumulator_from_b_c_address() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0xFF],
            a: 0x00,
            b: 0x00,
            c: 0x02,
            ..CPU::default()
        };

        ldax(&mut cpu, Register::B);

        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn ldax_d_accumulator_from_d_e_address() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0xFF],
            a: 0x00,
            d: 0x00,
            e: 0x02,
            ..CPU::default()
        };

        ldax(&mut cpu, Register::D);

        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn pchl_loads_pc_from_h_l_pair() {
        let mut cpu = CPU {
            pc: 0x1234,
            h: 0xDE,
            l: 0xAD,
            ..CPU::default()
        };

        pchl(&mut cpu);

        assert_eq!(cpu.pc, 0xDEAD);
    }
}
