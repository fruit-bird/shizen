use plugin_config::{PluginType, ShizenConfigBuilder};

#[test]
fn write_to_toml_test() {
    let conf = ShizenConfigBuilder::new()
        .categories(vec!["MIDI", "Synth", "Effect"])
        .vendor("Shizen Technologies")
        .plugin_type(PluginType::Instrument)
        .build();

    dbg!(&conf);
    assert!(conf.write_to_toml().is_ok());
}
