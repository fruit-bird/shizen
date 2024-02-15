mod vst_context;

pub use vst_context::VSTContext;

use crate::errors::PluginResult;

pub trait Plugin {
    fn initialize(&mut self, context: &VSTContext) -> PluginResult;
}
