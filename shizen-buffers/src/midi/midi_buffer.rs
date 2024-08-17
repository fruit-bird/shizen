use std::ops::{Deref, DerefMut};

use super::midi_message::MidiMessage;

pub type MidiMessageBytes = [u8; 3];
pub type _MidiEvent = (u64, MidiMessageBytes);

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct MidiBuffer {
    pub(crate) messages: Vec<MidiMessage>,
}

impl Deref for MidiBuffer {
    type Target = Vec<MidiMessage>;

    fn deref(&self) -> &Self::Target {
        &self.messages
    }
}

impl DerefMut for MidiBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.messages
    }
}

impl MidiBuffer {
    pub const fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn transpose(&mut self, interval: i8) {
        self.messages.iter_mut().for_each(|m| m.transpose(interval));
    }
}

impl FromIterator<MidiMessageBytes> for MidiBuffer {
    fn from_iter<T: IntoIterator<Item = MidiMessageBytes>>(iter: T) -> Self {
        Self {
            messages: iter.into_iter().map(MidiMessage::from_bytes).collect(),
        }
    }
}

impl FromIterator<MidiMessage> for MidiBuffer {
    fn from_iter<T: IntoIterator<Item = MidiMessage>>(iter: T) -> Self {
        Self {
            messages: iter.into_iter().collect(),
        }
    }
}

impl crate::Buffer for MidiBuffer {}
// impl<I> Buffer for I where I: Iterator<Item = MidiMessage> {}
