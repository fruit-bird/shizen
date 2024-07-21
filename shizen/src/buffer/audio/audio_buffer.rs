pub type Sample = f32;
pub type MonoBuffer = AudioBuffer<1>;
pub type StereoBuffer = AudioBuffer<2>;

#[derive(Debug, PartialEq, Clone)]
pub struct AudioBuffer<const CH: usize> {
    pub(crate) samples: Vec<[Sample; CH]>,
    pub sample_rate: u32,
}

// in case we don't need the sample_rate here, will expose all Vec methods,
// but will lose option to add methods and impls.
type _AudioBuffer<const CH: usize> = Vec<[Sample; CH]>;

impl<const CH: usize> AudioBuffer<CH> {
    pub const fn new(samples: Vec<[Sample; CH]>, sample_rate: u32) -> Self {
        assert!(CH != 0, "Number of channels must be non-zero");
        Self {
            samples,
            sample_rate,
        }
    }

    pub const fn new_empty(sample_rate: u32) -> Self {
        Self {
            samples: Vec::new(),
            sample_rate,
        }
    }
}

impl<const CH: usize> Iterator for AudioBuffer<CH> {
    type Item = [Sample; CH];

    fn next(&mut self) -> Option<Self::Item> {
        self.samples.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.samples.len(), Some(self.samples.len()))
    }
}

impl<const CH: usize> ExactSizeIterator for AudioBuffer<CH> {}
