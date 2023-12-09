use crate::cpal_abstraction::SamplesTrait;
use crate::samples::IntermediateSampleType;
use crate::samples::Samples;

use super::ModifierTrait;
use super::utils;

#[derive(Debug, Clone)]
/// It's quite the crude name, I think you can guess what it does.
/// Artificially flattens the audio, reduces the sample rate and transforms the sample type to u8
pub struct Shittify;

impl ModifierTrait for Shittify {
    fn modify(&self, samples: Samples<IntermediateSampleType>) -> Samples<IntermediateSampleType> {
        let original_metadata = samples.metadata.clone();

        let one_channel_samples = utils::into_n_channels(samples, 1);
        let low_sample_rate_samples = utils::into_sample_rate(one_channel_samples, 12000);
        let u8_samples = low_sample_rate_samples.into_t_samples::<u8>();

        let normal_sample_rate_samples = utils::into_sample_rate(u8_samples, original_metadata.sample_rate);
        let normal_type_samples = normal_sample_rate_samples.into_generic_representation_samples();
        let normal_channel_nb_samples = utils::into_n_channels(normal_type_samples, original_metadata.channels);

        Samples::new(normal_channel_nb_samples.samples, original_metadata)
    }
}