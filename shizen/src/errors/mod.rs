use std::{error::Error, fmt::Display};

// An opaque error type for the entire crate
// maybe use thiserror when the enum is populated
#[derive(Debug)]
pub enum PluginError {
    Initialize,
    Terminate,
}

impl Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initialize => writeln!(f, "Failed to initialize plugin"),
            Self::Terminate => writeln!(f, "Failed to terminate plugin"),
            // _ => writeln!(f,),
        }
    }
}

impl Error for PluginError {}

pub type PluginResult<T = ()> = std::result::Result<T, PluginError>;
