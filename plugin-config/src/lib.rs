//! A configuration helper crate for managing plugin metadata

use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

/// Represents the configuration for a plugin.
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct PluginConfig {
    /// The name of the plugin.
    pub name: String,
    /// The version of the plugin.
    pub version: String,
}

impl PluginConfig {
    /// Creates a new `PluginConfig` instance by reading from a JSON file
    ///
    /// The provided JSON file should have the following structure:
    /// ```json
    /// {
    ///   "name": "AwesomeDelay",
    ///   "version": "2.1.0"
    /// }
    /// ```
    ///
    /// ## Example
    /// ```ignore
    /// # use std::path::Path;
    /// let plugin_conf = PluginConfig::new("plugin.config.json")?;
    /// ```
    #[inline]
    pub fn new<P: AsRef<Path>>(config_path: P) -> io::Result<Self> {
        let file = File::open(config_path)?;
        let plugin_conf = serde_json::from_reader(&file)?;

        Ok(plugin_conf)
    }

    /// Writes out the plugin configuration to a JSON file
    ///
    /// # Example
    /// ```ignore
    /// let plugin_conf = PluginConf {
    ///     name: "MyPlugin".to_string(),
    ///     version: "1.0".to_string(),
    /// };
    ///
    /// plugin_conf.create_config_file()?;
    /// ```
    pub fn create_config_file(&self) -> io::Result<()> {
        // TODO: maybe have it get the version from the toml file from here?
        let json = serde_json::to_string_pretty(self)?;

        let mut file = File::create("plugin.conf.json")?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }
}
