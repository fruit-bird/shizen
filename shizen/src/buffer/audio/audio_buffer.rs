use std::marker::PhantomData;

use super::AudioIterator;

pub type Sample = f32;
pub type MonoBuffer<'a, const N: usize> = AudioBuffer<'a, N, 1>;
pub type StereoBuffer<'a, const N: usize> = AudioBuffer<'a, N, 2>;

#[derive(Debug, PartialEq)]
pub struct AudioBuffer<'a, const SAMPLES: usize, const CHANNELS: usize> {
    pub samples: [[Sample; SAMPLES]; CHANNELS],
    marker: PhantomData<&'a ()>,
}

impl<'a, const SAMPLES: usize, const CHANNELS: usize> AudioBuffer<'a, SAMPLES, CHANNELS> {
    pub const fn new(samples: [[Sample; SAMPLES]; CHANNELS]) -> Self {
        Self {
            samples,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn iter(&self) -> AudioIterator<'a, SAMPLES, CHANNELS> {
        AudioIterator::new(*self)
    }
}
