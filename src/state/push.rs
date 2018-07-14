use super::Register;
use super::State;

impl State {
    pub fn push(&mut self, register: Register) -> () {
        ()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_register_b_pushes_it() {}
}
