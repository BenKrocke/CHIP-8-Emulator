#[cfg(test)]
mod graphic_tests {
    use crate::chip8::*;

    fn set_up() -> Chip8 {
        let mut chip = init_chip();
        chip.load_rom(std::string::String::from("E07GraphicsRom.ch8"));
        chip.execute(0x6064);
        chip.execute(0x6127);
        chip.execute(0x6212);
        chip.execute(0x63AE);
        chip.execute(0x64FF);
        chip.execute(0x65B4);
        chip.execute(0x6642);
        chip.execute(0x673F);
        chip.execute(0x681F);
        chip.execute(0x6F25);
        chip
    }

    /**
     * The I Register holds memory addresses. A program can not read the value
     * currently set in the I register but may only set it.
     *
     * There are two opcodes for the IRegister : ANNN : Stores in the I register
     * the address 0xNNN FX1E : Adds to the value in the I-Register the value of
     * VX.
    */
    #[test]
    fn test_i_register() {
        let mut chip = set_up();
        chip.execute(0xA123);
        assert_eq!(0x123, *chip.get_i_register());

        chip.execute(0xF11E);
        assert_eq!(0x123 + 0x27, *chip.get_i_register());
    }

    /**
     *
     * Sprints in chip8 are represented in memory as columns of 8 bits with a
     * variable number of rows. The 8 bits each correspond to a pixel with 1
     * being toggled and 0 being transparent. The sprites are accessed starting
     * at the memory address pointed to by the I register.
     *
     * Sprites are XOR drawn. This means that a 1 will flip that particular
     * pixel. If a pixel is unset by this operation then the register VF is set
     * to 1. Otherwise it is 0.
     *
     * DXYN : Draw a Sprite of N rows at position VX,VY with the data pointed to
     * by the I register.
     *
     *
    */
    #[test]
    fn draw_sprite() {
        let mut chip = set_up();
        chip.execute(0xA202);
        chip.execute(0xD122); // Draw Sprite at 39, 18
        assert_eq!(0, chip.get_vf());

        let mut video = chip.get_screen();
        assert_eq!(1, video[39 + 64 * 18]);
        assert_eq!(1, video[40 + 64 * 18]);
        assert_eq!(1, video[41 + 64 * 18]);
        assert_eq!(1, video[42 + 64 * 18]);
        assert_eq!(1, video[43 + 64 * 18]);
        assert_eq!(1, video[44 + 64 * 18]);
        assert_eq!(1, video[45 + 64 * 18]);
        assert_eq!(1, video[46 + 64 * 18]);

        assert_eq!(0, video[39 + 64 * 19]);
        assert_eq!(0, video[40 + 64 * 19]);
        assert_eq!(1, video[41 + 64 * 19]);
        assert_eq!(1, video[42 + 64 * 19]);
        assert_eq!(1, video[43 + 64 * 19]);
        assert_eq!(1, video[44 + 64 * 19]);
        assert_eq!(0, video[45 + 64 * 19]);
        assert_eq!(0, video[46 + 64 * 19]);

        chip.execute(0xD122); // Draw Sprite at 39,18
        assert_eq!(1, chip.get_vf());

        video = chip.get_screen();
        assert_eq!(0, video[39 + 64 * 18]);
        assert_eq!(0, video[40 + 64 * 18]);
        assert_eq!(0, video[41 + 64 * 18]);
        assert_eq!(0, video[42 + 64 * 18]);
        assert_eq!(0, video[43 + 64 * 18]);
        assert_eq!(0, video[44 + 64 * 18]);
        assert_eq!(0, video[45 + 64 * 18]);
        assert_eq!(0, video[46 + 64 * 18]);

        assert_eq!(0, video[39 + 64 * 19]);
        assert_eq!(0, video[40 + 64 * 19]);
        assert_eq!(0, video[41 + 64 * 19]);
        assert_eq!(0, video[42 + 64 * 19]);
        assert_eq!(0, video[43 + 64 * 19]);
        assert_eq!(0, video[44 + 64 * 19]);
        assert_eq!(0, video[45 + 64 * 19]);
        assert_eq!(0, video[46 + 64 * 19]);

    }

    /**
     * Sprites wrap around on their axis. IE If you draw to X 65 it will wrap to
     * position 1.
    */
    #[test]
    fn draw_sprite_wrap() {
        let mut chip = set_up();
        chip.execute(0xA202);
        chip.execute(0xD212); // Draw Sprite at 18, 7 (39 wraps to 7)
        assert_eq!(0, chip.get_vf());

        let mut video = chip.get_screen();
        assert_eq!(1, video[18 + 64 * 7]);
        assert_eq!(1, video[19 + 64 * 7]);
        assert_eq!(1, video[20 + 64 * 7]);
        assert_eq!(1, video[21 + 64 * 7]);
        assert_eq!(1, video[22 + 64 * 7]);
        assert_eq!(1, video[23 + 64 * 7]);
        assert_eq!(1, video[24 + 64 * 7]);
        assert_eq!(1, video[25 + 64 * 7]);

        assert_eq!(0, video[18 + 64 * 8]);
        assert_eq!(0, video[19 + 64 * 8]);
        assert_eq!(1, video[20 + 64 * 8]);
        assert_eq!(1, video[21 + 64 * 8]);
        assert_eq!(1, video[22 + 64 * 8]);
        assert_eq!(1, video[23 + 64 * 8]);
        assert_eq!(0, video[24 + 64 * 8]);
        assert_eq!(0, video[25 + 64 * 8]);

    }

    /**
     * Sprites wrap around on their axis. IE If you draw to X 65 it will wrap to
     * position 1.
    */
    #[test]
    fn draw_sprite_bottom_right_edge() {
        let mut chip = set_up();
        chip.execute(0xA202);
        chip.execute(0xD781); 
        chip.execute(0xD871); 
        chip.execute(0xD781); 
        chip.execute(0xD871); 
    }
    
    /**
     * The opcode 00E0 clears the screen.
     *
    */
    #[test]
    fn test_clear_video() {
        let mut chip = set_up();
        chip.execute(0xA202);
        chip.execute(0xD212); // Draw Sprite at 18, 7 (39 wraps to 7)
        chip.execute(0x00E0);

        let mut video = chip.get_screen();

        assert_eq!(0, video[18 + 64 * 7]);
        assert_eq!(0, video[19 + 64 * 7]);
        assert_eq!(0, video[20 + 64 * 7]);
        assert_eq!(0, video[21 + 64 * 7]);
        assert_eq!(0, video[22 + 64 * 7]);
        assert_eq!(0, video[23 + 64 * 7]);
        assert_eq!(0, video[24 + 64 * 7]);
        assert_eq!(0, video[25 + 64 * 7]);

    }

    /**
     * Chip 8 has build in hexedecimal fonts. These fonts are stored in the 200
     * bytes of reserved data and accessed using a special opcode.
     *
     * FX29 : Set the I register to the address of the sprite corresponding to
     * the hex digit stored in VX.
    */
    #[test]
    fn test_hex_fonts() {
        let mut chip = set_up();

        chip.execute(0x6100); //Store 00 in V1
        chip.execute(0x6200); //Store 00 in V2

        for i in 0..0xf {
            chip.execute(0x6000 + i); //Store 00 in V0
            chip.execute(0xF029); // Set I to the memory address of the font sprite in V0
            chip.execute(0xD125); // Draw all 5 lines of the sprite at 0, 0.

            check_graphics(i);//Test for being drawn.
            chip.execute(0x00E0); // Clear Screen
        }
    }

    
    fn check_graphics(i: u32) {
        let mut chip = set_up();
        let mut video = chip.get_screen();
        match i {
            0 => {
                assert_eq!(0xF0, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 1, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 3, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 4, video));
            },
            1 => {
                assert_eq!(0x20, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x60, chip.get_sprite_row(0, 1, video));
                assert_eq!(0x20, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x20, chip.get_sprite_row(0, 3, video));
                assert_eq!(0x70, chip.get_sprite_row(0, 4, video));
            },               
            2 => {
                assert_eq!(0xF0, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x10, chip.get_sprite_row(0, 1, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x80, chip.get_sprite_row(0, 3, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 4, video));
            },               
            3 => {
                assert_eq!(0xF0, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x10, chip.get_sprite_row(0, 1, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x10, chip.get_sprite_row(0, 3, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 4, video));
            },             
            4 => {
                assert_eq!(0x90, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 1, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x10, chip.get_sprite_row(0, 3, video));
                assert_eq!(0x10, chip.get_sprite_row(0, 4, video));
            },               
            5 => {
                assert_eq!(0xF0, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x80, chip.get_sprite_row(0, 1, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x10, chip.get_sprite_row(0, 3, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 4, video));
            },               
            6 => {
                assert_eq!(0xF0, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x80, chip.get_sprite_row(0, 1, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 3, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 4, video));
            },
            7 => {
                assert_eq!(0xF0, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x10, chip.get_sprite_row(0, 1, video));
                assert_eq!(0x20, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x40, chip.get_sprite_row(0, 3, video));
                assert_eq!(0x40, chip.get_sprite_row(0, 4, video));
            },
            8 => {
                assert_eq!(0xF0, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 1, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 3, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 4, video));
            },
            9 => {
                assert_eq!(0xF0, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 1, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x10, chip.get_sprite_row(0, 3, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 4, video));
            },
            0xA => {
                assert_eq!(0xF0, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 1, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 3, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 4, video));
            },
            0xB => {
                assert_eq!(0xE0, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 1, video));
                assert_eq!(0xE0, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 3, video));
                assert_eq!(0xE0, chip.get_sprite_row(0, 4, video));
            },
            0xC => {
                assert_eq!(0xF0, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x80, chip.get_sprite_row(0, 1, video));
                assert_eq!(0x80, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x80, chip.get_sprite_row(0, 3, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 4, video));
            },
            0xD => {
                assert_eq!(0xE0, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 1, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x90, chip.get_sprite_row(0, 3, video));
                assert_eq!(0xE0, chip.get_sprite_row(0, 4, video));
            },
            0xE => {
                assert_eq!(0xF0, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x80, chip.get_sprite_row(0, 1, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x80, chip.get_sprite_row(0, 3, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 4, video));
            },
            0xF => {
                assert_eq!(0xF0, chip.get_sprite_row(0, 0, video));
                assert_eq!(0x80, chip.get_sprite_row(0, 1, video));
                assert_eq!(0xF0, chip.get_sprite_row(0, 2, video));
                assert_eq!(0x80, chip.get_sprite_row(0, 3, video));
                assert_eq!(0x80, chip.get_sprite_row(0, 4, video));
            },
            _ => panic!("Unsupported test case")
        }
    }
}