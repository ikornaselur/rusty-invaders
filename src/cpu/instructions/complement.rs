use cpu::CPU;

/// Complement the accumulator
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the complement in
///
pub fn cma(cpu: &mut CPU) -> u8 {
    cpu.a = !cpu.a;

    4
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cma_complements_accumulator() {
        let mut cpu = CPU {
            a: 0b1100_0101,
            ..CPU::default()
        };

        cma(&mut cpu);

        assert_eq!(cpu.a, 0b0011_1010);

        cma(&mut cpu);

        assert_eq!(cpu.a, 0b1100_0101);
    }
}
