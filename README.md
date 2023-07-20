This personal project puts forward an implementation of the famous
CHIP-8 interpreter, proposing a virtual machine (based on a regular
Von Neumann arch) on and thanks to which ops are run.

Some roms are included in "roms/", which are part of the more complete
roms repo which follows: https://github.com/esperz2019/chip8-roms

# Launching a game

To launch a game, simply run `cargo run roms/some_game`, or any other
path if you decided to add some roms. Running `cargo run` without
any argument will simply run the `tetris` game given with the
interpreter.

# Keyboard layout

The interpreter uses an hexadecimal keyboard. I give here the exact key actions (on a french kb) for the 3 roms included.

# Tetris
Rotate piece -> A

Move right -> E

Move left -> Z

Accelerate downwards -> Q

# Space invaders

Move right -> E

Move left -> A

Skip menu | Fire -> Z

# Landing

Launch -> S


# Have fun! :)
