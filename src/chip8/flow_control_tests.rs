#[cfg(test)]
mod flow_control_tests {
    use crate::chip8::*;

    fn set_up() -> Chip8 {
        let mut chip = init_chip();
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
    fn test_jump() {
        let mut chip8 = set_up();
        chip8.execute(0x1DAE);
        assert_eq!(0xDAE, chip8.get_pc());

        chip8.execute(0xB432);
        assert_eq!(1174, chip8.get_pc());
    }

    #[test]
    fn test_subroutines() {
        let mut chip8 = set_up();
        chip8.execute(0x2DAE);
        assert_eq!(0xDAE, chip8.get_pc());

        chip8.execute(0x00EE);
        assert_eq!(0x200, chip8.get_pc());
    }

    #[test]
    fn test_equal_jumps() {
        let mut chip8 = set_up();
        chip8.execute(0x3064);
        assert_eq!(0x202, chip8.get_pc());

        chip8.execute(0x3164);
        assert_eq!(0x202, chip8.get_pc());

        chip8.execute(0x6764);
        chip8.execute(0x5070);
        assert_eq!(0x204, chip8.get_pc());

        chip8.execute(0x5170);
        assert_eq!(0x204, chip8.get_pc());
    }

    #[test]
    fn test_non_equal_jumps() {
        let mut chip8 = set_up();
        chip8.execute(0x4064); // Skip if V0 != 0x64 (it won't skip)
       assert_eq!(0x200, chip8.get_pc());//Increment the PC by 2
       
       chip8.execute(0x4164); // Skip if V1 == 0x64, skips because V1 == 0x27
       assert_eq!(0x202, chip8.get_pc());//Do not increment the PC
       
       chip8.execute(0x6764); // Set V7 to 64
       chip8.execute(0x9070); // Skip if V0 != V7(It won't skip)
       assert_eq!(0x202, chip8.get_pc());//Increment the PC by 2
    
       chip8.execute(0x9170); // Skip if V1 != V7 
       assert_eq!(0x204, chip8.get_pc());//Increment the PC by 2
       
    }
}