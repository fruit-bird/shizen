use crate::buffers::AudioBuffer;
use crate::components::AudioProcessor;
use crate::Audio;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct DelayComponent {
    pub delay_time: f32,
    pub feedback: f32,
    pub wet_dry: f32,
    pub buffer: Vec<f32>,
}

impl DelayComponent {
    pub fn new(delay_time: f32, feedback: f32, wet_dry: f32) -> Self {
        Self {
            delay_time,
            feedback,
            wet_dry,
            buffer: Vec::new(),
        }
    }
}

impl AudioProcessor for DelayComponent {
    fn process_audio(&mut self, audio_buffer: AudioBuffer) -> AudioBuffer {
        let output = audio_buffer
            .iter()
            .enumerate()
            .map(|(i, sample)| {
                let delayed_sample = self.buffer.get(i).unwrap_or(&0.0);
                let wet_signal = sample + delayed_sample * self.feedback;
                let dry_signal = sample;

                let mixed_sample = wet_signal * self.wet_dry + dry_signal * (1.0 - self.wet_dry);
                self.buffer.push(*sample);

                if self.buffer.len() > self.delay_time as usize {
                    self.buffer.remove(0); // FIFO
                }

                mixed_sample
            })
            .collect();

        Audio![output as audio_buffer.channel_type]
    }
}
