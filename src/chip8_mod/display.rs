


pub const SCREEN_WIDTH : usize = 64;
pub const SCREEN_HEIGHT : usize = 32;


pub trait DisplayOutput {
    fn display_buffer (&mut self, buffer : &[u8 ; SCREEN_WIDTH/8 * SCREEN_HEIGHT]) ;
}

pub struct ScreenBuffer <'a> {
    pub packed_pixels : [u8 ; SCREEN_WIDTH/8 * SCREEN_HEIGHT],
    pub display_output : &'a mut (DisplayOutput + 'a)
}

impl <'a> ScreenBuffer <'a> {

    pub fn new (disp : &'a  mut DisplayOutput) -> ScreenBuffer<'a> {
        ScreenBuffer {
            packed_pixels : [0 ; SCREEN_WIDTH/ 8 * SCREEN_HEIGHT],
            display_output : disp
        }
    }

    pub fn clear_screen(&mut self) {
        self.packed_pixels = [0; SCREEN_WIDTH/8 * SCREEN_HEIGHT];
        self.display_output.display_buffer(&self.packed_pixels);
    }

    pub fn put_sprite(&mut self, x : u8,y : u8, sprite : &[u8]) -> bool {

        debug_log!("Got put request at {}, {} length {}.", x, y, sprite.len());
        if x % 8 == 0 {
            return self.put_sprite_simple(x, y, sprite);
        }


        let mut collided = false;
        let mut needs_draw = false;
        
        let offset = x % 8;
        let high_mask = 0xFF << (8 - offset);
        let low_mask = !high_mask;
        for (row_count, row_pixels) in sprite.iter().enumerate() {
            let cur_pos = (x as usize/8 + (y as usize + row_count) * SCREEN_WIDTH/8) as usize;

            let left_packet = self.packed_pixels[cur_pos];
            let right_packet = self.packed_pixels[cur_pos + 1];

            let top_bits = (left_packet & low_mask) << offset;
            let bottom_bits = (right_packet & high_mask) >> (8 - offset);
            let cur_pixels = top_bits + bottom_bits;
            
            let next_pixels = cur_pixels ^ row_pixels;

            if cur_pixels != next_pixels {
                self.packed_pixels[cur_pos] = (next_pixels >> (offset)) | (left_packet & high_mask);
                self.packed_pixels[cur_pos + 1] = (next_pixels << (8 - offset)) | (right_packet & low_mask);
                collided = collided || (0 != (cur_pixels & !next_pixels));
                needs_draw = true;
            }

        }
        if needs_draw {
            if cfg!(feature="log_frames") {
                println!();
                for (idx, byte) in self.packed_pixels.into_iter().enumerate() {
                    if idx % (SCREEN_WIDTH /8) == 0 {
                        println!();
                    }
                    for mask_num in 0 .. 8 {
                        let mask = 1 << (7 - mask_num);
                        let pval = if byte & mask != 0 { "1" } else { "0" };
                        print!("{}", pval);
                    }
                }
                println!();
            }
            self.display_output.display_buffer(&self.packed_pixels);
        }

        collided
    }

    fn put_sprite_simple(&mut self, x : u8, y : u8, sprite : &[u8]) -> bool {
        let mut collided = false;
        let mut needs_draw = false;
        for (row_count, row_pixels) in sprite.iter().enumerate() {
            let x_bonus = (x as usize)/8;
            let y_bonus = ((row_count + y as usize) * SCREEN_WIDTH/8) as usize;
            let cur_pos = x_bonus + y_bonus;

            let cur_pixels = self.packed_pixels[cur_pos];
            let next_pixels = cur_pixels ^ row_pixels;

            if cur_pixels != next_pixels {
                self.packed_pixels[cur_pos] = next_pixels;
                collided = collided || (0 != (cur_pixels & !next_pixels));
                needs_draw = true;
            }
        }
        if needs_draw {
            self.display_output.display_buffer(&self.packed_pixels);
        }
        collided
    }
}