//! Module exposing mechanisms linked to memory
use std::default;
use std::fmt::Display;
use super::display::{Sprite};
use super::font::FONT_SET;

const STACK_SIZE: u8 = 16;
const RAM_SIZE: usize = 0x1000; // 4096

const FONTS_BASE_ADDR: usize = 0x000; // Base adress for fonts in RAM
//const FONTS_LIM_ADDR: usize = 0x04F;
const ROM_BASE_ADDR: usize = 0x200; // Base adress for ROM in RAM

#[derive(Default)]
/// A set of registers, likely to be owned by a CPU
pub struct Registers {
    // Generl purpose regs, which can be written to and read from (VF is not accessible from programs though)
    v: [u8; 16],
    // A special adress holding reg (as RAM is 4KB, only the 12 lower bits are used (max value = 4095))
    i: u16,
    // CPU private regs
    pc: u16, // Program Counter
    // Timer registers: they are decremented at a 60Hz rate
    dt: u8, // Delay Timer
    st: u8, // Sound timer -> active whenever it's not 0
}

#[derive(Default)]
pub struct Stack {
    vec: Vec<u16>, // Default: all 0
    sp: u8, // Default: 0 -> empty stack
}

impl Stack {
    pub fn push(&mut self, val: u16) -> Option<()>/*FULL*/ {
        self.vec[self.sp as usize] = val;
        if self.sp+1 > 16 {
            return None;
        }
        self.sp+=1;
        Some(())
    }

    pub fn pop(&mut self) -> Option<()> {
        if self.sp > 1 {
            return None;
        }
        self.sp+=1;
        return Some(())
    }
}

#[derive(Debug)]
/// Main memory unit
pub struct Mem {
    ram: [u8; RAM_SIZE], // Main RAM
    rom: Vec<u8>,        // Embedded instructions, wich will be included in RAM
}

impl Mem {
    /// Creates a new Mem context with a given embedded rom
    pub fn new(rom: Vec<u8>) -> Self {
        let mut mem = Self {
            ram: [0; RAM_SIZE],
            rom,
        };
        mem.reset(); // Puts the loaded (embedded) rom into ram
        mem
    }

    /// Load a (new) custom rom into the mem context (for "rom switching") 
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.rom = rom;
    }

    /// (Re-)sets mem: initializes rom and fonts into ram
    pub fn reset(&mut self) {
        // Sets rom -> loads the embedded rom into the actual ram
        for (i, byte) in self.rom.iter().enumerate() {
            if i+ROM_BASE_ADDR<RAM_SIZE { // Still RAM available
                self.ram[ROM_BASE_ADDR + i] = *byte;
            } else {
                break; // RAM is full, the entire ROM can't be placed in
            }
        }
        //Sets fonts
        for (i, byte) in FONT_SET.iter().enumerate() {
            self.ram[FONTS_BASE_ADDR + i] = *byte;
        }
    }

    /// Reads the byte at the given address in ram
    pub fn read_byte(&self, addr: usize) -> u8 {
        self.ram[addr]
    }

    /// Writes over the byte at the given address in ram
    pub fn write_byte(&mut self, addr: usize, val: u8) {
        self.ram[addr] = val;
    }
    
    /// Reads a complete word (2-byte in CHIP-8) from ram beginning at addr
    // Two consecutive regs (addr and addr + 1) are OR'd and yield a new u16
    pub fn read_word(&self, addr: usize) -> u16 {
        ((self.ram[addr] as u16) << 8) | (self.ram[addr+1] as u16)
    }

    // Shouldn't be useful, as words are instructions and are supposed to be only read
    pub fn write_word(&mut self, addr: usize, val: u16) {
        let mask = 0b00000000;
        let a = ((val>>8) | mask) as u8;
        let b = ((val<<8>>8) | mask) as u8;
        self.ram[addr] = a ;
        self.ram[addr+1] = b;
    }
}

#[cfg(test)]
mod tests {
    use crate::chip8::font::FONT_SET;

    use super::{Mem};

    fn mem_setup() -> Mem {
        Mem::new(vec![])
    }

    fn mem_setup_filled(rom: Vec<u8>) -> Mem{
        Mem::new(rom)
    }

    #[test]
    fn mem_load_rom() {
        let mut mem = mem_setup();
        mem.load_rom(vec![4, 4, 3, 4]);
        assert_eq!(mem.rom, vec![4, 4, 3, 4]);
    }

    #[test]
    fn mem_set() {
        let mem = mem_setup(); // Should take care of putting an empty vec in ram (for rom) and putting fonts at head
        let should = &FONT_SET[..];
        assert_eq!(should, &mem.ram[..80]);
    }

    #[test]
    fn word_read_valid() {
        let mem = mem_setup_filled(vec![4, 4, 3, 4]);
        assert_eq!(mem.read_word(0), 61584);
    }

    #[test]
    #[should_panic]
    fn word_read_invalid() {
        let invalid_addr = 4096;
        let mem = mem_setup_filled(vec![4, 4, 3, 4]); 
        assert_eq!(mem.read_word(invalid_addr), 61584);
    }

    #[test]
    fn word_write_valid() {
        let mut mem = mem_setup(); // We don't care about rom here (empty)
        let val: u16 = 0b1000001000000011;
        println!("{}", val);
        mem.write_word(0, val);
        println!("{:?}", mem.ram);
    }

    #[test]
    #[should_panic]
    fn word_write_invalid() {
        let invalid_addr = 4096;
        let mut mem = mem_setup(); // We don't care about rom here (empty)
        let val: u16 = 0b1000001000000011;
        println!("{}", val);
        mem.write_word(invalid_addr, val);
        println!("{:?}", mem.ram);
    }
}
