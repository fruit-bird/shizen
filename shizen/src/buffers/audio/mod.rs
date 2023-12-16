mod audio_buffer;
mod channel_type;

pub use audio_buffer::{AudioBuffer, Sample};
pub use channel_type::ChannelType;

#[test]
fn audio_processing_concept() {
    let mut audio = AudioBuffer::new_stereo(vec![0.0, 1.0, 3.0, 5.0]);
    let _processed = audio.iter_mut().map(|s| *s * 2.0);
}
