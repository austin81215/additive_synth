use midly::num::u7;

/// basic MIDI control
pub trait MidiControllable {
    /// start a note with the given KeyPress
    fn start_note(&mut self, key_press: KeyPress);
    /// stop the given note if it is playing
    fn stop_note(&mut self, note: u7);
}

/// a component of a synth that modifies incoming sound.
/// eg. envelope, filter, LFO, effects
pub trait SynthComponent {
    /// outputs new sound samples given input sound
    fn apply(&mut self, sample: f32) -> f32;
}

/// a simple midi key press
#[derive(Clone, Copy)]
pub struct KeyPress {
    pub note: u7,
    pub velocity: u7
}

