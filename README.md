This personal project puts forward an implementation of the famous
CHIP-8 interpreter, proposing a virtual machine (based on a regular
Von Neumann arch) on and thanks to which ops are run.

Some roms are included in "roms/", which are part of the more complete
roms repo which follows: https://github.com/esperz2019/chip8-roms

To launch a game, simply run `cargo run roms/some_game`, or any other
path if you decided to add some roms. Running `cargo run` without
any argument will simply run the `tetris` game given with the
interpreter.

Have fun! :)
