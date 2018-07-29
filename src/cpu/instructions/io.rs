use cpu::CPU;

/// Read the input from a port
///
/// Goes through the io interface on the cpu
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to read the input in
///
pub fn input(cpu: &mut CPU) -> u8 {
    let port = cpu.read_byte().unwrap();
    cpu.a = cpu.io.read(port as usize);

    10
}

/// Write the output to a port
///
/// Goes through the io interface on the cpu
///
/// # Cycles
///
/// 10
///
/// # Arguments
///
/// * `cpu` - The cpu to write the output in
///
pub fn output(cpu: &mut CPU) -> u8 {
    let port = cpu.read_byte().unwrap();
    cpu.io.write(port as usize, cpu.a);

    10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_reads_from_input_port_into_accumulator() {
        let mut cpu = CPU {
            memory: vec![0x1, 0x2],
            ..CPU::default()
        };

        cpu.io.set(1, 0xDE);
        cpu.io.set(2, 0xAD);

        input(&mut cpu);
        assert_eq!(cpu.a, 0xDE);

        input(&mut cpu);
        assert_eq!(cpu.a, 0xAD);
    }

    #[test]
    fn output_writes_into_output_from_accumulator() {
        let mut cpu = CPU {
            memory: vec![0x1, 0x1],
            ..CPU::default()
        };

        cpu.a = 0xDE;
        output(&mut cpu);
        assert_eq!(cpu.io.read(1), 0xDE);

        cpu.a = 0xAD;
        output(&mut cpu);
        assert_eq!(cpu.io.read(1), 0xAD);
    }
}
