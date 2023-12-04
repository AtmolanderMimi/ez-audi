use ez_audi::audio_types::WavAudio;
use ez_audi::public_traits::*;

fn main() {
    let wav_audio = WavAudio::new("test_assets/helium.wav").unwrap();
    println!("Sample type: {:?}", wav_audio.metadata().sample_type());
    let samples = wav_audio.get_samples().unwrap();
    println!("Sample type sample type sample(????): {:?}", samples.metadata().sample_type());

    let mut player = wav_audio.play_on_default_output().unwrap();
    player.add_modifier(Box::new(ez_audi::modifiers::Volume(1.0)));

    std::thread::sleep(std::time::Duration::from_secs(1));

    player.stop();

    std::thread::sleep(std::time::Duration::from_secs(1));

    player.start();

    std::thread::sleep(std::time::Duration::from_secs(1));

    player.clear_modifiers();
    player.add_modifier(Box::new(ez_audi::modifiers::Volume(0.5)));

    std::thread::sleep(std::time::Duration::from_secs(1));
}