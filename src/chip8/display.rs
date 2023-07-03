use std::usize;

pub const SPRITE_SIZE: usize = 15;
pub const VRAM_WIDTH: usize = 64;
pub const VRAM_HEIGHT: usize = 32;

pub type VramType = [[u16; VRAM_WIDTH]; VRAM_HEIGHT];
pub const VRAM_DEF: VramType = [[0; VRAM_WIDTH]; VRAM_HEIGHT];

pub struct Sprite {
    pub data: [u8; SPRITE_SIZE],
}

impl From<[u8; SPRITE_SIZE]> for Sprite {
    fn from(data: [u8; SPRITE_SIZE]) -> Self {
        Self { data }
    }
}

#[derive(Debug)]
pub struct Vram {
    arr: VramType,
}

impl Default for Vram {
    fn default() -> Self {
        Self { arr: VRAM_DEF}
    }
}

impl Vram {
    pub fn clear(&mut self) {
        self.arr = VRAM_DEF;
    }

    pub fn inner(&self) -> VramType {
        self.arr
    }
    
    pub fn get_line(&self, idx: usize) -> Option<&[u16]> {
        if idx >= VRAM_WIDTH {
            return None;
        }
        Some(&self.arr[idx])
    }
}

