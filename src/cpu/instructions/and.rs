use cpu::register::Register;
use cpu::CPU;

/// Perform an and between accumulator and register and put the results into the accumulator
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
/// * `cpu` - The cpu to perform the and in
/// * `register` - The register to perform the and with
///
pub fn ana(cpu: &mut CPU, register: Register) -> u8 {
    let result = cpu.a & match register {
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
            panic!("ana doesn't support {:?}", unsupported);
        }
    };

    cpu.a = result;
    cpu.flags.set(result, false);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Perform an and between accumulator and the next immediate byte and put the results into the
/// accumulator
///
/// Sets condition flags
///
/// # Cycles
///
/// 7
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the and in
///
pub fn ani(cpu: &mut CPU) -> u8 {
    let byte = cpu.read_byte().unwrap();

    let result = cpu.a & byte;

    cpu.a = result;
    cpu.flags.set(result, false);

    7
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn ana_resets_carry_bit() {
        let mut cpu = CPU {
            a: 123,
            b: 123,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        ana(&mut cpu, Register::B);

        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn ana_b_ands_b_with_accumulator() {
        let mut cpu = CPU {
            a: 0b1111_1100,
            b: 0b0000_1111,
            ..CPU::default()
        };

        ana(&mut cpu, Register::B);

        assert_eq!(cpu.a, 0b0000_1100);
    }

    #[test]
    fn ana_c_ands_c_with_accumulator() {
        let mut cpu = CPU {
            a: 0b1111_1100,
            c: 0b0000_1111,
            ..CPU::default()
        };

        ana(&mut cpu, Register::C);

        assert_eq!(cpu.a, 0b0000_1100);
    }

    #[test]
    fn ana_d_ands_d_with_accumulator() {
        let mut cpu = CPU {
            a: 0b1111_1100,
            d: 0b0000_1111,
            ..CPU::default()
        };

        ana(&mut cpu, Register::D);

        assert_eq!(cpu.a, 0b0000_1100);
    }

    #[test]
    fn ana_e_ands_e_with_accumulator() {
        let mut cpu = CPU {
            a: 0b1111_1100,
            e: 0b0000_1111,
            ..CPU::default()
        };

        ana(&mut cpu, Register::E);

        assert_eq!(cpu.a, 0b0000_1100);
    }

    #[test]
    fn ana_h_ands_h_with_accumulator() {
        let mut cpu = CPU {
            a: 0b1111_1100,
            h: 0b0000_1111,
            ..CPU::default()
        };

        ana(&mut cpu, Register::H);

        assert_eq!(cpu.a, 0b0000_1100);
    }

    #[test]
    fn ana_l_ands_l_with_accumulator() {
        let mut cpu = CPU {
            a: 0b1111_1100,
            l: 0b0000_1111,
            ..CPU::default()
        };

        ana(&mut cpu, Register::L);

        assert_eq!(cpu.a, 0b0000_1100);
    }

    #[test]
    fn ana_a_ands_a_with_accumulator() {
        let mut cpu = CPU {
            a: 0b1111_1100,
            ..CPU::default()
        };

        ana(&mut cpu, Register::A);

        assert_eq!(cpu.a, 0b1111_1100);
    }

    #[test]
    fn ana_m_ands_byte_at_hl_address_to_accumulator() {
        let mut cpu = CPU {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0b0000_1111],
            a: 0b1111_1100,
            h: 0x00,
            l: 0x05,
            ..CPU::default()
        };

        ana(&mut cpu, Register::M);

        assert_eq!(cpu.a, 0b0000_1100);
    }

    #[test]
    fn ani_ands_immediate_byte_with_accumulator() {
        let mut cpu = CPU {
            memory: vec![0b0011_0101, 0b0010_0010],
            a: 0b1111_0000,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        ani(&mut cpu);
        assert_eq!(cpu.a, 0b0011_0000);
        assert_eq!(cpu.flags.carry, false);

        ani(&mut cpu);
        assert_eq!(cpu.a, 0b0010_0000);
        assert_eq!(cpu.flags.carry, false);
    }
}
