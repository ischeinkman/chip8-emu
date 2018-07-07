mod chip8_mod;
mod sdl_mod;

use chip8_mod::InterpretedCpu::InterpretedCpu;
use chip8_mod::cpu::OpcodeExecuter;
use chip8_mod::audio::AudioTimer;
use chip8_mod::display::ScreenBuffer;

use std::env;
use std::fs::File;
use std::io::{Read};
use std::time::SystemTime;

const TEST_ROM_1 : [u8 ; 34] = [

    0x65, 0x0A, //Run for 10 seconds

    // Get the location of E
    0x60, 0x0E,  // Store the argument in V0
    0xF0, 0x29,  // Set I to the memory location of E

    //OUTER LOOP LABEL

    // Set the delay at 1 second
    0x60, 0x3c, // Load the value 60 into V0
    0xF0, 0x15, // Set the delay timer = V0

    //INNER LOOP LABEL
    
    0x00, 0xE0, // Clear the Screen

    // Draw E
    0x60, 0x00,   // Zero out reg 0
    0x61, 0x00,   // Zero out reg 1
    0xD0, 0x15,   // Draw the E at 0, 0

    // Check the inner loop
    0xF0, 0x07,  //Load the timer into V0
    0x30, 0x00,  //Don't jump if V0 == 0
    0x12, 0x0A,  // Jump to the clear screen instruction, 10 bytes after start

    //Decrement V5
    0x68, 0x01, //Store 1 into V8 since we dont have a SUB CONST instruciton
    0x85, 0x85, // V5 = V5 - V8


    0x35, 0x00,  // If we ran for the wanted number of seconds, end
    0x12, 0x06,   // Otherwise, jump 

    0xFF, 0xFF, //DIE
];

fn main() {
    println!("STARTING EMU!");

    let mut flag = false;

    let mut window = sdl_mod::SdlRunner::new();
    let mut cpu = InterpretedCpu::new(
        ScreenBuffer::new(&mut window.video), 
        AudioTimer::new(&mut window.audio),
        &mut window.keys
    );

    let args : Vec<String> = env::args().collect();
    if(args.len() > 1) {
        let rompath = &args[args.len() - 1];
        println!("USING ROMPATH: {}", rompath);
        let mut buffer = Vec::new();
        let mut file = File::open(rompath).unwrap();
        file.read_to_end(&mut buffer).unwrap();
        println!("FINISHED READING FILE");
        cpu.load_rom(&buffer);
    }
    else {
        cpu.load_rom(&TEST_ROM_1);
    }

    let mut prevtime = SystemTime::now();
    while !cpu.has_died() && cpu.pc < 4094{
        println!("STARTING FRAME");
        let next_instr = cpu.get_next_instr();
        println!("OP: {:#X}", next_instr);
        println!("CPU: {}", cpu);
        cpu.process_instruction(next_instr);
        let curtime = SystemTime::now();
        let nsecs = curtime.duration_since(prevtime).unwrap().subsec_nanos();
        cpu.tick(nsecs as u64);
        prevtime = curtime;
        cpu.end_frame();
        println!("ENDING FRAME");
        let die = cpu.keyboard_input.check_should_die();
        if(die) {
            break;
        }
    }
}
