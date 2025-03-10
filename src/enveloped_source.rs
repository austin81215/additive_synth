use midly::num::u7;
use rodio::Source;
use crate::{controllable_source::{MidiControllable, KeyPress}, utils::lerp};

pub struct EnvelopedSource<T> where 
T: MidiControllable + Source, 
T: Iterator<Item = f32> {
    pub a: f32,
    pub d: f32,
    pub s: f32,
    pub r: f32,
    state: EnvState,
    pub source: T,
    current_note: KeyPress,
}

enum EnvState {
    Off,
    Playing{t: f32, start_level: f32},
    Releasing{t: f32, start_level: f32}
}

impl<T> EnvelopedSource<T> where 
T: MidiControllable + Source, 
T: Iterator<Item = f32> {
    pub fn new(source: T) -> Self {
        EnvelopedSource{a: 0.,
            d: 0., 
            s: 1., 
            r: 0., 
            state: EnvState::Off, 
            source: source, 
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

impl<T> MidiControllable for EnvelopedSource<T> where 
T: MidiControllable + Source, 
T: Iterator<Item = f32> {
    fn start_note(&mut self, key_press: KeyPress) {
        match self.state {
            EnvState::Off => {
                self.state = EnvState::Playing { t: 0., start_level: 0. };
                self.source.start_note(key_press);
                self.current_note = key_press;
            },
            EnvState::Playing { t: _, start_level: _ } => {
                self.source.start_note(KeyPress { 
                    note: key_press.note, 
                    velocity: self.current_note.velocity 
                });
                self.current_note.note = key_press.note;
            },
            EnvState::Releasing { t: _, start_level: _ } => {
                self.source.start_note(KeyPress { 
                    note: key_press.note, 
                    velocity: self.current_note.velocity 
                });
                self.current_note.note = key_press.note;
                self.state = EnvState::Playing { t: 0., start_level: self.amplitude() }
            },
        }
    }

    fn stop_note(&mut self, note: u7) {
        if self.current_note.note == note {
            self.source.stop_note(note);
            self.state = EnvState::Releasing{t: 0., start_level: self.amplitude()};
        }
    }
}

impl<T> Source for EnvelopedSource<T> where 
T: MidiControllable + Source, 
T: Iterator<Item = f32>  {
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

impl<T> Iterator for EnvelopedSource<T> where 
T: MidiControllable + Source, 
T: Iterator<Item = f32>  {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let EnvState::Playing{t, start_level} = self.state { // increment t
            self.state = EnvState::Playing{
                t: t + 1. / (self.sample_rate() as f32), 
                start_level: start_level
            };
        }
        else if let EnvState::Releasing{t, start_level} = self.state { 
            self.state = if t <= self.r {
                EnvState::Releasing{
                    t: t + 1. / (self.sample_rate() as f32),
                    start_level: start_level
                }
            }
            else {
                EnvState::Off
            }
        }

        return match self.source.next() { // multiply source by amplitude
            Some(sample) => Some(sample * self.amplitude()),
            None => None,
        }
    }
}