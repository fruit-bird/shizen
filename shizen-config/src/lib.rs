//! A configuration crate for plugin metadata

mod builder;
mod plugin_metadata;
mod plugin_type;

use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read as _, Write},
};

pub use crate::builder::ShizenConfigBuilder;
pub use crate::plugin_metadata::PluginMetadata;
pub use crate::plugin_type::PluginType;

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

        // check if the config table exists
        let mut buf = String::new();
        cargo.read_to_string(&mut buf)?;

        // so as not to write the config table multiple times
        // but if config is different, then it doesn't work
        if buf.contains("[shizen") {
            todo!("diff the config tables");
            // return Ok(());
        }

        cargo.write_all(b"\n")?;
        cargo.write_all(conf.as_bytes())?;

        Ok(())
    }
    // maybe add a `read_from_toml` function
}
