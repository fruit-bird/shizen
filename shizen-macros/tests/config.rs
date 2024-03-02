use shizen::prelude::*;
use shizen_macros::shizen;

#[shizen(config = "plugin.conf.toml")]
fn _plugin(audio: StereoBuffer) -> PluginResult<StereoBuffer> {
    // let mut gain = GainComponent::new(50.0);
    // let mut delay = DelayComponent::new(0.2, 0.4, 0.7);

    let _gained_audio = audio.iter();
    // .process_with(&mut gain)
    // .process_with(&mut delay);
    todo!()
}
