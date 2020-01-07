use std::fs;
mod arithmic_opcode_tests;
mod bitwise_opcode_tests;
mod clock_execution_and_memory_tests;
mod flow_control_tests;
mod timer_tests;
mod graphic_tests;

use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};
use rand::{Rng, SeedableRng, XorShiftRng};

use crate::input::Input;
use crate::display::Display;

pub struct Chip8 {
    program_counter: u32,  //The program counter (PC) should be 16-bit, and is used to store the currently executing address.
    i_register: u32, // There is also a 16-bit register called I. This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
    registers: [u32; 0x10], // Chip-8 has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit (0 through F). There is also a 16-bit register called I. This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
    sp: u32,// The stack pointer (SP) can be 8-bit, it is used to point to the topmost level of the stack.
    delay_timer: u32, // Chip-8 also has two special purpose 8-bit registers, for the delay and sound timers. When these registers are non-zero, they are automatically decremented at a rate of 60Hz. See the section 2.5, Timers & Sound, for more information on these.
    sound_timer: u32, // Chip-8 also has two special purpose 8-bit registers, for the delay and sound timers. When these registers are non-zero, they are automatically decremented at a rate of 60Hz. See the section 2.5, Timers & Sound, for more information on these.
    stack: [u32; 16], // The stack is an array of 16 16-bit values, used to store the address that the interpreter shoud return to when finished with a subroutine. Chip-8 allows for up to 16 levels of nested subroutines.
    memory: [u8; 4096],
    video: [u8; 64 * 32],
    next_timer: u32,
    pub input: Input,
    pub display: Display,
}

pub fn init_chip() -> Chip8 {
    let mut chip = Chip8 {
        program_counter: 0x200,
        i_register: 0,
        registers: [0; 0x10],
        sp: 0,
        delay_timer: 0,
        sound_timer: 0,
        stack: [0; 16],
        memory: [0; 4096],
        video: [0; 64 * 32],
        next_timer: 0,
        input: Input::new(),
        display: Display::new()
    };

    //for i in 0..80 { chip.memory[i] = fontset[i]; }
    //load_font();

    let mut i = 0;

    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;

    chip.memory[i] = 0x20;
    i += 1;
    chip.memory[i] = 0x60;
    i += 1;
    chip.memory[i] = 0x20;
    i += 1;
    chip.memory[i] = 0x20;
    i += 1;
    chip.memory[i] = 0x70;
    i += 1;

    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x10;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x80;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;

    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x10;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x10;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;

    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x10;
    i += 1;
    chip.memory[i] = 0x10;
    i += 1;

    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x80;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x10;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;

    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x80;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;

    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x10;
    i += 1;
    chip.memory[i] = 0x20;
    i += 1;
    chip.memory[i] = 0x40;
    i += 1;
    chip.memory[i] = 0x40;
    i += 1;

    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;

    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x10;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;

    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;

    chip.memory[i] = 0xE0;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0xE0;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0xE0;
    i += 1;

    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x80;
    i += 1;
    chip.memory[i] = 0x80;
    i += 1;
    chip.memory[i] = 0x80;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;

    chip.memory[i] = 0xE0;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0x90;
    i += 1;
    chip.memory[i] = 0xE0;
    i += 1;

    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x80;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x80;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;

    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x80;
    i += 1;
    chip.memory[i] = 0xF0;
    i += 1;
    chip.memory[i] = 0x80;
    i += 1;
    chip.memory[i] = 0x80;
    i += 1;
    
    return chip;
}

impl Chip8 {
    
    pub fn cycle(&mut self) {
        //println!("PC = {:#X} | SP = {:#X} I = {:#X} | V0 = {:#X} | V1 = {:#X} | V2 = {:#X} | V3 = {:#X} | V4 = {:#X} | V5 = {:#X} | V6 = {:#X} | V7 = {:#X} | V8 = {:#X} | V9 = {:#X} | VA = {:#X} | VB = {:#X} | VC = {:#X} | VD = {:#X} | VE = {:#X} | VF = {:#X} ", 
        //    self.program_counter, self.sp, self.i_register,
        //    self.get_v0(), self.get_v1(), self.get_v2(), self.get_v3(), self.get_v4(), self.get_v5(), self.get_v6(), self.get_v7(), self.get_v8(), self.get_v9(), self.get_va(), self.get_vb(), self.get_vc(), self.get_vd(), self.get_ve(), self.get_vf()
        //);
        thread::sleep(time::Duration::from_millis(1));
        let opcode_part_one = ((self.memory[self.program_counter as usize] as u32) << 8) & 0xFF00;

        self.program_counter += 1;
        let opcode_part_two = self.memory[self.program_counter as usize] as u32 & 0xFF;
        self.program_counter += 1;

        let instruction = opcode_part_one | opcode_part_two;

        let start = SystemTime::now();
        let time = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

        if (time.as_millis() > self.next_timer as u128) {
            self.countdown_timers();
            self.next_timer = (time.as_millis() + (1000 / 60)) as u32;
        }
        self.execute(instruction as u32);
    }

    // Packs a graphics row (8 pixels of the sprite) into a byte
    pub fn get_sprite_row(&mut self, mut x: u32, mut y: u32, video: [u8; 64 * 32]) -> u8 {
        x = x % 64;
        y = y % 32;

        let mut byte1 = self.video[(x + y * 64) as usize];
        let mut byte2 = self.video[((x + 1) % 64 + y * 64) as usize];
        let mut byte3 = self.video[((x + 2) % 64 + y * 64) as usize];
        let mut byte4 = self.video[((x + 3) % 64 + y * 64) as usize];
        let mut byte5 = self.video[((x + 4) % 64 + y * 64) as usize];
        let mut byte6 = self.video[((x + 5) % 64 + y * 64) as usize];
        let mut byte7 = self.video[((x + 6) % 64 + y * 64) as usize];
        let mut byte8 = self.video[((x + 7) % 64 + y * 64) as usize];

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
        } else {
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
        self.program_counter 
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
        let high = instruction & 0xF000;

        match high {
            0x1000 => {
                let low = instruction & 0x0FFF;
                self.program_counter = low;
            },
            0x2000 => {
                self.stack[self.sp as usize] = self.program_counter as u32;
                self.sp += 1;
                let low = instruction & 0x0FFF;
                self.program_counter = low;
            },
            0x3000 => {
                let low = 0x00FF & instruction;
                let register = (0x0F00 & instruction) >> 8;
                if self.get_vx(register as usize) == low
                {
                    self.program_counter += 2;
                }
            },
            0x4000 => { //Skip the following instruction if the value of register VX is not equal to NN
                let low = 0x00FF & instruction;
                let register = (0x0F00 & instruction) >> 8;
                if self.get_vx(register as usize) != low {
                    self.program_counter = self.program_counter + 0x2;
                }
            },
            0x5000 => { //Skip the following instruction if the value of register VX is equal to the value of register VY
                let reg_x = (instruction & 0x0F00) >> 8;
                let reg_y = (instruction & 0x00F0) >> 4;
                if self.get_vx(reg_x as usize) == self.get_vx(reg_y as usize) {
                    self.program_counter = self.program_counter + 2;
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
                        let v_x = self.get_vx(register_x as usize);
                        let v_y = self.get_vx(register_y as usize);
                        let (result, did_overflow) = v_x.overflowing_add(v_y);
                        self.set_vx(result, register_x as usize);
                        let val = if did_overflow { 1 } else { 0 };
                        self.set_vx(val, 0xF);
                    },
                    0x0005 => {
                        let v_x = self.get_vx(register_x as usize);
                        let v_y = self.get_vx(register_y as usize);
                        let val = if v_x > v_y { 1 } else { 0 };
                        self.set_vx(val, 0xF);
                        self.set_vx(v_x.wrapping_sub(v_y), register_x as usize);
                    },
                    0x0006 => {
                        self.registers[0xf] = self.get_vx(register_x as usize) & 0x01;
                        self.set_vx(self.get_vx(register_x as usize) >> 1, register_x as usize);
                    },
                    0x0007 => {
                        let v_x = self.get_vx(register_x as usize);
                        let v_y = self.get_vx(register_y as usize);
                        let val = if v_y > v_x { 1 } else { 0 };
                        self.set_vx(val, 0xF);
                        self.set_vx(v_y.wrapping_sub(v_x), register_x as usize);
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
            0x9000 => { 
                let register_y = (instruction & 0x00f0) >> 4;
                let register_x = (instruction & 0x0f00) >> 8;
                if self.get_vx(register_x as usize) != self.get_vx(register_y as usize) {
                    self.program_counter += 2;
                }
            },
            0x0000 => {
                match instruction {
                    0x0000 => {
                        self.video = [0; 64 * 32];
                        self.display.clear();
                    },
                    0x00EE => {
                        self.sp -= 1;
                        self.program_counter = self.stack[self.sp as usize] as u32;
                    },
                    0x00E0 => {
                        self.video = [0; 64 * 32];
                        self.display.clear();
                    },
                    _ => panic!("Unsupported opcode. {:#x}", instruction)
                }
            },
            0xA000 => {
                let low = instruction & 0x0FFF;
                self.i_register = low;
            },
            0xB000 => {
                let low = 0x0FFF & instruction;
                let computed_low = low.wrapping_add(self.get_v0());
                self.program_counter = computed_low;
            },
            0xC000 => {
                let low = 0x00FF & instruction;
                let register = (instruction & 0x0F00) >> 8;
                
                let rand = self.random(0xFF);
                let val = rand & low;

                self.set_vx(val, register as usize);
            },
            	// Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
                // Set VF to 01 if any set pixels are changed to unset, and 00 otherwise
            0xD000 => {

                let from = self.i_register as usize;
                let op_n = 0x000F & instruction;
                let op_x = (0x0F00 & instruction) >> 8;
                let op_y = (0x00F0 & instruction) >> 4;

                let to = (from + op_n as usize) as usize;

                let x = self.get_vx(op_x as usize);
                let y = self.get_vx(op_y as usize);
                let val = self.display.draw(x as usize, y as usize, &self.memory[from..to]);
                self.set_vx(val as u32, 0xf);
            },
            0xE000 => {
                let low = instruction & 0x00FF;
                let register = (instruction & 0x0F00) >> 8;

                self.program_counter += match low {
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
                    0x55 => {
                        for i in 0..register + 1{
                            self.memory[(self.i_register + i) as usize] = self.get_vx(i as usize) as u8;
                        }
                    },
                    0x65 => {
                        for i in 0..register + 1 {
                            self.set_vx(self.memory[(self.i_register + i) as usize] as u32, i as usize)
                        }                            

                    },
                    0x18 => {
                        self.sound_timer = self.get_vx(register as usize);
                    },
                    0x0A => {
                        let mut broken = false;
                        for i in 0..16 {
                            if self.input.pressed(i as usize) {
                                self.set_vx(i as u32, register as usize);
                                broken = true;
                                break;
                            }
                        }
                        if broken == true {
                            println!("BROKEN");
                            self.program_counter -= 0x2;
                        }
                    },
                    0x07 => {
                        self.set_vx(self.delay_timer, register as usize);
                    },
                    0x29 => {
                        let op_x = register;
                        self.i_register = self.get_vx(op_x as usize) * 5;
                    },
                    0x33 => {
                        let value = self.get_vx(register as usize);
                        self.memory[self.i_register as usize] = (value / 100) as u8;
                        self.memory[(self.i_register + 1) as usize] = (((value) % 100) / 10) as u8;
                        self.memory[(self.i_register + 2) as usize] = (((value) % 100) % 10) as u8;
                    },
                    0x1E => {
                        self.i_register = self.i_register + self.get_vx(register as usize);
                    },
                    _ => println!("Unsupported opcode. {:#x}", instruction)
                }
            },
            _ => panic!("Unsupported opcode. {:#x}", instruction)
        }
    }
}