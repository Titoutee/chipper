pub mod cpu;
pub mod display;
pub mod font;
pub mod input;
pub mod memory;

//use minifb::Key;
use cpu::{CPU, CpuState};
use input::KeyBoard;
use self::memory::Mem;

pub struct Interpreter {
    pub cpu: CPU,
    pub keyboard: KeyBoard,
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

    pub fn feed_key(&mut self, key: Option<u8>) {
        self.keyboard.feed_key(key);
    }

    pub fn tick(&mut self) -> CpuState {
        self.cpu.tick(&self.keyboard)
    }

    pub fn vram_changed(&self) -> bool {
        self.cpu.vram_changed()
    }
}
