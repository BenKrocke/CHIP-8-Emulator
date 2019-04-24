use std::fs;
use rand::Rng;
use piston::input::keyboard::Key;
mod arithmic_opcode_tests;
mod bitwise_opcode_tests;
mod clock_execution_and_memory_tests;
mod flow_control_tests;
mod timer_tests;
mod input_tests;
mod graphic_tests;

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
    
    pub fn cycle(&mut self) {
        let one = ((self.memory[self.pc as usize] as u16) << 8) & 0xFF00;
        self.pc += 1;
        let two = self.memory[self.pc as usize] as u16 & 0xFF;
        self.pc += 1;
        let instruction = one | two;
        self.execute(instruction as u32);
    }

    // Packs a graphics row (8 pixels of the sprite) into a byte
    pub fn get_sprite_row(&mut self, mut x: u32, mut y: u32, video: [u8; 64 * 32]) -> u8 {
        x = x % 64;
        y = y % 32;

        let mut byte1 = self.video[(x + y * 64) as usize];
        let mut byte2 = self.video[(x + 1 + y * 64) as usize];
        let mut byte3 = self.video[(x + 2 + y * 64) as usize];
        let mut byte4 = self.video[(x + 3 + y * 64) as usize];
        let mut byte5 = self.video[(x + 4 + y * 64) as usize];
        let mut byte6 = self.video[(x + 5 + y * 64) as usize];
        let mut byte7 = self.video[(x + 6 + y * 64) as usize];
        let mut byte8 = self.video[(x + 7 + y * 64) as usize];

        ((byte1 << 7)
                | (byte2 << 6)
                | (byte3 << 5)
                | (byte4 << 4)
                | (byte5 << 3)
                | (byte6 << 2)
                | (byte7 << 1)
                | (byte8))
    }

    pub fn get_screen(&mut self) -> [u8; 64 * 32] {
        self.video
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

    pub fn get_sp(&self) -> &u32 {
        &self.sp
    }

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
            0xA000 => {
                let low = instruction & 0x0FFF;
                self.i_register = low;
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
            0xD000 => {
                let lines = instruction & 0x00F;
                let reg_x = (instruction & 0x0F00) >> 8;
                let reg_y = (instruction & 0x00F0) >> 4;
                let x = self.get_vx(reg_x as usize);
                let y = self.get_vx(reg_y as usize);
                self.registers[0xF] = 0;

                for line in 0..lines {
                    
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
                    0x1E => {
                        self.i_register = self.i_register + self.get_vx(register as usize);
                    },
                    _ => panic!("Unsupported opcode.")
                }
            },
            _ => panic!("Unsupported opcode.")
        }
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