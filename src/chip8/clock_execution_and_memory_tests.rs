#[cfg(test)]
mod clock_execution_and_memory_tests {
    use crate::chip8::*;

    fn set_up_load_rom() -> Chip8 {
        let mut chip = init_chip();
        chip.load_rom(std::string::String::from("E03TestRom.ch8"));
        chip
    }


    #[test]
    fn test_cycle() {
        let mut chip8 = set_up_load_rom();
        chip8.cycle();
        chip8.cycle();
        chip8.cycle();
        chip8.cycle();
        assert_eq!(0x15,    chip8.get_v0());
        assert_eq!(0x20,    chip8.get_v1());
        assert_eq!(0x25,    chip8.get_v2());
        assert_eq!(0x30,    chip8.get_v3());
        assert_eq!(0x208,   chip8.get_pc());
    }
}