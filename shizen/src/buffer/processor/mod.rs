use super::AudioIterator;

pub(crate) trait Processor<I: Iterator> {
    fn process(input: I) -> I;
}

pub type AudioProcessor<'a, const CHANNELS: usize> = dyn Processor<AudioIterator<'a, CHANNELS>>;

pub trait ProcessWith<P>
where
    Self: Iterator + Sized,
    P: Processor<Self>,
{
    fn process_with(self, component: &mut P) -> Self {
        component.process(self)
    }
}
