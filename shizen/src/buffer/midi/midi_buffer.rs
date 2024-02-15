use std::vec::IntoIter;

use super::midi_message::MidiMessage;

pub type MidiMessageBytes = [u8; 3];

#[derive(Debug, Default, PartialEq)]
pub struct MidiBuffer {
    pub messages: Vec<MidiMessage>,
}

impl MidiBuffer {
    pub const fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, bytes: MidiMessageBytes) {
        self.messages.push(MidiMessage::from_bytes(&bytes))
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

    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

impl IntoIterator for MidiBuffer {
    type Item = MidiMessage;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.messages.into_iter()
    }
}

impl<'a> From<&'a [MidiMessageBytes]> for MidiBuffer {
    fn from(bytes_buffer: &'a [MidiMessageBytes]) -> Self {
        let messages = bytes_buffer.iter().map(MidiMessage::from_bytes).collect();
        Self { messages }
    }
}

impl FromIterator<MidiMessage> for MidiBuffer {
    fn from_iter<T: IntoIterator<Item = MidiMessage>>(iter: T) -> Self {
        Self {
            messages: iter.into_iter().collect(),
        }
    }
}
