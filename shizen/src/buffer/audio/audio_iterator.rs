use super::{AudioBuffer, Sample};

#[derive(Debug)]
pub struct AudioIterator<'a, const SAMPLES: usize, const CHANNELS: usize> {
    audio_buffer: AudioBuffer<'a, SAMPLES, CHANNELS>,
    sample: usize,
    channel: usize,
}

impl<'a, const SAMPLES: usize, const CHANNELS: usize> AudioIterator<'a, SAMPLES, CHANNELS> {
    pub(crate) const fn new(audio_buffer: AudioBuffer<'a, SAMPLES, CHANNELS>) -> Self {
        Self {
            audio_buffer,
            sample: 0,
            channel: 0,
        }
    }
}

impl<'a, const SAMPLES: usize, const CHANNELS: usize> Iterator
    for AudioIterator<'a, SAMPLES, CHANNELS>
{
    type Item = &'a Sample;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, const SAMPLES: usize, const CHANNELS: usize> IntoIterator
    for AudioBuffer<'a, SAMPLES, CHANNELS>
{
    type Item = Sample;
    type IntoIter = AudioIterator<'a, SAMPLES, CHANNELS>;

    fn into_iter(self) -> Self::IntoIter {
        AudioIterator::new(self)
    }
}
