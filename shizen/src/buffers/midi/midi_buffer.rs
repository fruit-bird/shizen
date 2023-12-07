use super::midi_message::{MidiMessage, MidiMessageBytes};

#[derive(Debug, Default, PartialEq)]
pub struct MidiBuffer {
    pub(crate) messages: Vec<MidiMessage>,
}

impl MidiBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_message(&mut self, bytes: MidiMessageBytes) {
        self.messages.push(MidiMessage::from_bytes(bytes))
    }

    pub fn transpose(&mut self, interval: i8) {
        for message in self.messages.iter_mut() {
            if let MidiMessage::NoteOn { note_number, .. } = message {
                let original_note = *note_number as i8;
                let new_note = (original_note + interval).clamp(0, 127) as u8;
                *note_number = new_note;
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &MidiMessage> {
        self.messages.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut MidiMessage> {
        self.messages.iter_mut()
    }
}
