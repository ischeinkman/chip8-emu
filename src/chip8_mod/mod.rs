pub mod display;
pub mod cpu;
pub mod audio;
pub mod input;
pub mod default_fontset;

mod interpretted_cpu;
pub use self::interpretted_cpu::InterpretedCpu;

#[cfg(test)]
mod tests {
    use super::*;
    use super::cpu::*;
    use std::time::SystemTime;

    

    struct TestDisplay { 
        screen : [[bool ; display::SCREEN_WIDTH ] ; display::SCREEN_HEIGHT],

    }

    impl TestDisplay {
        fn new() -> TestDisplay {
            TestDisplay {
                screen : [[false ; display::SCREEN_WIDTH] ; display::SCREEN_HEIGHT]
            }
        }

        fn print_screen (&mut self) {
            print!("\n\n");
            for row in 0 .. display::SCREEN_HEIGHT {
                for col in 0 .. display::SCREEN_WIDTH {
                    let sym = if self.screen[row][col] { '1' } else { '0' };
                    print!("{}", sym);
                }
                print!("\n");
            }
            print!("\n\n");
        }
    }

    impl display::DisplayOutput for TestDisplay {
        fn display_buffer (&mut self, buffer : &[u8 ; display::SCREEN_WIDTH/8 * display::SCREEN_HEIGHT]) {
            for row in 0 .. display::SCREEN_HEIGHT {
                for col in 0 .. display::SCREEN_WIDTH/8 {
                    let pos = row * display::SCREEN_WIDTH/8 + col;
                    let packed = buffer[pos];
                    for bitnum in 0 .. 8 {
                        let extracted_bit = 0 != (1 << (7 - bitnum)) & packed;
                        self.screen[row][col * 8 + bitnum] = extracted_bit;
                    }
                }
            }

            self.print_screen();
        }
    }

    struct TestAudio {
        play_count : usize
    }

    impl TestAudio {
        fn new() -> TestAudio {
            TestAudio {
                play_count : 0,
            }
        }
    }

    impl audio::AudioOutput for TestAudio {
        fn output_audio(&mut self) {
            println!("\nAUDIO\n");
            self.play_count += 1;
        }
    }

    struct TestInput { }


    impl input::InputReciever for TestInput {
        fn check_any_key(&mut self) -> Option<u8> {
            Some(0)
        }

        fn check_key(&mut self, _key : u8) -> bool {
            false
        }

        fn check_should_die(&mut self) -> bool { false }
    }

    #[test]
    fn test_screen_buffer() {
        println!("Starting basic print test.");
        let mut display = TestDisplay::new();
        let mut testbuffer = display::ScreenBuffer::new(&mut display);

        println!("Clearing screen.");
        testbuffer.clear_screen();
        println!("Putting sprite.");
        testbuffer.put_sprite(0, 0, &default_fontset::RAW_7);

        for row in 0 .. 5 {
            let packed_pixels = default_fontset::RAW_7[row];
            for bitnum in 0 .. 8 {
                let mask = 1 << (7 - bitnum);
                let expected = 0 != packed_pixels & mask;

                let buffer_packed_pixels = testbuffer.packed_pixels[(row * display::SCREEN_WIDTH/8) as usize];
                let actual = 0 != buffer_packed_pixels & mask;

                assert_eq!(expected, actual, "Failed at row {} bit number {} ; expected packed is {}, actual is {}. ", row, bitnum, packed_pixels, buffer_packed_pixels);
            }
        }
        
        println!("Clearing screen.");
        testbuffer.clear_screen();
        println!("Putting sprite.");
        testbuffer.put_sprite(1, 1, &default_fontset::RAW_7);
        for row in 0 .. 5 {
            let packed_pixels = default_fontset::RAW_7[row];
            for bitnum in 0 .. 7 {
                let mask = 1 << (7 - bitnum);
                let expected = 0 != packed_pixels & mask;

                let buffer_packed_pixels = testbuffer.packed_pixels[((row + 1) * display::SCREEN_WIDTH/8) as usize];
                let actual = 0 != buffer_packed_pixels & (mask >> 1);

                assert_eq!(expected, actual, "Failed at row {} bit number {} ; expected packed is {}, actual is {}. ", row, bitnum, packed_pixels, buffer_packed_pixels);
            }
        }

        let collided = testbuffer.put_sprite(1, 1, &default_fontset::RAW_7);
        assert!(collided, "Did not detect collision!");
        assert!(!testbuffer.packed_pixels.iter().any(|pixel| *pixel != 0), "Did not collide correctly!");
    }

    #[test]
    fn test_cpu_jumping() {
        let mut display = TestDisplay::new();
        let mut audio = TestAudio::new();
        let mut inp = TestInput { };
        
        let testvbuffer = display::ScreenBuffer::new(&mut display);
        let testabuffer = audio::AudioTimer::new(&mut audio);
        let mut test_cpu = InterpretedCpu::new(testvbuffer, testabuffer, &mut inp);

        let test_simple_jump = [
            0x12, 0x08, 
            0x70, 0x01, 
            0x70, 0x01, 
            0x70, 0x01, 
            0x70, 0x01, 
            0x70, 0x01, 
            0x70, 0x01, 
        ];
        test_cpu.load_rom(&test_simple_jump);

        let mut prevtime = SystemTime::now();
        while !test_cpu.has_died() && test_cpu.pc < 4094 {
            let next_instr = test_cpu.get_next_instr();
            test_cpu.process_instruction(next_instr);
            let curtime = SystemTime::now();
            let nsecs = curtime.duration_since(prevtime).unwrap().subsec_nanos();
            test_cpu.tick(nsecs as u64);
            prevtime = curtime;
            test_cpu.end_frame();
        }

        assert_eq!(test_cpu.registerV[0], 3);

        test_cpu.reset();
        assert_eq!(test_cpu.registerV[0], 0);

        let test_disconnected_jump = [
            0x12, 0x07, 
            0x01, 
            0x70, 0x01, 
            0x70, 0x01, 
            0x70, 0x01, 
            0x70, 0x01, 
            0x70, 0x01, 

        ];
        test_cpu.load_rom(&test_disconnected_jump);

        prevtime = SystemTime::now();
        while !test_cpu.has_died() && test_cpu.pc < 4094 {
            println!("a: {}", test_cpu.pc);
            let next_instr = test_cpu.get_next_instr();
            println!("b");
            test_cpu.process_instruction(next_instr);
            let curtime = SystemTime::now();
            let nsecs = curtime.duration_since(prevtime).unwrap().subsec_nanos();
            test_cpu.tick(nsecs as u64);
            println!("c");
            prevtime = curtime;
            test_cpu.end_frame();
            println!("d");
        }

        assert_eq!(test_cpu.registerV[0], 3);
        
        
        test_cpu.reset();
        assert_eq!(test_cpu.registerV[0], 0);

        let test_conditions = [
            0x62, 0x08,  // Load 8 into V[2]
            0x12, 0x06,  // Skip the next instruction
            0x75, 0x01,  // SHOULD NOT HIT
            0x71, 0x01,  // Increment V[1]
            0x31, 0x0A,  // Don't loop if we are at 10
            0x12, 0x02,  // Jump back to the 2nd statement
        ];
        test_cpu.load_rom(&test_conditions);

        prevtime = SystemTime::now();
        while !test_cpu.has_died() && test_cpu.pc < 4094 {
            println!("a: {}", test_cpu.pc);
            let next_instr = test_cpu.get_next_instr();
            println!("b");
            test_cpu.process_instruction(next_instr);
            let curtime = SystemTime::now();
            let nsecs = curtime.duration_since(prevtime).unwrap().subsec_nanos();
            test_cpu.tick(nsecs as u64);
            println!("c");
            prevtime = curtime;
            test_cpu.end_frame();
            println!("d");
        }

        assert_eq!(test_cpu.registerV[0], 0);
        assert_eq!(test_cpu.registerV[1], 10);
        assert_eq!(test_cpu.registerV[2], 8);
        assert_eq!(test_cpu.registerV[5], 0);
    }

}