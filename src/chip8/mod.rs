pub mod cpu;
pub mod display;
pub mod font;
pub mod input;
pub mod memory;

use cpu::{CPU, CpuState};
use input::{get_key_opcode, KeyBoard};
use self::memory::Mem;
use minifb::Key;

pub struct Interpreter {
    cpu: CPU,
    keyboard: KeyBoard,
}

impl Interpreter {
    pub fn new() -> Self {
        let mem = Mem::new(vec![]);
        Self {
            cpu: CPU::new(mem),
            keyboard: KeyBoard::new(),
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.cpu.load_rom(rom);
    }

    pub fn feed_key(&mut self, key: Key) {
        self.keyboard.feed_key(key);
    }

    pub fn tick(&mut self) -> CpuState {
        self.cpu.tick()
    }
}
