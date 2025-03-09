use rodio::Source;
use crate::{controllable_source::{ControllableSource, KeyPress}, utils::lerp};

pub struct EnvelopedSource<T: ControllableSource> {
    pub a: f32,
    pub d: f32,
    pub s: f32,
    pub r: f32,
    state: EnvState,
    pub source: T
}

enum EnvState {
    Off,
    Playing(f32),
    Releasing(f32)
}

impl<T: ControllableSource> EnvelopedSource<T> {
    pub fn new(source: T) -> Self {
        EnvelopedSource{ a: 0., d: 0., s: 1., r: 0., state: EnvState::Off, source: source }
    }
}

impl<T: ControllableSource> ControllableSource for EnvelopedSource<T> {
    fn start_note(&mut self, key_press: KeyPress) {
        self.source.start_note(key_press);
        self.state = EnvState::Playing(0.);
    }

    fn stop_note(&mut self) {
        self.source.stop_note();
        self.state = EnvState::Releasing(0.);
    }
}

impl<T: ControllableSource> Source for EnvelopedSource<T> {
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

impl<T: ControllableSource> Iterator for EnvelopedSource<T> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let amplitude = match self.state { // use position in envelope to find amplitude
            EnvState::Off => 0.,
            EnvState::Playing(t) if t <= self.a => lerp(t, 0., self.a, 0., 1.),
            EnvState::Playing(t) if t <= self.a + self.d => lerp(t, self.a, self.a + self.d, 1., self.s),
            EnvState::Playing(_) => self.s,
            EnvState::Releasing(t) => lerp(t, 0., self.r, self.s, 0.),
        };

        if let EnvState::Playing(t) = self.state { // increment t
            self.state = EnvState::Playing(t + 1. / (self.sample_rate() as f32));
        }
        else if let EnvState::Releasing(t) = self.state { 
            self.state = if t <= self.r {
                EnvState::Releasing(t + 1. / (self.sample_rate() as f32))
            }
            else {
                EnvState::Off
            }
        }

        return match self.source.next() { // multiply source by amplitude
            Some(sample) => Some(sample  * amplitude),
            None => None,
        }
    }
}