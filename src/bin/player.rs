use ez_audi::audio_files::WavAudio;
use ez_audi::{public_traits::*, SamplesPlayer, modifiers};

fn main() {
    const WAIT_TIME: f32 = 5.0;
    const FILE_NAME: &str = "test_assets/tanger-ike.wav";
    let wav_audio = WavAudio::new(FILE_NAME).unwrap();
    println!("Sample type: {:?}", wav_audio.metadata().sample_type());

    let samples = wav_audio.get_samples().unwrap();

    let true_before = std::time::SystemTime::now();
    println!("Playing strait from WavAudio...");
    let mut player = wav_audio.play_on_default_output().unwrap();
    std::thread::sleep(std::time::Duration::from_secs_f32(WAIT_TIME));

    println!("Adding loop modifier for convinience");
    let before = std::time::SystemTime::now();
    player.add_modifier(Box::new(modifiers::Loop(20)));
    println!("Loop application time {:?}", std::time::SystemTime::elapsed(&before));
    println!("TRUE TIME {:?}", std::time::SystemTime::elapsed(&true_before));

    println!("Stopping for 2 sec");
    player.stop().unwrap();
    std::thread::sleep(std::time::Duration::from_secs_f32(WAIT_TIME));

    println!("Playing with VOLUME modifier at 0.2");
    player.start().unwrap();
    let before = std::time::SystemTime::now();
    player.add_modifier(Box::new(modifiers::Volume(0.2)));
    println!("Volume + Loop application time {:?}", std::time::SystemTime::elapsed(&before));
    std::thread::sleep(std::time::Duration::from_secs_f32(WAIT_TIME));

    println!("Clearing modifiers");
    player.clear_modifiers();
    std::thread::sleep(std::time::Duration::from_secs_f32(WAIT_TIME));
    drop(player);
    
    println!("Playing from IntermediateRepresentation samples (don't worry it is meant to error out)");
    let gen_samples = samples.generic_representation_samples();
    let mut player = SamplesPlayer::new(gen_samples);
    println!("{}", player.play_on_default().unwrap_err());
    std::thread::sleep(std::time::Duration::from_secs_f32(WAIT_TIME));
    drop(player);

    println!("Playing from IntermediateRepresentation samples into i16");
    let gen_samples = samples.generic_representation_samples();
    let i16_samples = gen_samples.into_t_samples::<i16>();
    let mut player = SamplesPlayer::new(i16_samples);
    player.play_on_default().unwrap();
    std::thread::sleep(std::time::Duration::from_secs_f32(WAIT_TIME));
    drop(player);

    println!("Playing from FLATTENED (1 channel) samples");
    let gen_samples = samples.generic_representation_samples();
    //let flattened_samples = modifiers::utils::into_n_channels(gen_samples, 1);
    let mut player = SamplesPlayer::new(gen_samples.into_t_samples::<i16>());
    player.add_modifier(Box::new(modifiers::Flatten));
    player.play_on_default().unwrap();
    std::thread::sleep(std::time::Duration::from_secs_f32(WAIT_TIME));
    drop(player);

    println!("Playing with lower sample rate");
    let gen_samples = samples.generic_representation_samples();
    let lower_sample_rate_samples = modifiers::utils::into_sample_rate(gen_samples, 12000);
    let mut player = SamplesPlayer::new(lower_sample_rate_samples.into_t_samples::<i16>());
    player.play_on_default().unwrap();
    std::thread::sleep(std::time::Duration::from_secs_f32(WAIT_TIME));
    drop(player);

    println!("Playing \"Shittified\"");
    let gen_samples = samples.generic_representation_samples();
    let mut player = SamplesPlayer::new(gen_samples.into_t_samples::<i16>());
    player.add_modifier(Box::new(modifiers::Shittify));
    player.play_on_default().unwrap();
    std::thread::sleep(std::time::Duration::from_secs_f32(WAIT_TIME));
    drop(player);
}