///
/// The general trait to process opcodes.
/// 
/// ## A NOTE ABOUT CERTAIN OPCODES
/// There are actually 2 different Chip8 specifications which behave differently
/// for certain opcodes. These specifications go by many different names, but
/// here the more popular and well-known will be called the "CowGod" specification 
/// (after the website from which many have come to know it) while the other 
/// will be refered to as the "Legacy" specification.
/// Note that in this code base the opcodes are processed via the "Cowsay"
/// specification by default.
pub trait OpcodeExecuter {

    // Instruction process functions 

    ///
    /// Sets the display buffer to all 0 and redraws the screen.
    fn clear_screen(&mut self) ;

    ///
    /// Pops the top value of the stack pointer and puts it into the program counter.
    fn ret(&mut self) ;

    ///
    /// Jumps to ```addr```.
    fn jump(&mut self, addr : u16) ;

    ///
    /// Stores the current program counter in the stack and jumps to ```addr```.
    fn call(&mut self, addr : u16) ;

    ///
    /// If the value in register ```register``` is equal to ```byte```, then 
    /// the program counter is incremented, skipping over the next instruction.
    fn skip_if_equal_const(&mut self, register : usize, byte : u8) ; 
    
    ///
    /// If the value in register ```register``` is NOT equal to ```byte```, then 
    /// the program counter is incremented, skipping over the next instruction.
    fn skip_if_unequal_const(&mut self, register : usize, byte : u8) ;


    ///
    /// If the value in register ```register1``` is equal to the value in register ```register2``` 
    /// then the program counter is incremented, skipping over the next instruction.
    fn skip_if_equal_reg(&mut self, register1 : usize, register2 : usize) ; 

    ///
    /// Sets the value of register ```register``` to the value ```byte```.
    fn load_const(&mut self, register : usize, byte : u8) ;
    
    ///
    /// Adds the value ```byte``` to the value in register ```register``` and 
    /// stores the new value into ```register```..
    fn add_const(&mut self, register : usize, byte : u8) ;

    ///
    /// Copies the value in register ```reg``` into register ```acc```. 
    fn load_register(&mut self, acc : usize, reg : usize) ; 

    ///
    /// Sets the value in register ```acc``` to itself bitwise-ORed with the
    /// value in register ```reg```. 
    fn or_register(&mut self, acc : usize, reg : usize) ; 
    
    ///
    /// Sets the value in register ```acc``` to itself bitwise-ANDed with the
    /// value in register ```reg```. 
    fn and_register(&mut self, acc : usize, reg : usize) ;
    
    ///
    /// Sets the value in register ```acc``` to itself bitwise-XORed with the
    /// value in register ```reg```. 
    fn xor_register(&mut self, acc : usize, reg : usize) ;
    
    /// Sets the value in register ```acc``` to itself plus the
    /// value in register ```reg```. 
    /// 
    /// Register ```0xF``` is then set to 1 if the add caused an overflow, 0 otherwise.
    fn add_register(&mut self, acc : usize, reg : usize) ;
    
    /// Sets the value in register ```acc``` to itself minus the
    /// value in register ```reg```. 
    /// 
    /// Register ```0xF``` is then set to 0 if the minus caused an underflow, 1 otherwise.
    fn sub_register(&mut self, acc : usize, reg : usize) ;

    ///
    /// In the "CowGod" spec the value in register ```0xF``` (15) is 
    /// set to the lowest bit of the value of register ```acc``` and then the value
    /// in register ```acc```is divided by 2. 
    /// The "Legacy" spec states that instead of register ```acc```, 
    /// the value in register ```0xF``` is set to the lowest bit of the value in
    /// register ```reg```, the value of ```reg``` is divided by 2, and the freshly
    /// divided number is then copied into register ```acc```.
    fn right_shift_register(&mut self, acc : usize, reg : usize) ;
    
    /// Sets the value in register ```acc``` to 
    /// value in register ```reg``` minus itself.
    /// 
    /// Register ```0xF``` is then set to 0 if the minusd caused an underflow, 1 otherwise.
    fn rev_sub_register(&mut self, acc : usize, reg : usize) ;

    ///
    /// In the "CowGod" spec the value in register ```0xF``` (15) is 
    /// set to 1 if the highest bit of the value of register ```acc```  is 1, and 0 if it isnt.
    /// Then the value in register ```acc```is divided by 2. 
    /// The "Legacy" spec states that instead of register ```acc```, 
    /// the value in register ```0xF``` is set to 1 if the highest bit of the value in
    /// register ```reg``` is 1 and 0 otherwise. The value of ```reg``` is then 
    /// divided by 2, and the freshly divided number is then copied into register ```acc```.
    fn left_shift_register(&mut self, acc : usize, reg : usize) ;

    ///
    /// If the value in register ```register1``` is NOT equal to the value in register ```register2``` 
    /// then the program counter is incremented, skipping over the next instruction.
    fn skip_if_unequal_reg(&mut self, register1 : usize, register2 : usize) ;

    ///
    /// Sets the value in the address pointer ```I```to the constant value
    /// ```addr```.
    fn load_addr_const(&mut self, addr : u16) ;

    ///
    /// Sets the program counter to the value in register ```0x0```
    /// plus the value ```addr```.
    fn add_jump_v0(&mut self, addr_offset : u16) ;

    ///
    /// Sets the value stored in register ```reg``` to a pseudo-random number
    /// which is then bitwise-ANDed with ```mask```. 
    fn randomize(&mut self, reg : usize, mask : u8) ;

    ///
    /// Draws a sprite whose top left corner starts offset from the left
    /// edge by the value stored in ```xreg``` pixels, offset from the top
    /// by the value stored in ```yreg``` pixel, whose width is 8 pixels
    /// and whose height is equal to ```length```. 
    /// 
    /// The sprite is drawn by reading ```length``` bytes of memory, starting
    /// at the addres pointer ```I```. Each byte represents a row of pixels, 
    /// and each bit within the byte represents a single pixel; for example,
    /// if we wanted to draw a box 8 pixels wide and 5 high in the bottom right corner,
    /// we would set the value in ```xreg``` to 55 (63 - 8 for the box), the value
    /// in ```yreg``` to 26 (31 - 5 for the height), and length to 5. We would 
    /// then make sure that ```self.I[0 .. 5]``` is equal to ```[0xFF, 0x81, 0x81, 0x81, 0xFF]```,
    /// since ```0xFF``` has all bits set to 1 and ```0x81``` has only the outer bits set to 1. 
    fn draw_sprite(&mut self, xreg : usize, yreg : usize, length : u8);

    ///
    /// Skips the next instruciton if the key whose value is stored in ```reg``` is pressed;
    /// otherwise do nothing.
    fn skip_if_key_pressed(&mut self, reg : usize) ;
    
    ///
    /// Skips the next instruciton if the key whose value is stored in ```reg``` is NOT pressed;
    /// otherwise do nothing.
    fn skip_if_key_not_pressed(&mut self, reg : usize) ;

    ///
    /// Sets the value in register ```reg``` to the current number stored in the 
    /// external, non-audio timer.
    fn load_timer(&mut self, reg : usize) ;

    ///
    /// Pauses all CPU instructions until a key is pressed. The value of this
    /// key is then stored into register ```reg```.
    fn wait_for_key(&mut self, reg : usize) ;

    ///
    /// Sets the value in the external timer to the value stored in register ```reg```.
    fn set_timer (&mut self, reg : usize) ;

    ///
    /// Sets the audio timer to the value stored in register ```reg```.
    fn set_audio(&mut self, reg : usize) ; 

    ///
    /// Adds the value stored in register ```reg``` to the address pointer ```I```.
    fn add_addr_reg(&mut self, reg : usize) ;

    ///
    /// Sets the address pointer ```I``` to 5 * the value stored in register ```reg```.
    /// The chip 8 is set to automatically load in a simplistic fontset for the hex
    /// characters ```0 - F```, in order, at memory locations ```0x0000 - 0x2000`, 
    /// meaning after this instruction ```I``` will point to the sprite coresponding
    /// to the value stored in ```reg```. 
    fn set_addr_to_char(&mut self, reg : usize) ;
    
    ///
    /// Loads the value at the memory location ```self.I + x``` into register
    /// ```x```, for all ```x``` between ```0``` and ```reg```, inclusive.
    /// 
    /// In the ```LEGACY``` spec ```I``` is then incremented by ```reg + 1```,
    /// which is the first byte *after* the loaded values.
    fn store_digits(&mut self, reg : usize) ;

    ///
    /// Stores the values currently stored in registers ```0``` to ```reg```,
    /// inclusive, at ```reg```-sized block of memory starting at the address
    /// pointer ```I```. 
    /// 
    /// In the ```LEGACY``` spec ```I``` is then incremented by ```reg + 1```,
    /// which is the first byte *after* the stored values.
    fn save_registers(&mut self, reg : usize) ;

    ///
    /// Sets the values in registers ```0``` to ```reg``` to the values at
    /// memory location ```self.I + 0```, ```self.I + 1```, ... ```self.I + reg```, 
    /// where ```I``` is the CPU's 16-bit address pointer. 
    /// 
    /// In the "Legacy" spec, the address pointer is then increased by ```reg + 1```. 
    fn restore_registers(&mut self, reg : usize) ; 

    //Once per frame helper functions

    ///
    /// If necessary, decrements the values in the audio timer and other timer
    /// registers. 
    fn tick(&mut self, delta_t_in_ns : u64) ;

    ///
    /// Gets the value of the next instruction to be passed to ```process_instruction```.
    /// Runs at the beginning of the clock cycle.
    fn get_next_instr(&self) -> u16 ; 
    
    ///
    /// Ran at the end of the clock cycle. Does things like increment the program
    /// counter, etc. 
    fn end_frame(&mut self) ;

    //Misc management functions

    ///
    /// Halts the CPU and stops processing all instruction.
    /// The regular Chip8 does not have this as a specific instruction, but
    /// this may be called in the case of a bad opcode value being read.
    fn die(&mut self) ;

    ///
    /// Checks if the CPU has had the method ```die()``` called.
    /// Not used in the virtual Chip8 itself, but by the emulator to close
    /// the program if the CPU hangs.
    fn has_died(&self) -> bool;

    ///
    /// Loads a ROM into memory. The rom will be places into memory offset 
    /// by ```0x200```, or 512, due to the default fontset; in effect, this means
    /// that ```self.memory[0x200] = rom[0]```, ```self.memory[0x200 + 1] = rom[1]```, etc.
    fn load_rom(&mut self, rom : &[u8]) ;  

    ///
    /// Resets the CPU to its initial state. 
    /// All registers, timers, stack values, and screen buffer values will be set to 0.
    /// The memory will be cleared except for the default fontset at the beginning of the memory.
    fn reset(&mut self) ; 


    ///
    /// Decodes a raw opcode value into its constituent parts,
    /// figures out what function call and parameters that corresponds to,
    /// and calls the function with the correct parameters. 
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

///
/// An enum to set the CPU's instruction set between COWGOD, which is more
/// popular, and LEGACY, which is official. 
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