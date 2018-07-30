use cpu::register::Register;
use cpu::CPU;

/// Compare a register to the accumulator and set the flags based on the comparison
///
/// Sets conditions flags
///
/// # Cycles
///
/// * Register M: 7
/// * Other: 4
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the comparison in
/// * `register` - The register to compare to the accumulator
///
pub fn cmp(cpu: &mut CPU, register: Register) -> u8 {
    let (result, borrow) = match register {
        Register::A => cpu.a.overflowing_sub(cpu.a),
        Register::B => cpu.a.overflowing_sub(cpu.b),
        Register::C => cpu.a.overflowing_sub(cpu.c),
        Register::D => cpu.a.overflowing_sub(cpu.d),
        Register::E => cpu.a.overflowing_sub(cpu.e),
        Register::H => cpu.a.overflowing_sub(cpu.h),
        Register::L => cpu.a.overflowing_sub(cpu.l),
        Register::M => {
            let offset = (u16::from(cpu.h) << 8) + u16::from(cpu.l);
            cpu.a.overflowing_sub(cpu.memory[offset as usize])
        }
        unsupported => {
            panic!("sub doesn't support {:?}", unsupported);
        }
    };

    cpu.flags.set(result, borrow);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Compare the accumulator to the next immediate byte and set the flags based on the comparison
///
/// Sets conditions flags
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the comparison in
///
pub fn cpi(cpu: &mut CPU) -> u8 {
    let byte = cpu.read_byte().unwrap();

    let (result, borrow) = cpu.a.overflowing_sub(byte);

    cpu.flags.set(result, borrow);

    7
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cmp_b_with_smaller_b_compares_b_to_accumulator_and_sets_flags() {
        let mut cpu = CPU {
            a: 10,
            b: 9,
            ..CPU::default()
        };

        cmp(&mut cpu, Register::B);

        assert_eq!(cpu.a, 10);
        assert_eq!(cpu.b, 9);

        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.sign, false);
        assert_eq!(cpu.flags.parity, false);
    }

    #[test]
    fn cmp_b_with_equal_b_compares_b_to_accumulator_and_sets_flags() {
        let mut cpu = CPU {
            a: 10,
            b: 10,
            ..CPU::default()
        };

        cmp(&mut cpu, Register::B);

        assert_eq!(cpu.a, 10);
        assert_eq!(cpu.b, 10);

        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.sign, false);
        assert_eq!(cpu.flags.parity, true);
    }

    #[test]
    fn cmp_b_with_larger_b_compares_b_to_accumulator_and_sets_flags() {
        let mut cpu = CPU {
            a: 10,
            b: 11,
            ..CPU::default()
        };

        cmp(&mut cpu, Register::B);

        assert_eq!(cpu.a, 10);
        assert_eq!(cpu.b, 11);

        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.sign, true);
        assert_eq!(cpu.flags.parity, true);
    }

    #[test]
    fn cpi_with_smaller_immediate_byte_compares_it_to_accumulator_and_sets_flags() {
        let mut cpu = CPU {
            memory: vec![9],
            a: 10,
            ..CPU::default()
        };

        cpi(&mut cpu);

        assert_eq!(cpu.a, 10);

        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.sign, false);
        assert_eq!(cpu.flags.parity, false);
    }

    #[test]
    fn cpi_with_equal_immediate_byte_compares_it_to_accumulator_and_sets_flags() {
        let mut cpu = CPU {
            memory: vec![10],
            a: 10,
            ..CPU::default()
        };

        cpi(&mut cpu);

        assert_eq!(cpu.a, 10);

        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.sign, false);
        assert_eq!(cpu.flags.parity, true);
    }

    #[test]
    fn cpi_with_larget_immediate_byte_compares_it_to_accumulator_and_sets_flags() {
        let mut cpu = CPU {
            memory: vec![11],
            a: 10,
            ..CPU::default()
        };

        cpi(&mut cpu);

        assert_eq!(cpu.a, 10);

        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.sign, true);
        assert_eq!(cpu.flags.parity, true);
    }
}
