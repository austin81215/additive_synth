use midly::num::u7;
use crate::{traits::{KeyPress, MidiControllable, SynthComponent}, utils::lerp};

pub struct Envelope {
    pub a: f32,
    pub d: f32,
    pub s: f32,
    pub r: f32,
    state: EnvState,
    current_note: KeyPress,
}

enum EnvState {
    Off,
    Playing{t: f32, start_level: f32},
    Releasing{t: f32, start_level: f32}
}

impl Envelope {
    pub fn new() -> Self {
        Envelope{a: 0.,
            d: 0., 
            s: 1., 
            r: 0., 
            state: EnvState::Off, 
            current_note: KeyPress{note: u7::new(0), velocity: u7::new(0)} 
        }
    }

    fn amplitude(&self) -> f32{
        match self.state { // use position in envelope to find amplitude
            EnvState::Off => 0.,
            EnvState::Playing{t, start_level} if t <= self.a => lerp(t, 0., self.a, start_level, 1.),
            EnvState::Playing{t, start_level: _} if t <= self.a + self.d => lerp(t, self.a, self.a + self.d, 1., self.s),
            EnvState::Playing{t: _, start_level: _} => self.s,
            EnvState::Releasing{t, start_level: level_reached} => lerp(t, 0., self.r, level_reached, 0.),
        }
    }
}

impl MidiControllable for Envelope {
    fn start_note(&mut self, key_press: KeyPress) {
        match self.state {
            EnvState::Off => {
                self.state = EnvState::Playing { t: 0., start_level: 0. };
                self.current_note = key_press;
            },
            EnvState::Playing { t: _, start_level: _ } => {
                self.current_note.note = key_press.note;
            },
            EnvState::Releasing { t: _, start_level: _ } => {
                self.current_note.note = key_press.note;
                self.state = EnvState::Playing { t: 0., start_level: self.amplitude() }
            },
        }
    }

    fn stop_note(&mut self, note: u7) {
        if self.current_note.note == note {
            self.state = EnvState::Releasing{t: 0., start_level: self.amplitude()};
        }
    }
}

impl SynthComponent for Envelope {
    fn apply(&mut self, sample: f32) -> f32 {
        if let EnvState::Playing{t, start_level} = self.state { // increment t
            self.state = EnvState::Playing{
                t: t + 1. / (4800.), 
                start_level: start_level
            };
        }
        else if let EnvState::Releasing{t, start_level} = self.state { 
            self.state = if t <= self.r {
                EnvState::Releasing{
                    t: t + 1. / (4800.),
                    start_level: start_level
                }
            }
            else {
                EnvState::Off
            }
        }

        return sample * self.amplitude();
    }
}