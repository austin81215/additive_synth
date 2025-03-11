use midly::num::u7;

pub trait MidiControllable {
    fn start_note(&mut self, key_press: KeyPress);
    fn stop_note(&mut self, note: u7);
}

#[derive(Clone, Copy)]
pub struct KeyPress {
    pub note: u7,
    pub velocity: u7
}

