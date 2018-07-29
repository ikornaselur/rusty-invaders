use cpu::register::Register;
use cpu::CPU;

/// Push data onto the stack from the specified register pair
///
/// # Cycles
///
/// 11
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the pop in
/// * `register` - The register pair to push the data from
///
pub fn push(cpu: &mut CPU, register: Register) -> u8 {
    let (most, least) = match register {
        Register::B => (cpu.b, cpu.c),
        Register::D => (cpu.d, cpu.e),
        Register::H => (cpu.h, cpu.l),
        Register::PSW => (cpu.a, cpu.flags.as_bits()),
        unsupported => {
            panic!("pop doesn't support {:?}", unsupported);
        }
    };
    cpu.write_byte_to_stack(most);
    cpu.write_byte_to_stack(least);

    11
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn push_from_register_b_pushed_bytes_onto_the_stack_from_b_and_c() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0, 0, 0],
            b: 0xBB,
            c: 0xCC,
            sp: 0x0004,
            ..CPU::default()
        };

        push(&mut cpu, Register::B);

        assert_eq!(cpu.sp, 0x0002);
        assert_eq!(cpu.memory, vec![0, 0, 0xCC, 0xBB, 0, 0]);
    }

    #[test]
    fn push_from_register_d_pushed_bytes_onto_the_stack_from_d_and_e() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0, 0, 0],
            d: 0x8F,
            e: 0x9D,
            sp: 0x0004,
            ..CPU::default()
        };

        push(&mut cpu, Register::D);

        assert_eq!(cpu.sp, 0x0002);
        assert_eq!(cpu.memory, vec![0, 0, 0x9D, 0x8F, 0, 0]);
    }

    #[test]
    fn push_from_register_h_pushed_bytes_onto_the_stack_from_h_and_l() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0, 0, 0],
            h: 0xFF,
            l: 0x11,
            sp: 0x0004,
            ..CPU::default()
        };

        push(&mut cpu, Register::H);

        assert_eq!(cpu.sp, 2);
        assert_eq!(cpu.memory, vec![0, 0, 0x11, 0xFF, 0, 0]);
    }

    #[test]
    fn push_from_psw_pushed_bytes_onto_the_stack_from_accumulator_and_flags() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0, 0, 0],
            a: 0xAA,
            sp: 0x0004,
            flags: Flags {
                carry: true,
                sign: true,
                zero: true,
                parity: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        push(&mut cpu, Register::PSW);

        assert_eq!(cpu.sp, 2);
        assert_eq!(cpu.memory, vec![0, 0, 0b1100_0101, 0xAA, 0, 0]);
    }
}
