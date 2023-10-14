use cpal::{SupportedOutputConfigs, SupportedStreamConfig, SupportedStreamConfigRange, SampleRate, SupportedBufferSize, SampleFormat};

// TODO: Find a better way to filter this
/// Tryies to find a stream config that fits exactly the provided config.
/// For the moment it will not return anything if the config is not a almost perfect match (sample format, channel count and sample rate)
pub fn best_fitting_stream_config(config: &SupportedStreamConfig, range: SupportedOutputConfigs) -> Option<SupportedStreamConfigRange> {
    let mut supported_configs = range.collect::<Vec<_>>();

    let channels = config.channels();
    let sample_format = config.sample_format();
    let sample_rate = config.sample_rate();
    supported_configs = supported_configs.into_iter().filter(|s_config| {
        let sample_rate_range = (s_config.min_sample_rate())..=(s_config.max_sample_rate());

        (s_config.sample_format() == sample_format) &&
        (s_config.channels() == channels) &&
        sample_rate_range.contains(&sample_rate)
    }).collect();

    match supported_configs.get(0) {
        Some(s_config) => Some(s_config.clone()),
        None => None
    }
}

/// Creates a new SupportedStreamConfig whilst transfering the generic integer types into the library's types.
/// This creates a SupportedStreamConfig with the buffer size of Unknown for the moment.
pub fn new_supported_stream_config(channels: u16, sample_rate: u32, sample_format: SampleFormat) -> SupportedStreamConfig {
    let sample_rate = SampleRate(sample_rate);
    // TODO: Figure out what is a buffer size
    let buffer_size = SupportedBufferSize::Unknown;
    SupportedStreamConfig::new(channels, sample_rate, buffer_size, sample_format)
}