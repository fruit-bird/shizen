mod audio_buffer;
mod audio_iterator;

pub use audio_buffer::{AudioBuffer, Sample};
pub use audio_iterator::AudioIterator;

/// A macro to replace the redundant syntax of:
/// ```ignore
/// match audio_buffer.channel_type {
///     ChannelType::Mono => AudioBuffer::new_mono(output),
///     ChannelType::Stereo => AudioBuffer::new_stereo(output),
/// }
/// ```
/// with the shorter
/// ```ignore
/// Audio![output as audio_buffer.channel_type]
/// ```
///
/// This macro behaves similarly to a parameterized copy constructor for [`AudioBuffer`]
///
/// # Usage
/// ```ignore
/// Audio![output as Mono];
/// Audio![output as Stereo];
/// Audio![output as audio_buffer.channel_type];
/// ```
#[macro_export]
macro_rules! Audio {
    ($output:ident as $channel_type_name:ident) => {
        $crate::buffer::AudioBuffer::new($output, $crate::buffer::ChannelType::$channel_type_name)
    };

    ($output:ident as $channel_type:expr) => {
        match $channel_type {
            t @ $crate::buffer::ChannelType::Mono => $crate::buffer::AudioBuffer::new($output, t),
            t @ $crate::buffer::ChannelType::Stereo => $crate::buffer::AudioBuffer::new($output, t),

            // necessary for use outside of this crate because `ChannelType` is tagged as non_exhaustive
            #[allow(unreachable_patterns)]
            _ => unimplemented!("Whoops, this channel type has yet to be implemented"),
        }
    };
}
