use crate::buffers::AudioBuffer;
use crate::components::AudioProcessor;
use crate::Audio;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct GainComponent {
    pub gain: f32,
}

impl GainComponent {
    pub fn new(gain: f32) -> Self {
        Self { gain }
    }
}

impl AudioProcessor for GainComponent {
    fn process_audio(&mut self, audio_buffer: AudioBuffer) -> AudioBuffer {
        let output = audio_buffer
            .iter()
            .map(|sample| sample * self.gain)
            .collect();

        Audio![output as audio_buffer.channel_type]
    }
}