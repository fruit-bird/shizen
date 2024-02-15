use crate::AudioProcessor;

use super::{AudioBuffer, Sample};

#[derive(Debug)]
pub struct AudioIterator<'a> {
    audio_buffer: &'a AudioBuffer,
    index: usize,
}

impl<'a> AudioIterator<'a> {
    pub(crate) const fn new(audio_buffer: &'a AudioBuffer) -> Self {
        Self {
            audio_buffer,
            index: 0,
        }
    }

    pub fn process_with(self, component: &mut dyn AudioProcessor) -> AudioBuffer {
        component.process_audio(self)
    }
}

impl<'a> Iterator for AudioIterator<'a> {
    type Item = &'a Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.audio_buffer.samples.get(self.index);
        self.index += 1;
        sample
    }
}

impl<'a> IntoIterator for &'a AudioBuffer {
    type Item = &'a Sample;
    type IntoIter = AudioIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AudioIterator::new(self)
    }
}

impl<'a> FromIterator<&'a Sample> for AudioBuffer {
    fn from_iter<I: IntoIterator<Item = &'a Sample>>(iter: I) -> Self {
        let samples: Vec<Sample> = iter.into_iter().cloned().collect();
        Self {
            samples,
            ..Default::default()
        }
    }
}
