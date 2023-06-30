use super::memory::{Registers, Stack, Mem};

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
