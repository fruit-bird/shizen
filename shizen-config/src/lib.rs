//! A configuration crate for VST metadata
//!
//! The information configured in the `shizen` table of the `Shizen.toml` VST
//! manifest is used to populate the necessary information about a VST
//!
//! ```toml
//
// Doing it like this doesn't show the toml contents with the LSP
// but it ensures the docs are correct and up to date...
// #![doc = include_str!("../Shizen.toml")]
//
//! [shizen.metadata]
//! vendor = "Shizen Technologies"
//! categories = ["effect", "utility", "midi"]
//! plugin-type = "instrument"
//!
//! [shizen.processing]
//! block-size = 1024
//! ```
//!
//! This configuration will also extract the package information
//! from the package `Cargo.toml` or the workspace `Cargo.toml`
//! if the package is part of a workspace

mod enums;

use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::env;

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
#[serde(rename_all = "kebab-case")]
#[serde(default)]
#[non_exhaustive]
pub struct PluginProcessing {
    pub block_size: usize,
}

impl Default for PluginProcessing {
    fn default() -> Self {
        Self { block_size: 512 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    /// The plugin's metadata table at `[shizen.metadata]`
    pub metadata: PluginMetadata,

    /// Information on how the VST should process data, at `[shizen.processing]`
    #[serde(default)]
    pub processing: PluginProcessing,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
    pub homepage: Option<String>,
}

impl Default for PackageInfo {
    fn default() -> Self {
        let authors = match env::var("CARGO_PKG_AUTHORS").ok() {
            Some(authors) if authors.is_empty() => None,
            Some(authors) => Some(authors.split(':').map(|s| s.trim().to_string()).collect()),
            None => None,
        };

        // SAFETY: these env vars always exist
        Self {
            name: env::var("CARGO_PKG_NAME").unwrap(),
            version: env::var("CARGO_PKG_VERSION").unwrap(),
            description: env::var("CARGO_PKG_DESCRIPTION").ok(),
            authors,
            homepage: env::var("CARGO_PKG_HOMEPAGE").ok(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ShizenConfig {
    /// The plugin's metadata table `[shizen]`
    pub shizen: Metadata,
    /// Package info extracted from the `Cargo.toml` manifest
    ///
    /// This info is not fetched from the actual `Cargo.toml`,
    /// but is rather fetched from the cargo environment variables,
    /// to allow for workspaces and getting the fields even if they are
    /// `fieldname.workspace = true`
    #[serde(default)]
    pub package: PackageInfo,
}

impl ShizenConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let conf = Config::builder()
            .add_source(File::new("Shizen.toml", FileFormat::Toml))
            .build()?
            .try_deserialize()?;

        Ok(conf)
    }
}
