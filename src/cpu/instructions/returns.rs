use cpu::CPU;

/// Perform an unconditional return to an address
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the return in
///
pub fn ret(cpu: &mut CPU) -> u8 {
    cpu.pc = cpu.read_address_from_stack().unwrap();

    10
}

/// Perform a return, if the carry bit is set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the return in
///
pub fn rc(cpu: &mut CPU) -> u8 {
    if cpu.flags.carry {
        ret(cpu);
        11
    } else {
        5
    }
}

/// Perform a return, if the carry bit is not set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the return in
///
pub fn rnc(cpu: &mut CPU) -> u8 {
    if cpu.flags.carry {
        5
    } else {
        ret(cpu);
        11
    }
}

/// Perform a return, if the zero bit is set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the return in
///
pub fn rz(cpu: &mut CPU) -> u8 {
    if cpu.flags.zero {
        ret(cpu);
        11
    } else {
        5
    }
}

/// Perform a return, if the zero bit is not set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the return in
///
pub fn rnz(cpu: &mut CPU) -> u8 {
    if cpu.flags.zero {
        5
    } else {
        ret(cpu);
        11
    }
}

/// Perform a return, if the sign bit is set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the return in
///
pub fn rm(cpu: &mut CPU) -> u8 {
    if cpu.flags.sign {
        ret(cpu);
        11
    } else {
        5
    }
}

/// Perform a return, if the sign bit is not set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the return in
///
pub fn rp(cpu: &mut CPU) -> u8 {
    if cpu.flags.sign {
        5
    } else {
        ret(cpu);
        11
    }
}

/// Perform a return, if the parity bit is set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the return in
///
pub fn rpe(cpu: &mut CPU) -> u8 {
    if cpu.flags.parity {
        ret(cpu);
        11
    } else {
        5
    }
}

/// Perform a return, if the parity bit is not set, to an address
///
/// # Cycles
///
/// * If return is performed: 11
/// * If return is not performed: 5
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the return in
///
pub fn rpo(cpu: &mut CPU) -> u8 {
    if cpu.flags.parity {
        5
    } else {
        ret(cpu);
        11
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn ret_pops_the_address_off_the_stack_and_jumps_back() {
        let mut cpu = CPU {
            memory: vec![0, 0x08 /* SP */, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            ..CPU::default()
        };

        ret(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 0x0008);
    }

    #[test]
    fn rc_pops_the_address_off_the_stack_and_jumps_back_if_carry_flag_is_set() {
        let mut cpu = CPU {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            flags: Flags {
                carry: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        rc(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        cpu.flags.carry = true;
        rc(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 0x0008);
    }

    #[test]
    fn rnc_pops_the_address_off_the_stack_and_jumps_back_if_carry_flag_is_not_set() {
        let mut cpu = CPU {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        rnc(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        cpu.flags.carry = false;
        rnc(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 0x0008);
    }

    #[test]
    fn rz_pops_the_address_off_the_stack_and_jumps_back_if_zero_flag_is_set() {
        let mut cpu = CPU {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            flags: Flags {
                zero: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        rz(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        cpu.flags.zero = true;
        rz(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 0x0008);
    }

    #[test]
    fn rnz_pops_the_address_off_the_stack_and_jumps_back_if_zero_flag_is_not_set() {
        let mut cpu = CPU {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            flags: Flags {
                zero: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        rnz(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        cpu.flags.zero = false;
        rnz(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 0x0008);
    }

    #[test]
    fn rm_pops_the_address_off_the_stack_and_jumps_back_if_sign_flag_is_set() {
        let mut cpu = CPU {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            flags: Flags {
                sign: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        rm(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        cpu.flags.sign = true;
        rm(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 0x0008);
    }

    #[test]
    fn rp_pops_the_address_off_the_stack_and_jumps_back_if_sign_flag_is_not_set() {
        let mut cpu = CPU {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            flags: Flags {
                sign: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        rp(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        cpu.flags.sign = false;
        rp(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 0x0008);
    }

    #[test]
    fn rpe_pops_the_address_off_the_stack_and_jumps_back_if_parity_flag_is_set() {
        let mut cpu = CPU {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            flags: Flags {
                parity: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        rpe(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        cpu.flags.parity = true;
        rpe(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 0x0008);
    }

    #[test]
    fn rpo_pops_the_address_off_the_stack_and_jumps_back_if_parity_flag_is_not_set() {
        let mut cpu = CPU {
            memory: vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE],
            sp: 1,
            pc: 0xDEAD,
            flags: Flags {
                parity: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        rpo(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE]);

        cpu.flags.parity = false;
        rpo(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 0x0008);
    }
}
