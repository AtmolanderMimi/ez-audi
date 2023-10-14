use cpal::{traits::HostTrait, SupportedStreamConfig, Sample};
use cpal::traits::*;

use crate::cpal_abstraction::{Device, config};


/// Builds a mutable closure that contains samples.
/// It is meant to be used as a part of data_callback function argument
/// to the build_output_stream method on cpal's device struct.
pub fn build_sample_container<T: Sample>(samples: Vec<T>) -> impl FnMut() -> Option<T> {
    let mut samples_iter = samples.into_iter();
    let next_sample = move || {
        samples_iter.next()
    };

    next_sample
}

/// Function meant to be used as data_callback function argument
/// to the build_output_stream method on cpal's device struct.
pub fn play_default_output<T: Sample>(config: &SupportedStreamConfig, samples: Vec<T>) {
    let device = Device::default_output()
        .expect("no default output device on the default host");

    let config_range = device.inner_device().supported_output_configs()
        .expect("default output device of default host has no output configs");
    let config = config::best_fitting_stream_config(&config, config_range)
        .expect("default output device of default host has no matching output configs");

    todo!("create stream and play audio")
}