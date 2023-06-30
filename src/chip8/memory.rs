//! Module exposing mechanisms linked to memory

use std::fmt::Display;
use super::display::{Sprite};
use super::font::FONT_SET;

const STACK_SIZE: u8 = 16;
const RAM_SIZE: usize = 0x1000; // 4096

const FONTS_BASE_ADDR: usize = 0x000; // Base adress for fonts in RAM
//const FONTS_LIM_ADDR: usize = 0x04F;
const ROM_BASE_ADDR: usize = 0x200; // Base adress for ROM in RAM
/// A set of registers, likely to be owned by a CPU
/// 
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

/// Main memory unit
pub struct Mem {
    ram: [u8; RAM_SIZE], // Main RAM
    rom: Vec<u8>,        // Instructions, wich will be included in RAM
}

impl Mem {
    pub fn new(rom: Vec<u8>) -> Self {
        let mem = Self {
            ram: [0; RAM_SIZE],
            rom,
        };
        mem
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.rom = rom;
    }

    /// Sets mem: puts and initializes rom and fonts into ram
    pub fn set(&mut self) {
        // Sets rom
        for (i, byte) in self.rom.iter().enumerate() {
            if i+ROM_BASE_ADDR<RAM_SIZE { // Still RAM available
                self.ram[ROM_BASE_ADDR + i] = *byte;
            } else {
                break; // RAM is full
            }
        }
        //Sets fonts
        for (i, byte) in FONT_SET.iter().enumerate() {
            self.ram[FONTS_BASE_ADDR + i] = *byte;
        }
    }

    pub fn read_byte(&self, addr: usize) -> u8 {
        self.ram[addr]
    }

    pub fn write_byte(&mut self, addr: usize, val: u8) {
        self.ram[addr] = val;
    }
    
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

/// Stack representation
pub struct Stack {
    arr: [u16; 16],
    sp: StackPointer,
}

impl Stack {
    pub fn push(&mut self, val: u16) {
        if self.sp.inc().is_some() {
            self.arr[self.sp.inner()] = val;
        }
    }

    pub fn pop(&mut self) -> Option<u16> {
        if self.sp.dec().is_some() {
            return Some(self.arr[self.sp.inner() + 1]);
        }
        None
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            arr: [0; 16],
            sp: StackPointer::default(),
        }
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}\nsp: {}", self.arr, self.sp.inner())
    }
}


/// Wrapper around a u8, mostly for bounds-checking
pub struct StackPointer {
    inner: u8,
}

impl StackPointer {
    pub fn inner(&self) -> usize {
        self.inner as usize
    }

    pub fn inc(&mut self) -> Option<()> {
        if self.inner <= STACK_SIZE - 1 {
            self.inner += 1;
            return Some(());
        }
        None
    }

    pub fn dec(&mut self) -> Option<()> {
        if self.inner > 0 {
            self.inner -= 1;
            return Some(());
        }
        None
    }
}

impl Default for StackPointer {
    fn default() -> Self {
        Self { inner: 0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::chip8::font::FONT_SET;

    use super::{Stack, Mem};

    #[test]
    fn stack_pop_and_push_valid() {
        let mut stack = Stack::default();
        stack.push(92);
        println!("{}", stack);
        let popped = stack.pop();
        println!("{}", popped.unwrap());
        println!("{}", stack);
    }

    #[test]
    #[should_panic]
    fn stack_pop_and_push_invalid() {
        let mut stack = Stack::default();
        //stack.push(92);
        println!("{}", stack);
        let popped = stack.pop();
        println!("{}", popped.unwrap());
        println!("{}", stack);
    }

    fn mem_setup() -> Mem{
        Mem::new(vec![])
    }

    #[test]
    fn mem_load_rom() {
        let mut mem = mem_setup();
        mem.load_rom(vec![4, 4, 3, 4]);
        assert_eq!(mem.rom, vec![4, 4, 3, 4]);
    }

    #[test]
    fn mem_set() {
        let mut mem = mem_setup();
        mem.load_rom(vec![4, 4, 3, 4]);
        mem.set();
        let should = &FONT_SET[..];
        println!("{:?}", should);
        assert_eq!(should, &mem.ram[..80]);
    }

    #[test]
    fn word_read() {
        let mut mem = mem_setup();
        mem.load_rom(vec![4, 4, 3, 4]);
        mem.set();
        assert_eq!(mem.read_word(0), 61584);
    }

    #[test]
    fn word_write() {
        let mut mem = mem_setup();
        let val: u16 = 0b1000001000000011;
        println!("{}", val);
        mem.write_word(0, val);
        println!("{:?}", mem.ram);
    }
}
