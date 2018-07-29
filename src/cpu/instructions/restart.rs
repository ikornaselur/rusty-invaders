use cpu::CPU;

/// Write the next byte address to the stack and jump to a predefined RST address at the start
///
/// # Cycles
///
/// 11
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the restart in
/// * `rst` - Which restart to perform, from 0 to 7 (inclusive)
///
pub fn rst(cpu: &mut CPU, rst: usize) -> u8 {
    if rst > 7 {
        panic!("rst doesn't support {}", rst);
    }

    let most = (cpu.pc >> 8) as u8;
    let least = cpu.pc as u8;

    cpu.write_byte_to_stack(most);
    cpu.write_byte_to_stack(least);

    cpu.pc = (8 * rst) as u16;

    11
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rst_0_writes_pc_to_stack_and_sets_pc_to_0() {
        let mut cpu = CPU {
            memory: vec![0; 4],
            pc: 0xDEAD,
            sp: 4,
            ..CPU::default()
        };

        rst(&mut cpu, 0);

        assert_eq!(cpu.pc, 0x00);
        assert_eq!(cpu.memory, vec![0, 0, 0xAD, 0xDE]);
    }
}
