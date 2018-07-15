use super::State;

impl State {
    pub fn daa(&mut self) -> Option<u8> {
        // 4 cycles
        if self.a & 0x0f > 9 {
            self.a += 6;
        }
        if self.a & 0xf0 > 0x90 {
            let (result, carry) = self.a.overflowing_add(0x60);
            self.a = result;
            self.set_flags(result, carry);
        }
        None
    }
}
