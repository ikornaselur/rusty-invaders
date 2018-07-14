#[cfg(test)]
use super::State;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn full_step_test() {
        let mut state = State { ..State::default() };
        let mut manual_state = State { ..State::default() };

        // NOP
        state.memory.push(0x00);
        manual_state.memory.push(0x00);
        state.step();

        manual_state.pc = 1;
        assert_eq!(state, manual_state);

        state.memory.push(0x08);
        manual_state.memory.push(0x08);
        state.step();

        manual_state.pc = 2;
        assert_eq!(state, manual_state);

        state.memory.push(0x10);
        manual_state.memory.push(0x10);
        state.step();

        manual_state.pc = 3;
        assert_eq!(state, manual_state);

        state.memory.push(0x18);
        manual_state.memory.push(0x18);
        state.step();

        manual_state.pc = 4;
        assert_eq!(state, manual_state);

        state.memory.push(0x20);
        manual_state.memory.push(0x20);
        state.step();

        manual_state.pc = 5;
        assert_eq!(state, manual_state);

        state.memory.push(0x28);
        manual_state.memory.push(0x28);
        state.step();

        manual_state.pc = 6;
        assert_eq!(state, manual_state);

        state.memory.push(0x30);
        manual_state.memory.push(0x30);
        state.step();

        manual_state.pc = 7;
        assert_eq!(state, manual_state);

        state.memory.push(0x38);
        manual_state.memory.push(0x38);
        state.step();

        manual_state.pc = 8;
        assert_eq!(state, manual_state);

        // LXI B,d16
        state.memory.push(0x01);
        manual_state.memory.push(0x01);
        state.memory.push(0xAD);
        manual_state.memory.push(0xAD);
        state.memory.push(0xDE);
        manual_state.memory.push(0xDE);
        state.step();

        manual_state.pc = 11;
        manual_state.b = 0xDE;
        manual_state.c = 0xAD;
        assert_eq!(state, manual_state);

        // LXI D,d16
        state.memory.push(0x11);
        manual_state.memory.push(0x11);
        state.memory.push(0xAD);
        manual_state.memory.push(0xAD);
        state.memory.push(0xDE);
        manual_state.memory.push(0xDE);
        state.step();

        manual_state.pc = 14;
        manual_state.d = 0xDE;
        manual_state.e = 0xAD;
        assert_eq!(state, manual_state);

        // LXI H,d16
        state.memory.push(0x21);
        manual_state.memory.push(0x21);
        state.memory.push(0xAD);
        manual_state.memory.push(0xAD);
        state.memory.push(0xDE);
        manual_state.memory.push(0xDE);
        state.step();

        manual_state.pc = 17;
        manual_state.h = 0xDE;
        manual_state.l = 0xAD;
        assert_eq!(state, manual_state);

        // LXI SP,d16
        state.memory.push(0x31);
        manual_state.memory.push(0x31);
        state.memory.push(0xAD);
        manual_state.memory.push(0xAD);
        state.memory.push(0xDE);
        manual_state.memory.push(0xDE);
        state.step();

        manual_state.pc = 20;
        manual_state.sp = 0xDEAD;
        assert_eq!(state, manual_state);

        // INR B
        state.memory.push(0x04);
        manual_state.memory.push(0x04);
        state.step();

        manual_state.pc = 21;
        manual_state.b = 0xDF;
        manual_state.cc.sign = true;
        assert_eq!(state, manual_state);

        // INR D
        state.memory.push(0x14);
        manual_state.memory.push(0x14);
        state.step();

        manual_state.pc = 22;
        manual_state.d = 0xDF;
        assert_eq!(state, manual_state);

        // INR H
        state.memory.push(0x24);
        manual_state.memory.push(0x24);
        state.step();

        manual_state.pc = 23;
        manual_state.h = 0xDF;
        assert_eq!(state, manual_state);

        // INR M
        state.memory.push(0x34);
        manual_state.memory.push(0x34);
        state.h = 0x00;
        manual_state.h = 0x00;
        state.l = 0x05;
        manual_state.l = 0x05;
        state.memory[0x05] = 0x01;
        manual_state.memory[0x05] = 0x01;
        state.step();

        manual_state.pc = 24;
        manual_state.cc.sign = false;
        manual_state.memory[0x05] = 0x02;
        assert_eq!(state, manual_state);

        // INR C
        state.memory.push(0x0C);
        manual_state.memory.push(0x0C);
        state.step();

        manual_state.pc = 25;
        manual_state.c = 0xAE;
        manual_state.cc.sign = true;
        assert_eq!(state, manual_state);

        // INR E
        state.memory.push(0x1C);
        manual_state.memory.push(0x1C);
        state.step();

        manual_state.pc = 26;
        manual_state.e = 0xAE;
        assert_eq!(state, manual_state);

        // INR L
        state.memory.push(0x2C);
        manual_state.memory.push(0x2C);
        state.step();

        manual_state.pc = 27;
        manual_state.l = 0x06; // Because of INR M
        manual_state.cc.sign = false;
        manual_state.cc.parity = true;
        assert_eq!(state, manual_state);

        // INR A
        state.memory.push(0x3C);
        manual_state.memory.push(0x3C);
        state.step();

        manual_state.pc = 28;
        manual_state.a = 0x01;
        manual_state.cc.sign = false;
        manual_state.cc.parity = false;
        assert_eq!(state, manual_state);

        // DCR B
        state.memory.push(0x05);
        manual_state.memory.push(0x05);
        state.step();

        manual_state.pc = 29;
        manual_state.b = 0xDE;
        manual_state.cc.sign = true;
        manual_state.cc.parity = true;
        assert_eq!(state, manual_state);

        // DCR D
        // DCR H
        // DCR M
        // DCR C
        // DCR E
        // DCR L
        // DCR A
    }
}
