use cpu::register::Register;
use cpu::CPU;

/// Move a value from register to register
///
/// # Cycles
///
/// * To/from register M: 7
/// * Other: 5
///
/// # Arguments
/// * `cpu` - The cpu to perform the move in
/// * `to` - The register to move the value to
/// * `from` - The register to move the value from
///
pub fn mov(cpu: &mut CPU, to: Register, from: Register) -> u8 {
    let val = match from {
        Register::A => cpu.a,
        Register::B => cpu.b,
        Register::C => cpu.c,
        Register::D => cpu.d,
        Register::E => cpu.e,
        Register::H => cpu.h,
        Register::L => cpu.l,
        Register::M => {
            let offset = (u16::from(cpu.h) << 8) + u16::from(cpu.l);
            cpu.memory[offset as usize]
        }
        unsupported => {
            panic!("mov doesn't support moving from {:?}", unsupported);
        }
    };

    match to {
        Register::A => cpu.a = val,
        Register::B => cpu.b = val,
        Register::C => cpu.c = val,
        Register::D => cpu.d = val,
        Register::E => cpu.e = val,
        Register::H => cpu.h = val,
        Register::L => cpu.l = val,
        Register::M => {
            let offset = (u16::from(cpu.h) << 8) + u16::from(cpu.l);
            cpu.memory[offset as usize] = val;
        }
        unsupported => {
            panic!("mov doesn't support moving to {:?}", unsupported);
        }
    };

    match (to, from) {
        (Register::M, _) | (_, Register::M) => 7,
        _ => 5,
    }
}

/// Move an immediate byte to a register
///
/// # Cycles
///
/// * Register M: 10
/// * Other: 7
///
/// # Arguments
/// * `cpu` - The cpu to perform the move in
/// * `to` - The register to move the value to
///
pub fn mvi(cpu: &mut CPU, to: Register) -> u8 {
    let byte = cpu.read_byte().unwrap();

    match to {
        Register::A => cpu.a = byte,
        Register::B => cpu.b = byte,
        Register::C => cpu.c = byte,
        Register::D => cpu.d = byte,
        Register::E => cpu.e = byte,
        Register::H => cpu.h = byte,
        Register::L => cpu.l = byte,
        Register::M => {
            let offset = (u16::from(cpu.h) << 8) + u16::from(cpu.l);
            cpu.memory[offset as usize] = byte;
        }
        unsupported => {
            panic!("mov doesn't support moving to {:?}", unsupported);
        }
    };

    match to {
        Register::M => 10,
        _ => 7,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mov_moves_between_registers() {
        let mut cpu = CPU {
            a: 2,
            b: 3,
            c: 4,
            ..CPU::default()
        };

        mov(&mut cpu, Register::A, Register::B);

        assert_eq!(cpu.a, 3);

        mov(&mut cpu, Register::A, Register::C);

        assert_eq!(cpu.a, 4);

        mov(&mut cpu, Register::A, Register::A);

        assert_eq!(cpu.a, 4);
    }

    #[test]
    fn mov_moves_from_memory_address_if_from_m() {
        let mut cpu = CPU {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 5],
            a: 2,
            h: 0x00,
            l: 0x05,
            ..CPU::default()
        };

        mov(&mut cpu, Register::A, Register::M);

        assert_eq!(cpu.a, 5);
    }

    #[test]
    fn mov_moves_to_memory_address_if_to_m() {
        let mut cpu = CPU {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 5],
            a: 2,
            h: 0x00,
            l: 0x05,
            ..CPU::default()
        };

        mov(&mut cpu, Register::M, Register::A);

        assert_eq!(cpu.memory[5], 2);
    }

    #[test]
    fn mvi_sets_register_to_byte() {
        let mut cpu = CPU {
            memory: vec![0x11, 0x12],
            ..CPU::default()
        };

        mvi(&mut cpu, Register::A);

        assert_eq!(cpu.a, 0x11);

        mvi(&mut cpu, Register::B);

        assert_eq!(cpu.b, 0x12);
    }

    #[test]
    fn mvi_sets_byte_in_memory_to_byte_for_register_m() {
        let mut cpu = CPU {
            memory: vec![0x11, 0x00, 0x00, 0x00, 0x00, 0x00],
            h: 0x00,
            l: 0x05,
            ..CPU::default()
        };

        mvi(&mut cpu, Register::M);

        assert_eq!(cpu.memory[5], 0x11);
    }
}
