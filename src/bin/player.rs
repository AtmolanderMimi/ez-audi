use ez_audi::audio_types::WavAudio;
use ez_audi::public_traits::*;

fn main() {
    
    let wav_audio = WavAudio::new("test_assets/9000.wav").unwrap();
    println!("Sample type: {:?}", wav_audio.sample_type());

    let stream = wav_audio.play_on_default_output().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    stream.stop();

    std::thread::sleep(std::time::Duration::from_secs(1));

    stream.start();

    std::thread::sleep(std::time::Duration::from_secs(1));
}