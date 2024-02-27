mod midi_buffer;
mod midi_message;

pub use self::midi_buffer::{MidiBuffer, MidiMessageBytes};
pub use self::midi_message::MidiMessage;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_midi() {
        let mut midi_buf = MidiBuffer::new();
        midi_buf.add_message([0x90, 60, 100]);
        midi_buf.add_message([0x80, 40, 40]); // error in NoteOff message's velocity gets corrected to 0
        midi_buf.add_message([0x9E, 60, 100]);

        midi_buf.iter().for_each(|m| println!("{:?}", m));
    }

    #[test]
    fn midi_buffer_collect() {
        let midi_buffer = [
            MidiMessage::from([0x90, 1, 10]),
            MidiMessage::from([0x83, 1, 0]),
            MidiMessage::from([0x95, 1, 0]),
        ]
        .into_iter()
        .collect::<MidiBuffer>();

        eprintln!("{:#?}", midi_buffer);
    }
}
