use super::{AudioBuffer, Sample};

#[derive(Debug)]
pub struct AudioIterator<'a, const CHANNELS: usize> {
    audio_buffer: AudioBuffer<'a, CHANNELS>,
    samples: usize,
    channel: usize,
}

impl<'a, const CHANNELS: usize> AudioIterator<'a, CHANNELS> {
    pub(crate) const fn new(audio_buffer: AudioBuffer<'a, CHANNELS>) -> Self {
        Self {
            audio_buffer,
            samples: 0,
            channel: 0,
        }
    }
}

impl<'a, const CHANNELS: usize> Iterator for AudioIterator<'a, CHANNELS> {
    type Item = &'a Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.samples < self.audio_buffer.len() {
            let sample = unsafe {
                self.audio_buffer
                    .samples
                    .as_ref()?
                    .get(self.channel)?
                    .get(self.samples)?
            };
            self.channel = (self.channel + 1) % CHANNELS;
            if self.channel == 0 {
                self.samples += 1;
            }
            Some(sample)
        } else {
            None
        }
    }
}

impl<'a, const CHANNELS: usize> IntoIterator for AudioBuffer<'a, CHANNELS> {
    type Item = &'a Sample;
    type IntoIter = AudioIterator<'a, CHANNELS>;

    fn into_iter(self) -> Self::IntoIter {
        AudioIterator::new(self)
    }
}
