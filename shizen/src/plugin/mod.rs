mod vst_context;

pub use vst_context::VSTContext;

use crate::{
    buffers::{AudioBuffer, MidiBuffer},
    errors::PluginResult,
};

pub trait Plugin {
    fn initialize(&mut self, context: &VSTContext) -> PluginResult;

    fn process_audio(&mut self, audio_buffer: AudioBuffer) -> PluginResult<AudioBuffer>;
    fn process_midi(&mut self, midi_messages: MidiBuffer) -> PluginResult;
}
