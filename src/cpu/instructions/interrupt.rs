use cpu::CPU;

/// Enable interrupts
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `cpu` - The cpu to enable interrupts in
///
pub fn ei(cpu: &mut CPU) -> u8 {
    cpu.int_enabled = true;

    4
}

/// Disable interrupts
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `cpu` - The cpu to disable interrupts in
///
pub fn di(cpu: &mut CPU) -> u8 {
    cpu.int_enabled = false;

    4
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ei_enables_interrupts() {
        let mut cpu = CPU {
            int_enabled: false,
            ..CPU::default()
        };

        ei(&mut cpu);

        assert_eq!(cpu.int_enabled, true);
    }

    #[test]
    fn di_enables_interrupts() {
        let mut cpu = CPU {
            int_enabled: true,
            ..CPU::default()
        };

        di(&mut cpu);

        assert_eq!(cpu.int_enabled, false);
    }
}
