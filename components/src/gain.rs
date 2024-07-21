use shizen::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct GainComponent {
    pub gain: f32,
}

impl GainComponent {
    pub const fn new(gain: f32) -> Self {
        Self { gain }
    }
}

impl<const CH: usize> AudioProcessor<CH> for GainComponent {
    fn process_samples(&self, samples: [Sample; CH]) -> [f32; CH] {
        samples.map(|s| s * self.gain)
    }
}
