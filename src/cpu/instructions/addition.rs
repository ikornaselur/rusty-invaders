use cpu::register::Register;
use cpu::CPU;

/// Perform accumulator addition from a register
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
/// * `cpu` - The cpu to perform the addition in
/// * `register` - The register to add to the accumulator
///
pub fn add(cpu: &mut CPU, register: Register) -> u8 {
    let (result, carry) = match register {
        Register::A => cpu.a.overflowing_add(cpu.a),
        Register::B => cpu.a.overflowing_add(cpu.b),
        Register::C => cpu.a.overflowing_add(cpu.c),
        Register::D => cpu.a.overflowing_add(cpu.d),
        Register::E => cpu.a.overflowing_add(cpu.e),
        Register::H => cpu.a.overflowing_add(cpu.h),
        Register::L => cpu.a.overflowing_add(cpu.l),
        Register::M => {
            let offset = (u16::from(cpu.h) << 8) + u16::from(cpu.l);
            cpu.a.overflowing_add(cpu.memory[offset as usize])
        }
        unsupported => {
            panic!("add doesn't support {:?}", unsupported);
        }
    };

    cpu.a = result;
    cpu.flags.set(result, carry);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Perform accumulator addition with the next immediate byte
///
/// Sets condition flags
///
/// # Cycles
///
/// 7
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the addition in
///
pub fn adi(cpu: &mut CPU) -> u8 {
    let byte = cpu.read_byte().unwrap();
    let (result, carry) = cpu.a.overflowing_add(byte);

    cpu.a = result;
    cpu.flags.set(result, carry);

    7
}

/// Perform double addition to the pseudo register M
///
/// Sets carry flag
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the addition in
/// * `register` - The double register pair to add to M
///
pub fn dad(cpu: &mut CPU, register: Register) -> u8 {
    let current: u16 = (u16::from(cpu.h) << 8) + u16::from(cpu.l);
    let (result, carry) = match register {
        Register::B => current.overflowing_add((u16::from(cpu.b) << 8) + u16::from(cpu.c)),
        Register::D => current.overflowing_add((u16::from(cpu.d) << 8) + u16::from(cpu.e)),
        Register::H => current.overflowing_add((u16::from(cpu.h) << 8) + u16::from(cpu.l)),
        Register::SP => current.overflowing_add(cpu.sp),
        unsupported => {
            panic!("dad doesn't support {:?}", unsupported);
        }
    };

    cpu.l = result as u8;
    cpu.h = (result >> 8) as u8;
    cpu.flags.carry = carry;

    10
}

/// Perform accumulator addition from a register with the carry bit
///
/// Sets condition codes
///
/// # Cycles
///
/// * Register M: 7
/// * Other: 4
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the addition in
/// * `register` - The register to add to the accumulator
///
pub fn adc(cpu: &mut CPU, register: Register) -> u8 {
    let byte = match register {
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
            panic!("adc doesn't support {:?}", unsupported);
        }
    };

    let (byte, byte_carry) = if cpu.flags.carry {
        byte.overflowing_add(1)
    } else {
        (byte, false)
    };

    let (result, carry) = cpu.a.overflowing_add(byte);

    cpu.a = result;
    cpu.flags.set(result, carry || byte_carry);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Perform accumulator addition with the next immediate byte and carry bit
///
/// Sets condition codes
///
/// # Cycles
///
/// 7
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the addition in
///
pub fn aci(cpu: &mut CPU) -> u8 {
    let byte = cpu.read_byte().unwrap();

    let (byte, byte_carry) = if cpu.flags.carry {
        byte.overflowing_add(1)
    } else {
        (byte, false)
    };

    let (result, carry) = cpu.a.overflowing_add(byte);

    cpu.a = result;
    cpu.flags.set(result, carry || byte_carry);

    7
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn add_b_adds_b_to_accumulator() {
        let mut cpu = CPU {
            a: 1,
            b: 2,
            ..CPU::default()
        };

        add(&mut cpu, Register::B);

        assert_eq!(cpu.a, 3);
    }

    #[test]
    fn add_c_adds_c_to_accumulator() {
        let mut cpu = CPU {
            a: 1,
            c: 2,
            ..CPU::default()
        };

        add(&mut cpu, Register::C);

        assert_eq!(cpu.a, 3);
    }

    #[test]
    fn add_d_adds_d_to_accumulator() {
        let mut cpu = CPU {
            a: 1,
            d: 2,
            ..CPU::default()
        };

        add(&mut cpu, Register::D);

        assert_eq!(cpu.a, 3);
    }

    #[test]
    fn add_e_adds_e_to_accumulator() {
        let mut cpu = CPU {
            a: 1,
            e: 2,
            ..CPU::default()
        };

        add(&mut cpu, Register::E);

        assert_eq!(cpu.a, 3);
    }

    #[test]
    fn add_h_adds_h_to_accumulator() {
        let mut cpu = CPU {
            a: 1,
            h: 2,
            ..CPU::default()
        };

        add(&mut cpu, Register::H);

        assert_eq!(cpu.a, 3);
    }

    #[test]
    fn add_l_adds_l_to_accumulator() {
        let mut cpu = CPU {
            a: 1,
            l: 2,
            ..CPU::default()
        };

        add(&mut cpu, Register::L);

        assert_eq!(cpu.a, 3);
    }

    #[test]
    fn add_a_adds_a_to_accumulator() {
        let mut cpu = CPU {
            a: 1,
            ..CPU::default()
        };

        add(&mut cpu, Register::A);

        assert_eq!(cpu.a, 2);
    }

    #[test]
    fn add_m_adds_byte_at_hl_address_to_accumulator() {
        let mut cpu = CPU {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 5],
            a: 1,
            h: 0x00,
            l: 0x05,
            ..CPU::default()
        };

        add(&mut cpu, Register::M);

        assert_eq!(cpu.a, 6);
    }

    #[test]
    fn adi_adds_immediate_byte_to_accumulator() {
        let mut cpu = CPU {
            memory: vec![1, 5],
            a: 0xFF,
            ..CPU::default()
        };

        adi(&mut cpu);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.flags.carry, true);

        adi(&mut cpu);
        assert_eq!(cpu.a, 5);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn dad_b_double_adds_b_c_to_h_l() {
        let mut cpu = CPU {
            b: 0x33,
            c: 0x9F,
            h: 0xA1,
            l: 0x7B,
            ..CPU::default()
        };

        dad(&mut cpu, Register::B);

        assert_eq!(cpu.h, 0xD5);
        assert_eq!(cpu.l, 0x1A);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn dad_d_double_adds_d_e_to_h_l() {
        let mut cpu = CPU {
            d: 0x33,
            e: 0x9F,
            h: 0xA1,
            l: 0x7B,
            ..CPU::default()
        };

        dad(&mut cpu, Register::D);

        assert_eq!(cpu.h, 0xD5);
        assert_eq!(cpu.l, 0x1A);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn dad_h_double_adds_h_l_to_h_l() {
        let mut cpu = CPU {
            h: 0x11,
            l: 0x22,
            ..CPU::default()
        };

        dad(&mut cpu, Register::H);

        assert_eq!(cpu.h, 0x22);
        assert_eq!(cpu.l, 0x44);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn dad_sp_double_adds_sp_to_h_l() {
        let mut cpu = CPU {
            h: 0x11,
            l: 0x22,
            sp: 0x1111,
            ..CPU::default()
        };

        dad(&mut cpu, Register::SP);

        assert_eq!(cpu.h, 0x22);
        assert_eq!(cpu.l, 0x33);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn adc_b_adds_b_with_carry_to_accumulator() {
        let mut cpu = CPU {
            a: 1,
            b: 2,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        adc(&mut cpu, Register::B);

        assert_eq!(cpu.a, 4);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn adc_with_max_values() {
        let mut cpu = CPU {
            a: u8::max_value(),
            b: u8::max_value(),
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        adc(&mut cpu, Register::B);

        assert_eq!(cpu.a, 255u8);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn adc_where_carry_causes_carry() {
        let mut cpu = CPU {
            a: u8::max_value(),
            b: 0,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        adc(&mut cpu, Register::B);

        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn adc_c_adds_c_with_carry_to_accumulator() {
        let mut cpu = CPU {
            a: 1,
            c: 2,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        adc(&mut cpu, Register::C);

        assert_eq!(cpu.a, 4);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn adc_d_adds_d_with_carry_to_accumulator() {
        let mut cpu = CPU {
            a: 1,
            d: 2,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        adc(&mut cpu, Register::D);

        assert_eq!(cpu.a, 4);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn adc_e_adds_e_with_carry_to_accumulator() {
        let mut cpu = CPU {
            a: 1,
            e: 2,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        adc(&mut cpu, Register::E);

        assert_eq!(cpu.a, 4);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn adc_h_adds_h_with_carry_to_accumulator() {
        let mut cpu = CPU {
            a: 1,
            h: 2,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        adc(&mut cpu, Register::H);

        assert_eq!(cpu.a, 4);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn adc_l_adds_l_with_carry_to_accumulator() {
        let mut cpu = CPU {
            a: 1,
            l: 2,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        adc(&mut cpu, Register::L);

        assert_eq!(cpu.a, 4);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn adc_m_adds_m_with_carry_to_accumulator() {
        let mut cpu = CPU {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 5],
            a: 1,
            h: 0x00,
            l: 0x05,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        adc(&mut cpu, Register::M);

        assert_eq!(cpu.a, 7);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn adc_a_adds_a_with_carry_to_accumulator() {
        let mut cpu = CPU {
            a: 1,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        adc(&mut cpu, Register::A);

        assert_eq!(cpu.a, 3);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn aci_adds_immediate_byte_to_accumulator_with_carry() {
        let mut cpu = CPU {
            memory: vec![0xFF, 0xFF, 0x00, 0x01],
            a: 0xFF,
            ..CPU::default()
        };

        aci(&mut cpu);
        assert_eq!(cpu.a, 0xFE);
        assert_eq!(cpu.flags.carry, true);

        aci(&mut cpu);
        assert_eq!(cpu.a, 0xFE);
        assert_eq!(cpu.flags.carry, true);

        aci(&mut cpu);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.flags.carry, false);

        aci(&mut cpu);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.flags.carry, true);
    }
}
