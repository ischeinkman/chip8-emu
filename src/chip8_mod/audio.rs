
pub trait AudioOutput {
    fn output_audio (&mut self) ;
}

pub struct AudioTimer<'a> {
    time : u8,
    audio_output : &'a mut (AudioOutput + 'a)
}

impl <'a> AudioTimer <'a> {
    pub fn new(output : &'a mut AudioOutput) -> AudioTimer<'a> {
        AudioTimer {
            time : 0,
            audio_output : output
        }
    }

    pub fn set_timer(&mut self, ntime : u8) {
        self.time = ntime;
    }

    pub fn tick(&mut self) {
        if self.time != 0 {
            self.audio_output.output_audio();
            self.time -= 1;
        }
    }
}