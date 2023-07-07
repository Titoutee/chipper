use chipper::chip8::{
    cpu::{CpuState},
    Interpreter,
};
use minifb::{Key, Window, WindowOptions};
use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(filename) => filename.clone(),
        None => panic!("chipper [filename]"),
    };

    let mut file = File::open(filename).unwrap();
    let mut rom = Vec::new();
    if let Err(err) = file.read_to_end(&mut rom) {
        // Fit in ram is checked in rom loading
        panic!("An error occured: {}", err);
    }

    let mut chip8 = Interpreter::new();
    chip8.load_rom(rom);

    let mut window = Window::new("CHIP-8 Emulator", 640, 320, WindowOptions::default())
        .unwrap_or_else(|_| panic!("COuldn't create window"));
    window.set_title("CHIP-8 Emulator");
    //let menu = Menu::new("CHIP-8 menu").unwrap();
    
    while !window.is_key_down(Key::Escape) {}

    while window.is_open() {
        
        match chip8.tick() {
            CpuState::Error(err) => panic!("{}", err),
            CpuState::Finished => break,
            _ => (),
        }
    }

    println!("Program finished");
}
