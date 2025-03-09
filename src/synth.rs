use std::error::Error;

use midir::{ConnectError, ConnectErrorKind, MidiInput, MidiInputConnection};
use midly::{live::LiveEvent, MidiMessage};
use rodio::{OutputStream, OutputStreamHandle};

use crate::{controllable_source::{ControllableSource, KeyPress}, enveloped_source::EnvelopedSource, osc::SineOsc, threadsafe_controllable::ThreadsafeControllable};

pub struct Synth {
    pub source: ThreadsafeControllable<EnvelopedSource<SineOsc>>,
    output_stream: Option<OutputStream>,
    output_handle: Option<OutputStreamHandle>,
    midi_connection: Option<MidiInputConnection<()>>
}

impl Synth {
    fn new() -> Self {
        Synth{
            source: ThreadsafeControllable::new(EnvelopedSource::new(SineOsc::new(440.))),
            output_stream: None,
            output_handle: None,
            midi_connection: None
        }
    }

    fn connect_to_default_audio(&mut self) -> Result<(), Box<dyn Error>> {
        let (stream, handle) = OutputStream::try_default()?;
        (self.output_stream, self.output_handle) = (Some(stream), Some(handle));
        self.output_handle.as_ref().unwrap().play_raw(self.source.clone())?;
        Ok(())
    }

    fn connect_to_default_midi(&mut self) -> Result<(), Box<dyn Error>> {
        let midi_in = MidiInput::new("synth_client")?;
        let ports = midi_in.ports();
        if ports.is_empty() {
            return Err(Box::new(ConnectError::new(ConnectErrorKind::InvalidPort, "no midi device connected")));
        }

        let mut callback_src = self.source.clone();
        let connection = midi_in.connect(
            &ports[0],
            "synth_port",
            move |_timestamp, msg, _data|{
                midi_handler(&mut callback_src, msg);
            },
            ()
        )?;
        self.midi_connection = Some(connection);
        
        Ok(())
    }
}

fn midi_handler(controllable_source: &mut impl ControllableSource, raw_message: &[u8]) {
    let message = LiveEvent::parse(raw_message).unwrap();
    
    if let LiveEvent::Midi{channel: _, message} = message {
        match message {
            MidiMessage::NoteOff {key: _, vel: _} => controllable_source.stop_note(),
            MidiMessage::NoteOn {key, vel} => controllable_source.start_note(KeyPress{note: key, velocity: vel}),
            _ => ()
        }
    }
}

impl Default for Synth {
    fn default() -> Self {
        let mut s = Self::new();
        s.connect_to_default_audio().expect("could not connect to default audio");
        s.connect_to_default_midi().expect("could not connect to default midi");
        return s;
    }
}