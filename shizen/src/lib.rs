pub mod buffer;
pub mod errors;

pub mod prelude {
    use super::*;

    pub use crate::MidiProcessor;
    pub use buffer::*;
    pub use errors::*;
}

use crate::buffer::MidiBuffer;

pub trait MidiProcessor {
    fn process_midi(&mut self, midi_messages: MidiBuffer)
        -> <MidiBuffer as IntoIterator>::IntoIter;
}
