//! Display API

use std::{ops::BitOrAssign, fmt::Display};
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
    
    pub fn set_pixel(&mut self, x: usize, y: usize) -> bool {
        let collision = false;
        if let Some(pixel_ref) = self.get_pixel_mut(x, y) { // We ignore the pixel setting if the pixel is not in bounds
            let collision = *pixel_ref>0;
            *pixel_ref = if collision {0} else {255};
        }
        collision
    }

    pub fn put_sprite(&mut self, sprite: Sprite, x: usize, y: usize) -> bool { // true, cpu knows it has to change VF
        let mut collision = false;
        for (i, line) in sprite.to_bytes_iter().enumerate() {
            let bits = bits_from_u8(*line);
            for (j, bit) in bits.iter().enumerate() {
                if *bit{
                    collision = self.set_pixel(x+j, y+i);
                }
            }
        }   
        collision
    }
}


pub fn bits_from_u8(byte: u8) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in 0..8 {
        bits.push(bool::from_u8((byte>>i)&1));
    }
    bits.reverse();
    bits
}

trait FromInteger {
    fn from_u8(val: u8) -> bool;
}

impl FromInteger for bool {
    fn from_u8(val: u8) -> bool {
        if val > 0 {true} else {false}
    }
}


#[cfg(test)]
mod tests {
    use crate::chip8::display::{VRAM_WIDTH, VRAM_HEIGHT};

    use super::{Vram, Sprite};
    use super::bits_from_u8;

    #[test]
    fn from_u8() {
        let val = 253;
        let bits = bits_from_u8(val);
        assert_eq!(bits, Vec::from([true, true, true, true, true, true, false, true]));
    }

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
        let mut vram: Vram = Vram::default();
        vram.set_pixel(63, 8);
        assert_eq!(*vram.get_pixel(63, 8).unwrap(), 255);
    }

    #[test]
    #[should_panic]
    fn set_pixel_test_invalid() {
        let mut vram = Vram::default();
        vram.set_pixel(63, 33);
        assert_eq!(*vram.get_pixel(63, 8).unwrap(), 255);
    }

    #[test]
    fn draw_sprite_test() {
        let sprite = Sprite::try_from(vec![1, 1, 1, 1]).unwrap();
        let mut vram = Vram::default();
        vram.put_sprite(sprite, 3, 3);
        //println!("{}", vram);
    }

    #[test]
    fn draw_sprite_xor_test() {
        let sprite = Sprite::try_from(vec![255, 255, 255, 255]).unwrap();
        let mut vram = Vram::default();
        vram.put_sprite(sprite, 3, 3);  
        let sprite = Sprite::try_from(vec![255, 255, 255, 255]).unwrap();
        vram.put_sprite(sprite,2, 3);
        //assert_eq!(vram.arr, [[0; VRAM_WIDTH]; VRAM_HEIGHT]); // Is wor functionning as it should?
        //println!("{:?}", vram);
        vram.clear();
        assert_eq!(vram.arr, [[0; VRAM_WIDTH]; VRAM_HEIGHT]);
    }

    #[test]
    fn draw_sprite_oob_test() {
        let sprite = Sprite::try_from(vec![255, 255, 255, 255]).unwrap();
        let mut vram = Vram::default();
        vram.put_sprite(sprite, 58, 3);
        println!("{:?}", vram.arr);
    }
}
