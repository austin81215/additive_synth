use std::sync::{Arc, Mutex};

use midly::num::u7;
use rodio::Source;

use crate::controllable_source::ControllableSource;

pub struct ThreadsafeControllable<T: ControllableSource> {
    pub source: Arc<Mutex<T>>
}

impl<T: ControllableSource> ThreadsafeControllable<T>  {
    pub fn new(source: T) -> Self {
        ThreadsafeControllable{source: Arc::new(Mutex::new(source))}
    }
}

impl<T: ControllableSource> ControllableSource for ThreadsafeControllable<T> {
    fn start_note(&mut self, key_press: crate::controllable_source::KeyPress) {
        self.source.lock().unwrap().start_note(key_press);
    }

    fn stop_note(&mut self, note: u7) {
        self.source.lock().unwrap().stop_note(note);
    }
}

impl<T: ControllableSource> Source for ThreadsafeControllable<T> {
    fn current_frame_len(&self) -> Option<usize> {
        self.source.lock().unwrap().current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.source.lock().unwrap().channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.lock().unwrap().sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.source.lock().unwrap().total_duration()
    }
}

impl<T: ControllableSource> Iterator for ThreadsafeControllable<T> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.lock().unwrap().next()
    }
}

impl<T: ControllableSource> Clone for ThreadsafeControllable<T> {
    fn clone(&self) -> Self {
        Self {source: self.source.clone()}
    }
}