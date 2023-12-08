//! Contains premade modifiers and a trait to make your own modifiers

use crate::cpal_abstraction::{Samples, IntermediateSampleType};

mod r#loop;
pub use r#loop::Loop;
mod volume;
pub use volume::Volume;

pub mod utils;

/// A trait to implement on your sample modifiers (aka effects). 
/// Note that the modifiers are made to act upon cpal samples, go see the Sample trait cpal provides.
pub trait ModifierTrait: std::fmt::Debug {
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