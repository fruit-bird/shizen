use super::{AudioBuffer, Sample};

#[derive(Debug)]
pub struct AudioIterator<'a, const CHANNELS: usize> {
    audio_buffer: AudioBuffer<'a, CHANNELS>,
    sample: usize,
}

impl<'a, const CHANNELS: usize> AudioIterator<'a, CHANNELS> {
    pub(crate) const fn new(audio_buffer: AudioBuffer<'a, CHANNELS>) -> Self {
        Self {
            audio_buffer,
            sample: 0,
        }
    }
}

impl<'a, const CHANNELS: usize> Iterator for AudioIterator<'a, CHANNELS> {
    type Item = [&'a Sample; CHANNELS];

    fn next(&mut self) -> Option<Self::Item> {
        self.sample += 1;
        (self.sample <= self.audio_buffer.len()).then(|| {
            self.audio_buffer
                .samples
                .map(|channel| &channel[self.sample - 1])
        })
    }
}

impl<'a, const CHANNELS: usize> IntoIterator for AudioBuffer<'a, CHANNELS> {
    type Item = <<Self as IntoIterator>::IntoIter as Iterator>::Item;
    type IntoIter = AudioIterator<'a, CHANNELS>;

    fn into_iter(self) -> Self::IntoIter {
        AudioIterator::new(self)
    }
}
