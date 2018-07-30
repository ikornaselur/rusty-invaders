use cpu::register::Register;
use cpu::CPU;

/// Perform accumulator subtraction from a register
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
/// * `cpu` - The cpu to perform the subtraction in
/// * `register` - The register to subtract from the accumulator
///
pub fn sub(cpu: &mut CPU, register: Register) -> u8 {
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

    cpu.a = result;
    cpu.flags.set(result, borrow);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Perform accumulator subtraction with the next immediate byte
///
/// Sets the condition flags
///
/// # Cycles
///
/// 7
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the subtraction in
///
pub fn sui(cpu: &mut CPU) -> u8 {
    let byte = cpu.read_byte().unwrap();
    let (result, carry) = cpu.a.overflowing_sub(byte);

    cpu.a = result;
    cpu.flags.set(result, carry);

    7
}

/// Perform accumulator subtraction from a register with the carry bit
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
/// * `cpu` - The cpu to perform the subtraction in
/// * `register` - The register to subtract from the accumulator
///
pub fn sbb(cpu: &mut CPU, register: Register) -> u8 {
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
            panic!("sbb doesn't support {:?}", unsupported);
        }
    };

    let (byte, byte_carry) = if cpu.flags.carry {
        byte.overflowing_add(1)
    } else {
        (byte, false)
    };

    let (result, carry) = cpu.a.overflowing_sub(byte);

    cpu.a = result;
    cpu.flags.set(result, carry || byte_carry);

    match register {
        Register::M => 7,
        _ => 4,
    }
}

/// Perform accumulator subtraction with the next immediate byte and carry bit
///
/// Sets condition codes
///
/// # Cycles
///
/// 7
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the subtraction in
///
pub fn sbi(cpu: &mut CPU) -> u8 {
    let byte = cpu.read_byte().unwrap();

    let (byte, byte_carry) = if cpu.flags.carry {
        byte.overflowing_add(1)
    } else {
        (byte, false)
    };

    let (result, carry) = cpu.a.overflowing_sub(byte);

    cpu.a = result;
    cpu.flags.set(result, carry || byte_carry);

    7
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn sub_a_subs_a_from_accumulator() {
        let mut cpu = CPU {
            a: 10,
            ..CPU::default()
        };

        sub(&mut cpu, Register::A);

        assert_eq!(cpu.a, 0);
    }

    #[test]
    fn sub_b_subs_b_from_accumulator() {
        let mut cpu = CPU {
            a: 10,
            b: 3,
            ..CPU::default()
        };

        sub(&mut cpu, Register::B);

        assert_eq!(cpu.a, 7);
    }

    #[test]
    fn sub_c_subs_c_from_accumulator() {
        let mut cpu = CPU {
            a: 10,
            c: 3,
            ..CPU::default()
        };

        sub(&mut cpu, Register::C);

        assert_eq!(cpu.a, 7);
    }

    #[test]
    fn sub_d_subs_d_from_accumulator() {
        let mut cpu = CPU {
            a: 10,
            d: 3,
            ..CPU::default()
        };

        sub(&mut cpu, Register::D);

        assert_eq!(cpu.a, 7);
    }

    #[test]
    fn sub_e_subs_e_from_accumulator() {
        let mut cpu = CPU {
            a: 10,
            e: 3,
            ..CPU::default()
        };

        sub(&mut cpu, Register::E);

        assert_eq!(cpu.a, 7);
    }

    #[test]
    fn sub_h_subs_h_from_accumulator() {
        let mut cpu = CPU {
            a: 10,
            h: 3,
            ..CPU::default()
        };

        sub(&mut cpu, Register::H);

        assert_eq!(cpu.a, 7);
    }

    #[test]
    fn sub_l_subs_l_from_accumulator() {
        let mut cpu = CPU {
            a: 10,
            l: 3,
            ..CPU::default()
        };

        sub(&mut cpu, Register::L);

        assert_eq!(cpu.a, 7);
    }

    #[test]
    fn sub_m_subs_byte_at_hl_address_to_accumulator() {
        let mut cpu = CPU {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 3],
            a: 10,
            h: 0x00,
            l: 0x05,
            ..CPU::default()
        };

        sub(&mut cpu, Register::M);

        assert_eq!(cpu.a, 7);
    }

    #[test]
    fn sub_resets_the_carry_bit_if_no_borrow() {
        let mut cpu = CPU {
            a: 10,
            b: 3,
            ..CPU::default()
        };

        sub(&mut cpu, Register::B);

        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn sub_sets_the_carry_bit_if_borrow() {
        let mut cpu = CPU {
            a: 1,
            b: 3,
            ..CPU::default()
        };

        sub(&mut cpu, Register::B);

        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn sub_a_resets_the_carry_and_zeros_the_accumulator() {
        let mut cpu = CPU {
            a: 0x3e,
            ..CPU::default()
        };

        sub(&mut cpu, Register::A);

        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.a, 0);
    }

    #[test]
    fn sui_removes_immediate_byte_from_accumulator() {
        let mut cpu = CPU {
            memory: vec![1, 5],
            a: 0,
            ..CPU::default()
        };

        sui(&mut cpu);
        assert_eq!(cpu.a, 255);
        assert_eq!(cpu.flags.carry, true);

        sui(&mut cpu);
        assert_eq!(cpu.a, 250);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn sbb_b_subs_b_from_accumulator_with_carry_flag_and_borrowing() {
        let mut cpu = CPU {
            a: 4,
            b: 10,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        sbb(&mut cpu, Register::B);

        assert_eq!(cpu.a, 249);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn sbb_b_subs_b_from_accumulator_without_carry_flag_and_borrowing() {
        let mut cpu = CPU {
            a: 4,
            b: 10,
            flags: Flags {
                carry: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        sbb(&mut cpu, Register::B);

        assert_eq!(cpu.a, 250);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn sbb_b_subs_b_from_accumulator_with_carry_flag_and_not_borrowing() {
        let mut cpu = CPU {
            a: 4,
            b: 1,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        sbb(&mut cpu, Register::B);

        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn sbb_b_subs_b_from_accumulator_without_carry_flag_and_not_borrowing() {
        let mut cpu = CPU {
            a: 4,
            b: 1,
            flags: Flags {
                carry: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        sbb(&mut cpu, Register::B);

        assert_eq!(cpu.a, 3);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn sbb_c_subs_c_from_accumulator_with_carry_flag_set() {
        let mut cpu = CPU {
            a: 5,
            c: 10,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        sbb(&mut cpu, Register::C);

        assert_eq!(cpu.a, 250);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn sbb_d_subs_d_from_accumulator_with_carry_flag_set() {
        let mut cpu = CPU {
            a: 5,
            d: 10,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        sbb(&mut cpu, Register::D);

        assert_eq!(cpu.a, 250);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn sbb_e_subs_e_from_accumulator_with_carry_flag_set() {
        let mut cpu = CPU {
            a: 5,
            e: 10,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        sbb(&mut cpu, Register::E);

        assert_eq!(cpu.a, 250);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn sbb_h_subs_h_from_accumulator_with_carry_flag_set() {
        let mut cpu = CPU {
            a: 5,
            h: 10,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        sbb(&mut cpu, Register::H);

        assert_eq!(cpu.a, 250);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn sbb_m_subs_byte_at_hl_address_to_accumulator_with_carry_flag_set() {
        let mut cpu = CPU {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 10],
            a: 5,
            h: 0x00,
            l: 0x05,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        sbb(&mut cpu, Register::M);

        assert_eq!(cpu.a, 250);
    }

    #[test]
    fn sbb_sub_with_carry_bit() {
        let mut cpu = CPU {
            a: 4,
            l: 2,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        sbb(&mut cpu, Register::L);

        assert_eq!(cpu.a, 1);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);
    }

    #[test]
    fn sbb_sub_with_max_values() {
        let mut cpu = CPU {
            a: u8::max_value(),
            b: u8::max_value(),
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        sbb(&mut cpu, Register::B);

        assert_eq!(cpu.a, 255);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn sbi_removes_immediate_byte_from_accumulator_with_borrow() {
        let mut cpu = CPU {
            memory: vec![0xFF, 0xFF, 0x00, 0x01],
            a: 0x00,
            ..CPU::default()
        };

        sbi(&mut cpu);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.flags.carry, true);

        sbi(&mut cpu);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.flags.carry, true);

        sbi(&mut cpu);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.flags.carry, false);

        sbi(&mut cpu);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.flags.carry, true);
    }
}
