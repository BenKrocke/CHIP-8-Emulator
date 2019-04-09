#[cfg(test)]
mod arithmic_opcode_tests {
    use crate::chip8::*;

    #[test]
    fn test_pc_initialized_to_0x200() {
        assert_eq!(0x0200, init_chip().get_pc());
    }

    #[test]
    fn test_data_registers_initialized_to_0() {
        let mut chip8 = init_chip();
        assert_eq!(0x0, chip8.get_v0());
        assert_eq!(0x0, chip8.get_v1());
        assert_eq!(0x0, chip8.get_v2());
        assert_eq!(0x0, chip8.get_v3());
        assert_eq!(0x0, chip8.get_v4());
        assert_eq!(0x0, chip8.get_v5());
        assert_eq!(0x0, chip8.get_v6());
        assert_eq!(0x0, chip8.get_v7());
        assert_eq!(0x0, chip8.get_v8());
        assert_eq!(0x0, chip8.get_v9());
        assert_eq!(0x0, chip8.get_va());
        assert_eq!(0x0, chip8.get_vb());
        assert_eq!(0x0, chip8.get_vc());
        assert_eq!(0x0, chip8.get_vd());
        assert_eq!(0x0, chip8.get_ve());
        assert_eq!(0x0, chip8.get_vf());
    }

    #[test]
    fn test_load_constant() {
        let mut chip8 = init_chip();

        chip8.execute(0x6015);
        assert_eq!(0x15, chip8.get_v0());

        chip8.execute(0x6120);
        assert_eq!(0x20, chip8.get_v1());

        chip8.execute(0x6225);
        assert_eq!(0x25, chip8.get_v2());

        chip8.execute(0x6330);
        assert_eq!(0x30, chip8.get_v3());

        chip8.execute(0x6435);
        assert_eq!(0x35, chip8.get_v4());

        chip8.execute(0x6540);
        assert_eq!(0x40, chip8.get_v5());

        chip8.execute(0x6645);
        assert_eq!(0x45, chip8.get_v6());

        chip8.execute(0x6750);
        assert_eq!(0x50, chip8.get_v7());

        chip8.execute(0x6855);
        assert_eq!(0x55, chip8.get_v8());

        chip8.execute(0x6960);
        assert_eq!(0x60, chip8.get_v9());

        chip8.execute(0x6A65);
        assert_eq!(0x65, chip8.get_va());

        chip8.execute(0x6B70);
        assert_eq!(0x70, chip8.get_vb());

        chip8.execute(0x6C75);
        assert_eq!(0x75, chip8.get_vc());

        chip8.execute(0x6D80);
        assert_eq!(0x80, chip8.get_vd());

        chip8.execute(0x6E85);
        assert_eq!(0x85, chip8.get_ve());

        chip8.execute(0x6F90);
        assert_eq!(0x90, chip8.get_vf());
    }

    #[test]
    fn test_add_constant() {
        let mut chip8 = init_chip();

        chip8.execute(0x6015);
        chip8.execute(0x7015);
        assert_eq!(0x2A, chip8.get_v0());

        chip8.execute(0x6A42);
        chip8.execute(0x7A42);
        assert_eq!(0x84, chip8.get_va());

        chip8.execute(0x6EFF);
        chip8.execute(0x7E01);
        assert_eq!(0x0, chip8.get_ve());
    }

    #[test]
    fn test_copy_register() {
        let mut chip8 = init_chip();

        chip8.execute(0x6A42);
        chip8.execute(0x8EA0);
        assert_eq!(0x42, chip8.get_va());
        assert_eq!(0x42, chip8.get_ve());

        chip8.execute(0x6ADE);
        chip8.execute(0x8FA0);
        assert_eq!(0x42, chip8.get_ve());
        assert_eq!(0xDE, chip8.get_vf());
    }

    #[test]
    fn test_add_register() {
        let mut chip8 = init_chip();
        
        chip8.execute(0x6A42);
        chip8.execute(0x6E42);
        chip8.execute(0x8FA0);
        chip8.execute(0x8EA4);
        assert_eq!(0x42, chip8.get_va());
        assert_eq!(0x84, chip8.get_ve());
        assert_eq!(0x00, chip8.get_vf());

        chip8.execute(0x6AF0);
        chip8.execute(0x6E42);
        chip8.execute(0x8FA0);
        chip8.execute(0x8EA4);
        assert_eq!(0xF0, chip8.get_va());
        assert_eq!(0x32, chip8.get_ve());
        assert_eq!(0x01, chip8.get_vf());
    }

    #[test]
    fn test_subtract_register() {
        let mut chip8 = init_chip();
        
        chip8.execute(0x6B84);
        chip8.execute(0x6F84);
        chip8.execute(0x6D25);
        chip8.execute(0x8DB5);

        assert_eq!(0x84, chip8.get_vb());
        assert_eq!(161, chip8.get_vd());
        assert_eq!(0x00, chip8.get_vf());

        chip8.execute(0x6B84);
        chip8.execute(0x6F84);
        chip8.execute(0x6D25);
        chip8.execute(0x8DB7);

        assert_eq!(0x84, chip8.get_vb());
        assert_eq!(95, chip8.get_vd());
        assert_eq!(0x01, chip8.get_vf());
    }

}