#[derive(Debug, PartialEq, Default)]
#[non_exhaustive]
#[repr(u8)]
pub enum ChannelType {
    #[default]
    Mono = 1,
    Stereo = 2,
    // MultiChannel(NonZeroU8),
    // SurroundSound,
}