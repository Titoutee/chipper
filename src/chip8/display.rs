//! Display API

use super::utils::bits_from_u8;

pub const SPRITE_MAX_SIZE: usize = 15;
pub const VRAM_WIDTH: usize = 64;
pub const VRAM_HEIGHT: usize = 32;

pub type VramType = [[u8; VRAM_WIDTH]; VRAM_HEIGHT];
pub const VRAM_DEFAULT: VramType = [[0; VRAM_WIDTH]; VRAM_HEIGHT];

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

impl Sprite {
    pub fn to_bytes(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn to_bytes_iter(&self) -> impl Iterator<Item = &u8> {
        self.data.iter()
    }
}

#[derive(Debug)]
pub struct Vram {
    arr: VramType,
}

impl Default for Vram {
    fn default() -> Self {
        Self { arr: VRAM_DEFAULT }
    }
}

impl Vram {
    pub fn clear(&mut self) {
        self.arr = VRAM_DEFAULT;
    }

    pub fn inner(&self) -> VramType {
        self.arr
    }

    pub fn get_line_mut(&mut self, idx: usize) -> Option<&[u8]> {
        if idx >= VRAM_WIDTH {
            return None;
        }
        Some(&mut self.arr[idx])
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&u8> {
        if x < VRAM_WIDTH && y < VRAM_HEIGHT { // DO NOT wrap around
            return Some(&self.arr[y][x]);
        }
        None
    }
    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut u8> {
        if x < VRAM_WIDTH && y < VRAM_HEIGHT { // DO NOT wrap around
            return Some(&mut self.arr[y][x]);
        }
        None
    }
    
    pub fn set_pixel(&mut self, x: usize, y: usize, val: u8) -> Option<bool> {
        let pixel_ref = self.get_pixel_mut(x, y)?; // Propagate underlying pixel failures
        *pixel_ref = val;
        
        Some(true)
    }

    pub fn set_line(&mut self, byte: u8, idx: usize, val: u8) -> Option<bool> {
        let line = self.get_line_mut(idx)?;
        let bits = bits_from_u8(byte);
        for (r, have_to_print) in bits.iter().enumerate() {
            if *have_to_print {
                //self.set_pixel(x+r, idx, val)?;
            }
        }
        Some(true)
    }

    pub fn draw_sprite(&mut self, sprite: Sprite, x: usize, y: usize) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::{Vram};

    #[test]
    fn get_pixel_test_valid() {
        let vram = Vram::default();
        let pixel = vram.get_pixel(63, 31).unwrap();
        assert!(*pixel == 0);
    }

    #[test]
    #[should_panic]
    fn get_pixel_test_invalid() {
        let vram = Vram::default();
        let pixel = vram.get_pixel(64, 30).unwrap();
        assert!(*pixel == 0);
    }

    #[test]
    fn get_pixel_mut_test_valid() {
        let mut vram = Vram::default();
        let pixel = vram.get_pixel_mut(63, 31).unwrap();
        assert!(*pixel == 0);
        *pixel = 255;
    }

    #[test]
    #[should_panic]
    fn get_pixel_mut_test_invalid() {
        let mut vram = Vram::default();
        let pixel = vram.get_pixel_mut(64, 30).unwrap();
        assert!(*pixel == 0);
        *pixel = 255;
    }

    #[test]
    fn set_pixel_test_valid() {
        let mut vram = Vram::default();
        vram.set_pixel(63, 8, 255).unwrap();
        assert_eq!(*vram.get_pixel(63, 8).unwrap(), 255);
    }

    #[test]
    #[should_panic]
    fn set_pixel_test_invalid() {
        let mut vram = Vram::default();
        vram.set_pixel(63, 33, 255).unwrap();
        assert_eq!(*vram.get_pixel(63, 8).unwrap(), 255);
    }
}
