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
