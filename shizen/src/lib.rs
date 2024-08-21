pub mod prelude {
    pub use shizen_buffers::{
        audio::{AudioBuffer, AudioProcessor, MonoBuffer, StereoBuffer},
        midi::{MidiBuffer, MidiProcessor},
        Plugin,
    };
}

pub use shizen_buffers as buffers;
pub use shizen_macros::plugin;

#[cfg(feature = "components")]
pub use shizen_components as components;

#[cfg(feature = "config")]
// this should probably be removed, only used in the build.rs step crate,
// I'll leave it as optional for now
pub use shizen_config as config;
