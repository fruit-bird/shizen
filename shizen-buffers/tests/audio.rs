use shizen_buffers::prelude::*;

#[test]
fn audio_buffer_collect() {
    let audio_buffer = MonoBuffer::from(vec![[0.0]; 10]);
    let buffer = audio_buffer
        .iter()
        .enumerate()
        .map(|(i, [s])| [s + i as Sample])
        .collect::<MonoBuffer>();

    assert_eq!(buffer.len(), 10);
}
