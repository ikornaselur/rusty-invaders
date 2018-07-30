use cpu::register::Register;
use cpu::CPU;

/// Decrement a register
///
/// Sets condition flags
///
/// # Cycles
///
/// * Register M: 10
/// * Other: 5
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the decrement in
/// * `register` - The register to decrement
///
pub fn dcr(cpu: &mut CPU, register: Register) -> u8 {
    match register {
        Register::A => {
            let (result, carry) = cpu.a.overflowing_sub(1);
            cpu.a = result;
            cpu.flags.set(result, carry);
        }
        Register::B => {
            let (result, carry) = cpu.b.overflowing_sub(1);
            cpu.b = result;
            cpu.flags.set(result, carry);
        }
        Register::C => {
            let (result, carry) = cpu.c.overflowing_sub(1);
            cpu.c = result;
            cpu.flags.set(result, carry);
        }
        Register::D => {
            let (result, carry) = cpu.d.overflowing_sub(1);
            cpu.d = result;
            cpu.flags.set(result, carry);
        }
        Register::E => {
            let (result, carry) = cpu.e.overflowing_sub(1);
            cpu.e = result;
            cpu.flags.set(result, carry);
        }
        Register::H => {
            let (result, carry) = cpu.h.overflowing_sub(1);
            cpu.h = result;
            cpu.flags.set(result, carry);
        }
        Register::L => {
            let (result, carry) = cpu.l.overflowing_sub(1);
            cpu.l = result;
            cpu.flags.set(result, carry);
        }
        Register::M => {
            let offset = (u16::from(cpu.h) << 8) + u16::from(cpu.l);
            let byte = cpu.memory[offset as usize];

            let (result, carry) = byte.overflowing_sub(1);
            cpu.memory[offset as usize] = result;
            cpu.flags.set(result, carry);
        }
        unsupported => {
            panic!("add doesn't support {:?}", unsupported);
        }
    };

    match register {
        Register::M => 10,
        _ => 5,
    }
}

/// Decrement a register pair
///
/// Sets condition flags
///
/// # Cycles
///
/// 5
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the decrement in
/// * `register` - The register pair to decrement
///
pub fn dcx(cpu: &mut CPU, register: Register) -> u8 {
    match register {
        Register::B => {
            let result = ((u16::from(cpu.b) << 8) + u16::from(cpu.c)).wrapping_sub(1);
            cpu.b = (result >> 8) as u8;
            cpu.c = result as u8;
        }
        Register::D => {
            let result = ((u16::from(cpu.d) << 8) + u16::from(cpu.e)).wrapping_sub(1);
            cpu.d = (result >> 8) as u8;
            cpu.e = result as u8;
        }
        Register::H => {
            let result = ((u16::from(cpu.h) << 8) + u16::from(cpu.l)).wrapping_sub(1);
            cpu.h = (result >> 8) as u8;
            cpu.l = result as u8;
        }
        Register::SP => {
            cpu.sp = cpu.sp.wrapping_sub(1);
        }
        unsupported => {
            panic!("inx doesn't support {:?}", unsupported);
        }
    }

    5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dcr_b_decreases_b_by_one() {
        let mut cpu = CPU {
            b: 0x10,
            ..CPU::default()
        };

        dcr(&mut cpu, Register::B);

        assert_eq!(cpu.b, 0x0F);
    }

    #[test]
    fn dcr_wraps_and_sets_carry_flag() {
        let mut cpu = CPU {
            b: 0x00,
            ..CPU::default()
        };

        dcr(&mut cpu, Register::B);

        assert_eq!(cpu.b, 0xff);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn dcr_c_decreases_c_by_one() {
        let mut cpu = CPU {
            c: 0x10,
            ..CPU::default()
        };

        dcr(&mut cpu, Register::C);

        assert_eq!(cpu.c, 0x0F);
    }

    #[test]
    fn dcr_d_decreases_d_by_one() {
        let mut cpu = CPU {
            d: 0x10,
            ..CPU::default()
        };

        dcr(&mut cpu, Register::D);

        assert_eq!(cpu.d, 0x0F);
    }

    #[test]
    fn dcr_e_decreases_e_by_one() {
        let mut cpu = CPU {
            e: 0x10,
            ..CPU::default()
        };

        dcr(&mut cpu, Register::E);

        assert_eq!(cpu.e, 0x0F);
    }

    #[test]
    fn dcr_h_decreases_h_by_one() {
        let mut cpu = CPU {
            h: 0x10,
            ..CPU::default()
        };

        dcr(&mut cpu, Register::H);

        assert_eq!(cpu.h, 0x0F);
    }

    #[test]
    fn dcr_l_decreases_l_by_one() {
        let mut cpu = CPU {
            l: 0x10,
            ..CPU::default()
        };

        dcr(&mut cpu, Register::L);

        assert_eq!(cpu.l, 0x0F);
    }

    #[test]
    fn dcr_a_decreases_a_by_one() {
        let mut cpu = CPU {
            a: 0x10,
            ..CPU::default()
        };

        dcr(&mut cpu, Register::A);

        assert_eq!(cpu.a, 0x0F);
    }

    #[test]
    fn dcr_m_decreases_byte_at_hl_address() {
        let mut cpu = CPU {
            memory: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x03],
            h: 0x00,
            l: 0x05,
            ..CPU::default()
        };

        dcr(&mut cpu, Register::M);

        assert_eq!(cpu.memory[0x05], 0x02);
    }

    #[test]
    fn dcx_b_increments_b_c_pair() {
        let mut cpu = CPU {
            b: 0x98,
            c: 0x00,
            ..CPU::default()
        };

        dcx(&mut cpu, Register::B);

        assert_eq!(cpu.b, 0x97);
        assert_eq!(cpu.c, 0xFF);
    }

    #[test]
    fn dcx_d_increments_d_e_pair() {
        let mut cpu = CPU {
            d: 0x98,
            e: 0x00,
            ..CPU::default()
        };

        dcx(&mut cpu, Register::D);

        assert_eq!(cpu.d, 0x97);
        assert_eq!(cpu.e, 0xFF);
    }

    #[test]
    fn dcx_h_increments_h_l_pair() {
        let mut cpu = CPU {
            h: 0x98,
            l: 0x00,
            ..CPU::default()
        };

        dcx(&mut cpu, Register::H);

        assert_eq!(cpu.h, 0x97);
        assert_eq!(cpu.l, 0xFF);
    }

    #[test]
    fn dcx_sp_increments_sp() {
        let mut cpu = CPU {
            sp: 0x0001,
            ..CPU::default()
        };

        dcx(&mut cpu, Register::SP);

        assert_eq!(cpu.sp, 0x0000);

        dcx(&mut cpu, Register::SP);

        assert_eq!(cpu.sp, 0xFFFF);
    }
}
