use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PluginType {
    Instrument,
    Effect,
    #[default]
    Unknown,
}

impl From<&str> for PluginType {
    fn from(s: &str) -> Self {
        match s {
            "instrument" => PluginType::Instrument,
            "effect" => PluginType::Effect,
            _ => PluginType::Unknown,
        }
    }
}
