use shizen_buffers::{
    audio::{AudioProcessor, StereoBuffer},
    Plugin,
};
use shizen_components::GainComponent;
use shizen_macros::shizen;

#[shizen]
pub fn GainPlugin(audio_buffer: StereoBuffer) -> StereoBuffer {
    let gain = GainComponent::new(50.0);
    audio_buffer
        .iter()
        .map(|s| gain.process_samples(s))
        .map(|ref s| gain.process_samples(s))
        .map(|ref s| gain.process_samples(s))
        .map(|ref s| gain.process_samples(s))
        // .enumerate()
        // .map(|(i, _)| [i as f32, i as f32])
        .collect()
}

#[shizen]
pub fn MidSideSwapPlugin(audio_buffer: StereoBuffer) -> StereoBuffer {
    audio_buffer.iter().map(|[l, r]| [r, l]).collect()
}

#[test]
fn final_product() {
    let audio = StereoBuffer::from(vec![[1.0, 1.0]; 1_000_000]);
    let num_chunks = GainPlugin::process(audio)
        // chunking to process blocks would only make sense
        // if the plugin methods returned unconsumed iterators
        .chunks(8)
        .inspect(|chunk| eprintln!("{:?}", chunk))
        .count();

    eprintln!("{}", num_chunks);

    // let _ = MidSideSwapPlugin(audio);
}
