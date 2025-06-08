use midly::num::u7;
use std::{fmt::Display, iter::{once, repeat, repeat_n}};

/// convert a MIDI number to a frequency in Hz
pub fn midi_to_hz(midi_num: u7) -> f32 {
    440. * (2_f32).powf((midi_num.as_int() as i8 - 69) as f32/12.)
}

/// convert a frequency in Hz to the nearest MIDI number
#[allow(dead_code)]
pub fn hz_to_midi(hz: f32) -> u7 {
    u7::new((12. * (hz / 440.).log2() + 69.) as u8)
}

/// linearly interpolate between a and b given a value of t between t0 and t1
pub fn lerp(t: f32, t0: f32, t1: f32, a: f32, b: f32) -> f32 {
    (t - t0) / (t1 - t0) * (b - a) + a
}

/// the synth's preset waveforms
#[derive(Debug, Clone, PartialEq)]
pub enum Preset {
    Sine,
    Square,
    Sawtooth,
    Triangle,
    JazzOrgan,
    PopOrgan,
    FullOrgan
}

pub const PRESETS: [Preset; 7] = [
    Preset::Sine,
    Preset::Square,
    Preset::Sawtooth,
    Preset::Triangle,
    Preset::JazzOrgan,
    Preset::PopOrgan,
    Preset::FullOrgan
];

impl Display for Preset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Preset::Sine => "Sine",
            Preset::Square => "Square",
            Preset::Sawtooth => "Sawtooth",
            Preset::Triangle => "Triangle",
            Preset::JazzOrgan => "Jazz Organ",
            Preset::PopOrgan => "Pop Organ",
            Preset::FullOrgan => "Full Organ",
        };

        return write!(f, "{}", desc);
    }
}

/// gets harmonics for the given preset
pub fn preset_harmonics(preset: Preset, num_harmonics: usize) -> Vec<f32> {
    match preset {
        Preset::Sine => once(1.).chain(repeat(0.)).take(num_harmonics).collect(),
        Preset::Sawtooth => (1..).map(|i| 1./(i as f32)).take(num_harmonics).collect(),
        Preset::Square => (1..).map(|i| if i % 2 == 1 {1./(i as f32)} else {0.}).take(num_harmonics).collect(),
        Preset::Triangle => (1..).map(|i| 1./((i * i) as f32)).take(num_harmonics).collect(),
        Preset::JazzOrgan => repeat_n(1., 3).chain(repeat(0.)).take(num_harmonics).collect(),
        Preset::FullOrgan => repeat(1.).take(num_harmonics).collect(),
        Preset::PopOrgan => once(1.).chain(repeat_n(0., 2)).chain(once(1.)).chain(repeat(0.)).take(num_harmonics).collect(),
    }
}