use std::f32::consts::PI;

use rodio::Source;

use crate::{controllable_source::{ControllableSource, KeyPress}, utils::midi_to_hz};

/// An infinite sine wave oscillator with changeable frequency
pub struct SineOsc {
    pub freq: f32,
    phase: f32,
    volume: f32
}

impl SineOsc {
    fn new(freq: f32) -> SineOsc {
        SineOsc { freq, phase: 0., volume: 0. }
    }
}

impl ControllableSource for SineOsc {
    fn start_note(&mut self, key_press: KeyPress) {
        self.freq = midi_to_hz(key_press.note);
        self.volume = key_press.velocity.as_int() as f32;
    }

    fn stop_note(&mut self) {
        self.volume = 0.;
    }
}

impl Source for SineOsc {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        4800
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

// loosely based on https://blog.demofox.org/2012/05/19/diy-synthesizer-chapter-2-common-wave-forms/
impl Iterator for SineOsc {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.phase += 2. * PI * self.freq / (self.sample_rate() as f32);
        
        while self.phase > 2. * PI {
            self.phase -= 2. * PI;
        }

        return Some(self.phase.sin() * self.volume);
    }
}