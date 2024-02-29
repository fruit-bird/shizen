//! A configuration helper crate for managing plugin metadata

use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
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
    /// [metadata]
    /// name = "MyPlugin"
    /// version = "1.0"
    /// ```
    ///
    /// ## Example
    /// ```ignore
    /// let plugin_conf = Config::from_toml("plugin.conf.toml")?;
    /// println!("{:?}", plugin_conf.metadata.name); // "MyPlugin"
    /// ```
    pub fn from_toml(toml_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut contents = String::new();
        File::open(toml_path)?.read_to_string(&mut contents)?;

        let conf = toml::from_str(&contents)?;
        Ok(conf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_toml() {
        let conf = Config {
            metadata: Metadata {
                name: "MyPlugin".to_string(),
                version: "1.0".to_string(),
            },
        };

        assert!(conf.to_toml().is_ok());
    }

    #[test]
    fn test_from_toml() {
        let conf = Config::from_toml("plugin.conf.toml");
        assert!(conf.is_ok());
    }
}
