extern crate rusty_invaders;

use rusty_invaders::Config;

#[test]
fn full_cpu_test() {
    let config = Config {
        filename: String::from("tests/cpu.bin"),
        debug: true,
    };

    assert!(
        !rusty_invaders::run(config).is_err(),
        "Error running full cpu test"
    );
}
