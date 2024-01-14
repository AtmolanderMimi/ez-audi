use std::io::{self, BufReader, BufRead, Seek};
use std::fs::File;

/// Allows to tell if a file, representing audio data, is a WAVE file from its header
pub fn file_is_wav(path: &str) -> Result<bool, io::Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    reader_is_wav(reader)
}

/// Allows to tell if a reader, representing an audio file, is a WAVE file from its header
pub fn reader_is_wav<T: BufRead + Seek>(mut reader: T) -> Result<bool, io::Error> {
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
        let is_wav = file_is_wav("test_assets/helium.wav").unwrap();
        assert!(is_wav);
    }

    #[test]
    fn detects_non_wav_files() {
        let is_wav = file_is_wav("test_assets/ballon.mp3").unwrap();
        assert!(!is_wav);
    }
}