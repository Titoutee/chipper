

const RAM_SIZE: u16 = 4096;

pub struct Registers {
    v: [u8; 16], // General purpose registers (VF shouldn't be used by any program)
    i: u16, // Store memory adresses (only 12 lower bits are used thus)

    delay: u8,
    sound: u8,

    // Some private registers, not to be accessible by programs 
    pc: u16,
    sp: u8,
}




pub struct MMU {
    ram: [u8; RAM_SIZE],
}