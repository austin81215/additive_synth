use iced::widget::{column, slider, Column};

use crate::synth::Synth;

#[derive(Debug, Clone)]
pub enum Message {
    AChanged(f32),
    DChanged(f32),
    SChanged(f32),
    RChanged(f32),
}

pub fn view(synth: &Synth) -> Column<Message> {
    // TODO: need a better way to get synth components
    let env = synth.source.source.lock().unwrap(); 
    column![
        slider(0.0..=1., env.a, Message::AChanged),
        slider(0.0..=1., env.d, Message::DChanged),
        slider(0.0..=1., env.s, Message::SChanged),
        slider(0.0..=1., env.r, Message::RChanged),
    ].into()
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