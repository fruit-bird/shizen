//! A configuration crate for VST metadata
//!
//! The information configured in the `shizen` table of `Cargo.toml` manifest
//! is used to populate the necessary information about a VST
//!
//! # Usage
//! ```toml
//! [shizen.metadata]
//! vendor = "Shizen Technologies"
//! categories = ["utility", "midi"]
//! plugin-type = "instrument"
//! ```

mod enums;

use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};

pub use crate::enums::{Categories, PluginType};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct PluginMetadata {
    /// The `vendor` in the `[shizen.metadata]` table
    pub vendor: String,

    /// The `categories` list in the `[shizen.metadata]` table
    pub categories: Option<Vec<Categories>>,

    /// The `plugin_type` in the `[shizen.metadata]` table
    pub plugin_type: Option<PluginType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// The plugin's metadata table at `[shizen.metadata]`
    pub metadata: PluginMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShizenConfig {
    /// The plugin's metadata table `[shizen]`
    pub shizen: Metadata,
}

impl ShizenConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let conf = Config::builder()
            .add_source(File::new("Cargo.toml", FileFormat::Toml))
            .build()?
            .try_deserialize()?;

        Ok(conf)
    }
}
