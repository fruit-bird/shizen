# SHIZEN
Shizen is a project that aims to create a simple and easy way to create VST plugins. The goal is to make it easy to create VST plugins without having to navigate through the complexities

## Comparison
Here is a simple example of how to create a VST that swaps the left and right channels of a stereo audio using SHIZEN, with a comparison to JUCE

### JUCE Implementation
```cpp
class SwapPlugin : public AudioProcessor {
public:
    SwapPlugin() : AudioProcessor() {}

    void processBlock(
        AudioBuffer<float>& buffer,
        MidiBuffer&
    ) override {
        const int numChannels = buffer.getNumChannels();
        const int numSamples = buffer.getNumSamples();

        // Only swap channels if we have two channels (stereo)
        if (numChannels != 2) return;

        for (int i = 0; i < numSamples; i++) {
            const float leftSample = buffer.getSample(0, sample);
            const float rightSample = buffer.getSample(1, sample);

            buffer.setSample(0, sample, rightSample);
            buffer.setSample(1, sample, leftSample);
        }
    }
};
```

### SHIZEN Implementation
```rust
#[shizen]
pub fn SwapPlugin(audio_buffer: StereoBuffer) -> StereoBuffer {
    audio_buffer
        .map(|[l, r]| [r, l])
        .collect_audio(44_100)
}
```
The beauty in the SHIZEN implementation is that we inject the stereo buffer directly into the function, without having to worry about handling the number of channels or the length of the buffer. SHIZEN makes it so that you can apply this swap effect to only stereo audio

## What is This?
This project stemmed from my friend [JUKE YOU](https://soundcloud.com/jukeyou) giving me the idea of a VST plugin that would achieve a certain effect that is otherwise a hassle to create manually. I thought it was a great idea and decided to make it a reality

Unfortunately, the industry standard for making VSTs, [JUCE](https://juce.com), is in C++ (the devil's tongue). So, I decided to make my own framework that would make it easier to create VSTs. Obviously, nothing against JUCE, most of what I'm doing is inspired by their work and choices. I just wanted to make something that was more in line with my preferences

# THIS IS A WIP!!
