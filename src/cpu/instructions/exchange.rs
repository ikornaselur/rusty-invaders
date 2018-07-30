use cpu::CPU;
use std::mem::swap;

/// Swap the contents of register pairs HL with DE
///
/// # Cycles
///
/// 5
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the swap in
///
pub fn xchg(cpu: &mut CPU) -> u8 {
    swap(&mut cpu.h, &mut cpu.d);
    swap(&mut cpu.l, &mut cpu.e);

    5
}

/// Swap the contents of register pairs HL with the top of the stack
///
/// # Cycles
///
/// 18
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the swap in
///
pub fn xthl(cpu: &mut CPU) -> u8 {
    swap(&mut cpu.h, &mut cpu.memory[(cpu.sp + 1) as usize]);
    swap(&mut cpu.l, &mut cpu.memory[cpu.sp as usize]);

    18
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn xchg_exchanges_h_l_pair_with_d_e_pair() {
        let mut cpu = CPU {
            d: 0xDE,
            e: 0xAD,
            h: 0xBE,
            l: 0xEF,
            ..CPU::default()
        };

        xchg(&mut cpu);

        assert_eq!(cpu.d, 0xBE);
        assert_eq!(cpu.e, 0xEF);
        assert_eq!(cpu.h, 0xDE);
        assert_eq!(cpu.l, 0xAD);
    }

    #[test]
    fn xthl_exchanges_h_l_pair_with_bytes_on_stack() {
        let mut cpu = CPU {
            memory: vec![0, 0xDE, 0xAD],
            h: 0xBE,
            l: 0xEF,
            sp: 1,
            ..CPU::default()
        };

        xthl(&mut cpu);

        assert_eq!(cpu.h, 0xAD);
        assert_eq!(cpu.l, 0xDE);
        assert_eq!(cpu.memory, vec![0, 0xEF, 0xBE]);
    }
}
