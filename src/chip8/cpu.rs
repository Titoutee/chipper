use std::arch::x86_64::CpuidResult;

use super::memory::{Registers, Mem, Stack};

pub struct CPU {
    // Some registers
    registers: Registers,

    // A stack
    stack: Stack, // independant from main ram

    // Mem
    mem: Mem,

    // VRAM
    //vram: Vram,

    //Keypad
    //keypad: Keypad,
}

impl CPU {
    pub fn new(mem: Mem) -> Self {
        let mut cpu = Self {
            registers: Registers::default(),
            stack: Stack::default(),
            mem,
        };
        cpu.reset(); // Main use 
        cpu
    }

    pub fn reset(&mut self) {
        self.registers = Registers::default();
        self.stack = Stack::default();
        self.mem.reset();
    }

    pub fn execute() {

    }
}