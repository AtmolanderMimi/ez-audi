use ez_audi::{self, cpal_abstraction::Device, wav::WavAudio, PlayableTrait};

fn main() {
    let device = Device::default_output().unwrap();

    let wav_audio = WavAudio::new("test_assets/9000.wav").unwrap();

    let stream = wav_audio.play(device).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    stream.stop();

    std::thread::sleep(std::time::Duration::from_secs(1));

    stream.start();

    std::thread::sleep(std::time::Duration::from_secs(1));
}