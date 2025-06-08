use iced::widget::{column, pick_list, row, text, vertical_slider, Column, Row};

use crate::{synth_io::Synth, utils::{preset_harmonics, Preset, PRESETS}};

/// a message sent by the gui
#[derive(Debug, Clone)]
pub enum Message {
    AChanged(f32),
    DChanged(f32),
    SChanged(f32),
    RChanged(f32),
    HarmonicChanged(usize, f32),
    PresetPicked(Preset)
}

#[derive(Default)]
pub struct State {
    synth: Synth,
    current_preset: Option<Preset>
}

/// render the gui
pub fn view(state: &State) -> Column<Message> {
    let core = state.synth.threadsafe_source.contents.lock().unwrap(); 
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
        ).spacing(50),

        row![
            text("Preset Waveforms:"),
            pick_list(PRESETS, state.current_preset.clone(), Message::PresetPicked)
            .placeholder("Presets...")
        ]
    ]
}

/// handle gui messages
pub fn update(state: &mut State, message: Message) {
    let mut core = state.synth.threadsafe_source.contents.lock().unwrap();
    match message {
        Message::AChanged(val) => core.envelope.a = val,
        Message::DChanged(val) => core.envelope.d = val,
        Message::SChanged(val) => core.envelope.s = val,
        Message::RChanged(val) => core.envelope.r = val,
        Message::HarmonicChanged(harmonic, val) => core.source.set_harmonic(harmonic, val),
        Message::PresetPicked(preset) => {
            state.current_preset = Some(preset.clone());
            let num_harmonics = core.source.num_harmonics();
            core.source.set_harmonics(preset_harmonics(preset, num_harmonics));
        },
    }
}

/// a slider that is part of an envelope, with a current value, text label,
/// and message to send when slid
fn env_slider<'a>(val: f32, msg: impl Fn(f32) -> Message + 'a, label: &str) -> Column<'a, Message> {
    column![
        vertical_slider(0.0..=1., val, msg).height(200).step(0.01),
        text(format!("{label}: {val:.2}"))
    ]
}

/// a slider that controls a harmonic, 
/// with a number for which harmonic it is and a current value
fn harmonic_slider<'a>(harmonic: usize, val: f32) -> Column<'a, Message> {
    column![
        vertical_slider(0.0..=1., val, 
            move |v| Message::HarmonicChanged(harmonic, v)
        ).height(200).step(0.01),
        text(format!("{harmonic}: {val:.2}"))
    ]
}