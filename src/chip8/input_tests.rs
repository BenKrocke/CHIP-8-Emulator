#[cfg(test)]
mod input_tests {
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

    /**
     * The opcode FX0A will wait for a key press and store its value into
     * register VX.
     * 
     * This test will execute the opcode and then call cycle several times.  As 
     * long as the program counter does not increment we will assume that chip8
     * is not executing.
     * 
    */
    #[test]
    fn test_chip8_waits_for_keyboard_input() {
        let mut chip8 = set_up();
        let pc = chip8.get_pc();
        chip8.cycle();
        chip8.cycle();
        chip8.cycle();
        chip8.cycle();
        assert_eq!(pc, chip8.get_pc());
    }

    #[test]
    fn test_chip8_continues_after_keyboard_input() {
        let mut chip8 = set_up();
        let pc = chip8.get_pc();
        chip8.cycle();
        chip8.cycle();
        assert_eq!(pc, chip8.get_pc());

        chip8.input.press(Key::Z, true);
        chip8.cycle();
        chip8.cycle();
        assert_eq!(0xA, chip8.get_v6());
    }

    /**
     * The next opcode will skip the next instruction until the keypress matches 
     * a value in a certain register.
     * 
     * EX9E : Skip the next instruction of the key corresponding to the value in 
     * VX is pressed.
     * 
    */
    #[test]
    fn skip_if_pressed() {
        let mut chip8 = set_up();
        chip8.input.press(Key::NumPad1, true);
        chip8.execute(0x6002);//Store 0x02 into V0
        chip8.execute(0xE09E);//Skip if 0x02 is pressed (it isn't)
        assert_eq!(0x200, chip8.get_pc());
        
        chip8.input.press(Key::NumPad2, true);
        chip8.execute(0x6002);//Store 0x02 into V0
        chip8.execute(0xE09E);//Skip if 0x02 is pressed (it is)
        assert_eq!(0x202, chip8.get_pc());
    }

    /**
     * The next opcode will not skip the next instruction if the keypress 
     * matches a value in a certain register.
     * 
     * EXA1 : Skip the next instruction of the key corresponding to the value in 
     * VX is not pressed.
     * 
    */
    #[test]
    fn skip_if_not_pressed() {
        let mut chip8 = set_up();
        chip8.input.press(Key::NumPad1, true);
        chip8.execute(0x6002);//Store 0x02 into V0
        chip8.execute(0xE0A1);//Skip if 0x02 is not pressed (it isn't)
        assert_eq!(0x202, chip8.get_pc());
    
        chip8.input.press(Key::NumPad2, true);
        chip8.execute(0x6002);//Store 0x02 into V0
        chip8.execute(0xE0A1);//Skip if 0x02 is pressed (it is)
        assert_eq!(0x202, chip8.get_pc());
    }
}