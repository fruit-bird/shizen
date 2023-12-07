mod audio_buffer;

pub use audio_buffer::AudioBuffer;

#[test]
fn process_audio() {
    let mut audio = AudioBuffer::new(vec![0, 1, 3, 5]);
    let processed = audio.iter_mut().map(|s| *s * 2);
}
