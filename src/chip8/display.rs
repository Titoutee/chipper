//! Display API
use std::usize;

pub const SPRITE_MAX_SIZE: usize = 15;
pub const VRAM_WIDTH: usize = 64;
pub const VRAM_HEIGHT: usize = 32;

pub type VramType = [[u16; VRAM_WIDTH]; VRAM_HEIGHT];
pub const VRAM_DEF: VramType = [[0; VRAM_WIDTH]; VRAM_HEIGHT];

pub struct Sprite {
    pub data: Vec<u8>,
    pub len: usize,
}

impl TryFrom<Vec<u8>> for Sprite {
    type Error = ();
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let len = value.len();
        if len < 1 || len > SPRITE_MAX_SIZE {
            return Err(());
        }
        Ok(Self { data: value, len })
    }
}

#[derive(Debug)]
pub struct Vram {
    arr: VramType,
}

impl Default for Vram {
    fn default() -> Self {
        Self { arr: VRAM_DEF }
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

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&u16> {
        if x < VRAM_WIDTH && y < VRAM_HEIGHT { // DO NOT wrap
            return Some(&self.arr[x][y]);
        }
        None
    }
    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut u16> {
        if x < VRAM_WIDTH && y < VRAM_HEIGHT { // DO NOT wrap
            return Some(&mut self.arr[x][y]);
        }
        None
    }
    
    pub fn set_pixel(&mut self, x: usize, y: usize) -> Option<bool> {
        let pixel_ref = self.get_pixel_mut(x, y)?;
        let on = if *pixel_ref != 0 {true} else {false};
        todo!();
        Some(on)
    }
}

#[cfg(test)]
mod tests {
    use super::Sprite;

    #[test]
    fn from() {}
}
