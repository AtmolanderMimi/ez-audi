use ez_audi::{wav::WavAudio, traits::AudioFileTrait};

fn main() {
    let wav_audio = WavAudio::new("test_assets/9000.wav").unwrap();

    let stream = wav_audio.play_on_default_output().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    stream.stop();

    std::thread::sleep(std::time::Duration::from_secs(1));

    stream.start();

    std::thread::sleep(std::time::Duration::from_secs(1));
}