use shizen::prelude::*;

#[test]
fn audio_iterator() {
    let mut buffer = StereoBuffer::new(vec![[0.0, 0.0]; 10], 44_100)
        .enumerate()
        .map(|(i, [l, r])| match i % 2 == 0 {
            true => [l + 1.0, r + 1.0],
            false => [l - 1.0, r - 1.0],
        })
        .collect_audio(44_100);

    dbg!(&buffer);
    assert_eq!(buffer.len(), 10);

    buffer.next();
    assert_eq!(buffer.len(), 9);
}
