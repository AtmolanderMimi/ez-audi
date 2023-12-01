use std::sync::{Mutex, MutexGuard};

use cpal::traits::StreamTrait;

use crate::{Device, traits::AudioMetadataTrait};

use super::{Sample, Samples, ModifierTrait};

/// Manages the applying of modifiers and the sending of samples to audio streams
pub struct SamplesPlayer<T: Sample> {
    sample_index: usize,
    original_samples: Samples<T>,
    modifiers: Vec<Box<dyn ModifierTrait<T>>>,
    samples_with_modifiers: Option<Mutex<Samples<T>>>,
    stream: Option<cpal::Stream>,
}

impl<T: Sample> SamplesPlayer<T> {
    pub fn new(samples: Samples<T>) -> SamplesPlayer<T> {
        Self {
            sample_index: 0,
            original_samples: samples,
            modifiers: Vec::new(),
            samples_with_modifiers: None,
            stream: None,
        }
    }

    // Applies all the modifiers
    fn apply_modifiers(&mut self) {
        let mut modified_samples = self.original_samples.clone();
        for modifier in &self.modifiers {
            modified_samples = modifier.modify(&modified_samples)
        }

        self.change_samples_with_modifiers(modified_samples);
    }

    fn aquire_samples_with_modifiers_mutex_guard(&self) -> Option<MutexGuard<Samples<T>>> {
        let modified_samples_mutex = match &self.samples_with_modifiers {
            Some(m) => m,
            None => return None,
        };

        let mutex_lock = modified_samples_mutex.lock();
        match mutex_lock {
            Ok(l) => Some(l),
            // TODO: Better error handling on that 
            Err(_) => None,
        }
    }

    fn change_samples_with_modifiers(&mut self, samples: Samples<T>) {
        let mut guard = match self.aquire_samples_with_modifiers_mutex_guard() {
            // If the lock is failed to be aquired then it is probably because the thread that sent
            // the samples to the audio stream failed and so another mutex should be made when
            // the SamplesPlayer is used for another stream
            Some(g) => g,
            None => return,
        };

        *guard = samples;
    }
}

pub trait SamplesPlayerTrait<T: Sample> {
    /// Returns the metadata of the samples
    fn metadata(&self) -> Box<dyn AudioMetadataTrait>;

    /// Adds a modifier
    fn add_modifier(&mut self, modifier: Box<dyn ModifierTrait<T>>);

    /// Clears all modifiers and their effects
    fn clear_modifiers(&mut self);

    /// Gets the next sample
    fn next_sample(&self) -> Option<T>;

    /// Sets one of the fields of the player to a stream
    fn set_stream(&mut self, stream: cpal::Stream);

    /// Starts/Continues the playing
    fn start(&self);

    /// Stops the playing
    fn stop(&self);

    /// Starts playing on a device
    fn play_on_device(&mut self, device: Device) {
        // FIXME: Get Device struct to comply with the new system
        todo!("Here")
    }

    fn play_on_default(&mut self) {
        todo!("Here")
    }
}

impl<T: Sample> SamplesPlayerTrait<T> for SamplesPlayer<T> {
    fn metadata(&self) -> Box<dyn AudioMetadataTrait> {
        Box::new(self.original_samples.metadata.clone())
    }

    fn add_modifier(&mut self, modifier: Box<dyn ModifierTrait<T>>) {
        self.modifiers.push(modifier);
        
        self.apply_modifiers();
    }

    fn clear_modifiers(&mut self) {
        self.modifiers = Vec::new();

        self.apply_modifiers();
    }

    fn next_sample(&self) -> Option<T> {
        self.sample_index += 1;

        let samples = match self.aquire_samples_with_modifiers_mutex_guard() {
            Some(s) => s,
            None => return None,
        };
        
        let next_sample = samples.samples.get(self.sample_index - 1).copied();


        next_sample
    }

    fn set_stream(&mut self, stream: cpal::Stream) {
        // TODO: Should not have to put the whole original samples here since
        // it is reset when applying the modifiers
        self.samples_with_modifiers = Some(Mutex::new(self.original_samples.clone()));
        self.apply_modifiers();

        self.stream = Some(stream);
    }

    fn start(&self) {
        let stream = match &self.stream {
            Some(s) => s,
            None => return,
        };

        stream.play();
    }

    fn stop(&self) {
        let stream = match &self.stream {
            Some(s) => s,
            None => return,
        };

        stream.pause();
    }
}