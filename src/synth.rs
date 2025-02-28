use midir::MidiInputConnection;
use rodio::{OutputStream, OutputStreamHandle};

use crate::{enveloped_source::EnvelopedSource, osc::SineOsc, threadsafe_controllable::ThreadsafeControllable};

pub struct Synth {
    source: ThreadsafeControllable<EnvelopedSource<SineOsc>>,
    output_stream: Option<OutputStream>,
    output_handle: Option<OutputStreamHandle>,
    midi_connection: Option<MidiInputConnection<()>>
}

impl Synth {
    fn connect_to_default_audio(&mut self) -> Result<(), String> {
        todo!()
    }
    fn connect_to_default_midi(&mut self) -> Result<(), String> {
        todo!()
    }
}