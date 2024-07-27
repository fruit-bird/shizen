use shizen::prelude::*;

#[test]
fn audio_iterator() {
    let audio_buffer = MonoBuffer::from(vec![[0.0]; 10]);
    let buffer = audio_buffer
        .iter()
        .enumerate()
        .map(|(i, [s])| [s + i as Sample])
        .collect::<MonoBuffer>();

    eprintln!("{:?}", buffer);
    assert_eq!(buffer.len(), 10);
}
