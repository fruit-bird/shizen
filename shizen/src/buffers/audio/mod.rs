mod audio_buffer;
mod channel_type;

pub use audio_buffer::{AudioBuffer, Sample};
pub use channel_type::ChannelType;

/// A macro to replace the redundant syntax of:
/// ```
/// # let audio_buffer = AudioBuffer::default();
/// # let output = AudioBuffer::default();
/// match audio_buffer.channel_type {
///     ChannelType::Mono => AudioBuffer::new_mono(output),
///     ChannelType::Stereo => AudioBuffer::new_stereo(output),
/// }
/// ```
/// with the shorter
/// ```
/// Audio![output as audio_buffer.channel_type]
/// ```
///
/// This macro behaves similarly to a parameterized copy constructor for [`AudioBuffer`]
///
/// # Usage
/// ```
/// Audio![output as Mono];
/// Audio![output as Stereo];
/// Audio![output as audio_buffer.channel_type];
/// ```
#[macro_export]
macro_rules! Audio {
    ($output:ident as Mono) => {
        AudioBuffer::new_mono($output)
    };

    ($output:ident as Stereo) => {
        AudioBuffer::new_stereo($output)
    };

    ($output:ident as $channel_type:expr) => {
        match $channel_type {
            $crate::buffers::ChannelType::Mono => AudioBuffer::new_mono($output),
            $crate::buffers::ChannelType::Stereo => AudioBuffer::new_stereo($output),
            // _ => unimplemented!("Whoops, this channel type has yet to be implemented"),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_processing_concept() {
        let mut audio = AudioBuffer::new_stereo(vec![0.0, 1.0, 3.0, 5.0]);
        let _processed = audio.iter_mut().map(|s| *s * 2.0);
    }
}
