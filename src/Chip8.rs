use std::fs;
use rand::Rng;
use piston::input::keyboard::Key;

use crate::input::Input;

pub struct Chip8 {
    pc: u32,
    i_register: u32,
    registers: [u32; 0x10],
    sp: u32,
    delay_timer: u32,
    sound_timer: u32,
    stack: [u32; 16],
    memory: [u8; 4096],
    video: [u8; 64 * 32],
    next_timer: u32,
    input: Input
}

pub fn init_chip() -> Chip8 {
    let mut chip = Chip8 {
        pc: 0x200,
        i_register: 0,
        registers: [0; 0x10],
        sp: 0,
        delay_timer: 0,
        sound_timer: 0,
        stack: [0; 16],
        memory: [0; 4096],
        video: [0; 64 * 32],
        next_timer: 0,
        input: Input::new()
    };

    return chip;
}

impl Chip8 {
    
    pub fn load_rom(&mut self, game: std::string::String) {
        println!("Loading: {:?}", game);
        
        let contents = fs::read(game).expect("Something went wrong reading the file");
        let mut index = 0x200;
        for b in 0..contents.len() {
            self.memory[index] = contents[b];
            index += 1;
        }
    }
    
    pub fn random(&self, max: u32) -> u32 {       
        rand::thread_rng().gen_range(0, max)
    }

    pub fn get_pc(&self) -> u32 { 
        self.pc 
    }

    fn set_vx(&mut self, value: u32, register: usize) {
        self.registers[register] = 0x000000FF & value;
    }

    fn get_vx(&self, x: usize) -> u32 { 
        (0x000000FF & self.registers[x])
    }

    pub fn get_v0(&self) -> u32 { 
        (self.registers[0] & 0xFF)
    }
    
    pub fn get_v1(&self) -> u32 { 
        (self.registers[1] & 0xFF)
    }

    pub fn get_v2(&self) -> u32 { 
        (self.registers[0x2] & 0xFF)
    }

    pub fn get_v3(&self) -> u32 { 
        (self.registers[0x3] & 0xFF)
    }

    pub fn get_v4(&self) -> u32 { 
        (self.registers[0x4] & 0xFF)
    }

    pub fn get_v5(&self) -> u32 { 
        (self.registers[0x5] & 0xFF)
    }

    pub fn get_v6(&self) -> u32 { 
        (self.registers[0x6] & 0xFF)
    }

    pub fn get_v7(&self) -> u32 { 
        (self.registers[0x7] & 0xFF)
    }
    
    pub fn get_v8(&self) -> u32 { 
        (self.registers[0x8] & 0xFF)
    }

    pub fn get_v9(&self) -> u32 { 
        (self.registers[0x9] & 0xFF)
    }

    pub fn get_va(&self) -> u32 { 
        (self.registers[0xa] & 0xFF)
    }

    pub fn get_vb(&self) -> u32 { 
        (self.registers[0xb] & 0xFF)
    }

    pub fn get_vc(&self) -> u32 { 
        (self.registers[0xc] & 0xFF)
    }

    pub fn get_vd(&self) -> u32 { 
        (self.registers[0xd] & 0xFF)
    }

    pub fn get_ve(&self) -> u32 { 
        (self.registers[0xe] & 0xFF)
    }

    pub fn get_vf(&self) -> u32 { 
        (self.registers[0xf] & 0xFF)
    }

    // pub fn get_register(&self, register: ) -> u32 {

    // }


    pub fn execute(&mut self, instruction: u32) {
        // If opcode is 0x6015, bitflip it to 0x6000
        //              0xF000
        //              &
        //              0x6000
        let high = instruction & 0xF000;

        match high {
            0x1000 => {
                let low = 0x0FFF & instruction;
                self.pc = low;
            },
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp = self.sp + 1;
                let low = 0x0FFF & instruction;
                self.pc = low;
            },
            0x3000 => {
                let low = 0x00FF & instruction;
                let register = (0x0F00 & instruction) >> 8;
                if self.get_vx(register as usize) == low
                {
                    self.pc = self.pc + 0x2;
                }
            },
            0x4000 => { //Skip the following instruction if the value of register VX is not equal to NN
                let low = 0x0FF & instruction;
                let register = (instruction & 0x0f00) >> 8;
                if self.get_vx(register as usize) != low {
                    self.pc = self.pc + 0x2;
                }
            },
            0x5000 => { //Skip the following instruction if the value of register VX is equal to the value of register VY
                let reg_x = (instruction & 0x0F00) >> 8;
                let reg_y = (instruction & 0x00F0) >> 4;
                if self.get_vx(reg_x as usize) == self.get_vx(reg_y as usize) {
                    self.pc = self.pc + 2;
                }
            },
            0x6000 => { //6XNN	Store number NN in register VX
                let low = 0x00FF & instruction;
                let register = (instruction & 0x0F00) >> 8;
                self.set_vx(low, register as usize);
            },
            0x7000 => { //7XNN	Add number NN to register VX
                let register = (instruction & 0x0f00) >> 8;
                let low = (0x0FF & instruction) + self.get_vx(register as usize);
                self.set_vx(low, register as usize);
            },
            0x8000 => {
                let low = 0x000F & instruction;
                let register_y = (instruction & 0x00F0) >> 4;
                let register_x = (instruction & 0x0F00) >> 8;

                match low {
                    0x0000 => {
                        self.set_vx(self.get_vx(register_y as usize), register_x as usize);
                    },
                    0x0001 => {
                        self.set_vx(self.get_vx(register_y as usize) | self.get_vx(register_x as usize), register_x as usize);                    
                    },
                    0x0002 => {
                        self.set_vx(self.get_vx(register_y as usize) & self.get_vx(register_x as usize), register_x as usize);  
                    },
                    0x0003 => {
                        self.set_vx(self.get_vx(register_y as usize) ^ self.get_vx(register_x as usize), register_x as usize);  
                    },
                    0x0004 => {
                        let sum = self.get_vx(register_x as usize) + self.get_vx(register_y as usize);
                        self.registers[0xf] = if sum > 0xFF { 1 } else { 0 };
                        self.set_vx(sum, register_x as usize);
                    },
                    0x0005 => {
                        // Iets met Rust overflow - TODO: SET REGISTER F IF BORROW OCCURS
                        let x = self.get_vx(register_x as usize);
                        let y = self.get_vx(register_y as usize);
                        let diff;
                        let mut under_zero = false;
                        if x.checked_sub(y) == None {
                            diff = (256 - y) + x;
                            under_zero = true;
                        } else {
                            diff = x - y;
                        }
                        self.registers[0xf] = if !under_zero { 1 } else { 0 };
                        self.set_vx(diff, register_x as usize);
                    },
                    0x0006 => {
                        self.registers[0xf] = self.get_vx(register_x as usize) & 0x01;
                        self.set_vx(self.get_vx(register_x as usize) >> 1, register_x as usize);
                    },
                    0x0007 => {
                        let diff = self.get_vx(register_y as usize) - self.get_vx(register_x as usize);
                        self.registers[0xf] = if diff > 0 { 1 } else { 0 };
                        self.set_vx(diff, register_x as usize);
                    },
                    0x000E => {
                        self.registers[0xf] = (self.get_vx(register_x as usize) >> 7) & 0x01;
                        self.set_vx(self.get_vx(register_x as usize) << 1, register_x as usize);
                    },
                    _ => {
                        println!("Unsupported opcode.");
                    }
                }
            },
            0x9000 => { // Skip the following instruction if the value of register VX is not equal to the value of register VY
                let register_y = (instruction & 0x00f0) >> 4;
                let register_x = (instruction & 0x0f00) >> 8;
                if self.get_vx(register_x as usize) != self.get_vx(register_y as usize) {
                    self.pc = self.pc + 0x2;
                }
            },
            0x0000 => {
                match instruction {
                    0x00EE => {
                        self.sp = self.sp - 1;
                        self.pc = self.stack[self.sp as usize];
                    },
                    _ => panic!("Unsupported opcode.")
                }
            },
            0xE000 => {
                let low = instruction & 0x00FF;
                let register = (instruction & 0x0F00) >> 8;

                self.pc += match low {
                    0x9E => if self.input.pressed(self.get_vx(register as usize) as usize) { 2 } else { 0 },
                    0xA1 => if !self.input.pressed(self.get_vx(register as usize) as usize) { 2 } else { 0 },
                    _    => 0
                }
            },
            0xF000 => {
                let low = instruction & 0xFF;
                let register = (instruction & 0x0F00) >> 8;

                match low {
                    0x15 => {
                        self.delay_timer = self.get_vx(register as usize);
                    },
                    // 0x65 => {
                    //     usize maxRegister = (usize) register;
                    //         for (let i = 0; i <= maxRegister; i++) {
                    //             self.set_vx(self.memory[self.i_register as usize], i as usize);
                    //             self.i_register = self.i_register + 1;
                    //     }
                    // },
                    // 0x55 => {
                    //     usize maxRegister = (usize) register;
                    //     for (let i = 0; i <= maxRegister; i++) {
                    //         self.memory[self.i_register as usize] = (usize) self.get_vx(i as usize);
                    //         self.i_register = self.i_register + 1;
                    //     }
                    // },
                    0x18 => {
                        self.sound_timer = self.get_vx(register as usize);
                    },
                    0x0A => {
                        for i in 0u8..16 {
                            if self.input.pressed(i as usize) {
                                self.set_vx(i as u32, register as usize);
                                break;
                            }
                        }
                        self.pc -= 2;
                    },
                    0x07 => {
                        self.set_vx(self.delay_timer, register as usize);
                    },
                    // 0x29 => {
                    //     self.i_register = getCharacterAddress(self.get_vx(register as usize));
                    // },
                    // 0x33 => {
                    //     let value = self.get_vx(register as usize);
                    //     self.memory[self.i_register as usize] = (usize) (value / 100);
                    //     self.memory[(self.i_register + 1) as usize] = (usize) (((value) % 100) / 10);
                    //     self.memory[(self.i_register + 2) as usize] = (usize) (((value) % 100) % 10);
                    // },
                    // 0x1E => {
                    //     self.i_register = self.i_register + self.get_vx(register as usize);
                    // },
                    _ => panic!("Unsupported opcode.")
                }
            },
            0xB000 => {
                let low = 0x0FFF & instruction;
                self.pc = low + self.get_v0();
            },
            0xC000 => {
                let low = 0x0FF & instruction;
                println!("Low: {}.", low);

                let register = (instruction & 0x0F00) >> 8;
                println!("Register: {}.", register);
                
                let rand = self.random(0xFF);
                println!("Random: {}.", rand);

                let val = rand & low;
                println!("Value: {}.", val);

                self.set_vx(val, register as usize);
            },
            _ => panic!("Unsupported opcode.")
        }
    }


    // memory 1: 0x60
    // in binary: 0000 0000 0110 0000
    // memory 2: 0x15
    // in binary: 0000 0000 0001 0101

    // je wil: 0x6015

    // dit werkt natuurlijk niet: 60 + 15

    // shift memory 1 met 8, zodat je genoeg nullen toevoegt aan de binary zodat je 0x6000 krijgt
    // 0x60
    // << 8
    // wordt:

    // hex:
    // 1= 0x6000
    // 2= 0x15

    // en binary:
    // 1= 0000 0000 0110 0000 0000 0000 <- zie hoe hij is geshift
    // 2=           0000 0000 0001 0101

    // vervolgens inclusive bit OR met beide values
    // 1 | 2

    // dan krijg je dit:
    // 1= 0000 0000 0110 0000 0000 0000
    // 2=           0000 0000 0001 0101
    // r= 0000 0000 0110 0000 0001 0101

    // result binary to hex: 0x6015 = je opcode

    pub fn cycle(&mut self) {
        let one = ((self.memory[self.pc as usize] as u16) << 8) & 0xFF00;
        self.pc += 1;
        let two = self.memory[self.pc as usize] as u16 & 0xFF;
        self.pc += 1;
        let instruction = one | two;
        self.execute(instruction as u32);
    }

    fn countdown_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            //
        } else {
            //
        }}

    pub fn get_i_register(&self) -> &u32 { 
        &self.i_register
    }

    // pub fn get_screen() ->

    // pub fn get_memory() ->

    pub fn get_sp(&self) -> &u32 {
        &self.sp
    }
}

#[cfg(test)]
mod arithmetic_opcode_tests {
    use super::*;

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
        
        // Store 84 in regiser B
        chip8.execute(0x6B84);
        // Story 84 in register F
        chip8.execute(0x6F84);
        // Store 25 in register D
        chip8.execute(0x6D25);
        // Subtract 84 from 25 = 0 so register F = 1? 
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

#[cfg(test)]
mod bitwise_opcode_tests {
    use super::*;

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

    //fn controlRandom(chip: &Chip8) {
    //    //chip.
    //}

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


    // Werkt niet ivm pseudo random errors
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

#[cfg(test)]
mod clock_execution_and_memory_tests {
    use super::*;

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

#[cfg(test)]
mod flow_control_tests {
    use super::*;

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

#[cfg(test)]
mod timer_tests {
    use super::*;

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

#[cfg(test)]
mod input_tests {
    use super::*;

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