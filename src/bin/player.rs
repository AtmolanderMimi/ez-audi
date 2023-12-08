use ez_audi::audio_files::WavAudio;
use ez_audi::{public_traits::*, SamplesPlayer, modifiers};

fn main() {
    const FILE_NAME: &str = "test_assets/i16-stereo-lpcm.wav";
    let wav_audio = WavAudio::new(FILE_NAME).unwrap();
    println!("Sample type: {:?}", wav_audio.metadata().sample_type());

    let samples = wav_audio.get_samples().unwrap();

    println!("Playing strait from WavAudio...");
    let mut player = wav_audio.play_on_default_output().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Adding loop modifier for convinience");
    player.add_modifier(Box::new(modifiers::Loop(100)));

    println!("Stopping for 2 sec");
    player.stop().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Playing with VOLUME modifier at 0.2");
    player.start().unwrap();
    player.add_modifier(Box::new(modifiers::Volume(0.2)));
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Clearing modifiers");
    player.clear_modifiers();
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    println!("Playing from IntermediateRepresentation samples (don't worry it is meant to error out)");
    let gen_samples = samples.into_generic_representation_samples();
    let mut player = SamplesPlayer::new(gen_samples);
    println!("{}", player.play_on_default().unwrap_err());
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Playing from IntermediateRepresentation samples into i16");
    let gen_samples = samples.into_generic_representation_samples();
    let i16_samples = gen_samples.into_t_samples::<i16>();
    let mut player = SamplesPlayer::new(i16_samples);
    player.play_on_default().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Playing from FLATTENED (1 channel) samples");
    let gen_samples = samples.into_generic_representation_samples();
    let flattened_samples = modifiers::utils::into_n_channels(gen_samples, 1);
    let mut player = SamplesPlayer::new(flattened_samples.into_t_samples::<i16>());
    player.play_on_default().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Playing with lower sample rate");
    let gen_samples = samples.into_generic_representation_samples();
    let lower_sample_rate_samples = modifiers::utils::into_sample_rate(gen_samples, 12000);
    let mut player = SamplesPlayer::new(lower_sample_rate_samples.into_t_samples::<i16>());
    player.play_on_default().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(2));
}