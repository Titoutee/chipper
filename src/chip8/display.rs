pub const SPRITE_SIZE: usize = 15;

pub struct Sprite {
    pub data: [u8; SPRITE_SIZE],
}

impl From<[u8; SPRITE_SIZE]> for Sprite {
    fn from(data: [u8; SPRITE_SIZE]) -> Self {
        Self { data }
    }
}