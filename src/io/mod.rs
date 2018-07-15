#[derive(Debug, PartialEq)]
pub struct IO {
    ports: Vec<u8>,
}

impl IO {
    pub fn new(ports: usize) -> IO {
        IO {
            ports: vec![0; ports],
        }
    }

    pub fn read(&self, port: usize) -> u8 {
        println!("READING FROM IO PORT {}", port);
        self.ports[port]
    }

    pub fn write(&mut self, port: usize, value: u8) -> () {
        if port == 6 {
            return (); // Debug port, ignore for now
        }
        println!("WRITING TO IO PORT {}: {:02X?}", port, value);
        self.ports[port] = value;
    }
}
