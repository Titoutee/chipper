use core::panic;
use std::time::{Instant, Duration};

use super::display::{Sprite, Vram};
use super::font::FONT_UNIT_SIZE;
use super::input::KeyBoard;
use super::memory::{self, Mem, Registers, Stack, FONTS_BASE_ADDR, RAM_SIZE};
use rand::{self, Rng};

const TIMER_EPSILON: u64 = 16; // Appr. 60 Hz if expressed in ms

#[derive(Debug)]
pub struct CPU {
    // Some useful registers
    registers: Registers,
    last_timer_change: Instant,
    // A stack
    stack: Stack, // independant from main ram
    // Mem
    mem: Mem,
    // Vram
    vram: Vram,
    vram_changed: bool,
}

pub enum CpuState {
    Normal,
    Error(String),
    Finished,
}

impl CPU {
    pub fn new(mem: Mem) -> Self {
        let mut cpu = Self {
            last_timer_change: Instant::now(),
            registers: Registers::default(),
            stack: Stack::default(),
            vram: Vram::default(),
            mem,
            vram_changed: false,
        };
        cpu.reset(); // Just for mem and pc reinit.
        cpu
    }

    pub fn vram_changed(&self) -> bool {
        self.vram_changed
    }

    pub fn vram(&self) -> &Vram {
        &self.vram
    }

    pub fn reset(&mut self) {
        self.registers = Registers::default();
        self.last_timer_change = Instant::now();
        self.stack = Stack::default();
        self.vram = Vram::default();
        self.mem.reset();
        self.vram_changed = false;
        self.registers = Registers {
            pc: memory::ROM_BASE_ADDR as u16,
            ..Default::default() // other are default
        } // Ram gets reinitialized (rom, fonts), as pc
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.mem.load_rom(rom);
    }
    pub fn tick(&mut self, kb: &KeyBoard) -> CpuState {
        self.vram_changed = false;
        if self.registers.pc as usize >= RAM_SIZE {
            return CpuState::Finished;
        }
        let instruction = self
            .fetch(self.registers.pc)
            .expect("Out of bounds word reading");
        if Instant::now() - self.last_timer_change >= Duration::from_millis(TIMER_EPSILON) {
            self.decrease_delaytimer();
            self.decrease_soundtimer();
            self.last_timer_change = Instant::now();
        }
        self.execute(instruction, kb)
    }

    pub fn fetch(&self, pc: u16) -> Option<u16> {
        self.mem.read_word(pc as usize)
    }

    pub fn decrease_soundtimer(&mut self) {
        if (self.registers.st) != 0 {
            self.registers.st -= 1;
        }
    }

    pub fn decrease_delaytimer(&mut self) {
        if (self.registers.dt) != 0 {
            self.registers.dt -= 1;
        }
    }

    pub fn execute(&mut self, instruction: u16, kb: &KeyBoard) -> CpuState {
        //Nibbling
        let nnn = instruction & 0x0FFF;
        let kk = (instruction & 0x00FF) as u8;
        let n = (instruction & 0x000F) as u8; // 4 bits are unused
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let y = ((instruction & 0x00F0) >> 4) as usize;

        let first_bit = ((instruction & 0xF000) >> 12) as u8;

        match first_bit {
            0x0 => match nnn {
                0x0E0 => {
                    self.vram.clear(); // CLEAR screen
                    self.registers.pc += 2;
                }

                0x0EE => {
                    // RET
                    self.registers.pc = self
                        .stack
                        .pop()
                        .expect("Stack was empty already, ill-formed function nesting")
                }

                _ => {
                    return CpuState::Error(format!(
                        "Received an invalid opcode in source code: {:04X?}",
                        instruction
                    ))
                } // 0nnn is not supported by modern interpreters, and thus is not implemented on pupose here
            },

            0x1 => self.registers.pc = nnn, // JP addr

            0x2 => {
                // CALL addr
                self.stack
                    .push(self.registers.pc + 2) // save current pc
                    .expect("Program tried to overflow its stack");
                self.registers.pc = nnn; // JP
            }
            0x3 => {
                // SKIP if Vx == kk
                if self.registers.v[x] == kk {
                    self.registers.pc += 4;
                } else {
                    self.registers.pc += 2;
                }
            }
            0x4 => {
                // SKIP if Vx != kk
                if self.registers.v[x] != kk {
                    self.registers.pc += 4;
                } else {
                    self.registers.pc += 2;
                }
            }
            0x5 => {
                // SKIP if Vx == Vy
                if self.registers.v[x] == self.registers.v[y] {
                    self.registers.pc += 4;
                } else {
                    self.registers.pc += 2;
                }
            }
            0x6 => {
                self.registers.v[x] = kk;
                self.registers.pc += 2;
            }
            0x7 => {
                self.registers.v[x] = (self.registers.v[x] as u16 + kk as u16) as u8;
                self.registers.pc += 2;
            }
            0x8 => {
                // Op 8 instructions
                match n {
                    0x0 => self.registers.v[x] = self.registers.v[y],
                    0x1 => self.registers.v[x] |= self.registers.v[y], // Vx OR Vy
                    0x2 => self.registers.v[x] &= self.registers.v[y], // Vx AND Vy
                    0x3 => self.registers.v[x] ^= self.registers.v[y], // Vx XOR Vy
                    0x4 => {
                        // Vx += Vy, VF = carry
                        let addition = self.registers.v[x] as u16 + self.registers.v[y] as u16;
                        self.registers.v[x] = addition as u8;
                        self.registers.v[0xF] = (addition > 0xFF/*255*/) as u8;
                    }
                    0x5 => {
                        // Wrapping substraction, VF = BORROW
                        let of = if self.registers.v[x] >= self.registers.v[y] {1} else {0};
                        self.registers.v[x] = self.registers.v[x].wrapping_sub(self.registers.v[y]);
                        self.registers.v[0xF] = of;
                    }
                    0x6 => {
                        // VF = Vx LSb, Vx /= 2
                        let lsb = self.registers.v[x] & 0b1; // LSb
                        self.registers.v[x] = self.registers.v[y]>>1;
                        self.registers.v[0xF] = lsb
                    }
                    0x7 => {
                        // Wrapping substraction, VF = BORROW
                        let of = if self.registers.v[y] >= self.registers.v[x] {1} else {0};
                        self.registers.v[x] = self.registers.v[y].wrapping_sub(self.registers.v[x]);
                        self.registers.v[0xF] = of;
                    }
                    0xE => {
                        // VF = Vx MSb, Vx *= 2
                        let msb = (self.registers.v[x]>>7) & 0b1; // MSb
                        self.registers.v[x] = self.registers.v[y]<<1;
                        self.registers.v[0xF] = msb;
                    }
                    _ => {
                        return CpuState::Error(format!(
                            "Received an invalid opcode in source code: {:04X?}",
                            instruction
                        ))
                    }
                }
                self.registers.pc += 2;
            }
            0x9 => {
                // SKIP if Vx != Vy
                if self.registers.v[x] != self.registers.v[y] {
                    self.registers.pc += 4;
                } else {
                    self.registers.pc += 2;
                }
            }
            0xA => {
                self.registers.i = nnn; // Set i = nnn
                self.registers.pc += 2;
            }
            0xB => self.registers.pc = nnn + (self.registers.v[0] as u16), // Set pc = V0 + nnn
            0xC => {
                // Vx = rand AND kk
                let random: u8 = rand::thread_rng().gen(); // 0-255
                self.registers.v[x] = random & kk;
                self.registers.pc += 2;
            }
            0xD => {
                let (x, y) = (self.registers.v[x], self.registers.v[y]);
                let sprite_bytes = self
                    .mem
                    .read_segment(n as usize, self.registers.i as usize)
                    .expect("Segment is not contained in RAM (entirely)");
                let sprite = Sprite::try_from(sprite_bytes).expect("Sprite data size is invalid");
                self.registers.v[0xF] = self.vram.put_sprite(sprite, x.into(), y.into());
                self.vram_changed = true;
                self.registers.pc += 2;
            }
            0xE => match kk {
                0x9E => {
                    let key = self.registers.v[x];
                    if kb.is_key_pressed(key) {
                        self.registers.pc += 4;
                    } else {
                        self.registers.pc += 2;
                    }
                }
                0xA1 => {
                    let key = self.registers.v[x];
                    if kb.is_key_up(key) {
                        self.registers.pc += 4;
                    } else {
                        self.registers.pc += 2;
                    }
                }
                _ => {
                    return CpuState::Error(format!(
                        "Received an invalid opcode in source code: {:04X?}",
                        instruction
                    ))
                }
            },
            0xF => match kk {
                0x07 => {
                    self.registers.v[x] = self.registers.dt;
                    self.registers.pc += 2
                }
                0x0A => {
                    if let Some(key) = kb.get_key_pressed() {
                        self.registers.v[x] = key;
                        self.registers.pc += 2;
                    }
                }
                0x15 => {
                    self.registers.dt = self.registers.v[x];
                    self.registers.pc += 2
                }
                0x18 => {
                    self.registers.st = self.registers.v[x];
                    self.registers.pc += 2
                }
                0x1E => {
                    self.registers.i = self.registers.i as u16 + self.registers.v[x] as u16;
                    self.registers.pc += 2;
                }
                0x29 => {
                    self.registers.i = FONTS_BASE_ADDR as u16
                        + (self.registers.v[x] as u16 * FONT_UNIT_SIZE as u16);
                    self.registers.pc += 2
                }
                0x33 => {
                    let copy = self.registers.v[x];

                    self.mem.write_byte((self.registers.i) as usize, copy / 100);
                    self.mem
                        .write_byte((self.registers.i + 1) as usize, (copy % 100) / 10);
                    self.mem
                        .write_byte((self.registers.i + 2) as usize, copy % 10);

                    self.registers.pc += 2
                }
                0x55 => {
                    for off in 0..=x {
                        self.mem.write_byte(
                            (self.registers.i + off as u16) as usize,
                            self.registers.v[off as usize],
                        );
                    }
                    self.registers.i += x as u16 + 1;
                    self.registers.pc += 2
                }
                0x65 => {
                    for off in 0..=x {
                        self.registers.v[off] = self
                            .mem
                            .read_byte((self.registers.i + off as u16) as usize)
                            .unwrap();
                    }
                    self.registers.i += x as u16 + 1;
                    self.registers.pc += 2
                }
                _ => {
                    return CpuState::Error(format!(
                        "Received an invalid opcode in source code: {:04X?}",
                        instruction
                    ))
                }
            },
            _ => {
                return CpuState::Error(format!(
                    "Received an invalid opcode in source code: {:04X?}",
                    instruction
                ))
            }
        }
        CpuState::Normal
    }
}

#[cfg(test)]
mod test {
    use super::memory::ROM_BASE_ADDR;
    use super::CPU;
    use crate::chip8::display::VRAM_DEFAULT;
    use crate::chip8::input::KeyBoard;
    use crate::chip8::memory::Mem;
    //use super::CpuState;

    fn cpu_setup() -> CPU {
        CPU::new(Mem::new(Vec::from([1, 2, 3, 4]))) // Main setup with all default, but mem's rom (and ram) is filled with 4 bytes
    }

    #[test]
    fn cpu_new() {
        let cpu = cpu_setup();
        assert_eq!(cpu.vram.inner(), VRAM_DEFAULT);
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
        assert_eq!(cpu.mem.rom(), &Vec::from([1, 2, 3, 4]));
    }

    #[test]
    fn cpu_fetch() {
        let mut cpu = cpu_setup();
        let instr = cpu.fetch(cpu.registers.pc).unwrap();
        assert_eq!(instr, 258);
        cpu.registers.pc += 1;
        let instr = cpu.fetch(cpu.registers.pc).unwrap();
        assert_eq!(instr, 515);
    }

    #[test]
    fn nibbling() {
        let mut cpu = cpu_setup();
        cpu.execute(0xF0E0, &KeyBoard::new());
    }
}
