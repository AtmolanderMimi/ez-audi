use ez_audi::{self, cpal_abstraction::{Device, SamplesTrait}, wav::WavAudio};

fn main() {
    let device = Device::default_output().unwrap();

    let wav_audio = WavAudio::new("test_assets/9000.wav").unwrap();
    let samples = wav_audio.get_samples().unwrap();

    let stream = samples.play_on_device(device);

    std::thread::sleep(std::time::Duration::from_secs(1));

    stream.stop();

    std::thread::sleep(std::time::Duration::from_secs(1));

    stream.start();

    std::thread::sleep(std::time::Duration::from_secs(1));
}