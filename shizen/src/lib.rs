pub mod buffers;
pub mod components;
pub mod errors;
pub mod host;
pub mod plugin;

pub mod prelude {
    use super::*;

    pub use buffers::{AudioBuffer, MidiBuffer};
    pub use errors::PluginResult;
    // pub use host::DefaultHost;
    pub use components::{common_components::*, AudioProcessor, MidiProcessor};
    pub use plugin::{Plugin, VSTContext};
}

#[cfg(test)]
mod brainstorm {
    use crate::prelude::*;

    /// - idk what this will do of yet. guessing some jumbled background stuff
    /// - so the arguments won't be passed directly, they'll be forwarded through the host (i think)
    /// - or go fuck it and have everything implement the Audio|MidiProcessor traits since GainComponent is already a plugin
    /// - structs are better sara7a, cause we can store the plugin's attributes and associate
    ///
    /// Yeah just scrap this idea, have everything be a Component
    /// but now i have to find a solution for the &mut self in both AudioProcessor and MidiProcessor
    ///
    /// On second thought, components should just be some type of effect, and the actual plugin should combine
    /// many of those components to make a plugin
    ///
    /// plugin != component (update: is it???)
    /// 
    /// This is cool looking, but is unintuitive if the user wants to add methods or assoc fns 
    /// to the struct. if they even realize this gets turned into a struct. Think about it for now
    #[allow(non_snake_case, unused)]
    // #[shizen::plugin]
    fn GainDelayPlugin(audio_buffer: AudioBuffer) -> PluginResult<AudioBuffer> /* || MidiBuffer */ {
        let mut gain = GainComponent::new(50.0);
        let mut delay = DelayComponent::new(0.2, 0.4, 0.7);

        // components should be doing fast iterations with rayon when possible
        // parallelize the processing somehow, if it even makes sense
        let gained_audio = audio_buffer
            .process_with(&mut gain)
            .process_with(&mut delay);

        Ok(gained_audio)
    }

    // SHOULD BECOME THIS
    // struct GainDelayPlugin {
    //     // add fields from function args other than the first,
    //     // which should be the AudioBuffer
    // }

    // impl Plugin for GainDelayPlugin {
    //     fn initialize(&mut self, _context: &VST3Context) -> PluginResult<()> {
    //         todo!("some FFI stuff probably most definitely")
    //     }

    //     fn process_audio(&mut self, audio_buffer: AudioBuffer) -> PluginResult<AudioBuffer> {
    //         let mut gain = GainComponent::new(50.0);
    //         let mut delay = DelayComponent::new(0.2, 0.4, 0.7);
    //
    //         let gained_audio = audio_buffer
    //             .process_with(&mut gain)
    //             .process_with(&mut delay);
    //
    //         Ok(gained_audio)
    //     }
    //
    //     fn process_midi(&mut self, _midi_messages: MidiBuffer) -> PluginResult<()> {
    //         PluginResult::Ok(())
    //     }
    // }
}
