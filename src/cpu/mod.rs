use cpu::flags::Flags;
use cpu::instructions::addition::{aci, adc, add, adi, dad};
use cpu::instructions::and::{ana, ani};
use cpu::instructions::call::{call, cc, cm, cnc, cnz, cp, cpe, cpo, cz};
use cpu::instructions::carry::{cmc, stc};
use cpu::instructions::compare::{cmp, cpi};
use cpu::instructions::complement::cma;
use cpu::instructions::daa::daa;
use cpu::instructions::decrement::{dcr, dcx};
use cpu::instructions::exchange::{xchg, xthl};
use cpu::instructions::increment::{inr, inx};
use cpu::instructions::interrupt::{di, ei};
use cpu::instructions::io::{input, output};
use cpu::instructions::jump::{jc, jm, jmp, jnc, jnz, jp, jpe, jpo, jz};
use cpu::instructions::load::{lda, ldax, lhld, lxi, pchl, sphl};
use cpu::instructions::mov::{mov, mvi};
use cpu::instructions::or::{ora, ori};
use cpu::instructions::pop::pop;
use cpu::instructions::push::push;
use cpu::instructions::restart::rst;
use cpu::instructions::returns::{rc, ret, rm, rnc, rnz, rp, rpe, rpo, rz};
use cpu::instructions::rotate::{ral, rar, rlc, rrc};
use cpu::instructions::store::{shld, sta, stax};
use cpu::instructions::subtraction::{sbb, sbi, sub, sui};
use cpu::instructions::xor::{xra, xri};
use cpu::register::Register;
use machine::io::IO;

mod flags;
mod instructions;
mod register;

#[derive(Debug, PartialEq)]
pub struct CPU {
    pub(crate) a: u8,
    pub(crate) b: u8,
    pub(crate) c: u8,
    pub(crate) d: u8,
    pub(crate) e: u8,
    pub(crate) h: u8,
    pub(crate) l: u8,
    pub(crate) sp: u16,
    pub(crate) pc: u16,
    pub(crate) memory: Vec<u8>,
    pub(crate) flags: Flags,
    pub(crate) int_enabled: bool,
    pub(crate) exit: bool,
    pub(crate) debug: bool,
    pub(crate) io: IO,
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory: Vec::new(),
            flags: Flags::default(),
            int_enabled: false,
            exit: false,
            debug: false,
            io: IO::new(7),
        }
    }
}

impl CPU {
    pub fn new(memory: Vec<u8>, debug: bool) -> CPU {
        CPU {
            memory,
            debug,
            ..CPU::default()
        }
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if self.pc as usize >= self.memory.len() {
            None
        } else {
            let byte = self.memory[self.pc as usize];
            self.pc += 1;
            Some(byte)
        }
    }

    pub fn read_address(&mut self) -> Option<u16> {
        if let Some(least) = self.read_byte() {
            if let Some(most) = self.read_byte() {
                return Some((u16::from(most) << 8) + u16::from(least));
            }
        }
        None
    }

    pub fn read_byte_from_stack(&mut self) -> Option<u8> {
        if self.sp as usize >= self.memory.len() {
            None
        } else {
            let byte = self.memory[self.sp as usize];
            self.sp += 1;
            Some(byte)
        }
    }

    pub fn read_address_from_stack(&mut self) -> Option<u16> {
        if let Some(least) = self.read_byte_from_stack() {
            if let Some(most) = self.read_byte_from_stack() {
                return Some((u16::from(most) << 8) + u16::from(least));
            }
        }
        None
    }

    pub fn write_byte_to_stack(&mut self, byte: u8) -> () {
        if self.sp == 0 {
            panic!("Stack pointer out of bounds!")
        }
        self.sp -= 1;
        self.memory[self.sp as usize] = byte;
    }

    pub fn get_frame(&mut self) -> &[u8] {
        &self.memory[0x2400..0x4000]
    }

    pub fn set_input(&mut self, port: usize, byte: u8) -> () {
        if port < 1 || port > 2 {
            panic!("Can only write to port 1 and 2");
        }
        self.io.set(port, byte);
    }

    pub fn interrupt(&mut self, interrupt_num: usize) -> () {
        rst(self, interrupt_num);
    }

    pub fn disable_interrupt(&mut self) -> () {
        di(self);
    }

    pub fn nop(&mut self) -> u8 {
        4
    }

    pub fn hlt(&mut self) -> u8 {
        self.exit = true;
        7
    }

    pub fn step(&mut self) -> Option<u8> {
        if self.exit {
            return None;
        }
        let byte = match self.read_byte() {
            Some(byte) => byte,
            None => {
                panic!("Got no byte when reading");
            }
        };

        let cycles = match byte {
            // NOPs
            0x00 => self.nop(),
            0x08 => self.nop(),
            0x10 => self.nop(),
            0x18 => self.nop(),
            0x20 => self.nop(),
            0x28 => self.nop(),
            0x30 => self.nop(),
            0x38 => self.nop(),

            // Instructions with registers

            // LXI ?,d16
            0x01 => lxi(self, Register::B),
            0x11 => lxi(self, Register::D),
            0x21 => lxi(self, Register::H),
            0x31 => lxi(self, Register::SP),

            // INR ?
            0x04 => inr(self, Register::B),
            0x14 => inr(self, Register::D),
            0x24 => inr(self, Register::H),
            0x34 => inr(self, Register::M),
            0x0C => inr(self, Register::C),
            0x1C => inr(self, Register::E),
            0x2C => inr(self, Register::L),
            0x3C => inr(self, Register::A),

            // DCR ?
            0x05 => dcr(self, Register::B),
            0x15 => dcr(self, Register::D),
            0x25 => dcr(self, Register::H),
            0x35 => dcr(self, Register::M),
            0x0D => dcr(self, Register::C),
            0x1D => dcr(self, Register::E),
            0x2D => dcr(self, Register::L),
            0x3D => dcr(self, Register::A),

            // MVI ?, d8
            0x06 => mvi(self, Register::B),
            0x0E => mvi(self, Register::C),
            0x16 => mvi(self, Register::D),
            0x1E => mvi(self, Register::E),
            0x26 => mvi(self, Register::H),
            0x2E => mvi(self, Register::L),
            0x36 => mvi(self, Register::M),
            0x3E => mvi(self, Register::A),

            // MOV ?, ?
            0x40 => mov(self, Register::B, Register::B),
            0x41 => mov(self, Register::B, Register::C),
            0x42 => mov(self, Register::B, Register::D),
            0x43 => mov(self, Register::B, Register::E),
            0x44 => mov(self, Register::B, Register::H),
            0x45 => mov(self, Register::B, Register::L),
            0x46 => mov(self, Register::B, Register::M),
            0x47 => mov(self, Register::B, Register::A),
            0x48 => mov(self, Register::C, Register::B),
            0x49 => mov(self, Register::C, Register::C),
            0x4A => mov(self, Register::C, Register::D),
            0x4B => mov(self, Register::C, Register::E),
            0x4C => mov(self, Register::C, Register::H),
            0x4D => mov(self, Register::C, Register::L),
            0x4E => mov(self, Register::C, Register::M),
            0x4F => mov(self, Register::C, Register::A),

            0x50 => mov(self, Register::D, Register::B),
            0x51 => mov(self, Register::D, Register::C),
            0x52 => mov(self, Register::D, Register::D),
            0x53 => mov(self, Register::D, Register::E),
            0x54 => mov(self, Register::D, Register::H),
            0x55 => mov(self, Register::D, Register::L),
            0x56 => mov(self, Register::D, Register::M),
            0x57 => mov(self, Register::D, Register::A),
            0x58 => mov(self, Register::E, Register::B),
            0x59 => mov(self, Register::E, Register::C),
            0x5A => mov(self, Register::E, Register::D),
            0x5B => mov(self, Register::E, Register::E),
            0x5C => mov(self, Register::E, Register::H),
            0x5D => mov(self, Register::E, Register::L),
            0x5E => mov(self, Register::E, Register::M),
            0x5F => mov(self, Register::E, Register::A),

            0x60 => mov(self, Register::H, Register::B),
            0x61 => mov(self, Register::H, Register::C),
            0x62 => mov(self, Register::H, Register::D),
            0x63 => mov(self, Register::H, Register::E),
            0x64 => mov(self, Register::H, Register::H),
            0x65 => mov(self, Register::H, Register::L),
            0x66 => mov(self, Register::H, Register::M),
            0x67 => mov(self, Register::H, Register::A),
            0x68 => mov(self, Register::L, Register::B),
            0x69 => mov(self, Register::L, Register::C),
            0x6A => mov(self, Register::L, Register::D),
            0x6B => mov(self, Register::L, Register::E),
            0x6C => mov(self, Register::L, Register::H),
            0x6D => mov(self, Register::L, Register::L),
            0x6E => mov(self, Register::L, Register::M),
            0x6F => mov(self, Register::L, Register::A),

            0x70 => mov(self, Register::M, Register::B),
            0x71 => mov(self, Register::M, Register::C),
            0x72 => mov(self, Register::M, Register::D),
            0x73 => mov(self, Register::M, Register::E),
            0x74 => mov(self, Register::M, Register::H),
            0x75 => mov(self, Register::M, Register::L),
            0x77 => mov(self, Register::M, Register::A),
            0x78 => mov(self, Register::A, Register::B),
            0x79 => mov(self, Register::A, Register::C),
            0x7A => mov(self, Register::A, Register::D),
            0x7B => mov(self, Register::A, Register::E),
            0x7C => mov(self, Register::A, Register::H),
            0x7D => mov(self, Register::A, Register::L),
            0x7E => mov(self, Register::A, Register::M),
            0x7F => mov(self, Register::A, Register::A),

            // ADD ?
            0x80 => add(self, Register::B),
            0x81 => add(self, Register::C),
            0x82 => add(self, Register::D),
            0x83 => add(self, Register::E),
            0x84 => add(self, Register::H),
            0x85 => add(self, Register::L),
            0x86 => add(self, Register::M),
            0x87 => add(self, Register::A),

            // ADC ?
            0x88 => adc(self, Register::B),
            0x89 => adc(self, Register::C),
            0x8A => adc(self, Register::D),
            0x8B => adc(self, Register::E),
            0x8C => adc(self, Register::H),
            0x8D => adc(self, Register::L),
            0x8E => adc(self, Register::M),
            0x8F => adc(self, Register::A),

            // SUB ?
            0x90 => sub(self, Register::B),
            0x91 => sub(self, Register::C),
            0x92 => sub(self, Register::D),
            0x93 => sub(self, Register::E),
            0x94 => sub(self, Register::H),
            0x95 => sub(self, Register::L),
            0x96 => sub(self, Register::M),
            0x97 => sub(self, Register::A),

            // SBB ?
            0x98 => sbb(self, Register::B),
            0x99 => sbb(self, Register::C),
            0x9A => sbb(self, Register::D),
            0x9B => sbb(self, Register::E),
            0x9C => sbb(self, Register::H),
            0x9D => sbb(self, Register::L),
            0x9E => sbb(self, Register::M),
            0x9F => sbb(self, Register::A),

            // ANA ?
            0xA0 => ana(self, Register::B),
            0xA1 => ana(self, Register::C),
            0xA2 => ana(self, Register::D),
            0xA3 => ana(self, Register::E),
            0xA4 => ana(self, Register::H),
            0xA5 => ana(self, Register::L),
            0xA6 => ana(self, Register::M),
            0xA7 => ana(self, Register::A),

            // XRA ?
            0xA8 => xra(self, Register::B),
            0xA9 => xra(self, Register::C),
            0xAA => xra(self, Register::D),
            0xAB => xra(self, Register::E),
            0xAC => xra(self, Register::H),
            0xAD => xra(self, Register::L),
            0xAE => xra(self, Register::M),
            0xAF => xra(self, Register::A),

            // ORA ?
            0xB0 => ora(self, Register::B),
            0xB1 => ora(self, Register::C),
            0xB2 => ora(self, Register::D),
            0xB3 => ora(self, Register::E),
            0xB4 => ora(self, Register::H),
            0xB5 => ora(self, Register::L),
            0xB6 => ora(self, Register::M),
            0xB7 => ora(self, Register::A),

            // CMP ?
            0xB8 => cmp(self, Register::B),
            0xB9 => cmp(self, Register::C),
            0xBA => cmp(self, Register::D),
            0xBB => cmp(self, Register::E),
            0xBC => cmp(self, Register::H),
            0xBD => cmp(self, Register::L),
            0xBE => cmp(self, Register::M),
            0xBF => cmp(self, Register::A),

            // POP ?
            0xC1 => pop(self, Register::B),
            0xD1 => pop(self, Register::D),
            0xE1 => pop(self, Register::H),
            0xF1 => pop(self, Register::PSW),

            // PUSH ?
            0xC5 => push(self, Register::B),
            0xD5 => push(self, Register::D),
            0xE5 => push(self, Register::H),
            0xF5 => push(self, Register::PSW),

            // DAD ?
            0x09 => dad(self, Register::B),
            0x19 => dad(self, Register::D),
            0x29 => dad(self, Register::H),
            0x39 => dad(self, Register::SP),

            // INX ?
            0x03 => inx(self, Register::B),
            0x13 => inx(self, Register::D),
            0x23 => inx(self, Register::H),
            0x33 => inx(self, Register::SP),

            // DCX ?
            0x0B => dcx(self, Register::B),
            0x1B => dcx(self, Register::D),
            0x2B => dcx(self, Register::H),
            0x3B => dcx(self, Register::SP),

            // STAX ?
            0x02 => stax(self, Register::B),
            0x12 => stax(self, Register::D),

            // LDAX ?
            0x0A => ldax(self, Register::B),
            0x1A => ldax(self, Register::D),

            // Instructions without registers
            // ADI d8
            0xC6 => adi(self),
            // SUI d8
            0xD6 => sui(self),
            // ANI d8
            0xE6 => ani(self),
            // ORI d8
            0xF6 => ori(self),
            // ACI d8
            0xCE => aci(self),
            // SBI d8
            0xDE => sbi(self),
            // XRI d8
            0xEE => xri(self),
            // CPI d8
            0xFE => cpi(self),

            // Rotate accumulator
            0x07 => rlc(self),
            0x0F => rrc(self),
            0x17 => ral(self),
            0x1F => rar(self),

            // Decimal Adjustment Accumulator
            0x27 => daa(self),

            // Set carry
            0x37 => stc(self),

            // Complement accumulator
            0x2F => cma(self),

            // Complement carry
            0x3F => cmc(self),

            // Exchange registers
            0xEB => xchg(self),

            // Exchange stack
            0xE3 => xthl(self),

            // Load SP from H and L
            0xF9 => sphl(self),

            // Store accumulator direct a16
            0x32 => sta(self),

            // Load accumulator direct a16
            0x3A => lda(self),

            // Store H and L direct a16
            0x22 => shld(self),

            // Load H and L direct a16
            0x2A => lhld(self),

            // Load Program Counter
            0xE9 => pchl(self),

            // Jumps
            0xC3 => jmp(self),
            0xCB => jmp(self),
            0xDA => jc(self),  // Jump if carry
            0xD2 => jnc(self), // Jump if no carry
            0xCA => jz(self),  // Jump if zero
            0xC2 => jnz(self), // Jump if not zero
            0xFA => jm(self),  // Jump if minus
            0xF2 => jp(self),  // Jump if positive
            0xEA => jpe(self), // Jump if parity even
            0xE2 => jpo(self), // Jump if parity odd

            // Calls
            0xCD => call(self),
            0xDD => call(self),
            0xED => call(self),
            0xFD => call(self),

            0xDC => cc(self),  // Call if carry
            0xD4 => cnc(self), // Call if no carry
            0xCC => cz(self),  // Call if zero
            0xC4 => cnz(self), // Call if not zero
            0xFC => cm(self),  // Call if minus
            0xF4 => cp(self),  // Call if plus
            0xEC => cpe(self), // Call if parity even
            0xE4 => cpo(self), // Call if parity odd

            // Returns
            0xC9 => ret(self),
            0xD9 => ret(self),

            0xD8 => rc(self),  // Return if carry
            0xD0 => rnc(self), // Return if no carry
            0xC8 => rz(self),  // Return if zero
            0xC0 => rnz(self), // Return if not zero
            0xF8 => rm(self),  // Return if minus
            0xF0 => rp(self),  // Return if plus
            0xE8 => rpe(self), // Return if parity even
            0xE0 => rpo(self), // Return if parity odd

            // Halt
            0x76 => self.hlt(),

            // IO
            0xD3 => output(self),
            0xDB => input(self),

            // Interrupts
            0xF3 => di(self),
            0xFB => ei(self),

            // Restarts
            0xC7 => rst(self, 0),
            0xCF => rst(self, 1),
            0xD7 => rst(self, 2),
            0xDF => rst(self, 3),
            0xE7 => rst(self, 4),
            0xEF => rst(self, 5),
            0xF7 => rst(self, 6),
            0xFF => rst(self, 7),

            byte => {
                panic!("Unknown OP: 0x{:02X?}", byte);
            }
        };
        Some(cycles)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_byte_returns_byte_and_increases_pc() {
        let mut cpu = CPU {
            memory: vec![0x01, 0x02],
            pc: 0,
            ..CPU::default()
        };

        let byte = cpu.read_byte();
        assert_eq!(byte, Some(0x01));
        assert_eq!(cpu.pc, 1);

        let byte = cpu.read_byte();
        assert_eq!(byte, Some(0x02));
        assert_eq!(cpu.pc, 2);

        let byte = cpu.read_byte();
        assert_eq!(byte, None);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn read_byte_from_stack_returns_byte_and_increases_sp() {
        let mut cpu = CPU {
            memory: vec![0x01, 0x02],
            sp: 0,
            ..CPU::default()
        };

        let byte = cpu.read_byte_from_stack();
        assert_eq!(byte, Some(0x01));
        assert_eq!(cpu.sp, 1);

        let byte = cpu.read_byte_from_stack();
        assert_eq!(byte, Some(0x02));
        assert_eq!(cpu.sp, 2);

        let byte = cpu.read_byte_from_stack();
        assert_eq!(byte, None);
        assert_eq!(cpu.sp, 2);
    }

    #[test]
    fn read_address_returns_double_and_increases_pc() {
        let mut cpu = CPU {
            memory: vec![0x01, 0x02, 0x03, 0x04],
            pc: 0,
            ..CPU::default()
        };

        let address = cpu.read_address();
        assert_eq!(address, Some(0x0201));
        assert_eq!(cpu.pc, 2);

        let address = cpu.read_address();
        assert_eq!(address, Some(0x0403));
        assert_eq!(cpu.pc, 4);

        let address = cpu.read_address();
        assert_eq!(address, None);
    }

    #[test]
    fn write_byte_to_stack_writes_byte_and_decrements_pc() {
        let mut cpu = CPU {
            memory: vec![0x01, 0x02, 0x03],
            sp: 3,
            ..CPU::default()
        };

        cpu.write_byte_to_stack(0xFF);
        assert_eq!(cpu.memory, vec![0x01, 0x02, 0xFF]);
        assert_eq!(cpu.sp, 2);

        cpu.write_byte_to_stack(0xFF);
        assert_eq!(cpu.memory, vec![0x01, 0xFF, 0xFF]);
        assert_eq!(cpu.sp, 1);

        cpu.write_byte_to_stack(0xFF);
        assert_eq!(cpu.memory, vec![0xFF, 0xFF, 0xFF]);
        assert_eq!(cpu.sp, 0);
    }

    #[test]
    fn get_frame_returns_whole_frame_buffer() {
        let mut cpu = CPU {
            memory: vec![0x00; 0x4000],
            ..CPU::default()
        };

        let frame = cpu.get_frame();

        assert_eq!(frame.len(), 224 * 256 / 8);
    }
}
