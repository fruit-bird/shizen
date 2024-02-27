use super::AudioIterator;

pub type Sample = f32;
pub type MonoBuffer<'a> = AudioBuffer<'a, 1>;
pub type StereoBuffer<'a> = AudioBuffer<'a, 2>;

#[derive(Debug, PartialEq, Clone, Copy, derive_more::From, derive_more::Into)]
pub struct AudioBuffer<'a, const CHANNELS: usize> {
    pub(crate) samples: [&'a [Sample]; CHANNELS],
}

impl<'a, const CHANNELS: usize> AudioBuffer<'a, CHANNELS> {
    pub const fn new(samples: [&'a [Sample]; CHANNELS]) -> Self {
        // assert_ne!(CHANNELS, 0, "CHANNELS must be non-zero");
        Self { samples }
    }

    #[inline]
    pub const fn iter(&self) -> AudioIterator<'a, CHANNELS> {
        AudioIterator::new(*self)
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.samples[0].len()
    }
}
