///
/// Code related to processing audio.
/// 
/// The Chip 8 handles audio using a special external timer. This timer 
/// can be set using an opcode of the form Fx18, where x is the register
/// containing the value to set the timer to. The timer will then 
/// tick down to 0 by 1 every 1/60th of a second, and plays a constant
/// note until it hits 0. 

///
/// The interface for a frontend audio output device.
pub trait AudioOutput {

    ///
    /// Called whenever the emulator needs to play the constant tone.
    /// Note that this is called once per tick if the timer is non-zero;
    /// IE if the timer is set to 60, then this method will be called 60 times
    /// before the timer hits 0.
    fn output_audio (&mut self) ;

    ///
    /// Called the frame that the timer hits 0 for the sake of cleanup.
    fn stop_audio (&mut self) ;
}


///
/// The audio timer.
/// 
/// The lifetime parameter ```'a``` allows us to to store a reference
/// to the ```AudioOutput``` device instead of having to take ownership,
/// meaning that creation and cleanup can be handled externally.
pub struct AudioTimer<'a> {
    time : u8,
    audio_output : &'a mut (AudioOutput + 'a)
}

impl <'a> AudioTimer <'a> {

    ///
    /// Constructs a new ```AudioTimer``` from an output.
    pub fn new(output : &'a mut AudioOutput) -> AudioTimer<'a> {
        AudioTimer {
            time : 0,
            audio_output : output
        }
    }

    ///
    /// Sets the timer to ```ntime```.
    pub fn set_timer(&mut self, ntime : u8) {
        self.time = ntime;
    }

    ///
    /// If the timer is non-zero, plays a tone and decrements the timer.
    /// If the timer hits 0, stops the tone.
    pub fn tick(&mut self) {
        if self.time != 0 {
            self.audio_output.output_audio();
            self.time -= 1;
            if self.time == 0 {
                self.audio_output.stop_audio();
            }
        }
    }
}