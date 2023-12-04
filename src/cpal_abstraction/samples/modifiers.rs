use super::{Samples, IntermediateSampleType};

/// A trait to implement on your sample modifiers (aka effects). 
/// Note that the modifiers are made to act upon cpal samples, go see the Sample trait cpal provides.
pub trait ModifierTrait {
    /// Modifies the samples it is used upon.
    /// # NOTES:
    /// * Take into consideration that audio with two channels will be arranged like so: 
    /// Left1, Right1, Left2, Right2...
    /// ## When using inside SamplesPlayer
    /// * The samples you will return will be converted back into the sample type of the original samples
    /// * Do not try to modify the metadata as it is not taken into consideration, changes things such as
    /// the sample rate or channel number will do nothing on how the samples are played
    /// * The order of modifiers is important for SamplesPlayer, the modifiers are applied to the result of the previous one
    fn modify(&self, samples: Samples<IntermediateSampleType>) -> Samples<IntermediateSampleType>;
}

/// Multiples the amplitude by the IntermediateSampleType (f64) value
pub struct Volume(pub IntermediateSampleType);

impl ModifierTrait for Volume {
    fn modify(&self, samples: Samples<IntermediateSampleType>) -> Samples<IntermediateSampleType> {
        let new_samples = samples.samples.into_iter()
            .map(|s| s * self.0)
            .collect();

        Samples::new(new_samples, samples.metadata)
    }
}

///// Adds channels or flattens existing ones into the desired amount, also changes the metadata to match
//pub fn into_n_channels(samples: Samples<IntermediateSampleType>, nb_channels: u16) -> Samples<IntermediateSampleType> {
//    let metadata = samples.metadata;
//
//    if nb_channels < metadata.channels {
//        
//    }
//}