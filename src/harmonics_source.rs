use midly::num::u7;
use rodio::Source;

use crate::{osc::SineOsc, traits::{KeyPress, MidiControllable}, utils::midi_to_hz};

/// a mono source made of layered sine waves
pub struct HarmonicsSource {
    harmonics: Vec<(SineOsc, f32)>
}

impl HarmonicsSource {
    /// a new HarmonicsSource with the given number of harmonics
    pub fn new(num_harmonics: usize) -> Self {
        HarmonicsSource { harmonics: vec![(SineOsc::new(440.), 1.); num_harmonics] }
    }

    /// the volume of each harmonic from lowest to highest, from 0 (silent) to 1 (full)
    pub fn harmonics(&self) -> impl Iterator<Item = f32> + '_ {
        self.harmonics.iter().map(|(_osc, vol)| *vol)
    }

    /// sets the ith harmonic to the given volume, from 0 (silent) to 1 (full)
    pub fn set_harmonic(&mut self, i: usize, vol: f32) {
        self.harmonics[i].1 = vol;
    }
}

impl MidiControllable for HarmonicsSource {
    fn start_note(&mut self, key_press: KeyPress) {
        for (i, (osc, _vol)) in self.harmonics.iter_mut().enumerate() {
            let freq = midi_to_hz(key_press.note) * (i as f32 + 1.);
            osc.start_freq(freq, key_press.velocity);
        }
    }

    fn stop_note(&mut self, _note: u7) {}
}

impl Source for HarmonicsSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        48000
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl Iterator for HarmonicsSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let total_vol: f32 = self.harmonics.iter().map(|(_osc, vol)| vol).sum();
        
        let sample: f32 = self.harmonics.iter_mut()
            .map(|(osc, vol)| osc.next().unwrap() * *vol)
            .sum();

        Some(sample/total_vol)
    }
}
