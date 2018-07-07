extern crate sdl2;
use sdl_mod::sdl2::keyboard::Keycode;
use sdl_mod::sdl2::video::Window;
use sdl_mod::sdl2::render::Canvas;
use sdl_mod::sdl2::EventPump;
use sdl_mod::sdl2::pixels::Color;
use sdl_mod::sdl2::rect::Rect;
use sdl_mod::sdl2::event::Event;

use chip8_mod::display::{DisplayOutput, SCREEN_HEIGHT, SCREEN_WIDTH};
use chip8_mod::audio::AudioOutput;
use chip8_mod::input::InputReciever;

const DEFAULT_KEY_CONFIG : [Keycode; 0x10] = [
    Keycode::X, 
    Keycode::Num1, Keycode::Num2, Keycode::Num3, 
    Keycode::Q, Keycode::W, Keycode::E, 
    Keycode::A, Keycode::S, Keycode::D,
    Keycode::Z, Keycode::C, 
    Keycode::Num4, Keycode::R, Keycode::F, Keycode::V,
];

pub struct Config {
    chip8_keys : [Keycode ; 0x10],
}

pub struct SdlKeyProcessor {
    key_map : [Keycode ; 0x10],
    key_buffer : [bool ; 0x10], 
    event_pump : EventPump,
    pub has_quit : bool,
}

pub struct SdlDisplayProcessor {
    canvas : Canvas<Window>, 
}

pub struct SdlAudioProcessor {
    count : u8,
}

pub struct SdlRunner {
    pub conf : Config, 
    pub video : SdlDisplayProcessor, 
    pub audio : SdlAudioProcessor, 
    pub keys : SdlKeyProcessor, 
}

impl SdlRunner {
    pub fn new() -> SdlRunner {
        let sdl_context = sdl2::init().unwrap();
        let video_sys = sdl_context.video().unwrap();
        let window_obj = video_sys.window("CHIP8 EMU", 800, 600)
            .build()
            .unwrap();
        let canvas_obj = window_obj.into_canvas().build().unwrap();
        SdlRunner {
            conf : Config {
                chip8_keys : DEFAULT_KEY_CONFIG,
            },
            video : SdlDisplayProcessor {
                canvas : canvas_obj, 
            },
            audio : SdlAudioProcessor {
                count : 0,
            },
            keys : SdlKeyProcessor {
                key_map : DEFAULT_KEY_CONFIG,
                key_buffer : [false ; 0x10],
                has_quit : false, 
                event_pump : sdl_context.event_pump().unwrap()
            },

        }
    }

}

impl DisplayOutput for SdlDisplayProcessor {
    fn display_buffer (&mut self, buffer : &[u8 ; SCREEN_WIDTH/8 * SCREEN_HEIGHT]) {
        println!("PRINTING:");
        for (idx, byte) in buffer.into_iter().enumerate() {
            if idx % (SCREEN_WIDTH /8) == 0 {
                println!("");
            }
            for mask_num in 0 .. 8 {
                let mask = 1 << (7 - mask_num);
                let pval = if byte & mask != 0 { "1" } else { "0" };
                print!("{}", pval);
            }
        }
        self.canvas.set_draw_color(Color::RGB(255,255,255));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(0,0,0));

        let (width, height) = self.canvas.output_size().unwrap();
        let pixel_width = width/SCREEN_WIDTH as u32;
        let pixel_height = height/SCREEN_HEIGHT as u32;
        

        let mut x = 0;
        let mut y = 0;

        for (idx, byte) in buffer.into_iter().enumerate() {
            if(idx == 0) { }
            else if(idx %(SCREEN_WIDTH/8) == 0) {
                y += pixel_height;
                x = 0;
            }
            else {
                x += pixel_width * 8;
            }
            for mask_num in 0 .. 8 {
                let mask = 1 << (7 - mask_num);
                let pval = byte & mask != 0;
                if(pval) {
                    let offset : i32 = (mask_num * pixel_width) as i32;
                    let _res = self.canvas.fill_rect(
                        Rect::from((offset + x as i32, y as i32, pixel_width, pixel_height))
                    );
                }
            }
        }
        self.canvas.present();
    }
}

impl SdlKeyProcessor {
    fn process_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | 
                Event::MouseButtonDown { .. } | 
                Event::AppTerminating{ .. }  => {
                    println!("DYING!");
                    self.has_quit = true;
                },
                Event::KeyDown { keycode : Some(code), ..} => {
                    for buffer_idx in 0 .. 0x10 {
                        if self.key_map[buffer_idx] == code {
                            self.key_buffer[buffer_idx] = true;
                        }
                    }
                },
                Event::KeyUp { keycode : Some(code), .. } => {
                    for buffer_idx in 0 .. 0x10 {
                        if self.key_map[buffer_idx] == code {
                            self.key_buffer[buffer_idx] = false;
                        }
                    }
                },
                _ => ()
            }
        }
    }
}

impl InputReciever for SdlKeyProcessor {
    fn check_key(&mut self, key : u8) -> bool {
        self.process_events();
        self.key_buffer[key as usize]
    }
    fn check_any_key(&mut self) -> Option<u8> {
        self.process_events();
        for idx in 0x00 .. 0x10 {
            if self.key_buffer[idx] {
                return Some(idx as u8);
            }
        }
        None
    }

    fn check_should_die(&mut self) -> bool {
        self.process_events();
        self.has_quit
    }
}

impl AudioOutput for SdlAudioProcessor {
    fn output_audio (&mut self) {
        println!("AUDIO: {}", self.count);
        self.count += 1;
    }
}