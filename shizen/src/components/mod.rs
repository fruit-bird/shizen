mod delay;
mod gain;

pub mod common_components {
    pub use super::delay::DelayComponent;
    pub use super::gain::GainComponent;
}

use crate::buffers::{AudioBuffer, MidiBuffer};

// TODO (when phasing these traits out and replacing them with the Plugin trait):
//      since we have to capture self as &mut, might as well keep mutating
//      the same AudioBuffer rather than creating an entirely new one,
//      so either return a result of a &mut AudioBuffer (idk if this makes sense. i want it to,
//                                                       so i can have the option to chain multiple processors)
//      or pass audio_buffer as a &mut AudioBuffer
pub trait AudioProcessor {
    fn process_audio(&mut self, audio_buffer: AudioBuffer) -> AudioBuffer;
}

pub trait MidiProcessor {
    fn process_midi(&mut self, midi_buffer: MidiBuffer) -> MidiBuffer;
}
