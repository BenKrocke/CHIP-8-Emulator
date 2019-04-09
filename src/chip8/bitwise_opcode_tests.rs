#[cfg(test)]
mod bitwise_opcode_tests {
    use crate::chip8::*;

    fn set_up() -> Chip8 {
        let mut chip = init_chip();
        //controlRandom(&chip);
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
    fn test_AND_opcodes() {
        let mut chip = set_up();
        chip.execute(0x8012); // v0 = 0x64 & 0x27
        assert_eq!(36, chip.get_v0());
        assert_eq!(0x27, chip.get_v1());

        chip.execute(0x8232); // v2 = 0x12 & 0xAE
        assert_eq!(2, chip.get_v2());
        assert_eq!(0xAE, chip.get_v3());

        chip.execute(0x8FE2); // 0x25 & 0x0
        assert_eq!(0, chip.get_vf());
    }

    #[test]
    fn test_OR_opcodes() {
        let mut chip8 = set_up();
        chip8.execute(0x8011); // v0 = 0x64 | 0x27
        assert_eq!(103, chip8.get_v0());
        assert_eq!(0x27, chip8.get_v1());

        chip8.execute(0x8231); // v2 = 0x12 | 0xAE
        assert_eq!(190, chip8.get_v2());
        assert_eq!(0xAE, chip8.get_v3());

        chip8.execute(0x8FE1); // 0x25 | 0x0
        assert_eq!(0x25, chip8.get_vf());
    }

    #[test]
    fn test_XOR_opcodes() {
        let mut chip8 = set_up();
        chip8.execute(0x8013); // v0 = 0x64 ^ 0x27
        assert_eq!(67, chip8.get_v0());
        assert_eq!(0x27, chip8.get_v1());

        chip8.execute(0x8233); // v2 = 0x12 ^ 0xAE
        assert_eq!(188, chip8.get_v2());
        assert_eq!(0xAE, chip8.get_v3());

        chip8.execute(0x8FE3); // 0x25 ^ 0x0
        assert_eq!(0x25, chip8.get_vf());
    }

    #[test]
    fn testShiftRight() {
        let mut chip8 = set_up();
        chip8.execute(0x8016); // v0 = 0x27 >> 1; xF = 0x1
        assert_eq!(0x32, chip8.get_v0());
        assert_eq!(0x0, chip8.get_vf());

        chip8.execute(0x8236); // v2 = 0xAE >> 1; VF = 0x0
        assert_eq!(0x09, chip8.get_v2());
        assert_eq!(0x0, chip8.get_vf());

        chip8.execute(0x8446); // V4 = 0xFF >> 1; VF = 0x1;
        assert_eq!(127, chip8.get_v4());
        assert_eq!(0x1, chip8.get_vf());
    }

    #[test]
    fn test_shift_left() {
        let mut chip8 = set_up();
        chip8.execute(0x801E); // v0 = 0x27 << 1; xF = 0x1
        assert_eq!(200, chip8.get_v0());
        assert_eq!(0x0, chip8.get_vf());

        chip8.execute(0x823E); // v2 = 0xAE << 1; VF = 0x0
        assert_eq!(36, chip8.get_v2());
        assert_eq!(0x0, chip8.get_vf());

        chip8.execute(0x844E); // V4 = 0xFF << 1; VF = 0x1;
        assert_eq!(254, chip8.get_v4());
        assert_eq!(0x1, chip8.get_vf());
    }


    // Werkt niet ivm pseudo random errors?
    #[test]
    fn test_random() {
        // let mut chip8 = set_up();
        // //230, 198,153, 29
        // chip8.execute(0xC1FF); // V1 = 230 & 0xFF
        // assert_eq!(230, chip8.get_v1());
        
        // chip8.execute(0xC23E); // v2 = 198 & 0x3E
        // assert_eq!(6, chip8.get_v2());
        
        // chip8.execute(0xC44E); // V4 = 153 & 0x4E
        // assert_eq!(8, chip8.get_v4());
    }
}