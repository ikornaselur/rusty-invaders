use cpu::register::Register;
use cpu::CPU;

/// Pop data off the stack into the specified register pair
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the pop in
/// * `register` - The register pair to pop the data into
///
pub fn pop(cpu: &mut CPU, register: Register) -> u8 {
    let least = cpu.read_byte_from_stack().unwrap();
    let most = cpu.read_byte_from_stack().unwrap();

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
        Register::PSW => {
            cpu.flags.set_from_bits(least);
            cpu.a = most;
        }
        unsupported => {
            panic!("pop doesn't support {:?}", unsupported);
        }
    };

    10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pop_into_register_b_pops_two_bytes_off_the_stack_into_b_and_c() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0x15, 0x26, 0x37],
            sp: 3,
            ..CPU::default()
        };

        pop(&mut cpu, Register::B);

        assert_eq!(cpu.c, 0x15);
        assert_eq!(cpu.b, 0x26);
        assert_eq!(cpu.sp, 5);
    }

    #[test]
    fn pop_into_register_d_pops_two_bytes_off_the_stack_into_d_and_e() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0x15, 0x26, 0x37],
            sp: 3,
            ..CPU::default()
        };

        pop(&mut cpu, Register::D);

        assert_eq!(cpu.e, 0x15);
        assert_eq!(cpu.d, 0x26);
        assert_eq!(cpu.sp, 5);
    }

    #[test]
    fn pop_into_register_h_pops_two_bytes_off_the_stack_into_h_and_l() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0x15, 0x26, 0x37],
            sp: 3,
            ..CPU::default()
        };

        pop(&mut cpu, Register::H);

        assert_eq!(cpu.l, 0x15);
        assert_eq!(cpu.h, 0x26);
        assert_eq!(cpu.sp, 5);
    }

    #[test]
    fn pop_into_psq_pops_two_bytes_off_the_stack_into_accumulator_and_flags() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0b0100_0100, 0x26, 0b1000_0001, 0x37],
            sp: 3,
            ..CPU::default()
        };

        pop(&mut cpu, Register::PSW);

        assert_eq!(cpu.a, 0x26);
        assert_eq!(cpu.sp, 5);
        assert_eq!(cpu.flags.sign, false);
        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.parity, true);
        assert_eq!(cpu.flags.carry, false);

        pop(&mut cpu, Register::PSW);
        assert_eq!(cpu.a, 0x37);
        assert_eq!(cpu.sp, 7);
        assert_eq!(cpu.flags.sign, true);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.parity, false);
        assert_eq!(cpu.flags.carry, true);
    }
}
