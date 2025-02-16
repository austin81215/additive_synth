use rodio::Source;
use crate::controllable_source::ControllableSource;

struct Envelope<T: Source + Iterator<Item = f32>> {
    a: f32,
    d: f32,
    s: f32,
    r: f32,
    t: f32,
    source: T
}

impl<T: Source + Iterator<Item = f32>> ControllableSource for Envelope<T> {
    fn start_note(&self, note: midly::num::u7, velocity: midly::num::u7) {
        todo!()
    }

    fn stop_note(&self) {
        todo!()
    }
}

impl<T: Source + Iterator<Item = f32>> Source for Envelope<T> {
    fn current_frame_len(&self) -> Option<usize> {
        todo!()
    }

    fn channels(&self) -> u16 {
        todo!()
    }

    fn sample_rate(&self) -> u32 {
        todo!()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        todo!()
    }
}

impl<T: Source + Iterator<Item = f32>> Iterator for Envelope<T> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}