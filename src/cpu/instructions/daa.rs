use cpu::CPU;

/// Perform decimal adjustment, ignoring the Auxiliary Carry
///
/// Sets conditions flags
///
/// # Cycles
///
/// 4
///
/// # Arguments
///
/// * `cpu` - The cpu to perform the DDA in
///
pub fn daa(cpu: &mut CPU) -> u8 {
    if cpu.a & 0x0f > 9 {
        cpu.a += 6;
    }
    if cpu.a & 0xf0 > 0x90 {
        let (result, carry) = cpu.a.overflowing_add(0x60);
        cpu.a = result;
        cpu.flags.set(result, carry);
    }

    4
}
