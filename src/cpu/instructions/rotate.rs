use cpu::CPU;

/// Rotate the accumulator left
///
/// Sets the carry flag if the left most bit is set before the rotation
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the rotation in
///
pub fn rlc(cpu: &mut CPU) -> u8 {
    let carry = cpu.a >> 7 == 1;
    let result = cpu.a.rotate_left(1);

    cpu.a = result;
    cpu.flags.set(result, carry);

    4
}

/// Rotate the accumulator left, through the carry bit
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the rotation in
///
pub fn ral(cpu: &mut CPU) -> u8 {
    let carry = cpu.a >> 7 == 1;
    let mut result = cpu.a << 1;

    if cpu.flags.carry {
        result |= 0x01;
    }

    cpu.a = result;
    cpu.flags.set(result, carry);

    4
}

/// Rotate the accumulator right
///
/// Sets the carry flag if the right most bit is set before the rotation
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the rotation in
///
pub fn rrc(cpu: &mut CPU) -> u8 {
    let carry = cpu.a & 0x01 == 1;
    let result = cpu.a.rotate_right(1);

    cpu.a = result;
    cpu.flags.set(result, carry);

    4
}

/// Rotate the accumulator right, through the carry bit
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the rotation in
///
pub fn rar(cpu: &mut CPU) -> u8 {
    let carry = cpu.a & 0x01 == 1;
    let mut result = cpu.a >> 1;

    if cpu.flags.carry {
        result |= 0x01 << 7;
    }

    cpu.a = result;
    cpu.flags.set(result, carry);

    4
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::flags::Flags;

    #[test]
    fn rlc_rotates_accumulator_left() {
        let mut cpu = CPU {
            a: 0b0111_0010,
            flags: Flags {
                carry: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        rlc(&mut cpu);

        assert_eq!(cpu.a, 0b1110_0100);
        assert_eq!(cpu.flags.carry, false);

        rlc(&mut cpu);

        assert_eq!(cpu.a, 0b1100_1001);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn ral_rotates_accumulator_left_through_carry() {
        let mut cpu = CPU {
            a: 0b0111_0010,
            flags: Flags {
                carry: true,
                ..Flags::default()
            },
            ..CPU::default()
        };

        ral(&mut cpu);

        assert_eq!(cpu.a, 0b1110_0101);
        assert_eq!(cpu.flags.carry, false);

        ral(&mut cpu);

        assert_eq!(cpu.a, 0b1100_1010);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn rrc_rotates_accumulator_right() {
        let mut cpu = CPU {
            a: 0b1111_0010,
            flags: Flags {
                carry: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        rrc(&mut cpu);

        assert_eq!(cpu.a, 0b0111_1001);
        assert_eq!(cpu.flags.carry, false);

        rrc(&mut cpu);

        assert_eq!(cpu.a, 0b1011_1100);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn rar_rotates_accumulator_right_through_carry() {
        let mut cpu = CPU {
            a: 0b1111_0011,
            flags: Flags {
                carry: false,
                ..Flags::default()
            },
            ..CPU::default()
        };

        rar(&mut cpu);

        assert_eq!(cpu.a, 0b0111_1001);
        assert_eq!(cpu.flags.carry, true);

        rar(&mut cpu);

        assert_eq!(cpu.a, 0b1011_1100);
        assert_eq!(cpu.flags.carry, true);

        rar(&mut cpu);

        assert_eq!(cpu.a, 0b1101_1110);
        assert_eq!(cpu.flags.carry, false);
    }
}
