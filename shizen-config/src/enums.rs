use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
// these variants are only filler for now
pub enum Categories {
    Effect,
    Midi,
    Utility,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PluginType {
    Instrument,
    Effect,
}
