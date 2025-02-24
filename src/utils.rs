use midly::num::u7;

pub fn midi_to_hz(midi_num: u7) -> f32 {
    440. * (2_f32).powf((midi_num.as_int() as i8 - 69) as f32/12.)
}

pub fn hz_to_midi(hz: f32) -> u7 {
    u7::new((12. * (hz / 440.).log2() + 69.) as u8)
}

pub fn lerp(t: f32, t0: f32, t1: f32, a: f32, b: f32) -> f32 {
    (t - t0) / (t1 - t0) * (b - a) + a
}