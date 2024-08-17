use super::Sample;

pub trait AudioProcessor<const CH: usize> {
    fn process_samples(&self, samples: &[Sample; CH]) -> [Sample; CH];
}
