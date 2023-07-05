use core::panic;

use super::display::{Vram, SPRITE_MAX_SIZE, VRAM_DEF, VRAM_HEIGHT, VRAM_WIDTH, Sprite};
use super::memory::{self, Mem, Registers, Stack};
use rand::{self, Rng};

#[derive(Debug)]
pub struct CPU {
    // Some useful registers
    registers: Registers,
    // A stack
    stack: Stack, // independant from main ram
    // Mem
    mem: Mem,
    // Vram
    vram: Vram,
    //keypad: Keypad,
}

impl CPU {
    pub fn new(mem: Mem) -> Self {
        let mut cpu = Self {
            registers: Registers::default(),
            stack: Stack::default(),
            vram: Vram::default(),
            mem,
        };
        cpu.reset(); // Just for mem and pc reinit.
        cpu
    }

    pub fn reset(&mut self) {
        self.registers = Registers::default();
        self.stack = Stack::default();
        self.vram = Vram::default();
        self.mem.reset();
        self.registers = Registers {
            pc: memory::ROM_BASE_ADDR as u16,
            ..Default::default() // other are default
        } // Ram gets reinitialized (rom, fonts), as pc
    }

    // Provide no load_rom method, as this will be redundant (parent chip8 will though, for practical use)

    pub fn fetch(&self) -> Option<u16> {
        let pc = self.registers.pc as usize;
        self.mem.read_word(pc)
    }

    fn bad_opcode(&self, opcode: u16) {
        panic!("Received an invalid opcode in source code: {:X?}", opcode);
    }

    pub fn execute(&mut self) {
        // Main exec routine
        let instruction = self.fetch().expect("Out of bounds word reading"); // We get the

        //Nibbling
        let nnn = instruction & 0x0FFF;
        let kk = (instruction & 0x00FF) as u8;
        let n = (instruction & 0x000F) as u8; // 4 bits are unused
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let y = ((instruction & 0x00F0) >> 4) as usize;

        if x > 0xF || y > 0xF {
            // Vx regs are 16 long
            panic!("Ill-formed given x and y indexes...");
        }

        let first_bit = ((instruction & 0xF000) >> 12) as u8;
        //println!("{}", first_bit);
        //(nnn, kk, n, x, y)

        match first_bit {
            0x0 => match kk {
                0xE0 => self.vram.clear(), // CLEAR screen

                0xEE => {
                    // RET
                    self.registers.pc = self
                        .stack
                        .pop()
                        .expect("Stack was empty already, ill-formed function nesting")
                }

                any => self.bad_opcode(any as u16),
            },

            0x1 => self.registers.pc = nnn, // JP addr

            0x2 => {
                // CALL addr
                self.stack
                    .push(self.registers.pc) // save current pc
                    .expect("Program tried to overflow its stack");
                self.registers.pc = nnn; // JP
            }
            0x3 => {
                // SKIP if Vx == kk
                if self.registers.v[x] == kk {
                    self.registers.pc += 2;
                }
            }
            0x4 => {
                // SKIP if Vx != kk
                if self.registers.v[x] != kk {
                    self.registers.pc += 2;
                }
            }
            0x5 => {
                // SKIP if Vx == Vy
                if self.registers.v[x] == self.registers.v[y] {
                    self.registers.pc += 2;
                }
            }
            0x6 => self.registers.v[x] = kk,
            0x7 => self.registers.v[x] = (self.registers.v[x] as u16 + kk as u16) as u8, // Removes overflow
            0x8 => {
                // Op 8 instructions
                match n {
                    0x0 => self.registers.v[x] = self.registers.v[y],
                    0x1 => self.registers.v[x] |= self.registers.v[y], // Vx OR Vy
                    0x2 => self.registers.v[x] &= self.registers.v[y], // Vx AND Vy
                    0x3 => self.registers.v[x] ^= self.registers.v[y], // Vx XOR Vy
                    0x4 => {
                        // Vx += Vy, VF = carry
                        let addition = self.registers.v[x] as u16 + self.registers.v[y] as u16; // We need another variable for u8 overflow checking
                        self.registers.v[x] = addition as u8; // Removes overflow
                        self.registers.v[0xF] = (addition > 0xFF/*255*/) as u8;
                    }
                    0x5 => {
                        // Wrapping substraction, VF = BORROW
                        self.registers.v[0xF] = (self.registers.v[x] > self.registers.v[y]) as u8;
                        self.registers.v[x] = self.registers.v[x].wrapping_sub(self.registers.v[y]);
                    }
                    0x6 => {
                        // VF = Vx LSb, Vx /= 2
                        self.registers.v[0xF] = self.registers.v[x] & 0b1; // LSb
                        self.registers.v[x] /= 2;
                    }
                    0x7 => {
                        // Wrapping substraction, VF = BORROW
                        self.registers.v[0xF] = (self.registers.v[y] > self.registers.v[x]) as u8;
                        self.registers.v[x] = self.registers.v[y].wrapping_sub(self.registers.v[x]);
                    }
                    0xE => {
                        // VF = Vx MSb, Vx *= 2
                        self.registers.v[0xF] = self.registers.v[x] & 0b10000000; // MSb
                        self.registers.v[x] = (self.registers.v[x] as u16 * 2_u16) as u8;
                        // Removes overflow
                    }
                    any => self.bad_opcode(any as u16),
                }
            }
            0x9 => {
                // SKIP if Vx != Vy
                if self.registers.v[x] != self.registers.v[y] {
                    self.registers.pc += 2;
                }
            }
            0xA => self.registers.i = nnn, // Set i = nnn
            0xB => self.registers.pc = nnn + (self.registers.v[0] as u16), // Set pc = V0 + nnn
            0xC => {
                // Vx = rand AND kk
                let random: u8 = rand::thread_rng().gen(); // 0-255
                self.registers.v[x] = random & kk;
            }
            0xD => {
                let coords = (self.registers.v[x], self.registers.v[y]);
                let sprite_bytes = self.mem.read_segment(n as usize, self.registers.i as usize).expect("Segemnt is not contained in ram (entirely)");
                let sprite = Sprite::try_from(sprite_bytes).expect("Sprite data size is invalid");

                
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use super::memory::ROM_BASE_ADDR;
    use super::CPU;
    use super::{VRAM_HEIGHT, VRAM_WIDTH};
    use crate::chip8::display::VRAM_DEF;
    use crate::chip8::memory::Mem;

    fn cpu_setup() -> CPU {
        CPU::new(Mem::new(Vec::from([1, 2, 3, 4]))) // Main setup with all default, but mem's rom (and ram) is filled with 4 bytes
    }

    #[test]
    fn cpu_new() {
        let cpu = cpu_setup();
        assert_eq!(cpu.vram.inner(), VRAM_DEF);
        println!("{:?}", cpu.mem);
        println!("{:?}", cpu.registers);
        println!("{:?}", cpu.stack);
        let rom = [1, 2, 3, 4];
        let mut basket = Vec::with_capacity(4);
        for i in 0..4 {
            basket.push(cpu.mem.read_byte(ROM_BASE_ADDR + i).unwrap());
        }
        assert_eq!(Vec::from(rom), basket);
    }

    #[test]
    fn cpu_reset() {
        let mut cpu = cpu_setup();
        cpu.reset();
        assert_eq!(cpu.mem.rom, Vec::from([1, 2, 3, 4]));
    }

    #[test]
    fn cpu_fetch() {
        let mut cpu = cpu_setup();
        let instr = cpu.fetch().unwrap();
        assert_eq!(instr, 258);
        cpu.registers.pc += 1;
        let instr = cpu.fetch().unwrap();
        assert_eq!(instr, 515);
    }

    //#[test]
    //fn cpu_execute() {
    //    let mut cpu = cpu_setup(); // rom: [1, 2, 3, 4]
    //    //word(1, 2) = 258 (u16)
    //    cpu.execute();
    //}
}
