use iced::widget::{column, row, text, vertical_slider, Column, Row};

use crate::synth_io::Synth;

#[derive(Debug, Clone)]
pub enum Message {
    AChanged(f32),
    DChanged(f32),
    SChanged(f32),
    RChanged(f32),
    HarmonicChanged(usize, f32)
}

pub fn view(synth: &Synth) -> Column<Message> {
    let core = synth.threadsafe_source.contents.lock().unwrap(); 
    column![
        row![
            env_slider(core.envelope.a, Message::AChanged, "Attack"),
            env_slider(core.envelope.d, Message::DChanged, "Decay"),
            env_slider(core.envelope.s, Message::SChanged, "Sustain"),
            env_slider(core.envelope.r, Message::RChanged, "Release"),
        ].spacing(50),

        Row::with_children(
            core.source.harmonics()
                .enumerate()
                .map(|(harmonic, val)| harmonic_slider(harmonic, val).into())
        ).spacing(50)
    ]
}

pub fn update(synth: &mut Synth, message: Message) {
    let mut core = synth.threadsafe_source.contents.lock().unwrap();
    match message {
        Message::AChanged(val) => core.envelope.a = val,
        Message::DChanged(val) => core.envelope.d = val,
        Message::SChanged(val) => core.envelope.s = val,
        Message::RChanged(val) => core.envelope.r = val,
        Message::HarmonicChanged(harmonic, val) => core.source.set_harmonic(harmonic, val),
    }
}

fn env_slider<'a>(val: f32, msg: impl Fn(f32) -> Message + 'a, label: &str) -> Column<'a, Message> {
    column![
        vertical_slider(0.0..=1., val, msg).height(200).step(0.01),
        text(format!("{label}: {val:.2}"))
    ]
}

fn harmonic_slider<'a>(harmonic: usize, val: f32) -> Column<'a, Message> {
    column![
        vertical_slider(0.0..=1., val, 
            move |v| Message::HarmonicChanged(harmonic, v)
        ).height(200).step(0.01),
        text(format!("{harmonic}: {val:.2}"))
    ]
}