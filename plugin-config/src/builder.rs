use crate::{plugin_type::PluginType, Metadata, PluginMetadata, ShizenConfig};

#[derive(Debug)]
#[non_exhaustive]
pub struct ShizenConfigBuilder {
    pub name: String,
    pub version: String,
    pub description: String,
    pub categories: Option<Vec<String>>,
    pub vendor: Option<String>,
    pub plugin_type: Option<PluginType>,
}

impl ShizenConfigBuilder {
    pub fn new() -> Self {
        ShizenConfigBuilder {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: env!("CARGO_PKG_DESCRIPTION").to_string(),
            categories: None,
            vendor: None,
            plugin_type: None,
        }
    }

    pub fn categories(mut self, categories: &[&str]) -> Self {
        self.categories = Some(categories.to_vec().iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn vendor(mut self, vendor: &str) -> Self {
        self.vendor = Some(vendor.to_string());
        self
    }

    pub fn plugin_type(mut self, plugin_type: PluginType) -> Self {
        self.plugin_type = Some(plugin_type);
        self
    }

    pub fn build(self) -> ShizenConfig {
        ShizenConfig {
            shizen: Metadata {
                metadata: PluginMetadata {
                    name: self.name,
                    version: self.version,
                    description: self.description,
                    categories: self.categories,
                    vendor: self.vendor,
                    plugin_type: self.plugin_type,
                },
            },
        }
    }
}
