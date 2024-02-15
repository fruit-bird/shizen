#[derive(Debug, PartialEq, Default)]
#[non_exhaustive]
#[repr(u8)]
pub enum ChannelType {
    Mono = 1,
    #[default]
    Stereo = 2,
    // MultiChannel(NonZeroU8),
    // SurroundSound,
}
