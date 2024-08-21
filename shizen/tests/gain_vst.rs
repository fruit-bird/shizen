use shizen::components::GainComponent;
use shizen::prelude::*;

#[shizen::plugin]
pub fn GainPlugin(audio_buffer: StereoBuffer) -> StereoBuffer {
    let gain = GainComponent::new(50.0);
    audio_buffer
        .iter()
        .map(|s| gain.process_samples(s))
        .map(|ref s| gain.process_samples(s))
        // .enumerate()
        // .map(|(i, _)| [1 as f32, 1 as f32])
        .collect()
}

#[test]
fn plugin_processing_with_block_sizes() {
    // As long as processing time for samples of len `sample_rate` is less than 1 second,
    // the plugin is hitting the minimum required performance

    let sample_rate = 44_100 * 2;
    let audio = StereoBuffer::from(vec![[1.0, 1.0]; sample_rate]);
    let num_chunks = GainPlugin::process(audio)
        // chunking to process blocks would only make sense
        // if the plugin methods returned unconsumed iterators
        //
        // this number should be configurable in the plugin toml
        .chunks(1024)
        // .inspect(|chunk| eprintln!("{:?}", chunk))
        .count();

    eprintln!("{}", num_chunks);
}
