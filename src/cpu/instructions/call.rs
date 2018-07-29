use cpu::CPU;

fn process_call(cpu: &mut CPU, address: u16) -> () {
    // A specific hack for full cpu test
    if cpu.debug && address == 5 && cpu.c == 9 {
        let offset = (u16::from(cpu.d) << 8) + u16::from(cpu.e);
        if offset == 0x018B {
            panic!("CPU HAS FAILED");
        } else if offset == 0x0174 {
            println!("*** CPU IS OPERATIONAL ***");
            cpu.exit = true;
        } else {
            panic!("UNKNOWN PRINT");
        }
    }
    // End of said hack

    let least = cpu.pc as u8;
    let most = (cpu.pc >> 8) as u8;

    cpu.write_byte_to_stack(most);
    cpu.write_byte_to_stack(least);

    cpu.pc = address;
}

/// Call a subroutine at a specified address, storing the address of the next instruction on the
/// stack
///
/// # Cycles
///
/// 17
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the addition in
///
pub fn call(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();

    process_call(cpu, address);

    17
}

/// Call a subroutine at a specified address, if the carry bit is set, storing the address of the
/// next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the addition in
///
pub fn cc(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if cpu.flags.carry {
        process_call(cpu, address);
        17
    } else {
        11
    }
}

/// Call a subroutine at a specified address, if the carry bit is not set, storing the address of
/// the next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the addition in
///
pub fn cnc(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if cpu.flags.carry {
        11
    } else {
        process_call(cpu, address);
        17
    }
}

/// Call a subroutine at a specified address, if the zero bit is set, storing the address of the
/// next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the addition in
///
pub fn cz(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if cpu.flags.zero {
        process_call(cpu, address);
        17
    } else {
        11
    }
}

/// Call a subroutine at a specified address, if the zero bit is not set, storing the address of
/// the next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the addition in
///
pub fn cnz(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if cpu.flags.zero {
        11
    } else {
        process_call(cpu, address);
        17
    }
}

/// Call a subroutine at a specified address, if the sign bit is set, storing the address of the
/// next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the addition in
///
pub fn cm(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if cpu.flags.sign {
        process_call(cpu, address);
        17
    } else {
        11
    }
}

/// Call a subroutine at a specified address, if the sign bit is not set, storing the address of
/// the next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the addition in
///
pub fn cp(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if cpu.flags.sign {
        11
    } else {
        process_call(cpu, address);
        17
    }
}

/// Call a subroutine at a specified address, if the parity bit is set, storing the address of the
/// next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the addition in
///
pub fn cpe(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if cpu.flags.parity {
        process_call(cpu, address);
        17
    } else {
        11
    }
}

/// Call a subroutine at a specified address, if the parity bit is not set, storing the address of
/// the next instruction on the stack
///
/// # Cycles
///
/// * If subroutine called: 17
/// * If subroutine not called: 11
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the addition in
///
pub fn cpo(cpu: &mut CPU) -> u8 {
    let address = cpu.read_address().unwrap();
    if cpu.flags.parity {
        11
    } else {
        process_call(cpu, address);
        17
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn call_pushes_the_address_after_to_the_stack_and_jumps() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0 /* SP */, 0, 0, 0xAD /* PC */, 0xDE],
            sp: 3,
            pc: 6,
            ..CPU::default()
        };

        call(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(
            cpu.memory,
            vec![0, 0x08 /* SP */, 0x00, 0, 0, 0, 0xAD, 0xDE]
        )
    }

    #[test]
    fn cc_pushes_the_address_after_to_the_stack_and_jumps_if_carry_flag_is_set() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            flags: Flags {
                carry: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        cc(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 8);
        assert_eq!(cpu.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        cpu.pc = 6;
        cpu.flags.carry = true;
        cc(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cnc_pushes_the_address_after_to_the_stack_and_jumps_if_carry_flag_is_not_set() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        cnc(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 8);
        assert_eq!(cpu.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        cpu.pc = 6;
        cpu.flags.carry = false;
        cnc(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cz_pushes_the_address_after_to_the_stack_and_jumps_if_zero_flag_is_set() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            flags: Flags {
                zero: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        cz(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 8);
        assert_eq!(cpu.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        cpu.pc = 6;
        cpu.flags.zero = true;
        cz(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cnz_pushes_the_address_after_to_the_stack_and_jumps_if_zero_flag_is_not_set() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            flags: Flags {
                zero: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        cnz(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 8);
        assert_eq!(cpu.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        cpu.pc = 6;
        cpu.flags.zero = false;
        cnz(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cm_pushes_the_address_after_to_the_stack_and_jumps_if_sign_flag_is_set() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            flags: Flags {
                sign: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        cm(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 8);
        assert_eq!(cpu.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        cpu.pc = 6;
        cpu.flags.sign = true;
        cm(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cp_pushes_the_address_after_to_the_stack_and_jumps_if_sign_flag_is_not_set() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            flags: Flags {
                sign: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        cp(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 8);
        assert_eq!(cpu.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        cpu.pc = 6;
        cpu.flags.sign = false;
        cp(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cpe_pushes_the_address_after_to_the_stack_and_jumps_if_parity_flag_is_set() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            flags: Flags {
                parity: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        cpe(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 8);
        assert_eq!(cpu.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        cpu.pc = 6;
        cpu.flags.parity = true;
        cpe(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }

    #[test]
    fn cpo_pushes_the_address_after_to_the_stack_and_jumps_if_parity_flag_is_not_set() {
        let mut cpu = CPU {
            memory: vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE],
            sp: 3,
            pc: 6,
            flags: Flags {
                parity: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        cpo(&mut cpu);

        assert_eq!(cpu.sp, 3);
        assert_eq!(cpu.pc, 8);
        assert_eq!(cpu.memory, vec![0, 0, 0, 0, 0, 0, 0xAD, 0xDE]);

        cpu.pc = 6;
        cpu.flags.parity = false;
        cpo(&mut cpu);

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.memory, vec![0, 0x08, 0x00, 0, 0, 0, 0xAD, 0xDE])
    }
}
