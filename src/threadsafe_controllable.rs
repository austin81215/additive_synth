use std::sync::{Arc, Mutex};

use midly::num::u7;
use rodio::Source;

use crate::traits::MidiControllable;

pub struct ThreadsafeControllable<T> where
T: MidiControllable + Source,
T: Iterator<Item = f32> {
    pub contents: Arc<Mutex<T>>
}

impl<T> ThreadsafeControllable<T> where
T: MidiControllable + Source,
T: Iterator<Item = f32> {
    pub fn new(source: T) -> Self {
        ThreadsafeControllable{contents: Arc::new(Mutex::new(source))}
    }
}

impl<T> MidiControllable for ThreadsafeControllable<T> where
T: MidiControllable + Source,
T: Iterator<Item = f32> {
    fn start_note(&mut self, key_press: crate::traits::KeyPress) {
        self.contents.lock().unwrap().start_note(key_press);
    }

    fn stop_note(&mut self, note: u7) {
        self.contents.lock().unwrap().stop_note(note);
    }
}

impl<T> Source for ThreadsafeControllable<T> where
T: MidiControllable + Source,
T: Iterator<Item = f32> {
    fn current_frame_len(&self) -> Option<usize> {
        self.contents.lock().unwrap().current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.contents.lock().unwrap().channels()
    }

    fn sample_rate(&self) -> u32 {
        self.contents.lock().unwrap().sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.contents.lock().unwrap().total_duration()
    }
}

impl<T> Iterator for ThreadsafeControllable<T> where
T: MidiControllable + Source,
T: Iterator<Item = f32> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.contents.lock().unwrap().next()
    }
}

impl<T> Clone for ThreadsafeControllable<T> where
T: MidiControllable + Source,
T: Iterator<Item = f32> {
    fn clone(&self) -> Self {
        Self {contents: self.contents.clone()}
    }
}