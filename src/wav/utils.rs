use std::io::{self, BufReader, Read};
use std::fs::File;

/// Allows to tell if a file is wav from its header
pub fn file_is_wav(path: &str) -> Result<bool, io::Error> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = [0u8; 44];

    reader.read_exact(&mut buffer)?;

    let mut header = String::new();
    for byte in buffer {
        header.push(byte as char);
    }

    Ok(header.contains("WAVE") && header.contains("RIFF"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_wav_files() {
        let is_wav = file_is_wav("test_assets/9000.wav").unwrap();
        assert!(is_wav);
    }

    #[test]
    fn detects_non_wav_files() {
        let is_wav = file_is_wav("test_assets/ballon.mp3").unwrap();
        assert!(!is_wav);
    }
}