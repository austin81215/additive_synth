use midly::num::u7;
use rodio::Source;

pub trait ControllableSource: Source + Iterator<Item = f32> {
    fn start_note(&self, note: u7, velocity: u7);
    fn stop_note(&self);
}