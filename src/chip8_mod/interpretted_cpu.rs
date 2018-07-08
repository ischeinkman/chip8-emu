use chip8_mod::cpu::OpcodeExecuter;
use chip8_mod::display::{ScreenBuffer, SCREEN_HEIGHT, SCREEN_WIDTH};
use chip8_mod::audio::AudioTimer;
use chip8_mod::input::InputReciever;
use chip8_mod::default_fontset::*;

use std::fmt;

extern crate rand;

const NANO_BETWEEN_TICKS : u64 = (1000 * 1000 * 1000)/60; // Equal to 60 Hz

pub struct InterpretedCpu <'a>  {
    pub pc : u16, 
    pub registerV : [u8 ; 16],
    pub I : u16, 

    pub stack : [u16 ; 24],
    pub sp : usize, 

    pub timer : u8,
    pub ns_since_last_tick : u64,


    pub memory : [u8 ; 4096],

    pub display_output : ScreenBuffer<'a>,
    pub audio_output : AudioTimer<'a>,
    pub keyboard_input : &'a mut (InputReciever + 'a),
    pub dead : bool,
}

impl <'a> InterpretedCpu <'a> {
    pub fn new(disp : ScreenBuffer<'a>, audp : AudioTimer<'a>, keyb : &'a mut (InputReciever + 'a)) -> InterpretedCpu<'a> {
        let mut rval = InterpretedCpu {
            pc : 0x0200,
            registerV : [0 ; 16],
            I : 0,

            stack : [0 ; 24],
            sp : 0,

            timer : 0,
            ns_since_last_tick : 0,

            memory : [0 ; 4096],

            display_output : disp, 
            audio_output : audp, 
            keyboard_input : keyb,
            dead : false,
        };
        rval.initialize_memory();
        rval
    }
    fn initialize_memory(&mut self) {
        self.memory = [0 ; 4096];
        for letter in 0x00 .. 0x10  {
            let offset = letter * 5;
            let letter_bits = get_raw_char(letter as u8);
            self.memory[offset+0] = letter_bits[0];
            self.memory[offset+1] = letter_bits[1];
            self.memory[offset+2] = letter_bits[2];
            self.memory[offset+3] = letter_bits[3];
            self.memory[offset+4] = letter_bits[4];
        }
    }
}

impl <'a> OpcodeExecuter for InterpretedCpu <'a> {

    fn load_rom(&mut self, rom : &[u8]) {
        for(idx, byte) in rom.into_iter().enumerate() {
            debug_log!("PUTTING BYTE {:#X} INTO SLOT {:#X}", byte, 0x200 + idx);
            self.memory[0x200 + idx] = *byte;
        }
    }

    fn tick(&mut self, ns_since_last_frame : u64) {
        self.ns_since_last_tick += ns_since_last_frame;
        debug_log!("NS SINCE LAST: {}", self.ns_since_last_tick);
        if self.ns_since_last_tick >= NANO_BETWEEN_TICKS {
            debug_log!("TICKING: {} > {}", self.ns_since_last_tick, NANO_BETWEEN_TICKS);
            if self.timer > 0 {
                self.timer -= 1;
            }
            self.audio_output.tick();
            self.ns_since_last_tick -= NANO_BETWEEN_TICKS;
        }
    }

    fn reset(&mut self) {
        self.pc = 0x0200;
        self.registerV = [0 ; 16];
        self.I = 0;
        self.stack = [0 ; 24];
        self.sp = 0;
        self.timer = 0;
        self.ns_since_last_tick = 0;
        self.memory = [0 ; 4096];
        self.dead = false;
        self.initialize_memory();
    }

    fn get_next_instr(&self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16
    }

    fn has_died(&self) -> bool {
        self.dead
    }

    fn die(&mut self) {
        self.dead = true;
    }

    fn end_frame(&mut self) {
        self.pc += 2;
    } 
    
    fn clear_screen(&mut self) {
        self.display_output.clear_screen()
    }

    fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }
    fn jump(&mut self, addr : u16) {
        self.pc = addr - 2; //To counter the next progression
    }
    fn call(&mut self, addr : u16) {
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = addr - 2; //To counter the next progression
    }

    fn skip_if_equal_const(&mut self, register : usize, byte : u8) {
        if self.registerV[register] == byte {
            self.pc += 2;
        }
    }
    fn skip_if_unequal_const(&mut self, register : usize, byte : u8) {
        if self.registerV[register] != byte {
            self.pc += 2;
        }
    }
    fn skip_if_equal_reg(&mut self, register1 : usize, register2 : usize) {
        if self.registerV[register1] == self.registerV[register2] {
            self.pc += 2;
        }
    }

    fn load_const(&mut self, register : usize, byte : u8) {
        self.registerV[register] = byte;
    }

    fn add_const(&mut self, register : usize, byte : u8) {
        self.registerV[register] += byte;
    } 

    fn load_register(&mut self, acc : usize, reg : usize) {
        self.registerV[acc] = self.registerV[reg];
    } 
    fn or_register(&mut self, acc : usize, reg : usize) {
        self.registerV[acc] |= self.registerV[reg];
    }
    fn and_register(&mut self, acc : usize, reg : usize) {
        self.registerV[acc] &= self.registerV[reg];
    }
    fn xor_register(&mut self, acc : usize, reg : usize) {
        self.registerV[acc] ^= self.registerV[reg];
    }
    fn add_register(&mut self, acc : usize, reg : usize) {
        let (value, overflowed) = self.registerV[acc].overflowing_add(self.registerV[reg]);
        self.registerV[0xF] = if overflowed { 1 } else { 0 };
        self.registerV[acc] = value;
    }
    fn sub_register(&mut self, acc : usize, reg : usize) {
        let (value, overflowed) = self.registerV[acc].overflowing_sub(self.registerV[reg]);
        self.registerV[0xF] = if overflowed { 1 } else { 0 };
        self.registerV[acc] = value;
    }
    fn right_shift_register(&mut self, acc : usize, reg : usize) { 
        self.registerV[acc] = self.registerV[reg] >> 1;
        self.registerV[0xF] = self.registerV[reg] & 0x01;
    }
    fn rev_sub_register(&mut self, acc : usize, reg : usize) {
        let (value, overflowed) = self.registerV[reg].overflowing_sub(self.registerV[acc]);
        self.registerV[0xF] = if overflowed { 1 } else { 0 };
        self.registerV[acc] = value;
    }
    fn left_shift_register(&mut self, acc : usize, reg : usize) { 
        self.registerV[0xF] = self.registerV[reg] & 0x80;
        self.registerV[reg] = self.registerV[reg] << 1;
        self.registerV[acc] = self.registerV[reg];
    }
    fn skip_if_unequal_reg(&mut self, register1 : usize, register2 : usize) {
        if self.registerV[register1] != self.registerV[register2] {
            self.pc += 2;
        }
    }

    fn load_addr_const(&mut self, addr : u16) {
        self.I = addr;
    }
    fn add_jump_v0(&mut self, addr_offset : u16) { 
        let next_adder = self.registerV[0x00] as u16 + addr_offset;
        self.jump(next_adder);
    }
    fn randomize(&mut self, reg : usize, mask : u8) { 
        self.registerV[reg] = rand::random::<u8>() & mask;
    }
    fn draw_sprite(&mut self, xreg : usize, yreg : usize, length : u8) {
        let x = self.registerV[xreg];
        let y = self.registerV[yreg];
        let sprite = &self.memory[self.I as usize .. (self.I + length as u16) as usize];
        debug_log!("CPU Draw sprite using {} => {}, {} => {}, length {}.", xreg, x, yreg, y, length);
        if x + 8 >= SCREEN_WIDTH as u8 || y + length >= SCREEN_HEIGHT as u8 {
            error_log!("Bad draw dims: ({} -> {}) by ({} -> {}).\nCPU: {}", x, x+8, y, y + length, self);
        }
        self.registerV[0xF] = if self.display_output.put_sprite(x, y, &sprite) { 1 } else { 0 };
    }
    fn skip_if_key_pressed(&mut self, reg : usize) {
        let key = self.registerV[reg];
        if self.keyboard_input.check_key(key) {
            self.pc += 2;
        }
    }
    fn skip_if_key_not_pressed(&mut self, reg : usize) {
        let key = self.registerV[reg];
        if !self.keyboard_input.check_key(key) {
            self.pc += 2;
        }
    }

    fn load_timer(&mut self, reg : usize) { 
        self.registerV[reg] = self.timer;
    }
    fn wait_for_key(&mut self, reg : usize) {
        match self.keyboard_input.check_any_key() {
            Some(key) => self.registerV[reg] = key,
            None => self.pc -= 1
        }
    }
    fn set_timer (&mut self, reg : usize) {
        self.timer = self.registerV[reg];
    }
    fn set_audio(&mut self, reg : usize) {
        self.audio_output.set_timer(self.registerV[reg]);
    }
    fn add_addr_reg(&mut self, reg : usize) { 
        self.I += self.registerV[reg] as u16;
    }
    fn set_addr_to_char(&mut self, reg : usize) { 
        self.I = (self.registerV[reg] * 5) as u16;
     }
    fn store_digits(&mut self, reg : usize) {
        self.memory[self.I as usize] = self.registerV[reg] / 100;
        self.memory[self.I as usize + 1] = (self.registerV[reg] / 10) % 10;
        self.memory[self.I as usize + 2] = self.registerV[reg] % 10;
    }
    fn save_registers(&mut self, reg : usize) {
        for regnum in 0 .. reg + 1 {
            self.memory[self.I as usize] = self.registerV[regnum];
            self.I += 1;
        }
    }
    fn restore_registers(&mut self, reg : usize) {
        for regnum in 0 .. reg + 1 {
            self.registerV[regnum] = self.memory[self.I as usize];
            self.I += 1;
        }
    }

}

impl <'a> fmt::Display for InterpretedCpu<'a> {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{{ pc: {:#X}, I : {:#X}, reg: {:?}, timer : {}, offset : {} }}", self.pc, self.I, self.registerV, self.timer, self.ns_since_last_tick)
    }
}