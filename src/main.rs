extern crate rand;
mod chip8;
use chip8::{
    init_chip
};

fn main() {
    let mut chip = init_chip();
    chip.cycle();
}