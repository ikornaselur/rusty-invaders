use super::Register;
use super::State;

impl State {
    pub fn sbb(&mut self, register: Register) -> () {
        // 4 instructions
        ()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sbb_b_subs_b_from_accumulator_with_borrow() {}
}
