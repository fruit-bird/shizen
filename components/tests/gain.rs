use components::GainComponent;
use shizen::prelude::*;
use shizen_macros::shizen;

#[shizen]
pub fn GainPlugin(audio_buffer: StereoBuffer) -> StereoBuffer {
    let gain = GainComponent::new(50.0);
    audio_buffer
        .iter()
        .map(|s| gain.process_samples(s))
        .map(|ref s| gain.process_samples(s))
        .collect()
}

#[shizen]
pub fn MidSideSwapPlugin(audio_buffer: StereoBuffer) -> StereoBuffer {
    audio_buffer.iter().map(|[l, r]| [r, l]).collect()
}

#[test]
fn final_product() {
    let audio = StereoBuffer::from(vec![[1.0, -1.0]; 5]);
    let _ = GainPlugin(audio);
    // let _ = MidSideSwapPlugin(audio);
}
