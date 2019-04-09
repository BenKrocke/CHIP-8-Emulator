#[cfg(test)]
mod timer_tests {
    use crate::chip8::*;

    fn set_up() -> Chip8 {
        let mut chip = init_chip();
        chip.load_rom(std::string::String::from("E05TimerLoop.ch8"));
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

    #[test]
    fn test_delay_timer_opcodes() {
        let mut chip8 = set_up();
        chip8.execute(0xF015);
        chip8.execute(0xF107);
        assert_eq!(0x64, chip8.get_v1());
    }

// NEEDS FIXING
    // #[test]
    // fn test_delay_timer_counter() {
    //     let mut chip8 = set_up();
    //     while chip8.get_v5() != 255 {
    //         chip8.cycle();
    //     }
    // }

    #[test]
    fn test_sound_timer() {
        let mut chip8 = set_up();
        chip8.execute(0xF018); //Set timer to 0x64
        chip8.cycle();
    }

// NEEDS FIXING
    // #[test]
    // fn test_emit_sound_timer() {
    //     let mut chip = init_chip();
    //     chip.load_rom(std::string::String::from("E05SoundLoop.ch8"));
    //     while chip.get_v5() != 255 {
    //         chip.cycle();
    //     }
    // }

}