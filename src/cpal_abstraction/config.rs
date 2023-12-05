use cpal::{SupportedOutputConfigs, SupportedStreamConfig, SupportedStreamConfigRange, SampleRate, SupportedBufferSize, SampleFormat};

use crate::{traits::AudioMetadataTrait, errors::{PlayError, AudioSettings}, Error};

// TODO: Find a better way to filter this
/// Tryies to find a stream config that fits exactly the provided config.
/// For the moment it will not return anything if the config is not a almost perfect match (sample format, channel count and sample rate)
pub fn find_fitting_stream_config(metadata: &impl AudioMetadataTrait, range: SupportedOutputConfigs) -> Error<SupportedStreamConfigRange> {
    let mut supported_configs = range.collect::<Vec<_>>();

    let channels = metadata.channels() as u16;

    let sample_format = match metadata.sample_type() {
        Some(t) => Some(t.into()),
        None => None,
    };

    let sample_rate = SampleRate(metadata.sample_rate());

    let new_supported_configs = supported_configs.iter()
        .filter(|s_config| {
            let sample_rate_range = (s_config.min_sample_rate())..=(s_config.max_sample_rate());

            (Some(s_config.sample_format()) == sample_format) &&
            (s_config.channels() == channels) &&
            sample_rate_range.contains(&sample_rate)
        })
        .collect::<Vec<&SupportedStreamConfigRange>>();

    match new_supported_configs.get(0) {
        Some(s_config) => return Ok((*s_config).clone()),
        None => ()
    };

    // Error report, if it gets here in execution then its because it failed to aquire a stream config
    let mut error_list = Vec::new();

    // Was it the sample type?
    let new_supported_configs = supported_configs.iter()
        .filter(|s_config| {
            Some(s_config.sample_format()) == sample_format
        })
        .collect::<Vec<&SupportedStreamConfigRange>>();

    if new_supported_configs.len() == 0 {
        error_list.push(AudioSettings::SampleType(metadata.sample_type()))
    }

    // Was it the sample rate?
    let new_supported_configs = supported_configs.iter()
        .filter(|s_config| {
            let sample_rate_range = (s_config.min_sample_rate())..=(s_config.max_sample_rate());

            sample_rate_range.contains(&sample_rate)
        })
        .collect::<Vec<&SupportedStreamConfigRange>>();
    
    if new_supported_configs.len() == 0 {
        error_list.push(AudioSettings::SampleRate(metadata.sample_rate()))
    }

    // Was it the channels?
    let new_supported_configs = supported_configs.iter()
        .filter(|s_config| {
            s_config.channels() == channels
        })
        .collect::<Vec<&SupportedStreamConfigRange>>();
    
    if new_supported_configs.len() == 0 {
        error_list.push(AudioSettings::Channels(metadata.channels()))
    }

    // Was it none individually?
    if error_list.len() == 0 {
        error_list.push(AudioSettings::Combinaison)
    }

    Err(PlayError::DeviceDoesNotSupportAudioSettings(error_list, None))
}

/// Creates a new SupportedStreamConfig whilst transfering the generic integer types into the library's types.
/// This creates a SupportedStreamConfig with the buffer size of Unknown for the moment.
pub fn new_supported_stream_config(channels: u16, sample_rate: u32, sample_format: SampleFormat) -> SupportedStreamConfig {
    let sample_rate = SampleRate(sample_rate);
    // TODO: Figure out what is a buffer size
    let buffer_size = SupportedBufferSize::Unknown;
    SupportedStreamConfig::new(channels, sample_rate, buffer_size, sample_format)
}