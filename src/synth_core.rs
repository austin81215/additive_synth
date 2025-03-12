use midly::num::u7;
use rodio::Source;

use crate::{envelope::Envelope, traits::{KeyPress, MidiControllable, SynthComponent}};

pub struct SynthCore<T> where 
T: MidiControllable + Source,
T: Iterator<Item = f32> {
    pub source: T,
    pub envelope: Envelope
}

impl<T> SynthCore<T> where 
T: MidiControllable + Source,
T: Iterator<Item = f32> {
    pub fn new(source: T) -> Self {
        SynthCore { source, envelope: Envelope::new() }
    }
}

impl<T> MidiControllable for SynthCore<T> where 
T: MidiControllable + Source,
T: Iterator<Item = f32> {
    fn start_note(&mut self, key_press: KeyPress) {
        self.source.start_note(key_press);
        self.envelope.start_note(key_press);
    }

    fn stop_note(&mut self, note: u7) {
        self.source.stop_note(note);
        self.envelope.stop_note(note);
    }
}

impl<T> Source for SynthCore<T> where 
T: MidiControllable + Source,
T: Iterator<Item = f32> {
    fn current_frame_len(&self) -> Option<usize> {
        self.source.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.source.total_duration()
    }
}

impl<T> Iterator for SynthCore<T> where 
T: MidiControllable + Source,
T: Iterator<Item = f32> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let source_sample = self.source.next()?;
        let res = self.envelope.apply(source_sample);
        return Some(res);
    }
}