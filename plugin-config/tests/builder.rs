use plugin_config::{PluginType, ShizenConfigBuilder};

#[test]
fn builder_pattern() {
    let conf = ShizenConfigBuilder::new()
        .categories(&["MIDI", "Synth", "Effect"])
        .vendor("Shizen Technologies")
        .plugin_type(PluginType::Effect)
        .build();

    dbg!(conf);
}
