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
}
