#[derive(Debug, PartialEq)]
pub struct IO {
    ports: Vec<u8>,
    shift_offset: u8,
    shift_value: u16,
}

impl IO {
    pub fn new(ports: usize) -> IO {
        IO {
            ports: vec![0; ports],
            shift_offset: 0,
            shift_value: 0,
        }
    }

    pub fn read(&self, port: usize) -> u8 {
        match port {
            3 => (self.shift_value >> (8 - self.shift_offset)) as u8,
            _ => self.ports[port],
        }
    }

    pub fn set(&mut self, port: usize, value: u8) -> () {
        self.ports[port] = value;
    }

    pub fn write(&mut self, port: usize, value: u8) -> () {
        match port {
            2 => self.shift_offset = value & 0x7,
            4 => self.shift_value = (self.shift_value >> 8) | (value as u16) << 8,
            6 => (),
            _ => self.ports[port] = value,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn writing_port_4() {
        let mut io = IO::new(8);

        io.write(4, 0xAB);
        assert_eq!(io.shift_value, 0xAB00);

        io.write(4, 0xCD);
        assert_eq!(io.shift_value, 0xCDAB);

        io.write(4, 0xEF);
        assert_eq!(io.shift_value, 0xEFCD);
    }

    #[test]
    fn writing_port_2() {
        let mut io = IO::new(8);

        io.write(2, 1);
        assert_eq!(io.shift_offset, 1);

        io.write(2, 7);
        assert_eq!(io.shift_offset, 7);

        io.write(2, 8);
        assert_eq!(io.shift_offset, 0);
    }

    #[test]
    fn read_port_3() {
        let mut io = IO::new(8);

        io.shift_value = 0xDEAD;

        io.shift_offset = 0;
        assert_eq!(io.read(3), 0xDE);

        io.shift_offset = 4;
        assert_eq!(io.read(3), 0xEA);

        io.shift_offset = 7;
        assert_eq!(io.read(3), 0x56);
    }
}
