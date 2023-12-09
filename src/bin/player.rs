use ez_audi::audio_files::WavAudio;
use ez_audi::{public_traits::*, modifiers};

fn main() {
    const FILE_NAME: &str = "test_assets/tanger-ike.wav";
    const EXACT: bool = false;
    const VOLUME: f32 = 1.0;
    let wav_audio = WavAudio::new(FILE_NAME).unwrap();

    let mut player = wav_audio.play_on_default_output(EXACT).unwrap();

    // Add modifiers here
    player.add_modifier(Box::new(modifiers::Volume(VOLUME)));
    loop {}
}