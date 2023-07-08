use minifb::Key;

pub struct KeyBoard {
    key: Option<u8>,
}

impl KeyBoard {
    pub fn new() -> Self {
        KeyBoard { key: None }
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.key
    }

    pub fn feed_key(&mut self, key: Option<u8>) {
        self.key = key;
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        if self.key.is_none() {return false;}
        self.key.unwrap() == key
    }

    pub fn is_key_up(&self, key: u8) -> bool {
        if self.key.is_none() {return true;}
        self.key.unwrap() != key
    }
}

pub fn get_key_opcode(key: Option<Key>) -> Option<u8> {
    match key {
        Some(Key::Key1) => Some(0x1),
        Some(Key::Key2) => Some(0x2),
        Some(Key::Key3) => Some(0x3),
        Some(Key::Key4) => Some(0x4),

        Some(Key::A) => Some(0x4),
        Some(Key::Z) => Some(0x5),
        Some(Key::E) => Some(0x6),
        Some(Key::R) => Some(0x7),

        Some(Key::Q) => Some(0x8),
        Some(Key::S) => Some(0x9),
        Some(Key::D) => Some(0xA),
        Some(Key::F) => Some(0xB),

        Some(Key::W) => Some(0xC),
        Some(Key::X) => Some(0xD),
        Some(Key::C) => Some(0xE),
        Some(Key::V) => Some(0xF),

        _ => None,
    }

}