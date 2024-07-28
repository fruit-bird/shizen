use super::MidiMessageBytes;

pub trait MidiProcessor {
    fn process_midi(&mut self, midi_messages: &MidiMessageBytes) -> MidiMessageBytes;
}
