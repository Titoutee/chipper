use chipper::chip8::{cpu::CpuState, Interpreter, input::{get_key_opcode}, display::{SCREEN_HEIGHT, SCREEN_WIDTH}};

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::time::{Duration, Instant};
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

    let height = 320;
    let width = 640;

    let mut chip8 = Interpreter::new();
    chip8.load_rom(rom);

    let mut window = Window::new("CHIP-8 Emulator", width, height, WindowOptions::default())
        .unwrap_or_else(|_| panic!("Couldn't create window"));
    window.set_title("CHIP-8 Emulator");

    let mut last_keyboard_instant = Instant::now();
    let kb_epsilon = 100;
    let mut last_instruction_instant = Instant::now();
    let instruction_epsilon = 2;
    let mut last_display_instant = Instant::now();
    let display_epsilon = 2;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let keys_pressed = window.get_keys_pressed(KeyRepeat::Yes);
        let key = if !keys_pressed.is_empty() {
            Some(keys_pressed[0])
        } else {
            None
        };

        //key getting clock
        if key.is_some() && Instant::now() - last_keyboard_instant >= Duration::from_millis(kb_epsilon) {
            if get_key_opcode(key).is_some() {
                println!("Got some valid key");
            }
            chip8.feed_key(get_key_opcode(key)); // We feed into KeyBoard the just (valid) got key
            last_keyboard_instant = Instant::now();
        }

        //instruction executing clock
        if Instant::now() - last_instruction_instant > Duration::from_millis(instruction_epsilon) {
            match chip8.tick() {
                CpuState::Error(err) => panic!("{}", err),
                CpuState::Finished => break,
                _ => (),
            }
            last_instruction_instant = Instant::now();
        }
        
        //display clock
        if Instant::now() - last_display_instant > Duration::from_millis(display_epsilon) && chip8.vram_changed() {
            window.update_with_buffer(&chip8.cpu.vram().to_screen_buffer(), width, height).unwrap();
            last_display_instant = Instant::now();
        }
    }

    println!("Program finished was that cool?\nYessir.");
}
