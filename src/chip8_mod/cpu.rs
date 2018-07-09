pub trait OpcodeExecuter {

    // Instruction process functions 

    fn clear_screen(&mut self) ;
    fn ret(&mut self) ;
    fn jump(&mut self, addr : u16) ;
    fn call(&mut self, addr : u16) ;
    fn skip_if_equal_const(&mut self, register : usize, byte : u8) ; 
    fn skip_if_unequal_const(&mut self, register : usize, byte : u8) ;
    fn skip_if_equal_reg(&mut self, register1 : usize, register2 : usize) ; 
    fn load_const(&mut self, register : usize, byte : u8) ;
    fn add_const(&mut self, register : usize, byte : u8) ; 
    fn load_register(&mut self, acc : usize, reg : usize) ; 
    fn or_register(&mut self, acc : usize, reg : usize) ; 
    fn and_register(&mut self, acc : usize, reg : usize) ;
    fn xor_register(&mut self, acc : usize, reg : usize) ;
    fn add_register(&mut self, acc : usize, reg : usize) ;
    fn sub_register(&mut self, acc : usize, reg : usize) ;
    fn right_shift_register(&mut self, acc : usize, reg : usize) ;
    fn rev_sub_register(&mut self, acc : usize, reg : usize) ;
    fn left_shift_register(&mut self, acc : usize, reg : usize) ;
    fn skip_if_unequal_reg(&mut self, register1 : usize, register2 : usize) ;
    fn load_addr_const(&mut self, addr : u16) ;
    fn add_jump_v0(&mut self, addr_offset : u16) ;
    fn randomize(&mut self, reg : usize, mask : u8) ;
    fn draw_sprite(&mut self, xreg : usize, yreg : usize, length : u8);
    fn skip_if_key_pressed(&mut self, reg : usize) ;
    fn skip_if_key_not_pressed(&mut self, reg : usize) ;
    fn load_timer(&mut self, reg : usize) ;
    fn wait_for_key(&mut self, reg : usize) ;
    fn set_timer (&mut self, reg : usize) ;
    fn set_audio(&mut self, reg : usize) ; 
    fn add_addr_reg(&mut self, reg : usize) ;
    fn set_addr_to_char(&mut self, reg : usize) ;
    fn store_digits(&mut self, reg : usize) ;
    fn save_registers(&mut self, reg : usize) ;
    fn restore_registers(&mut self, reg : usize) ; 

    //Once per frame helper functions

    fn tick(&mut self, delta_t_in_ns : u64) ;

    fn get_next_instr(&self) -> u16 ; 
    
    fn end_frame(&mut self) ;

    //Misc management functions

    fn die(&mut self) ;

    fn has_died(&self) -> bool;

    fn load_rom(&mut self, rom : &[u8]) ;  

    fn reset(&mut self) ; 

    // The instruction delegator

    fn process_instruction(&mut self, op : u16) { 

        if op == 0 {
            return;
        }
        else if op == 0x00E0 { 
            self.clear_screen() 
        } 

        else if op == 0x00EE { 
            self.ret() 
        } 

        else if op & 0xF000 == 0 {
            error_log!("GOT A ZERO OP: {:#X}", op);
        }

        else if op & 0xF000 == 0x1000 {
            self.jump(addr(op))
        }

        else if op & 0xF000 == 0x2000 {
            self.call(addr(op))
        }

        else if op & 0xF000 == 0x3000 {
            self.skip_if_equal_const(acc(op), num(op))
        }

        else if op & 0xF000 == 0x4000 {
            self.skip_if_unequal_const(acc(op), num(op))
        }

        else if op & 0xF000 == 0x5000 {
            self.skip_if_equal_reg(acc(op), reg(op))
        }

        else if op & 0xF000 == 0x6000 {
            self.load_const(acc(op), num(op))
        }

        else if op & 0xF000 == 0x7000 {
            self.add_const(acc(op), num(op))
        }

        else if op & 0xF00F == 0x8000 {
            self.load_register(acc(op), reg(op))
        }
        
        else if op & 0xF00F == 0x8001 {
            self.or_register(acc(op), reg(op))
        }
        
        else if op & 0xF00F == 0x8002 {
            self.and_register(acc(op), reg(op))
        }

        else if op & 0xF00F == 0x8003 {
            self.xor_register(acc(op), reg(op))
        }

        else if op & 0xF00F == 0x8004 {
            self.add_register(acc(op), reg(op))
        } 

        else if op & 0xF00F == 0x8005 {
            self.sub_register(acc(op), reg(op))
        }

        else if op & 0xF00F == 0x8006 {
            self.right_shift_register(acc(op), reg(op))
        }

        else if op & 0xF00F == 0x8007 {
            self.rev_sub_register(acc(op), reg(op))
        }

        else if op & 0xF00F == 0x800E {
            self.left_shift_register(acc(op), reg(op))
        }

        else if op & 0xF00F == 0x9000 {
            self.skip_if_unequal_reg(acc(op), reg(op))
        }

        else if op & 0xF000 == 0xA000 {
            self.load_addr_const(addr(op))
        }

        else if op & 0xF000 == 0xB000 {
            self.add_jump_v0(addr(op))
        }

        else if op & 0xF000 == 0xC000 {
            self.randomize(acc(op), num(op))
        }

        else if op & 0xF000 == 0xD000 {
            let rows = (op & 0x000F) as u8;
            self.draw_sprite(acc(op), reg(op), rows)
        }

        else if op & 0xF0FF == 0xE09E {
            self.skip_if_key_pressed(acc(op))
        }

        else if op & 0xF0FF == 0xE0A1 {
            self.skip_if_key_not_pressed(acc(op))
        }

        else if op & 0xF0FF == 0xF007 {
            self.load_timer(acc(op))
        }

        else if op & 0xF0FF == 0xF00A {
            self.wait_for_key(acc(op))
        }

        else if op & 0xF0FF == 0xF015 {
            self.set_timer(acc(op))
        }

        else if op & 0xF0FF == 0xF018 {
            self.set_audio(acc(op))
        }

        else if op & 0xF0FF == 0xF01E {
            self.add_addr_reg(acc(op))
        }

        else if op & 0xF0FF == 0xF029 {
            self.set_addr_to_char(acc(op))
        }

        else if op & 0xF0FF == 0xF033 {
            self.store_digits(acc(op))
        }

        else if op & 0xF0FF == 0xF055 {
            self.save_registers(acc(op))
        }        

        else if op & 0xF0FF == 0xF065 {
            self.restore_registers(acc(op))
        }

        else {
            error_log!("BAD OPCODE: {:#X}", op);
            self.die();
        }
    }
}

pub enum InstructionSet {
    LEGACY,
    COWGOD,
}


#[inline(always)]
fn addr(instruction : u16) -> u16 {
    if (instruction & 0x0FFF) % 2 == 1 {
        debug_log!("Got odd jump!");
    }
    instruction & 0x0FFF
}

#[inline(always)]
fn acc(instruction : u16) -> usize {
    ((instruction & 0x0F00) >> 8) as usize
}

#[inline(always)]
fn reg(instruction : u16) -> usize {
    ((instruction & 0x00F0) >> 4) as usize
}


#[inline(always)]
fn num(instruction : u16) -> u8 {
    (instruction & 0x00FF) as u8
}