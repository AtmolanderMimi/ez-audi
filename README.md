# Ez-Audi

**A lightweight Linux/WASM compatible audio library based on cpal**

## Features (as of now):

* Read and play LPcm WAVE (.wav) files
* Apply modifiers to the samples for Volume, Looping, etc..
* Control over the raw audio samples
* Get audio file metadata

## Supports (as of now):

* Linux
* LPcm WAVE files
* WASM (See `Usage in WASM environment`)
  *(Windows is has not been tested yet)*

## Get started

```rust
use ez_audi::audio_files::WavAudio;
use ez_audi::public_traits::*;

let wav_audio = WavAudio::build_from_path("test_assets/u8-stereo-lpcm.wav").unwrap();

// Creates an audio player, keep it in scope to keep the audio playing
let player = wav_audio.play_on_default_output(false).unwrap();

std::thread::sleep(std::time::Duration::from_secs(2));
```

## Usage in WASM environment:

Whilst ez-audi is fairly easy to use in WASM, there are some surprises:

* Playback needs to be started/resumed inside a user gesture (button click, etc.). **Playback does not start automatically**.
* You most likely want to get files (or their data at least) from your website through GET requests. I would recomend using the `Cursor` struct from std to turn a `Vec` of bytes into a reader. Whilst the functions for building struct from files/paths still exists, they are useless in WASM.
* Note that, like in all other enviroments, you need to keep your SamplesPlayer in memory for playback.

## What is left to do:

* Pretty much everything, don't use this in any serious projects unless you enjoy relying on unstable, buggy code
* The documentation is also very much lacking
