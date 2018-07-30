use cpu::register::Register;
use cpu::CPU;

/// Perform and or between accumulator and register and put results into the accumulator
///
/// Sets condition flags
///
/// # Cycles
///
/// * Register M: 7
/// * Other: 4
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the or in
/// * `register` - The register to perform the or with
///
pub fn ora(cpu: &mut CPU, register: Register) -> u8 {
    let result = cpu.a | match register {
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
            panic!("ora doesn't support {:?}", unsupported);
        }
    };

    cpu.a = result;
    cpu.flags.set(result, false);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Perform and or between accumulator and the next immediate byte and put results into the accumulator
///
/// Sets condition flags
///
/// # Cycles
///
/// 7
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the or in
///
pub fn ori(cpu: &mut CPU) -> u8 {
    let byte = cpu.read_byte().unwrap();

    let result = cpu.a | byte;

    cpu.a = result;
    cpu.flags.set(result, false);

    7
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn ora_resets_carry_bit() {
        let mut cpu = CPU {
            a: 123,
            b: 123,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        ora(&mut cpu, Register::B);

        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn ora_b_ors_b_with_accumulator() {
        let mut cpu = CPU {
            a: 0b0101_1100,
            b: 0b0111_1000,
            ..CPU::default()
        };

        ora(&mut cpu, Register::B);

        assert_eq!(cpu.a, 0b0111_1100);
    }

    #[test]
    fn ora_c_ors_c_with_accumulator() {
        let mut cpu = CPU {
            a: 0b0101_1100,
            c: 0b0111_1000,
            ..CPU::default()
        };

        ora(&mut cpu, Register::C);

        assert_eq!(cpu.a, 0b0111_1100);
    }

    #[test]
    fn ora_d_ors_d_with_accumulator() {
        let mut cpu = CPU {
            a: 0b0101_1100,
            d: 0b0111_1000,
            ..CPU::default()
        };

        ora(&mut cpu, Register::D);

        assert_eq!(cpu.a, 0b0111_1100);
    }

    #[test]
    fn ora_e_ors_e_with_accumulator() {
        let mut cpu = CPU {
            a: 0b0101_1100,
            e: 0b0111_1000,
            ..CPU::default()
        };

        ora(&mut cpu, Register::E);

        assert_eq!(cpu.a, 0b0111_1100);
    }

    #[test]
    fn ora_h_ors_h_with_accumulator() {
        let mut cpu = CPU {
            a: 0b0101_1100,
            h: 0b0111_1000,
            ..CPU::default()
        };

        ora(&mut cpu, Register::H);

        assert_eq!(cpu.a, 0b0111_1100);
    }

    #[test]
    fn ora_l_ors_l_with_accumulator() {
        let mut cpu = CPU {
            a: 0b0101_1100,
            l: 0b0111_1000,
            ..CPU::default()
        };

        ora(&mut cpu, Register::L);

        assert_eq!(cpu.a, 0b0111_1100);
    }

    #[test]
    fn ora_a_ors_a_with_accumulator() {
        let mut cpu = CPU {
            a: 0b1111_1100,
            ..CPU::default()
        };

        ora(&mut cpu, Register::A);

        assert_eq!(cpu.a, 0b1111_1100);
    }

    #[test]
    fn ora_m_ors_byte_at_hl_address_to_accumulator() {
        let mut cpu = CPU {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0b0111_1000],
            a: 0b0101_1100,
            h: 0x00,
            l: 0x05,
            ..CPU::default()
        };

        ora(&mut cpu, Register::M);

        assert_eq!(cpu.a, 0b0111_1100);
    }

    #[test]
    fn ori_ors_immediate_byte_with_accumulator() {
        let mut cpu = CPU {
            memory: vec![0b0011_0101, 0b0010_0110],
            a: 0b0111_0000,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        ori(&mut cpu);
        assert_eq!(cpu.a, 0b0111_0101);
        assert_eq!(cpu.flags.carry, false);

        ori(&mut cpu);
        assert_eq!(cpu.a, 0b0111_0111);
        assert_eq!(cpu.flags.carry, false);
    }
}
