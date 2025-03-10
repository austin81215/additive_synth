use midly::num::u7;
use rodio::Source;

pub trait ControllableSource: Source + Iterator<Item = f32> {
    fn start_note(&mut self, key_press: KeyPress);
    fn stop_note(&mut self, note: u7);
}

#[derive(Clone, Copy)]
pub struct KeyPress {
    pub note: u7,
    pub velocity: u7
}