use super::{Samples, IntermediateSampleType};

/// A trait to implement on your sample modifiers (aka effects). 
/// Note that the modifiers are made to act upon cpal samples, go see the Sample trait cpal provides.
pub trait ModifierTrait {
    /// Modifies the samples it is used upon
    fn modify(&self, samples: Samples<IntermediateSampleType>) -> Samples<IntermediateSampleType>;
}

/// Multiples the amplitude by the f32 value
pub struct Volume(pub IntermediateSampleType);

impl ModifierTrait for Volume {
    fn modify(&self, samples: Samples<IntermediateSampleType>) -> Samples<IntermediateSampleType> {
        let new_samples = samples.samples.into_iter()
            .map(|s| s * self.0)
            .collect();

        Samples::new(new_samples, samples.metadata)
    }
}