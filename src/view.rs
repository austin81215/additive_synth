use iced::widget::{column, row, text, vertical_slider, Row};

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
        column![
            vertical_slider(0.0..=1., env.a, Message::AChanged).height(200).step(0.01),
            text(format!("A: {:.2}", env.a))
        ],
        column![
            vertical_slider(0.0..=1., env.d, Message::DChanged).height(200).step(0.01),
            text(format!("D: {:.2}", env.d))
        ],
        column![
            vertical_slider(0.0..=1., env.s, Message::SChanged).height(200).step(0.01),
            text(format!("S: {:.2}", env.s))
        ],
        column![
            vertical_slider(0.0..=1., env.r, Message::RChanged).height(200).step(0.01),
            text(format!("R: {:.2}", env.r))
        ],
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
