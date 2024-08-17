use shizen_config::ShizenConfig;

#[test]
fn read_from_cargo_toml() {
    let conf = ShizenConfig::new().expect("FUCK");
    eprintln!("{:#?}", conf);
}
