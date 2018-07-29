#[derive(Debug, PartialEq)]
pub(crate) struct Flags {
    pub(crate) zero: bool,   // Zero - when arithmetic result is 0
    pub(crate) sign: bool,   // Sign - when the most significant bit is set
    pub(crate) parity: bool, // Parity - when the answer has even parity
    pub(crate) carry: bool,  // Carry - when the instruction resulted in carry
    pub(crate) zc: u8,
    pub(crate) pad: u8,
}

impl Default for Flags {
    fn default() -> Flags {
        Flags {
            zero: false,
            sign: false,
            parity: false,
            carry: false,
            zc: 0,
            pad: 0,
        }
    }
}

impl Flags {
    pub fn as_bits(&self) -> u8 {
        let mut bits = 0;
        if self.sign {
            bits |= 0b1000_0000
        }
        if self.zero {
            bits |= 0b0100_0000
        }
        if self.parity {
            bits |= 0b0000_0100
        }
        if self.carry {
            bits |= 0b0000_0001
        }
        bits
    }

    pub fn set(&mut self, byte: u8, carry: bool) -> () {
        self.sign = (byte & 0x80) != 0;
        self.zero = byte == 0u8;
        self.parity = byte.count_ones() % 2 == 0;
        self.carry = carry;
    }

    pub fn set_from_bits(&mut self, bits: u8) -> () {
        self.sign = bits & 0b1000_0000 != 0;
        self.zero = bits & 0b0100_0000 != 0;
        self.parity = bits & 0b0000_0100 != 0;
        self.carry = bits & 0b0000_0001 != 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn as_bits_returns_correct_bits() {
        let mut flags = Flags::default();

        assert_eq!(flags.as_bits(), 0b0000_0000);

        flags.carry = true;

        assert_eq!(flags.as_bits(), 0b0000_0001);

        flags.parity = true;

        assert_eq!(flags.as_bits(), 0b0000_0101);

        flags.zero = true;

        assert_eq!(flags.as_bits(), 0b0100_0101);

        flags.sign = true;

        assert_eq!(flags.as_bits(), 0b1100_0101);
    }

    #[test]
    fn set_sets_sign_flag() {
        let mut flags = Flags::default();

        let signed: u8 = 0b1000_0000;
        flags.set(signed, false);
        assert_eq!(flags.sign, true);

        let unsigned: u8 = 0b0111_1111;
        flags.set(unsigned, false);
        assert_eq!(flags.sign, false);
    }

    #[test]
    fn set_sets_carry_flag() {
        let mut flags = Flags::default();

        flags.set(0, true);
        assert_eq!(flags.carry, true);

        flags.set(0, false);
        assert_eq!(flags.carry, false);
    }

    #[test]
    fn set_sets_parity_flag() {
        let mut flags = Flags::default();

        let even1: u8 = 0b0000_0000;
        let even2: u8 = 0b0110_0000;
        let even3: u8 = 0b0001_1011;

        flags.set(even1, false);
        assert_eq!(flags.parity, true);

        flags.set(even2, false);
        assert_eq!(flags.parity, true);

        flags.set(even3, false);
        assert_eq!(flags.parity, true);

        let odd1: u8 = 0b0000_0001;
        let odd2: u8 = 0b0101_0001;
        let odd3: u8 = 0b1011_0101;

        flags.set(odd1, false);
        assert_eq!(flags.parity, false);

        flags.set(odd2, false);
        assert_eq!(flags.parity, false);

        flags.set(odd3, false);
        assert_eq!(flags.parity, false);
    }

    #[test]
    fn set_from_bits_sets_flags() {
        let mut flags = Flags::default();

        let sign = 0b1000_0000;
        let zero = 0b0100_0000;
        let parity = 0b0000_0100;
        let carry = 0b0000_0001;

        flags.set_from_bits(sign);
        assert_eq!(flags.sign, true);
        assert_eq!(flags.zero, false);
        assert_eq!(flags.parity, false);
        assert_eq!(flags.carry, false);

        flags.set_from_bits(zero);
        assert_eq!(flags.sign, false);
        assert_eq!(flags.zero, true);
        assert_eq!(flags.parity, false);
        assert_eq!(flags.carry, false);

        flags.set_from_bits(parity);
        assert_eq!(flags.sign, false);
        assert_eq!(flags.zero, false);
        assert_eq!(flags.parity, true);
        assert_eq!(flags.carry, false);

        flags.set_from_bits(carry);
        assert_eq!(flags.sign, false);
        assert_eq!(flags.zero, false);
        assert_eq!(flags.parity, false);
        assert_eq!(flags.carry, true);

        flags.set_from_bits(0b1111_1111);
        assert_eq!(flags.sign, true);
        assert_eq!(flags.zero, true);
        assert_eq!(flags.parity, true);
        assert_eq!(flags.carry, true);
    }

}
