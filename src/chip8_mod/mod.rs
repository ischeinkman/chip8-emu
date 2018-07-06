pub mod display;
pub mod default_fontset;

#[cfg(test)]
mod tests {
    use super::*;

    

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

}