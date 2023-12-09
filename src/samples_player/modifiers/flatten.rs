use crate::samples::IntermediateSampleType;
use crate::samples::Samples;

use super::ModifierTrait;
use super::utils;

#[derive(Debug, Clone)]
/// Flatten the audio by applying util::into_n_channels(1) to the samples and than going back to the original
pub struct Flatten;

impl ModifierTrait for Flatten {
    fn modify(&self, samples: Samples<IntermediateSampleType>) -> Samples<IntermediateSampleType> {
        let original_metadata = samples.metadata.clone();
        let one_channel_samples = utils::into_n_channels(samples, 1);

        let original_channel_count_samples = utils::into_n_channels(one_channel_samples, original_metadata.channels.clone());

        Samples::new(original_channel_count_samples.samples, original_metadata)
    }
}