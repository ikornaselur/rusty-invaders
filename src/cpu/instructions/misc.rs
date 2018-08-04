use cpu::CPU;

pub fn nop() -> u8 {
    4
}

pub fn hlt(cpu: &mut CPU) -> u8 {
    cpu.exit = true;
    7
}
