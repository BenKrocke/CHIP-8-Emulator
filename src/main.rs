extern crate rand;

mod chip8;
mod input;

fn main() {
    let mut chip = chip8::init_chip();
    chip.cycle();
} 