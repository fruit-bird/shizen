use components::GainComponent;
use shizen::prelude::*;
use shizen_macros::shizen;

#[shizen]
pub fn GainPlugin(audio_buffer: StereoBuffer) -> PluginResult<StereoBuffer> {
    let gain = GainComponent::new(50.0);
    let gained_audio = audio_buffer
        .map(|s| gain.process_samples(s))
        .collect_audio(44_100);

    dbg!(&gained_audio);
    Ok(gained_audio)
}

#[shizen]
pub fn MidSideSwapPlugin(audio_buffer: StereoBuffer) -> PluginResult<StereoBuffer> {
    let swapped_audio = audio_buffer.map(|[l, r]| [r, l]).collect_audio(44_100);

    dbg!(&swapped_audio);
    Ok(swapped_audio)
}

#[test]
fn final_product() {
    let audio = StereoBuffer::new(vec![[1.0, -1.0]; 5], 44_100);
    // let _ = GainPlugin(audio);
    let _ = MidSideSwapPlugin(audio);
}
