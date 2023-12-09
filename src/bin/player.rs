use ez_audi::audio_files::WavAudio;
use ez_audi::{public_traits::*, modifiers};

fn main() {
    const FILE_NAME: &str = "test_assets/tanger-ike.wav";
    const EXACT: bool = false;
    let wav_audio = WavAudio::new(FILE_NAME).unwrap();

    let mut player = wav_audio.play_on_default_output(EXACT).unwrap();

    player.add_modifier(Box::new(modifiers::Shittify));
    loop {}
}