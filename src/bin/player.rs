use std::io::Read;

use ez_audi::audio_files::WavAudio;
use ez_audi::{public_traits::*, modifiers};

fn main() {
    const FILE_NAME: &str = "test_assets/IGN_tanger-ike.wav";
    const EXACT: bool = false;
    const VOLUME: f32 = 1.0;
    //let wav_audio = WavAudio::build_from_path(FILE_NAME).unwrap();
    //let mut player = wav_audio.play_on_default_output(EXACT).unwrap();

    // More WASM frendly way.
    // We get a the bytes then make a reader with it:
    // Note that we can't use File in WASM, so the bytes
    // should probably come from a GET request 
    let file = std::fs::File::open(FILE_NAME).unwrap();
    let bytes = get_bytes(file);
    
    let wav: WavAudio<std::io::Cursor<Vec<u8>>> = WavAudio::build_from_reader(std::io::Cursor::new(bytes)).unwrap();
    
    let mut player = wav.play_on_default_output(EXACT).unwrap();


    // Add modifiers here
    player.add_modifier(Box::new(modifiers::Volume(VOLUME)));
    loop {}
}

fn get_bytes(mut file: std::fs::File) -> Vec<u8> {
    let metadata = file.metadata().expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overflow");
    buffer
} 