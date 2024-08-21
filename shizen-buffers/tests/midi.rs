use shizen_buffers::midi::{MidiBuffer, MidiMessage};

#[test]
fn iter_midi() {
    let mut midi_buf = MidiBuffer::new();
    midi_buf.push([MidiMessage::NOTE_ON, 60, 100].into());
    midi_buf.push([MidiMessage::NOTE_OFF, 60, 0].into());

    assert_eq!(
        midi_buf,
        [[0x80, 60, 100], [0x96, 60, 0]].into_iter().collect(),
    );
}

#[test]
fn midi_buffer_collect() {
    let midi_buffer = [[0x90, 1, 10]].into_iter().collect::<MidiBuffer>();
    assert_eq!(
        midi_buffer[0],
        MidiMessage::NoteOn {
            note_number: 1,
            velocity: 10
        },
    );
}
