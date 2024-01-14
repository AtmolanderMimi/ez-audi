use std::sync::{Mutex, MutexGuard, Arc};

use crate::{Device, traits::AudioMetadataTrait, cpal_abstraction, Error, modifiers::ModifierTrait};

use cpal_abstraction::{Sample, Samples, SamplesTrait, IntermediateSampleType};

use super::SamplesPlayerTrait;

/// Manages the applying of modifiers and the sending of samples to audio streams, **DOES NOT transform the original sample into IntermediateSampleType which is much slower**.
/// Go see SamplesPlayer for a more efficient samples player.
pub struct ExactSamplesPlayer<T: Sample> {
    original_samples: Samples<T>,
    modifiers: Vec<Box<dyn ModifierTrait>>,
    samples_with_modifiers: Option<Arc<Mutex<Samples<T>>>>,
    stream: Option<cpal_abstraction::Stream>,
}

impl<T: Sample> ExactSamplesPlayer<T>
where IntermediateSampleType: cpal::FromSample<T> {
    /// Creates a new `SamplePlayer` directly from `Samples<T>`
    pub fn new(samples: Samples<T>) -> ExactSamplesPlayer<T> {
        Self {
            original_samples: samples,
            modifiers: Vec::new(),
            samples_with_modifiers: None,
            stream: None,
        }
    }

    fn aquire_samples_with_modifiers_mutex_guard(&self) -> Option<MutexGuard<Samples<T>>> {
        let modified_samples_mutex = match &self.samples_with_modifiers {
            Some(m) => m,
            None => return None,
        };

        let mutex_lock = modified_samples_mutex.lock();
        match mutex_lock {
            Ok(l) => Some(l),
            // TODO: Better error handling on that // Kinda? the Option enum signifies that
            Err(_) => None,
        }
    }

    fn change_samples_with_modifiers(&mut self, samples: Samples<T>) {
        let mutex_guard_option = self.aquire_samples_with_modifiers_mutex_guard();

        if let Some(mut guard) = mutex_guard_option {
            *guard = samples;
        } else {
            // Creates a Arc if there is none yet
            drop(mutex_guard_option);
            self.samples_with_modifiers = Some(Arc::new(Mutex::new(samples)))
        }
    }

    /// Applies all the modifiers
    fn apply_modifiers(&mut self) {
        // Most of the time seems to be spent moving samples, cloning and transfering from one type to another
        let mut modified_samples = self.original_samples.clone().into_generic_representation_samples();
        for modifier in &self.modifiers {
            modified_samples = modifier.modify(modified_samples);
        }

        let t_samples = modified_samples.into_t_samples();
        
        self.change_samples_with_modifiers(t_samples);
    }

    fn set_stream(&mut self, stream: cpal_abstraction::Stream) {
        self.apply_modifiers();

        self.stream = Some(stream);
    }
}

impl<T: Sample> SamplesPlayerTrait for ExactSamplesPlayer<T>
where IntermediateSampleType: cpal::FromSample<T> {
    fn metadata(&self) -> Box<dyn AudioMetadataTrait> {
        Box::new(self.original_samples.metadata.clone())
    }

    fn add_modifier(&mut self, modifier: Box<dyn ModifierTrait>) {
        self.modifiers.push(modifier);
        
        self.apply_modifiers();
    }

    fn clear_modifiers(&mut self) {
        self.modifiers = Vec::new();

        self.apply_modifiers();
    }

    fn start(&self) -> Error<()> {
        let stream = match &self.stream {
            Some(s) => s,
            None => return Ok(()), // No stream to start
        };

        stream.start()
    }

    fn stop(&self) -> Error<()> {
        let stream = match &self.stream {
            Some(s) => s,
            None => return Ok(()), // No stream to stop
        };

        stream.stop()
    }

    fn play_on_device(&mut self, device: Device) -> Error<()> {
        // Makes sure that there is a Sample in self.samples_with_modifiers
        self.apply_modifiers();

        let samples_arc = self.samples_with_modifiers
            .as_ref()
            .expect("no samples with modifiers");
        let samples_arc = Arc::clone(samples_arc);

        let stream = device.create_stream(&self.original_samples.metadata,
            samples_arc)?;

        // Makes sure that the stream is started
        stream.start()?;

        self.set_stream(stream);

        Ok(())
    }
}