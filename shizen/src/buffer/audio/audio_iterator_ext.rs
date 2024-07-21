use super::{AudioBuffer, Sample};

pub trait AudioIteratorExt<const CH: usize>: Iterator<Item = [Sample; CH]> {
    fn collect_audio(&mut self, sample_rate: u32) -> AudioBuffer<CH> {
        AudioBuffer {
            samples: Iterator::collect(self),
            sample_rate,
        }
    }
}

impl<I, const CH: usize> AudioIteratorExt<CH> for I where I: Iterator<Item = [Sample; CH]> {}
