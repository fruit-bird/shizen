pub mod buffers;
pub mod errors;
pub mod host;
pub mod plugin;

pub mod prelude {
    use super::*;

    pub use buffers::{AudioBuffer, MidiBuffer};
    pub use errors::Result;
    // pub use host::DefaultHost;
    pub use plugin::{Plugin, PluginInfo};
}
