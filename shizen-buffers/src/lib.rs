pub mod audio;
pub mod midi;

/// A marker trait for [`AudioBuffer<CH>`](`audio::AudioBuffer`) and [`MidiBuffer`](`midi::MidiBuffer`).
// I should make this private
pub trait Buffer {}

pub trait Plugin {
    type InputBuffer: Buffer;
    type OutputBuffer: Buffer;

    // check with rak about iterator logic
    // + check plugin_main! macro from that lib
    //
    // im using this as a contract for generating the plugin?
    fn process(buffer: Self::InputBuffer) -> Self::OutputBuffer;
}
