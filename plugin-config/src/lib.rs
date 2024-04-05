//! A configuration crate for plugin metadata

mod builder;
mod plugin_type;
mod plugin_metadata;

use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Write},
};

pub use crate::builder::ShizenConfigBuilder;
pub use crate::plugin_type::PluginType;
pub use crate::plugin_metadata::PluginMetadata;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShizenConfig {
    /// The plugin's metadata table `[shizen]`
    pub shizen: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// The plugin's metadata table at `[shizen.metadata]`
    pub metadata: PluginMetadata,
}

impl ShizenConfig {
    /// Writes the `ShizenConfig` instance to a TOML file
    pub fn write_to_toml(&self) -> Result<(), io::Error> {
        let conf = toml::to_string(self).map_err(|e| io::Error::other(e))?;
        let mut cargo = File::options().append(true).open("Cargo.toml")?;

        cargo.write_all(b"\n")?;
        cargo.write_all(conf.as_bytes())?;

        Ok(())
    }
    // maybe add a `read_from_toml` function
}
