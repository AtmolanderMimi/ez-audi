# Ez-Audi
**A lightweight (to be cross-platform/wasm-compilable(?)) audio playback library based on cpal**

## Features (as of now):
* Read and play LPcm WAVE (.wav) files
* Apply modifiers to the samples for Volume, Looping, etc..
* Control over the raw audio samples
* Get audio file metadata

## Supports (as of now):
* Linux
* LPcm WAVE files

## Get started
```rust
use ez_audi::audio_types::WavAudio;
use ez_audi::public_traits::*;

let wav_audio = WavAudio::new("yourfilehere.wav").unwrap();

// Creates an audio player, keep it in scope to keep the audio playing
let player = wav_audio.play_on_default_output(false).unwrap();

std::thread::sleep(std::time::Duration::from_secs(2));
```

## What is left to do:
* Pretty much everything, don't use this in any serious projects unless you enjoy relying on unstable, buggy code
* The documentation is also very much lacking