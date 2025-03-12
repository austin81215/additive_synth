mod osc;
mod traits;
mod envelope;
mod utils;
mod threadsafe_controllable;
mod synth_io;
mod view;
mod harmonics_source;
mod synth_core;

fn main() {
    iced::run("additive synth", view::update, view::view)
        .expect("failed to start the gui");
}
