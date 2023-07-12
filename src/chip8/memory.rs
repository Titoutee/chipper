//! Module exposing mechanisms linked to memory
use super::font::FONT_SET;

const STACK_SIZE: usize = 16;
pub const RAM_SIZE: usize = 0x1000; // 4096

pub const FONTS_BASE_ADDR: usize = 0x000; // Base adress for fonts in RAM
pub const ROM_BASE_ADDR: usize = 0x200; // Base adress for ROM in RAM

#[derive(Debug, Default)]
/// A set of registers, likely to be owned by a CPU
pub struct Registers {
    // General purpose regs, which can be written to and read from (VF is not accessible from programs though)
    pub v: [u8; 16],
    // A special adress holding reg (as RAM is 4KB, only the 12 lower bits are used (max value = 4095))
    pub i: u16,
    // CPU private regs
    pub pc: u16, // Program Counter
    // Timer registers: they are decremented at a 60Hz rate
    pub dt: u8, // Delay Timer
    pub st: u8, // Sound timer -> active whenever it's not 0
}

#[derive(Debug)]
pub struct Stack {
    vec: Vec<u16>, // Default: all 0
}

impl Stack {
    pub fn push(&mut self, val: u16) -> Option<()>/*FULL*/ {
        if self.vec.len()+1 > STACK_SIZE {
            return None;
        }

        self.vec.push(val);
        Some(())
    }

    pub fn pop(&mut self) -> Option<u16> { // Responsability of the caller to handle the empty stack
        self.vec.pop()
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            vec: Vec::with_capacity(STACK_SIZE as usize),
            //sp: 0,
        }
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

    pub fn rom(&self) -> &[u8] {
        self.rom.as_slice()
    }

    /// Load a (new) custom rom into the mem context (for "rom switching") 
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.rom = rom;
        self.reset();
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
    pub fn read_byte(&self, addr: usize) -> Option<u8> {
        if addr >= RAM_SIZE {
            return None
        }
        Some(self.ram[addr])
    }

    /// Writes over the byte at the given address in ram
    pub fn write_byte(&mut self, addr: usize, val: u8) {
        if addr < RAM_SIZE {
            self.ram[addr] = val;
        }
    }

    pub fn read_segment(&self, n_bytes: usize, addr: usize) -> Option<Vec<u8>> {
        let mut segment = Vec::<u8>::new();
        for offset in 0..n_bytes {
            segment.push(self.read_byte(addr+offset)?);
        }
        Some(segment)
    }  
    
    /// Reads a complete word (2-byte in CHIP-8) from ram beginning at addr
    // Two consecutive regs (addr and addr + 1) are OR'd and yield a new u16
    pub fn read_word(&self, addr: usize) -> Option<u16> {
        if addr >= RAM_SIZE || addr + 1 >= RAM_SIZE {
            return None
        }
        Some(((self.ram[addr] as u16) << 8) | (self.ram[addr+1] as u16))
    }
}

#[cfg(test)]
mod tests {
    use crate::chip8::font::FONT_SET;

    use super::{Mem, Stack};

    #[test]
    fn stack_push_valid() {
        let mut stack = Stack::default();
        stack.push(1).unwrap();
    }

    #[test]
    #[should_panic]
    fn stack_push_valid_invalid() {
        let mut stack = Stack::default();
        let very_high_lim = 100;
        for _ in 0..very_high_lim {
            stack.push(1).unwrap();
        }
    }

    #[test]
    fn stack_pop_valid() {
        let mut stack = Stack::default();
        stack.push(1).unwrap();
        //test
        let popped = stack.pop().unwrap();
        assert_eq!(popped, 1);
    }

    #[test]
    #[should_panic]
    #[allow(unused)]
    fn stack_pop_invalid() {
        let mut stack = Stack::default();
        let popped = stack.pop().unwrap();
    }

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
        assert_eq!(mem.read_word(0).expect("Out of bounds"), 61584);
    }

    #[test]
    #[should_panic]
    fn word_read_invalid() {
        let invalid_addr = 4096;
        let mem = mem_setup_filled(vec![4, 4, 3, 4]); 
        assert_eq!(mem.read_word(invalid_addr).expect("out of bounds"), 61584);
    }

    #[test]
    fn word_read_segment_valid() {
        let mem = mem_setup_filled(vec![4, 4, 3, 4]);
        assert_eq!(mem.read_segment(5, 4000).unwrap(), vec![0, 0, 0, 0, 0]);
    }

    #[test]
    #[should_panic]
    fn word_read_segment_invalid() {
        let mem = mem_setup_filled(vec![4, 4, 3, 4]);
        assert_eq!(mem.read_segment(5, 4095).unwrap(), vec![0, 0, 0, 0, 0]);
    }
}
