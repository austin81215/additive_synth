use std::sync::{Arc, Mutex};

use rodio::{source, Source};

use crate::controllable_source::ControllableSource;

pub struct ThreadsafeControllable<T: ControllableSource> {
    pub source: Arc<Mutex<T>>
}

impl<T: ControllableSource> ControllableSource for ThreadsafeControllable<T> {
    fn start_note(&mut self, key_press: crate::controllable_source::KeyPress) {
        self.source.lock().unwrap().start_note(key_press);
    }

    fn stop_note(&mut self) {
        self.source.lock().unwrap().stop_note();
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