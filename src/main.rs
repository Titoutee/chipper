use chipper::chip8::{cpu::CpuState, Interpreter, input::get_key_opcode, display::{SCREEN_HEIGHT, SCREEN_WIDTH}};

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::time::{Duration, Instant};
use std::{env, fs::File, io::Read};
fn main() {
    let filename: String = match env::args().nth(1) {
        Some(filename) => filename,
        None => String::from("roms/TETRIS"),
    };

    let mut file = File::open(filename).unwrap();
    let mut rom = Vec::new();
    if let Err(err) = file.read_to_end(&mut rom) {
        // Fit in ram is checked in rom loading
        panic!("An error occured: {}", err);
    }

    let mut chip8 = Interpreter::new();
    chip8.load_rom(rom); // Each byte is loaded as is, the cpu then assembles words

    let mut window = Window::new("CHIP-8 Emulator", SCREEN_WIDTH, SCREEN_HEIGHT, WindowOptions::default())
        .unwrap_or_else(|_| panic!("Couldn't create window"));
    window.set_title("CHIP-8 Emulator");

    let mut last_keyboard_instant = Instant::now();
    let kb_epsilon = 50;
    let mut last_instruction_instant = Instant::now();
    let instruction_epsilon = 1;
    let mut last_display_instant = Instant::now();
    let display_epsilon = 10;

    while window.is_open() && !window.is_key_down(Key::Escape) { // Escape to exit
        let keys_pressed = window.get_keys_pressed(KeyRepeat::Yes); // get all the presently pressed keys
        let key = if !keys_pressed.is_empty() {
            Some(keys_pressed[0]) // Interest only for the first one
        } else {
            None
        };

        //key getting clock
        if key.is_some() || Instant::now() - last_keyboard_instant >= Duration::from_millis(kb_epsilon) {
            chip8.feed_key(get_key_opcode(key)); // We feed into KeyBoard the just (valid) got key
            last_keyboard_instant = Instant::now(); // Instant refresh
        }

        //instruction executing clock
        if Instant::now() - last_instruction_instant > Duration::from_millis(instruction_epsilon) {
            match chip8.tick() { // get cpu state
                CpuState::Error(err) => panic!("{}", err),
                CpuState::Finished => break,
                _ => (),
            }
            last_instruction_instant = Instant::now(); // Instant refresh
        }

        //display clock
        if Instant::now() - last_display_instant > Duration::from_millis(display_epsilon) {
            window.update_with_buffer(&chip8.cpu.vram().to_screen_buffer(), SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
            last_display_instant = Instant::now(); // Instant refresh
        }
    }

    println!("Program finished was that cool?\nYessir.");
}
