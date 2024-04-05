use serde::{Deserialize, Serialize};

use crate::PluginType;

/// The metadata table for a plugin
///
/// This table is used to store information about the plugin
/// that is not directly related to the plugin's functionality
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PluginMetadata {
    /// The crate's `name` in the `[package]` table
    #[serde(skip_serializing)]
    pub name: String,
    /// The crate's `version` in the `[package]` table
    #[serde(skip_serializing)]
    pub version: String,
    /// The crate's `description` in the `[package]` table
    ///
    /// Will be an empty string if not provided
    #[serde(skip_serializing)]
    pub description: String,

    /// The `categories` list in the `[shizen.metadata]` table
    pub categories: Option<Vec<String>>,
    /// The `vendor` in the `[shizen.metadata]` table
    pub vendor: Option<String>,
    /// The `plugin_type` in the `[shizen.metadata]` table
    pub plugin_type: Option<PluginType>,
}

impl Default for PluginMetadata {
    fn default() -> Self {
        PluginMetadata {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: env!("CARGO_PKG_DESCRIPTION").to_string(),
            vendor: None,
            plugin_type: None,
            categories: None,
        }
    }
}
