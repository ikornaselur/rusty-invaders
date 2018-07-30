use cpu::register::Register;
use cpu::CPU;

/// Store the accumulator at the immediate address
///
/// # Cycles
///
/// 13
///
/// # Argumnets
///
/// * `cpu` - The cpu to perform the storing in
///
pub fn sta(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();

    cpu.memory[address as usize] = cpu.a;

    13
}

/// Store H and L at the immediate address
///
/// # Cycles
///
/// 16
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the storing in
///
pub fn shld(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();

    cpu.memory[address as usize] = cpu.l;
    cpu.memory[(address + 1) as usize] = cpu.h;

    16
}

/// Store the accumulator in the address from the given register pair
///
/// # Cycles
///
/// 13
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the storing in
/// * `register` - The register pair to read the address from
///
pub fn stax(cpu: &mut CPU, register: Register) -> u8 {
    let address = match register {
        Register::B => (u16::from(cpu.b) << 8) + u16::from(cpu.c),
        Register::D => (u16::from(cpu.d) << 8) + u16::from(cpu.e),
        unsupported => {
            panic!("stax doesn't support {:?}", unsupported);
        }
    };

    cpu.memory[address as usize] = cpu.a;

    13
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sta_stores_accumulator_at_address() {
        let mut cpu = CPU {
            memory: vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAA],
            a: 0xFF,
            pc: 2,
            ..CPU::default()
        };

        sta(&mut cpu);

        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.memory, vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xFF]);
    }

    #[test]
    fn shld_stores_h_and_l_at_address() {
        let mut cpu = CPU {
            memory: vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0x22, 0x21],
            h: 0xDE,
            l: 0xAD,
            pc: 2,
            ..CPU::default()
        };

        shld(&mut cpu);

        assert_eq!(cpu.pc, 4);
        assert_eq!(
            cpu.memory,
            vec![0x11, 0x12, 0x06, 0x00, 0x13, 0x14, 0xAD, 0xDE]
        );
    }

    #[test]
    fn stax_b_stores_accumulator_at_address_from_b_c() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0],
            a: 0xFF,
            b: 0x00,
            c: 0x02,
            ..CPU::default()
        };

        stax(&mut cpu, Register::B);

        assert_eq!(cpu.memory, vec![0, 0, 0xFF]);
    }

    #[test]
    fn stax_d_stores_accumulator_at_address_from_d_e() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0],
            a: 0xFF,
            d: 0x00,
            e: 0x02,
            ..CPU::default()
        };

        stax(&mut cpu, Register::D);

        assert_eq!(cpu.memory, vec![0, 0, 0xFF]);
    }
}
