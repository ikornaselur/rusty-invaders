use cpu::CPU;

/// Perform a unconditional jump to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the jump in
///
pub fn jmp(cpu: &mut CPU) -> u8 {
    cpu.pc = cpu.read_address().unwrap();

    10
}

/// Perform a jump, if the carry bit is set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the jump in
///
pub fn jc(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if cpu.flags.carry {
        cpu.pc = address;
    }

    10
}

/// Perform a jump, if the carry bit is not set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the jump in
///
pub fn jnc(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if !cpu.flags.carry {
        cpu.pc = address;
    }

    10
}

/// Perform a jump, if the zero bit is set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the jump in
///
pub fn jz(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if cpu.flags.zero {
        cpu.pc = address;
    }

    10
}

/// Perform a jump, if the zero bit is not set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the jump in
///
pub fn jnz(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if !cpu.flags.zero {
        cpu.pc = address;
    }

    10
}

/// Perform a jump, if the sign bit is set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the jump in
///
pub fn jm(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if cpu.flags.sign {
        cpu.pc = address;
    }

    10
}

/// Perform a jump, if the sign bit is not set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the jump in
///
pub fn jp(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if !cpu.flags.sign {
        cpu.pc = address;
    }

    10
}

/// Perform a jump, if the parity bit is set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the jump in
///
pub fn jpe(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if cpu.flags.parity {
        cpu.pc = address;
    }

    10
}

/// Perform a jump, if the parity bit is not set, to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the jump in
///
pub fn jpo(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if !cpu.flags.parity {
        cpu.pc = address;
    }

    10
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn jmp_sets_pc_to_new_address() {
        let mut cpu = CPU {
            memory: vec![0xAD, 0xDE],
            ..CPU::default()
        };

        jmp(&mut cpu);

        assert_eq!(cpu.pc, 0xDEAD);
    }

    #[test]
    fn jc_sets_pc_to_new_address_if_carry_flag_set() {
        let mut cpu = CPU {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                carry: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        jc(&mut cpu);

        assert_eq!(cpu.pc, 2);

        cpu.flags.carry = true;
        jc(&mut cpu);

        assert_eq!(cpu.pc, 0xDEAD);
    }

    #[test]
    fn jnc_sets_pc_to_new_address_if_carry_flag_is_not_set() {
        let mut cpu = CPU {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        jnc(&mut cpu);

        assert_eq!(cpu.pc, 2);

        cpu.flags.carry = false;
        jnc(&mut cpu);

        assert_eq!(cpu.pc, 0xDEAD);
    }

    #[test]
    fn jz_sets_pc_to_new_address_if_zero_flag_is_set() {
        let mut cpu = CPU {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                zero: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        jz(&mut cpu);

        assert_eq!(cpu.pc, 2);

        cpu.flags.zero = true;
        jz(&mut cpu);

        assert_eq!(cpu.pc, 0xDEAD);
    }

    #[test]
    fn jnz_sets_pc_to_new_address_if_zero_flag_is_not_set() {
        let mut cpu = CPU {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                zero: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        jnz(&mut cpu);

        assert_eq!(cpu.pc, 2);

        cpu.flags.zero = false;
        jnz(&mut cpu);

        assert_eq!(cpu.pc, 0xDEAD);
    }

    #[test]
    fn jm_sets_pc_to_new_address_if_sign_flag_is_set() {
        let mut cpu = CPU {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                sign: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        jm(&mut cpu);

        assert_eq!(cpu.pc, 2);

        cpu.flags.sign = true;
        jm(&mut cpu);

        assert_eq!(cpu.pc, 0xDEAD);
    }

    #[test]
    fn jp_sets_pc_to_new_address_if_sign_flag_is_not_set() {
        let mut cpu = CPU {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                sign: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        jp(&mut cpu);

        assert_eq!(cpu.pc, 2);

        cpu.flags.sign = false;
        jp(&mut cpu);

        assert_eq!(cpu.pc, 0xDEAD);
    }

    #[test]
    fn jpe_sets_pc_to_new_address_if_parity_flag_is_set() {
        let mut cpu = CPU {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                parity: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        jpe(&mut cpu);

        assert_eq!(cpu.pc, 2);

        cpu.flags.parity = true;
        jpe(&mut cpu);

        assert_eq!(cpu.pc, 0xDEAD);
    }

    #[test]
    fn jpo_sets_pc_to_new_address_if_parity_flag_is_not_set() {
        let mut cpu = CPU {
            memory: vec![0xEF, 0xBE, 0xAD, 0xDE],
            flags: Flags {
                parity: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        jpo(&mut cpu);

        assert_eq!(cpu.pc, 2);

        cpu.flags.parity = false;
        jpo(&mut cpu);

        assert_eq!(cpu.pc, 0xDEAD);
    }
}
