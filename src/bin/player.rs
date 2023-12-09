use ez_audi::audio_files::WavAudio;
use ez_audi::{public_traits::*, modifiers};

fn main() {
    const FILE_NAME: &str = "test_assets/tanger-ike.wav";
    let wav_audio = WavAudio::new(FILE_NAME).unwrap();

    let true_before = std::time::SystemTime::now();
    let mut player = wav_audio.play_on_default_output().unwrap();
    println!("Getting samples out time {:?}", std::time::SystemTime::elapsed(&true_before));

    // Adding motif
    player.add_modifier(Box::new(modifiers::Loop(20)));
    println!("TRUE TIME {:?}", std::time::SystemTime::elapsed(&true_before));
    std::thread::sleep(std::time::Duration::from_secs(111111111111111))
}
