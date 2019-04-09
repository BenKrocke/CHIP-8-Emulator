extern crate rand;

use chip8::{
    init_chip
};

mod chip8;
mod input;

fn main() {
    let mut chip = init_chip();
    chip.cycle();
} 