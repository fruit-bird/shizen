use std::vec::IntoIter;

use crate::buffers::audio::ChannelType;
use crate::components::AudioProcessor;

pub type Sample = f32;

#[derive(Debug, PartialEq)]
pub struct AudioBuffer {
    pub samples: Vec<Sample>,
    pub sample_rate: f32,
    pub buffer_size: usize,
    pub channel_type: ChannelType,
}

impl Default for AudioBuffer {
    fn default() -> Self {
        Self {
            samples: vec![0.0; (2 << 9) * ChannelType::default() as usize],
            sample_rate: 44100.0,
            buffer_size: 2 << 9,
            channel_type: ChannelType::default(),
        }
    }
}

impl AudioBuffer {
    pub fn new_stereo(samples: Vec<Sample>) -> Self {
        Self {
            buffer_size: samples.len() / 2,
            samples,
            channel_type: ChannelType::Stereo,
            ..Default::default()
        }
    }

    pub fn new_mono(samples: Vec<Sample>) -> Self {
        Self {
            buffer_size: samples.len(),
            samples,
            channel_type: ChannelType::Mono,
            ..Default::default()
        }
    }

    // TODO: do something better than wrap everything in an Option
    //       maybe return a tuple with left full and right empty if mono
    //       or return the same signal twice? or split in half, like half the gain??
    //       it's cheap either way cause it's a lazy iterator
    /// Returns None if the AudioBuffer is mono
    pub fn split_channels(
        &self,
    ) -> Option<(impl Iterator<Item = &Sample>, impl Iterator<Item = &Sample>)> {
        let (l, r) = match self.is_stereo() {
            true => self.samples.split_at(self.buffer_size),
            false => return None,
        };

        Some((l.iter(), r.iter()))
    }

    pub fn process_with(self, component: &mut dyn AudioProcessor) -> Self {
        component.process_audio(self)
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
    pub fn iter(&self) -> impl Iterator<Item = &Sample> {
        self.samples.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Sample> {
        self.samples.iter_mut()
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.samples.len()
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
