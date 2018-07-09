#[macro_use]
mod utils_mod;

mod chip8_mod;
mod sdl_mod;


use chip8_mod::*;
use chip8_mod::cpu::{OpcodeExecuter, InstructionSet};
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
    error_log!("Error logging started!");
    debug_log!("Debug logging started!");

    let args : Vec<String> = env::args().collect();

    let mut min_frame_time : u32 = 0; //default to maxing at the maximum FPS
    let mut rompath : &str = "";
    let mut legacy = false;

    let mut arg_idx = 1;
    while arg_idx < args.len() {
        let to_proc = args[arg_idx].as_ref();
        match to_proc {
            "--fps" => {
                arg_idx += 1;
                let max_fps = args[arg_idx].parse::<u32>().unwrap();
                min_frame_time = (1000 * 1000 * 1000) / max_fps;
                debug_log!("Setting max fps to {}, meaning min_dt = {}.", max_fps, min_frame_time);
            }
            "--legacy" => {
                legacy = true;
                debug_log!("Set legacy to ON.");
            }
            _ => {
                rompath = &args[arg_idx];
            }
        };
        arg_idx += 1;
    } 

    let mut window = sdl_mod::SdlRunner::new();
    let mut cpu = InterpretedCpu::new(
        if legacy { InstructionSet::LEGACY } else { InstructionSet::COWGOD },
        ScreenBuffer::new(&mut window.video), 
        AudioTimer::new(&mut window.audio),
        &mut window.keys
    );

    if !rompath.is_empty() {
        debug_log!("USING ROMPATH: {}", rompath);
        let mut buffer = Vec::new();
        let mut file = File::open(rompath).unwrap();
        file.read_to_end(&mut buffer).unwrap();
        debug_log!("FINISHED READING FILE");
        cpu.load_rom(&buffer);
    }
    else {
        cpu.load_rom(&TEST_ROM_1);
    }

    let mut prevtime = SystemTime::now();
    while !cpu.has_died() && cpu.pc < 4094{
        debug_log!("STARTING FRAME");

        //Run the next instruction
        let next_instr = cpu.get_next_instr();
        debug_log!("OP: {:#X}", next_instr);
        debug_log!("CPU: {}", cpu);
        cpu.process_instruction(next_instr);

        //Update the timing
        let mut curtime = SystemTime::now();
        let mut nsecs = curtime.duration_since(prevtime).unwrap().subsec_nanos();
        while nsecs < min_frame_time {
            curtime = SystemTime::now();
            nsecs = curtime.duration_since(prevtime).unwrap().subsec_nanos();
        }
        if cfg!(feature="log_fps") {
            println!("FPS: {}", (1000 * 1000 * 1000)/nsecs);
        }
        prevtime = curtime;
        cpu.tick(nsecs as u64);

        //End the frame
        cpu.end_frame();
        debug_log!("ENDING FRAME");

        //Check for emulation end
        let die = cpu.keyboard_input.check_should_die();
        if die {
            break;
        }
    }
}
