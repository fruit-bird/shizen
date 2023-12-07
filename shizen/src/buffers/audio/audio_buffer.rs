#[derive(Debug, Default, PartialEq)]
pub struct AudioBuffer<T = f32> {
    pub(crate) samples: Vec<T>,
}

impl<T> AudioBuffer<T> {
    pub fn new(samples: Vec<T>) -> Self {
        Self { samples }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.samples.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.samples.iter_mut()
    }
}
