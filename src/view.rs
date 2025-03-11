use iced::widget::{column, row, text, vertical_slider, Column, Row};

use crate::synth::Synth;

#[derive(Debug, Clone)]
pub enum Message {
    AChanged(f32),
    DChanged(f32),
    SChanged(f32),
    RChanged(f32),
}

pub fn view(synth: &Synth) -> Row<Message> {
    // TODO: need a better way to get synth components
    let env = synth.source.source.lock().unwrap(); 
    row![
        env_slider(env.a, Message::AChanged, "A"),
        env_slider(env.d, Message::DChanged, "D"),
        env_slider(env.s, Message::SChanged, "S"),
        env_slider(env.r, Message::RChanged, "R"),
    ].spacing(50).into()
}

pub fn update(synth: &mut Synth, message: Message) {
    let mut env = synth.source.source.lock().unwrap();
    match message {
        Message::AChanged(val) => env.a = val,
        Message::DChanged(val) => env.d = val,
        Message::SChanged(val) => env.s = val,
        Message::RChanged(val) => env.r = val,
    }
}

fn env_slider<'a>(val: f32, msg: impl Fn(f32) -> Message + 'a, label: &str) -> Column<'a, Message> {
    column![
        vertical_slider(0.0..=1., val, msg).height(200).step(0.01),
        text(format!("{label}: {val:.2}"))
    ]
}