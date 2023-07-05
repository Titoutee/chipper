use chipper::chip8::{cpu::{CPU}, memory::{Mem}};
use std::{env, fs::File, io::Read};
use minifb::{Window, Key, WindowOptions, Menu};
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(filename) => filename.clone(),
        None => panic!("chipper [filename]"),
    };

    let mut file = File::open(filename).unwrap();
    let mut rom = Vec::new();
    if let Err(err) = file.read_to_end(&mut rom) { // Fit in ram is checked in rom loading
        panic!("An error occured: {}", err);
    }
    let mut window = Window::new("CHIP-8 Emulator", 640, 320, WindowOptions::default()).unwrap();
    window.set_title("CHIP-8 Emulator");
    let menu = Menu::new("CHIP-8 menu").unwrap();
    //while window.is_open() {
    //    window.update();
    //}
}