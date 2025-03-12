use iced::widget::{column, row, text, vertical_slider, Column, Row};

use crate::synth_io::Synth;

#[derive(Debug, Clone)]
pub enum Message {
    AChanged(f32),
    DChanged(f32),
    SChanged(f32),
    RChanged(f32),
}

pub fn view(synth: &Synth) -> Row<Message> {
    let core = synth.threadsafe_source.contents.lock().unwrap(); 
    row![
        env_slider(core.envelope.a, Message::AChanged, "A"),
        env_slider(core.envelope.d, Message::DChanged, "D"),
        env_slider(core.envelope.s, Message::SChanged, "S"),
        env_slider(core.envelope.r, Message::RChanged, "R"),
    ].spacing(50).into()
}

pub fn update(synth: &mut Synth, message: Message) {
    let mut core = synth.threadsafe_source.contents.lock().unwrap();
    match message {
        Message::AChanged(val) => core.envelope.a = val,
        Message::DChanged(val) => core.envelope.d = val,
        Message::SChanged(val) => core.envelope.s = val,
        Message::RChanged(val) => core.envelope.r = val,
    }
}

fn env_slider<'a>(val: f32, msg: impl Fn(f32) -> Message + 'a, label: &str) -> Column<'a, Message> {
    column![
        vertical_slider(0.0..=1., val, msg).height(200).step(0.01),
        text(format!("{label}: {val:.2}"))
    ]
}