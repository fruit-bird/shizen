mod delay;
mod gain;

pub use crate::delay::DelayComponent;
pub use crate::gain::GainComponent;

use shizen::prelude::*;

#[allow(non_snake_case, unused)]
fn GainDelayPlugin(audio_buffer: AudioBuffer) -> PluginResult<AudioBuffer> /* || MidiBuffer */ {
    let mut gain = GainComponent::new(100.0);
    let mut delay = DelayComponent::new(0.2, 0.4, 0.7);

    // components should be doing fast iterations with rayon when possible
    // parallelize the processing somehow, if it even makes sense
    let gained_audio = audio_buffer
        .iter()
        .process_with(&mut gain);
        // .process_with(&mut delay);

    Ok(gained_audio)
}

#[cfg(test)]
mod tests {
    use super::*;
    use shizen::buffer::ChannelType;

    #[test]
    fn test_gain_delay_plugin() -> PluginResult<()> {
        // Create an example audio buffer
        let audio_buffer = AudioBuffer::new(vec![1.0, 2.0, 3.0, 4.0], ChannelType::Mono);

        // Call the GainDelayPlugin function
        let result = GainDelayPlugin(audio_buffer)?;

        // Assert that the result is as expected
        // assert_eq!(
        //     result,
        //     AudioBuffer::new(vec![100.0, 200.0, 300.0, 400.0], ChannelType::Mono)
        // );
        dbg!(result);

        Ok(())
    }
}
