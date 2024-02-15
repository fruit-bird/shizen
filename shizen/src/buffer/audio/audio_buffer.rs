use crate::{buffer::audio::ChannelType, AudioProcessor};
use std::vec::IntoIter;

use super::AudioIterator;

pub type Sample = f32;

#[derive(Debug, PartialEq, Default)]
pub struct AudioBuffer {
    pub samples: Vec<Sample>,
    pub channel_type: ChannelType,
}

impl AudioBuffer {
    pub const fn new(samples: Vec<Sample>, channel_type: ChannelType) -> Self {
        Self {
            samples,
            channel_type,
        }
    }

    pub fn process_with(self, component: &mut dyn AudioProcessor) -> AudioBuffer {
        component.process_audio(self.iter())
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        match self.channel_type {
            ChannelType::Mono => self.samples.len(),
            ChannelType::Stereo => self.samples.len() / 2,
        }
    }

    // TODO: do something better than wrap everything in an Option
    //       maybe return a tuple with left full and right empty if mono
    //       or return the same signal twice? or split in half, like half the gain??
    //       it's cheap either way cause it's a lazy iterator
    /// Returns None if the AudioBuffer is mono
    // TODO: change the data types so that only a Stereo AudioBuffer can be split
    pub fn split_channels(
        &self,
    ) -> Option<(impl Iterator<Item = &Sample>, impl Iterator<Item = &Sample>)> {
        let (l, r) = match self.is_stereo() {
            true => self.samples.split_at(self.len()),
            false => return None,
        };

        Some((l.iter(), r.iter()))
    }

    #[must_use]
    pub const fn is_stereo(&self) -> bool {
        matches!(self.channel_type, ChannelType::Stereo)
    }

    #[must_use]
    pub const fn is_mono(&self) -> bool {
        matches!(self.channel_type, ChannelType::Mono)
    }

    #[inline]
    pub fn iter(&self) -> AudioIterator<'_> {
        AudioIterator::new(self)
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

impl IntoIterator for AudioBuffer {
    type Item = Sample;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.samples.into_iter()
    }
}
