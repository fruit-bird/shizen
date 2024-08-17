use std::ops::{Deref, DerefMut};

pub type Sample = f32;
pub type MonoBuffer = AudioBuffer<1>;
pub type StereoBuffer = AudioBuffer<2>;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct AudioBuffer<const CH: usize> {
    pub(crate) samples: Vec<[Sample; CH]>,
}

impl<const CH: usize> From<Vec<[Sample; CH]>> for AudioBuffer<CH> {
    /// Creates a new audio buffer from a vector of samples
    ///
    /// Use this as a constructor for AudioBuffer, and use [`AudioBuffer::new()`] to create an empty buffer
    ///
    /// # Panics
    /// Panics if the number of channels is zero
    fn from(samples: Vec<[Sample; CH]>) -> Self {
        assert!(CH != 0, "Number of channels must be non-zero");
        Self { samples }
    }
}

impl<const CH: usize> AudioBuffer<CH> {
    /// Creates a new empty audio buffer
    /// # Example
    /// ```
    /// # use shizen::prelude::*;
    /// let buffer = StereoBuffer::new();
    /// assert_eq!(buffer.len(), 0);
    /// ```
    ///
    /// # Panics
    /// Panics if the number of channels is zero
    /// ```should_panic
    /// # use shizen::prelude::*;
    /// let buffer = AudioBuffer::<0>::new();
    /// ```
    pub const fn new() -> Self {
        assert!(CH != 0, "Number of channels must be non-zero");
        Self {
            samples: Vec::new(),
        }
    }

    /// Returns an iterator over the samples in a specific channel within the buffer
    /// # Example
    /// ```
    /// # use shizen::prelude::*;
    /// let buffer = StereoBuffer::from(vec![[0.0, 1.0]; 10]);
    /// let channel = buffer.channel(0).collect::<Vec<_>>();
    ///
    /// assert_eq!(channel, vec![&0.0; 10]);
    /// ```
    ///
    /// # Panics
    /// Panics if the channel index is out of bounds
    /// ```should_panic
    /// # use shizen::prelude::*;
    /// let buffer = StereoBuffer::from(vec![[0.0, 1.0]; 10]);
    /// let panics_here = buffer.channel(10).collect::<Vec<_>>();
    /// ```
    pub fn iter_channels(&self, channel: usize) -> impl Iterator<Item = &Sample> {
        assert!(channel < CH, "Channel index out of bounds");
        self.samples.iter().map(move |s| &s[channel])
    }
}

impl<const CH: usize> Deref for AudioBuffer<CH> {
    type Target = Vec<[Sample; CH]>;

    fn deref(&self) -> &Self::Target {
        &self.samples
    }
}

impl<const CH: usize> DerefMut for AudioBuffer<CH> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.samples
    }
}

impl<const CH: usize> FromIterator<[Sample; CH]> for AudioBuffer<CH> {
    fn from_iter<T: IntoIterator<Item = [Sample; CH]>>(iter: T) -> Self {
        Self {
            samples: iter.into_iter().collect(),
        }
    }
}

impl<'a, const CH: usize> FromIterator<[&'a Sample; CH]> for AudioBuffer<CH> {
    fn from_iter<T: IntoIterator<Item = [&'a Sample; CH]>>(iter: T) -> Self {
        Self {
            samples: iter
                .into_iter()
                .map(|samples| samples.map(|&s| s))
                .collect(),
        }
    }
}

impl<const CH: usize> crate::Buffer for AudioBuffer<CH> {}

impl<I, const CH: usize> crate::Buffer for I where I: Iterator<Item = [Sample; CH]> {}
