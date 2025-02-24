use rodio::{source, Source};
use crate::{controllable_source::{ControllableSource, KeyPress}, osc::SineOsc, utils::{lerp, midi_to_hz}};

struct EnvelopedSine {
    a: f32,
    d: f32,
    s: f32,
    r: f32,
    state: EnvState,
    source: SineOsc
}

enum EnvState {
    Off,
    Playing(f32),
    Releasing(f32)
}

impl ControllableSource for EnvelopedSine {
    fn start_note(&mut self, key_press: KeyPress) {
        self.source.freq = midi_to_hz(key_press.note);
        self.state = EnvState::Playing(0.);
    }

    fn stop_note(&mut self) {
        self.state = EnvState::Off;
    }
}

impl Source for EnvelopedSine {
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

impl Iterator for EnvelopedSine {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let amplitude = match self.state { // use position in envelope to find amplitude
            EnvState::Off => 0.,
            EnvState::Playing(t) if t <= self.a => lerp(t, 0., self.a, 0., 1.),
            EnvState::Playing(t) if t <= self.d => lerp(t, self.a, self.d, 1., self.s),
            EnvState::Playing(_) => self.s,
            EnvState::Releasing(t) => lerp(t, 0., self.r, self.s, 0.),
        };

        if let EnvState::Playing(t) = self.state { // increment t
            self.state = EnvState::Playing(t + 1. / (self.sample_rate() as f32));
        }
        else if let EnvState::Releasing(t) = self.state { 
            self.state = EnvState::Releasing(t + 1. / (self.sample_rate() as f32));
        }

        return match self.source.next() { // multiply source by amplitude
            Some(sample) => Some(sample  * amplitude),
            None => None,
        }
    }
}