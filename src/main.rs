use chipper::chip8::{cpu::{CPU}, memory::{Mem}};
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
        panic!("An error occured: {}", err);
    }

    let mut main_mem: Mem = Mem::new(rom);
    println!("{:?}", main_mem);
}