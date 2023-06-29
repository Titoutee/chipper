use super::memory;

pub const WIDTH: u8 = 64;
pub const HEIGHT: u8 = 32;

pub struct CPU {
    // Some registers
    register: Registers,

    // A stack
    stack: [u16; 16],

    // Video Random Access Memory
    vram: [u16; HEIGHT*WIDTH],
}
