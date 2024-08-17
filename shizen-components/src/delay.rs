use shizen_buffers::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct DelayComponent {
    pub delay_time: f32,
    pub feedback: f32,
    pub wet_dry: f32,
}

impl DelayComponent {
    pub const fn new(delay_time: f32, feedback: f32, wet_dry: f32) -> Self {
        Self {
            delay_time,
            feedback,
            wet_dry,
        }
    }
}

impl<const CH: usize> AudioProcessor<CH> for DelayComponent {
    fn process_samples(&self, samples: &[Sample; CH]) -> [Sample; CH] {
        let mut delay_buffer = VecDeque::new();
        samples.map(|s| {
            delay_buffer.push_back(s);
            let delayed_sample = delay_buffer.pop_front().unwrap();
            delayed_sample * self.wet_dry + s * (1.0 - self.wet_dry)
        })
    }
}
