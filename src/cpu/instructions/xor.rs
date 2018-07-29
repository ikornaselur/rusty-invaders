use cpu::register::Register;
use cpu::CPU;

/// Logical xor accumulator to register
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
/// * `cpu` - The cpu to perform the xor in
/// * `register` - The register to perform the xor with
///
pub fn xra(cpu: &mut CPU, register: Register) -> u8 {
    let result = cpu.a ^ match register {
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
            panic!("xra doesn't support {:?}", unsupported);
        }
    };

    cpu.a = result;
    cpu.flags.set(result, false);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Logical xor accumulator to the immediate byte
///
/// Sets condition flags
///
/// # Cycles
///
/// 7
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the xor in
///
pub fn xri(cpu: &mut CPU) -> u8 {
    let byte = cpu.read_byte().unwrap();

    let result = cpu.a ^ byte;

    cpu.a = result;
    cpu.flags.set(result, false);

    7
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn xra_resets_carry_bit() {
        let mut cpu = CPU {
            a: 123,
            b: 123,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        xra(&mut cpu, Register::B);

        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn xra_b_xors_b_with_accumulator() {
        let mut cpu = CPU {
            a: 0b0101_1100,
            b: 0b0111_1000,
            ..CPU::default()
        };

        xra(&mut cpu, Register::B);

        assert_eq!(cpu.a, 0b0010_0100);
    }

    #[test]
    fn xra_c_xors_c_with_accumulator() {
        let mut cpu = CPU {
            a: 0b0101_1100,
            c: 0b0111_1000,
            ..CPU::default()
        };

        xra(&mut cpu, Register::C);

        assert_eq!(cpu.a, 0b0010_0100);
    }

    #[test]
    fn xra_d_xors_d_with_accumulator() {
        let mut cpu = CPU {
            a: 0b0101_1100,
            d: 0b0111_1000,
            ..CPU::default()
        };

        xra(&mut cpu, Register::D);

        assert_eq!(cpu.a, 0b0010_0100);
    }

    #[test]
    fn xra_e_xors_e_with_accumulator() {
        let mut cpu = CPU {
            a: 0b0101_1100,
            e: 0b0111_1000,
            ..CPU::default()
        };

        xra(&mut cpu, Register::E);

        assert_eq!(cpu.a, 0b0010_0100);
    }

    #[test]
    fn xra_h_xors_h_with_accumulator() {
        let mut cpu = CPU {
            a: 0b0101_1100,
            h: 0b0111_1000,
            ..CPU::default()
        };

        xra(&mut cpu, Register::H);

        assert_eq!(cpu.a, 0b0010_0100);
    }

    #[test]
    fn xra_l_xors_l_with_accumulator() {
        let mut cpu = CPU {
            a: 0b0101_1100,
            l: 0b0111_1000,
            ..CPU::default()
        };

        xra(&mut cpu, Register::L);

        assert_eq!(cpu.a, 0b0010_0100);
    }

    #[test]
    fn xra_a_xors_a_with_accumulator() {
        let mut cpu = CPU {
            a: 0b1111_1100,
            ..CPU::default()
        };

        xra(&mut cpu, Register::A);

        assert_eq!(cpu.a, 0b0000_0000);
    }

    #[test]
    fn xra_m_xors_byte_at_hl_address_to_accumulator() {
        let mut cpu = CPU {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0b0111_1000],
            a: 0b0101_1100,
            h: 0x00,
            l: 0x05,
            ..CPU::default()
        };

        xra(&mut cpu, Register::M);

        assert_eq!(cpu.a, 0b0010_0100);
    }

    #[test]
    fn xri_xors_immediate_byte_with_accumulator() {
        let mut cpu = CPU {
            memory: vec![0b0011_0101, 0b0010_0110],
            a: 0b0111_0000,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        xri(&mut cpu);
        assert_eq!(cpu.a, 0b0100_0101);
        assert_eq!(cpu.flags.carry, false);

        xri(&mut cpu);
        assert_eq!(cpu.a, 0b0110_0011);
        assert_eq!(cpu.flags.carry, false);
    }
}
