mod osc;
mod controllable_source;
mod enveloped_source;
mod utils;
mod threadsafe_controllable;
mod synth;
mod view;
mod harmonics_source;

fn main() {
    iced::run("additive synth", view::update, view::view)
        .expect("failed to start the gui");
}
