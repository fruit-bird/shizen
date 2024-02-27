use derive_more::{From, Into};

use super::AudioIterator;

pub type Sample = f32;
pub type MonoBuffer<'a> = AudioBuffer<'a, 1>;
pub type StereoBuffer<'a> = AudioBuffer<'a, 2>;

#[derive(Debug, PartialEq, Clone, Copy, From, Into)]
pub struct AudioBuffer<'a, const CHANNELS: usize> {
    pub(crate) samples: [&'a [Sample]; CHANNELS],
}

impl<'a, const CHANNELS: usize> AudioBuffer<'a, CHANNELS> {
    pub const fn new(samples: [&'a [Sample]; CHANNELS]) -> Self {
        assert!(CHANNELS != 0, "CHANNELS must be non-zero");
        Self { samples }
    }

    #[inline]
    #[must_use]
    pub const fn iter(&self) -> AudioIterator<'a, CHANNELS> {
        AudioIterator::new(*self)
    }

    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.samples[0].len()
    }
}
