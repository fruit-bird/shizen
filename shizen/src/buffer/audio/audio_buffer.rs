use std::marker::PhantomData;

use super::AudioIterator;

pub type Sample = f32;
pub type MonoBuffer<'a> = AudioBuffer<'a, 1>;
pub type StereoBuffer<'a> = AudioBuffer<'a, 2>;

#[derive(Debug, PartialEq)]
pub struct AudioBuffer<'a, const CHANNELS: usize> {
    pub(crate) samples: *const [*mut [Sample]; CHANNELS],
    marker: PhantomData<&'a [Sample]>,
}

impl<'a, const CHANNELS: usize> AudioBuffer<'a, CHANNELS> {
    pub const fn new(samples: [&'a [Sample]; CHANNELS]) -> Self {
        Self {
            samples: samples.as_ptr() as *const _,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn iter(&self) -> AudioIterator<'a, CHANNELS> {
        AudioIterator::new(*self)
    }

    #[inline]
    pub const fn len(&self) -> usize {
        unsafe {
            self.samples
                .as_ref()
                .and_then(|s| s.get(0))
                .and_then(|s| s.as_ref())
                .map_or(0, |s| s.len())
        }
    }
}
