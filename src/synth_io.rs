use std::error::Error;

use midir::{ConnectError, ConnectErrorKind, MidiInput, MidiInputConnection};
use midly::{live::LiveEvent, MidiMessage};
use rodio::{OutputStream, OutputStreamHandle};

use crate::{harmonics_source::HarmonicsSource, synth_core::SynthCore, threadsafe_controllable::ThreadsafeControllable, traits::{KeyPress, MidiControllable}};

/// a synth that interacts with the outside world, taking MIDI input and outputting sound.
/// currently just connects to the default MIDI and sound on startup.
pub struct Synth {
    pub threadsafe_source: ThreadsafeControllable<SynthCore<HarmonicsSource>>,
    output_stream: Option<OutputStream>,
    output_handle: Option<OutputStreamHandle>,
    midi_connection: Option<MidiInputConnection<()>>
}

impl Synth {
    fn new() -> Self {
        Synth{
            threadsafe_source: ThreadsafeControllable::new(SynthCore::new(HarmonicsSource::new(10))),
            output_stream: None,
            output_handle: None,
            midi_connection: None
        }
    }

    fn connect_to_default_audio(&mut self) -> Result<(), Box<dyn Error>> {
        let (stream, handle) = OutputStream::try_default()?;
        (self.output_stream, self.output_handle) = (Some(stream), Some(handle));
        self.output_handle.as_ref().unwrap().play_raw(self.threadsafe_source.clone())?;
        Ok(())
    }

    fn connect_to_default_midi(&mut self) -> Result<(), Box<dyn Error>> {
        let midi_in = MidiInput::new("synth_client")?;
        let ports = midi_in.ports();
        if ports.is_empty() {
            return Err(Box::new(ConnectError::new(ConnectErrorKind::InvalidPort, "no midi device connected")));
        }

        let mut callback_src = self.threadsafe_source.clone();
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

fn midi_handler(controllable_source: &mut impl MidiControllable, raw_message: &[u8]) {
    let message = LiveEvent::parse(raw_message).unwrap();

    if let LiveEvent::Midi{channel: _, message} = message {
        match message {
            MidiMessage::NoteOff {key, vel: _} => controllable_source.stop_note(key),
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