#[cfg(test)]
mod graphic_tests {
    use crate::chip8::*;

    fn set_up() -> Chip8 {
        let mut chip = init_chip();
        chip.load_rom(std::string::String::from("E06KeypadLoop.ch8"));
        chip.execute(0x6064);
        chip.execute(0x6127);
        chip.execute(0x6212);
        chip.execute(0x63AE);
        chip.execute(0x64FF);
        chip.execute(0x65B4);
        chip.execute(0x6642);
        chip.execute(0x6F25);
        chip
    }

}