use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let f = File::open(config.filename)?;

    let mut bytes = f.bytes();
    let mut op_bytes = 0;

    while let Some(byte) = bytes.next() {
        match byte {
            Ok(0x00) => {
                println!("{:04X} {}", op_bytes, "NOP");
                op_bytes += 1;
            }
            Ok(0x01) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "LXI",
                    format!("B,#${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0x02) => {
                println!("{:04X} {:06} {}", op_bytes, "STAX", "B");
                op_bytes += 1;
            }
            Ok(0x03) => {
                println!("{:04X} {:06} {}", op_bytes, "INX", "B");
                op_bytes += 1;
            }
            Ok(0x04) => {
                println!("{:04X} {:06} {}", op_bytes, "INR", "B");
                op_bytes += 1;
            }
            Ok(0x05) => {
                println!("{:04X} {:06} {}", op_bytes, "DCR", "B");
                op_bytes += 1;
            }
            Ok(0x06) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "MVI",
                    format!("B,#${:02X?}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0x07) => {
                println!("{:04X} {}", op_bytes, "RLC");
                op_bytes += 1;
            }
            Ok(0x08) => {
                println!("{:04X} {}", op_bytes, "NOP");
                op_bytes += 1;
            }
            Ok(0x09) => {
                println!("{:04X} {:06} {}", op_bytes, "DAD", "B");
                op_bytes += 1;
            }
            Ok(0x0A) => {
                println!("{:04X} {:06} {}", op_bytes, "LDAX", "B");
                op_bytes += 1;
            }
            Ok(0x0B) => {
                println!("{:04X} {:06} {}", op_bytes, "DCX", "B");
                op_bytes += 1;
            }
            Ok(0x0C) => {
                println!("{:04X} {:06} {}", op_bytes, "INR", "C");
                op_bytes += 1;
            }
            Ok(0x0D) => {
                println!("{:04X} {:06} {}", op_bytes, "DCR", "C");
                op_bytes += 1;
            }
            Ok(0x0E) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "MVI",
                    format!("C,#${:02X?}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0x0F) => {
                println!("{:04X} {}", op_bytes, "RRC");
                op_bytes += 1;
            }
            Ok(0x10) => {
                println!("{:04X} {}", op_bytes, "NOP");
                op_bytes += 1;
            }
            Ok(0x11) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "LXI",
                    format!("D,#${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0x12) => {
                println!("{:04X} {:06} {}", op_bytes, "STAX", "D");
                op_bytes += 1;
            }
            Ok(0x13) => {
                println!("{:04X} {:06} {}", op_bytes, "INX", "D");
                op_bytes += 1;
            }
            Ok(0x14) => {
                println!("{:04X} {:06} {}", op_bytes, "INR", "D");
                op_bytes += 1;
            }
            Ok(0x15) => {
                println!("{:04X} {:06} {}", op_bytes, "DCR", "D");
                op_bytes += 1;
            }
            Ok(0x16) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "MVI",
                    format!("D,#${:02X?}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0x18) => {
                println!("{:04X} {}", op_bytes, "NOP");
                op_bytes += 1;
            }
            Ok(0x19) => {
                println!("{:04X} {:06} {}", op_bytes, "DAD", "D");
                op_bytes += 1;
            }
            Ok(0x1A) => {
                println!("{:04X} {:06} {}", op_bytes, "LDAX", "D");
                op_bytes += 1;
            }
            Ok(0x1B) => {
                println!("{:04X} {:06} {}", op_bytes, "DCX", "D");
                op_bytes += 1;
            }
            Ok(0x1C) => {
                println!("{:04X} {:06} {}", op_bytes, "INR", "E");
                op_bytes += 1;
            }
            Ok(0x1D) => {
                println!("{:04X} {:06} {}", op_bytes, "DCR", "E");
                op_bytes += 1;
            }
            Ok(0x1E) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "MVI",
                    format!("E,#${:02X?}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0x1F) => {
                println!("{:04X} {}", op_bytes, "RAR");
                op_bytes += 1;
            }
            Ok(0x20) => {
                println!("{:04X} {}", op_bytes, "NOP");
                op_bytes += 1;
            }
            Ok(0x21) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "LXI",
                    format!("H,#${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0x22) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "SHLD",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0x23) => {
                println!("{:04X} {:06} {}", op_bytes, "INX", "H");
                op_bytes += 1;
            }
            Ok(0x24) => {
                println!("{:04X} {:06} {}", op_bytes, "INR", "H");
                op_bytes += 1;
            }
            Ok(0x25) => {
                println!("{:04X} {:06} {}", op_bytes, "DCR", "H");
                op_bytes += 1;
            }
            Ok(0x26) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "MVI",
                    format!("H,#${:02X?}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0x27) => {
                println!("{:04X} {}", op_bytes, "DAA");
                op_bytes += 1;
            }
            Ok(0x28) => {
                println!("{:04X} {}", op_bytes, "NOP");
                op_bytes += 1;
            }
            Ok(0x29) => {
                println!("{:04X} {:06} {}", op_bytes, "DAD", "H");
                op_bytes += 1;
            }
            Ok(0x2A) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "LHLD",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0x2B) => {
                println!("{:04X} {:06} {}", op_bytes, "DCX", "H");
                op_bytes += 1;
            }
            Ok(0x2C) => {
                println!("{:04X} {:06} {}", op_bytes, "INR", "L");
                op_bytes += 1;
            }
            Ok(0x2D) => {
                println!("{:04X} {:06} {}", op_bytes, "DCR", "L");
                op_bytes += 1;
            }
            Ok(0x2E) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "MVI",
                    format!("L,#${:02X?}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0x2F) => {
                println!("{:04X} {}", op_bytes, "CMA");
                op_bytes += 1;
            }
            Ok(0x30) => {
                println!("{:04X} {}", op_bytes, "NOP");
                op_bytes += 1;
            }
            Ok(0x31) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "LXI",
                    format!("SP,#${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0x32) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "STA",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0x33) => {
                println!("{:04X} {:06} {}", op_bytes, "INX", "M");
                op_bytes += 1;
            }
            Ok(0x34) => {
                println!("{:04X} {:06} {}", op_bytes, "INR", "M");
                op_bytes += 1;
            }
            Ok(0x35) => {
                println!("{:04X} {:06} {}", op_bytes, "DCR", "M");
                op_bytes += 1;
            }
            Ok(0x36) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "MVI",
                    format!("M,#${:02X?}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0x37) => {
                println!("{:04X} {}", op_bytes, "STC");
                op_bytes += 1;
            }
            Ok(0x38) => {
                println!("{:04X} {}", op_bytes, "NOP");
                op_bytes += 1;
            }
            Ok(0x39) => {
                println!("{:04X} {:06} {}", op_bytes, "DAD", "SP");
                op_bytes += 1;
            }
            Ok(0x3A) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "LDA",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0x3B) => {
                println!("{:04X} {:06} {}", op_bytes, "DCX", "SP");
                op_bytes += 1;
            }
            Ok(0x3C) => {
                println!("{:04X} {:06} {}", op_bytes, "INR", "A");
                op_bytes += 1;
            }
            Ok(0x3D) => {
                println!("{:04X} {:06} {}", op_bytes, "DCR", "A");
                op_bytes += 1;
            }
            Ok(0x3E) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "MVI",
                    format!("A,#${:02X?}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0x3F) => {
                println!("{:04X} {}", op_bytes, "CMC");
                op_bytes += 1;
            }
            Ok(0x40) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "B,B");
                op_bytes += 1;
            }
            Ok(0x41) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "B,C");
                op_bytes += 1;
            }
            Ok(0x42) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "B,D");
                op_bytes += 1;
            }
            Ok(0x43) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "B,E");
                op_bytes += 1;
            }
            Ok(0x44) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "B,H");
                op_bytes += 1;
            }
            Ok(0x45) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "B,L");
                op_bytes += 1;
            }
            Ok(0x46) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "B,M");
                op_bytes += 1;
            }
            Ok(0x47) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "D,A");
                op_bytes += 1;
            }
            Ok(0x48) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "C,B");
                op_bytes += 1;
            }
            Ok(0x49) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "C,C");
                op_bytes += 1;
            }
            Ok(0x4A) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "C,D");
                op_bytes += 1;
            }
            Ok(0x4B) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "C,E");
                op_bytes += 1;
            }
            Ok(0x4C) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "C,H");
                op_bytes += 1;
            }
            Ok(0x4D) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "C,L");
                op_bytes += 1;
            }
            Ok(0x4E) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "C,M");
                op_bytes += 1;
            }
            Ok(0x4F) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "C,A");
                op_bytes += 1;
            }
            Ok(0x50) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "D,B");
                op_bytes += 1;
            }
            Ok(0x51) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "D,C");
                op_bytes += 1;
            }
            Ok(0x52) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "D,D");
                op_bytes += 1;
            }
            Ok(0x53) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "D,E");
                op_bytes += 1;
            }
            Ok(0x54) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "D,H");
                op_bytes += 1;
            }
            Ok(0x55) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "D,L");
                op_bytes += 1;
            }
            Ok(0x56) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "D,M");
                op_bytes += 1;
            }
            Ok(0x57) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "D,A");
                op_bytes += 1;
            }
            Ok(0x58) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "E,B");
                op_bytes += 1;
            }
            Ok(0x59) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "E,C");
                op_bytes += 1;
            }
            Ok(0x5A) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "E,D");
                op_bytes += 1;
            }
            Ok(0x5B) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "E,E");
                op_bytes += 1;
            }
            Ok(0x5C) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "E,H");
                op_bytes += 1;
            }
            Ok(0x5D) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "E,L");
                op_bytes += 1;
            }
            Ok(0x5E) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "E,M");
                op_bytes += 1;
            }
            Ok(0x5F) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "E,A");
                op_bytes += 1;
            }
            Ok(0x60) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "H,B");
                op_bytes += 1;
            }
            Ok(0x61) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "H,C");
                op_bytes += 1;
            }
            Ok(0x62) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "H,D");
                op_bytes += 1;
            }
            Ok(0x63) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "H,E");
                op_bytes += 1;
            }
            Ok(0x64) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "H,H");
                op_bytes += 1;
            }
            Ok(0x65) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "H,L");
                op_bytes += 1;
            }
            Ok(0x66) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "H,M");
                op_bytes += 1;
            }
            Ok(0x67) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "H,A");
                op_bytes += 1;
            }
            Ok(0x68) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "L,B");
                op_bytes += 1;
            }
            Ok(0x69) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "L,C");
                op_bytes += 1;
            }
            Ok(0x6A) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "L,D");
                op_bytes += 1;
            }
            Ok(0x6B) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "L,E");
                op_bytes += 1;
            }
            Ok(0x6C) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "L,H");
                op_bytes += 1;
            }
            Ok(0x6D) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "L,L");
                op_bytes += 1;
            }
            Ok(0x6E) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "L,M");
                op_bytes += 1;
            }
            Ok(0x6F) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "L,A");
                op_bytes += 1;
            }
            Ok(0x70) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "M,B");
                op_bytes += 1;
            }
            Ok(0x71) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "M,C");
                op_bytes += 1;
            }
            Ok(0x72) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "M,D");
                op_bytes += 1;
            }
            Ok(0x73) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "M,E");
                op_bytes += 1;
            }
            Ok(0x74) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "M,H");
                op_bytes += 1;
            }
            Ok(0x75) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "M,L");
                op_bytes += 1;
            }
            Ok(0x76) => {
                println!("{:04X} {}", op_bytes, "HLT");
                op_bytes += 1;
            }
            Ok(0x77) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "M,A");
                op_bytes += 1;
            }
            Ok(0x78) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "A,B");
                op_bytes += 1;
            }
            Ok(0x79) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "A,C");
                op_bytes += 1;
            }
            Ok(0x7A) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "A,D");
                op_bytes += 1;
            }
            Ok(0x7B) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "A,E");
                op_bytes += 1;
            }
            Ok(0x7C) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "A,H");
                op_bytes += 1;
            }
            Ok(0x7D) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "A,L");
                op_bytes += 1;
            }
            Ok(0x7E) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "A,M");
                op_bytes += 1;
            }
            Ok(0x7F) => {
                println!("{:04X} {:06} {}", op_bytes, "MOV", "A,A");
                op_bytes += 1;
            }
            Ok(0x80) => {
                println!("{:04X} {:06} {}", op_bytes, "ADD", "B");
                op_bytes += 1;
            }
            Ok(0x81) => {
                println!("{:04X} {:06} {}", op_bytes, "ADD", "C");
                op_bytes += 1;
            }
            Ok(0x82) => {
                println!("{:04X} {:06} {}", op_bytes, "ADD", "D");
                op_bytes += 1;
            }
            Ok(0x83) => {
                println!("{:04X} {:06} {}", op_bytes, "ADD", "E");
                op_bytes += 1;
            }
            Ok(0x84) => {
                println!("{:04X} {:06} {}", op_bytes, "ADD", "H");
                op_bytes += 1;
            }
            Ok(0x85) => {
                println!("{:04X} {:06} {}", op_bytes, "ADD", "L");
                op_bytes += 1;
            }
            Ok(0x86) => {
                println!("{:04X} {:06} {}", op_bytes, "ADD", "M");
                op_bytes += 1;
            }
            Ok(0x87) => {
                println!("{:04X} {:06} {}", op_bytes, "ADD", "A");
                op_bytes += 1;
            }
            Ok(0x88) => {
                println!("{:04X} {:06} {}", op_bytes, "ADC", "B");
                op_bytes += 1;
            }
            Ok(0x89) => {
                println!("{:04X} {:06} {}", op_bytes, "ADC", "C");
                op_bytes += 1;
            }
            Ok(0x8A) => {
                println!("{:04X} {:06} {}", op_bytes, "ADC", "D");
                op_bytes += 1;
            }
            Ok(0x8B) => {
                println!("{:04X} {:06} {}", op_bytes, "ADC", "E");
                op_bytes += 1;
            }
            Ok(0x8C) => {
                println!("{:04X} {:06} {}", op_bytes, "ADC", "H");
                op_bytes += 1;
            }
            Ok(0x8D) => {
                println!("{:04X} {:06} {}", op_bytes, "ADC", "L");
                op_bytes += 1;
            }
            Ok(0x8E) => {
                println!("{:04X} {:06} {}", op_bytes, "ADC", "M");
                op_bytes += 1;
            }
            Ok(0x8F) => {
                println!("{:04X} {:06} {}", op_bytes, "ADC", "A");
                op_bytes += 1;
            }
            Ok(0x90) => {
                println!("{:04X} {:06} {}", op_bytes, "SUB", "B");
                op_bytes += 1;
            }
            Ok(0x91) => {
                println!("{:04X} {:06} {}", op_bytes, "SUB", "C");
                op_bytes += 1;
            }
            Ok(0x92) => {
                println!("{:04X} {:06} {}", op_bytes, "SUB", "D");
                op_bytes += 1;
            }
            Ok(0x93) => {
                println!("{:04X} {:06} {}", op_bytes, "SUB", "E");
                op_bytes += 1;
            }
            Ok(0x94) => {
                println!("{:04X} {:06} {}", op_bytes, "SUB", "H");
                op_bytes += 1;
            }
            Ok(0x95) => {
                println!("{:04X} {:06} {}", op_bytes, "SUB", "L");
                op_bytes += 1;
            }
            Ok(0x96) => {
                println!("{:04X} {:06} {}", op_bytes, "SUB", "M");
                op_bytes += 1;
            }
            Ok(0x97) => {
                println!("{:04X} {:06} {}", op_bytes, "SUB", "A");
                op_bytes += 1;
            }
            Ok(0x98) => {
                println!("{:04X} {:06} {}", op_bytes, "SBB", "B");
                op_bytes += 1;
            }
            Ok(0x99) => {
                println!("{:04X} {:06} {}", op_bytes, "SBB", "C");
                op_bytes += 1;
            }
            Ok(0x9A) => {
                println!("{:04X} {:06} {}", op_bytes, "SBB", "D");
                op_bytes += 1;
            }
            Ok(0x9B) => {
                println!("{:04X} {:06} {}", op_bytes, "SBB", "E");
                op_bytes += 1;
            }
            Ok(0x9C) => {
                println!("{:04X} {:06} {}", op_bytes, "SBB", "H");
                op_bytes += 1;
            }
            Ok(0x9D) => {
                println!("{:04X} {:06} {}", op_bytes, "SBB", "L");
                op_bytes += 1;
            }
            Ok(0x9E) => {
                println!("{:04X} {:06} {}", op_bytes, "SBB", "M");
                op_bytes += 1;
            }
            Ok(0x9F) => {
                println!("{:04X} {:06} {}", op_bytes, "SBB", "A");
                op_bytes += 1;
            }
            Ok(0xA0) => {
                println!("{:04X} {:06} {}", op_bytes, "ANA", "B");
                op_bytes += 1;
            }
            Ok(0xA1) => {
                println!("{:04X} {:06} {}", op_bytes, "ANA", "C");
                op_bytes += 1;
            }
            Ok(0xA2) => {
                println!("{:04X} {:06} {}", op_bytes, "ANA", "D");
                op_bytes += 1;
            }
            Ok(0xA3) => {
                println!("{:04X} {:06} {}", op_bytes, "ANA", "E");
                op_bytes += 1;
            }
            Ok(0xA4) => {
                println!("{:04X} {:06} {}", op_bytes, "ANA", "H");
                op_bytes += 1;
            }
            Ok(0xA5) => {
                println!("{:04X} {:06} {}", op_bytes, "ANA", "L");
                op_bytes += 1;
            }
            Ok(0xA6) => {
                println!("{:04X} {:06} {}", op_bytes, "ANA", "M");
                op_bytes += 1;
            }
            Ok(0xA7) => {
                println!("{:04X} {:06} {}", op_bytes, "ANA", "A");
                op_bytes += 1;
            }
            Ok(0xA8) => {
                println!("{:04X} {:06} {}", op_bytes, "XRA", "B");
                op_bytes += 1;
            }
            Ok(0xA9) => {
                println!("{:04X} {:06} {}", op_bytes, "XRA", "C");
                op_bytes += 1;
            }
            Ok(0xAA) => {
                println!("{:04X} {:06} {}", op_bytes, "XRA", "D");
                op_bytes += 1;
            }
            Ok(0xAB) => {
                println!("{:04X} {:06} {}", op_bytes, "XRA", "E");
                op_bytes += 1;
            }
            Ok(0xAC) => {
                println!("{:04X} {:06} {}", op_bytes, "XRA", "H");
                op_bytes += 1;
            }
            Ok(0xAD) => {
                println!("{:04X} {:06} {}", op_bytes, "XRA", "L");
                op_bytes += 1;
            }
            Ok(0xAE) => {
                println!("{:04X} {:06} {}", op_bytes, "XRA", "M");
                op_bytes += 1;
            }
            Ok(0xAF) => {
                println!("{:04X} {:06} {}", op_bytes, "XRA", "A");
                op_bytes += 1;
            }
            Ok(0xB0) => {
                println!("{:04X} {:06} {}", op_bytes, "ANA", "B");
                op_bytes += 1;
            }
            Ok(0xB1) => {
                println!("{:04X} {:06} {}", op_bytes, "ORA", "C");
                op_bytes += 1;
            }
            Ok(0xB2) => {
                println!("{:04X} {:06} {}", op_bytes, "ORA", "D");
                op_bytes += 1;
            }
            Ok(0xB3) => {
                println!("{:04X} {:06} {}", op_bytes, "ORA", "E");
                op_bytes += 1;
            }
            Ok(0xB4) => {
                println!("{:04X} {:06} {}", op_bytes, "ORA", "H");
                op_bytes += 1;
            }
            Ok(0xB5) => {
                println!("{:04X} {:06} {}", op_bytes, "ORA", "L");
                op_bytes += 1;
            }
            Ok(0xB6) => {
                println!("{:04X} {:06} {}", op_bytes, "ORA", "M");
                op_bytes += 1;
            }
            Ok(0xB7) => {
                println!("{:04X} {:06} {}", op_bytes, "ORA", "A");
                op_bytes += 1;
            }
            Ok(0xB8) => {
                println!("{:04X} {:06} {}", op_bytes, "CMP", "B");
                op_bytes += 1;
            }
            Ok(0xB9) => {
                println!("{:04X} {:06} {}", op_bytes, "CMP", "C");
                op_bytes += 1;
            }
            Ok(0xBA) => {
                println!("{:04X} {:06} {}", op_bytes, "CMP", "D");
                op_bytes += 1;
            }
            Ok(0xBB) => {
                println!("{:04X} {:06} {}", op_bytes, "CMP", "E");
                op_bytes += 1;
            }
            Ok(0xBC) => {
                println!("{:04X} {:06} {}", op_bytes, "CMP", "H");
                op_bytes += 1;
            }
            Ok(0xBD) => {
                println!("{:04X} {:06} {}", op_bytes, "CMP", "L");
                op_bytes += 1;
            }
            Ok(0xBE) => {
                println!("{:04X} {:06} {}", op_bytes, "CMP", "M");
                op_bytes += 1;
            }
            Ok(0xBF) => {
                println!("{:04X} {:06} {}", op_bytes, "CMP", "A");
                op_bytes += 1;
            }
            Ok(0xC0) => {
                println!("{:04X} {}", op_bytes, "RNZ");
                op_bytes += 1;
            }
            Ok(0xC1) => {
                println!("{:04X} {:06} {}", op_bytes, "POP", "B");
                op_bytes += 1;
            }
            Ok(0xC2) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "JNZ",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xC3) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "JMP",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xC4) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "CNZ",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xC5) => {
                println!("{:04X} {:06} {}", op_bytes, "PUSH", "B");
                op_bytes += 1;
            }
            Ok(0xC6) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04x} {:06} {}",
                    op_bytes,
                    "ADI",
                    format!("#${:02X}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0xC8) => {
                println!("{:04X} {}", op_bytes, "RZ");
                op_bytes += 1;
            }
            Ok(0xC9) => {
                println!("{:04X} {}", op_bytes, "RET");
                op_bytes += 1;
            }
            Ok(0xCA) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "JZ",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xCC) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "CZ",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xCD) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "CALL",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xCE) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04x} {:06} {}",
                    op_bytes,
                    "ACI",
                    format!("#${:02X}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0xD0) => {
                println!("{:04X} {}", op_bytes, "RNC");
                op_bytes += 1;
            }
            Ok(0xD1) => {
                println!("{:04X} {:06} {}", op_bytes, "POP", "D");
                op_bytes += 1;
            }
            Ok(0xD2) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "JNC",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xD3) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04x} {:06} {}",
                    op_bytes,
                    "OUT",
                    format!("#${:02X}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0xD4) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "CNC",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xD5) => {
                println!("{:04X} {:06} {}", op_bytes, "PUSH", "D");
                op_bytes += 1;
            }
            Ok(0xD6) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04x} {:06} {}",
                    op_bytes,
                    "SUI",
                    format!("#${:02X}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0xD8) => {
                println!("{:04X} {}", op_bytes, "RC");
                op_bytes += 1;
            }
            Ok(0xDA) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "JC",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xDB) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04x} {:06} {}",
                    op_bytes,
                    "IN",
                    format!("#${:02X}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0xDC) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "CC",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xDE) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04x} {:06} {}",
                    op_bytes,
                    "SBI",
                    format!("#${:02X}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0xE0) => {
                println!("{:04X} {}", op_bytes, "RPO");
                op_bytes += 1;
            }
            Ok(0xE1) => {
                println!("{:04X} {:06} {}", op_bytes, "POP", "H");
                op_bytes += 1;
            }
            Ok(0xE2) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "JPO",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xE3) => {
                println!("{:04X} {}", op_bytes, "XTHL");
                op_bytes += 1;
            }
            Ok(0xE4) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "CPO",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xE5) => {
                println!("{:04X} {:06} {}", op_bytes, "PUSH", "H");
                op_bytes += 1;
            }
            Ok(0xE6) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04x} {:06} {}",
                    op_bytes,
                    "ANI",
                    format!("#${:02X}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0xE9) => {
                println!("{:04X} {}", op_bytes, "PCHL");
                op_bytes += 1;
            }
            Ok(0xEB) => {
                println!("{:04X} {}", op_bytes, "XCHG");
                op_bytes += 1;
            }
            Ok(0xEC) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "CPE",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xEE) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04x} {:06} {}",
                    op_bytes,
                    "XRI",
                    format!("#${:02X}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0xF0) => {
                println!("{:04X} {}", op_bytes, "RP");
                op_bytes += 1;
            }
            Ok(0xF1) => {
                println!("{:04X} {:06} {}", op_bytes, "POP", "PSW");
                op_bytes += 1;
            }
            Ok(0xF2) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "JP",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xF4) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "CP",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xF5) => {
                println!("{:04X} {:06} {}", op_bytes, "PUSH", "PSW");
                op_bytes += 1;
            }
            Ok(0xF6) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04x} {:06} {}",
                    op_bytes,
                    "ORI",
                    format!("#${:02X}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0xF7) => {
                println!("{:04X} {:06} {}", op_bytes, "RST", "6");
                op_bytes += 1;
            }
            Ok(0xF8) => {
                println!("{:04X} {}", op_bytes, "RM");
                op_bytes += 1;
            }
            Ok(0xFA) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "JM",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xFB) => {
                println!("{:04X} {}", op_bytes, "EI");
                op_bytes += 1;
            }
            Ok(0xFC) => {
                let byte2 = bytes.next().unwrap().unwrap();
                let byte3 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04X} {:06} {}",
                    op_bytes,
                    "CM",
                    format!("${:02X?}{:02X?}", byte3, byte2)
                );
                op_bytes += 3;
            }
            Ok(0xFE) => {
                let byte2 = bytes.next().unwrap().unwrap();
                println!(
                    "{:04x} {:06} {}",
                    op_bytes,
                    "CPI",
                    format!("#${:02X}", byte2)
                );
                op_bytes += 2;
            }
            Ok(0xFF) => {
                println!("{:04X} {:06} {}", op_bytes, "RST", "7");
                op_bytes += 1;
            }
            Ok(byte) => {
                panic!("Unknown OP: 0x{:02X?}", byte);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Filename missing"),
        };

        Ok(Config { filename })
    }
}
