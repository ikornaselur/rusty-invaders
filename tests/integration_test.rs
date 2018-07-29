extern crate rusty_invaders;

use std::fs::File;
use std::io::prelude::*;

use rusty_invaders::cpu::state::State;

#[test]
fn full_cpu_test() {
    let mut f = File::open("tests/cpu.bin").expect("Failed to open cpu.bin");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)
        .expect("Failed to read cpu.bin into buffer");
    buffer.resize(0x10000, 0);

    let mut state = State::new(buffer, true);

    loop {
        match state.step() {
            None => break,
            _ => (),
        }
    }
}
