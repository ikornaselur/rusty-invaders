use cpu::CPU;

/// Set the carry flag
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `cpu` - The cpu to set the flag in
///
pub fn stc(cpu: &mut CPU) -> u8 {
    cpu.flags.carry = true;
    4
}

/// Complement the carry flag
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `cpu` - The cpu to complement the flag in
///
pub fn cmc(cpu: &mut CPU) -> u8 {
    cpu.flags.carry = !cpu.flags.carry;
    4
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stc_sets_carry_bit() {
        let mut cpu = CPU { ..CPU::default() };

        stc(&mut cpu);

        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn cmc_reverses_carry_bit() {
        let mut cpu = CPU { ..CPU::default() };

        cmc(&mut cpu);
        assert_eq!(cpu.flags.carry, true);

        cmc(&mut cpu);
        assert_eq!(cpu.flags.carry, false);

        cmc(&mut cpu);
        assert_eq!(cpu.flags.carry, true);
    }
}
