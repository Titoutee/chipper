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


impl 