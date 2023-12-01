use cpal::Sample as CpalSampleTrait;

use super::{Samples, Sample};

/// A trait to implement on your sample modifiers (aka effects). 
/// Note that the modifiers are made to act upon cpal samples, go see the Sample trait cpal provides.
pub trait ModifierTrait<T: Sample>: 'static {
    /// 
    fn modify(&self, samples: &Samples<T>) -> Samples<T>;
}

/// Multiples the amplitude by the f32 value
pub struct Volume(f32);

impl<T: Sample> ModifierTrait<T> for Volume
where f32: cpal::FromSample<T> {
    fn modify(&self, samples: &Samples<T>) -> Samples<T> {
        let mut new_samples = Vec::new();

        for sample in &samples.samples {
            let new_sample = sample.to_sample::<f32>().mul_amp(self.0).to_sample();
            new_samples.push(new_sample)
        };

        Samples::new(new_samples, samples.metadata.clone())
    }
}