mod midi_buffer;
mod midi_message;

pub use self::midi_buffer::MidiBuffer;
pub use self::midi_message::{MidiMessage, MidiMessageBytes};

#[test]
fn iter_midi() {
    let mut midi_buf = MidiBuffer::new();
    midi_buf.add_message([0x90, 60, 100]);
    midi_buf.add_message([0x80, 40, 40]);
    midi_buf.add_message([0x9E, 60, 100]);

    for message in midi_buf.iter() {
        println!("{:?}", message);
    }
}
