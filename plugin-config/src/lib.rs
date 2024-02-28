//! A configuration helper crate for managing plugin metadata

use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub info: Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub version: String,
}

impl Config {
    pub fn to_toml(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conf = toml::to_string(self)?;
        File::create("plugin.conf.toml")?.write_all(conf.as_bytes())?;

        Ok(())
    }

    /// Creates a new `Config` instance by reading from a TOML file
    ///
    /// The provided TOML file should have the following structure:
    /// ```toml
    /// [info]
    /// name = "MyPlugin"
    /// version = "1.0"
    /// ```
    ///
    /// ## Example
    /// ```ignore
    /// # use std::path::Path;
    /// let plugin_conf = Config::from_toml("plugin.conf.toml")?;
    /// ```
    pub fn from_toml(toml_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(toml_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let conf = toml::from_str(toml_path)?;
        Ok(conf)
    }
}
