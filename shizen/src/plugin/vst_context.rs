/// This struct represents contextual information provided by the host to a VST3 plugin
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct VSTContext {
    /// The sample rate of the audio processing environment
    pub sample_rate: f32,
    /// The buffer size used for audio processing
    // is this the same as block size?? if not, replace it
    pub buffer_size: usize,
    pub is_playing: bool,
    /// BPM
    pub tempo: f32,
}
