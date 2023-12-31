use std::{fmt::Debug, sync::{Mutex, Arc}};

use cpal::{self, traits::{HostTrait, DeviceTrait}, Host};

use crate::{traits::AudioMetadataTrait, Error, errors::PlayError};
use crate::samples_player::SamplesPlayerTrait;

use super::{config, Samples, Sample, Stream};

/// An abstraction over cpal::Device, represents a physical output device
pub struct Device {
    device: cpal::Device,
}

impl Device {
    fn new(device: cpal::Device) -> Device {
        Device {
            device
        }
    }

    /// Plays the samples on the default device of the default host
    pub fn play_default_output(player: &mut impl SamplesPlayerTrait) -> Error<()> {
        let device =  match Device::default_output() {
            Some(d) => d,
            None => return Err(PlayError::DeviceDoesNotExist { name: "default".to_string() })
        };
    
        player.play_on_device(device)
    }

    /// Creates a stream that will play the metadata based on the metadata given
    pub fn create_stream<T: Sample>(&self, metadata: &impl AudioMetadataTrait, samples: Arc<Mutex<Samples<T>>>) -> Error<Stream> {
        let config_range = match self.inner_device().supported_output_configs() {
            Ok(c) => c,
            Err(e) => return Err(PlayError::DeviceIoError(
                format!("the device had an issue fetching configs"), Some(Box::new(e))))
        };

        let config = config::find_fitting_stream_config(metadata, config_range)?;

        let sample_rate = cpal::SampleRate(metadata.sample_rate());
        let config = config.with_sample_rate(sample_rate);

        let mut index = 0;
        let data_callback = move |samples_out: &mut [T], _info: &_| {
            // TODO: This should maybe not crash // Removed expect so that it does not print text
            let samples = samples.lock().unwrap(); //.expect("samples are inaccessible to audio stream");
            for sample in samples_out {
                *sample = match samples.samples.get(index) {
                    Some(s) => *s,
                    None => T::EQUILIBRIUM,
                };
                index += 1;
            }
        };

        let error_callback = |err| {
            // FIXME: I will puke uncontrolably if this is not removed within a reasonable amount of time :)
            panic!("{:?}", err);
        };
        
        let stream_err = self
            .inner_device()
            .build_output_stream(&config.config(), data_callback, error_callback, None);

        let stream = match stream_err {
            Ok(s) => s,
            Err(e) => return Err(PlayError::DeviceIoError(
                "device had an error while trying to build an audio stream".to_string(),
                Some(Box::new(e)))),
        };

        Ok(stream.into())
    }

    /// Plays the samples in the SamplesPlayer on this device
    pub fn play<T: Sample>(self, player: &mut impl SamplesPlayerTrait) -> Error<()> {
        player.play_on_device(self)
    }

    /// Returns all devices from all hosts
    fn list_cpal_devices() -> Vec<cpal::Device> {
        // We like Iterators, You like Iterators, Everybody likes Iterators!
        let hosts = cpal::available_hosts();

        // Gets all hosts, discards ones that cause an error
        let hosts = hosts.into_iter()
            .map(|id| cpal::host_from_id(id))
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect::<Vec<Host>>();

        // Gets all devices form all hosts, discards ones that cause an error
        hosts.into_iter()
            .map(|h| h.devices())
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap().collect::<Vec<cpal::Device>>())
            .flatten()
            .collect()
    } 

    /// Returns the default output device of the default host.
    /// Be aware that there may be none
    pub fn default_output() -> Option<Device> {
        let inner_device = cpal::default_host().default_output_device()?;

        Some(Device::new(inner_device))
    }

    /// Gives the name of all devices on all hosts
    pub fn list_device_names() -> Vec<String> {
        let devices = Device::list_cpal_devices();

        // Gets all device names, discards ones that cause an error 
        devices.into_iter()
            .map(|d| d.name())
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect()
    }

    /// Creates a new device from its device name
    pub fn new_from_name(device_name: &str) -> Option<Device> {
        let devices = Device::list_cpal_devices();

        let the_device = devices.into_iter()
            .filter(|d| d.name().unwrap_or("".to_string()) == device_name)
            .next()?;

        Some(Device::new(the_device))
    }

    #[doc(hidden)]
    /// Gives a reference to the inner cpal device struct
    pub fn inner_device(&self) -> &cpal::Device {
        &self.device
    }
}

impl Device {
    /// Returns the name of the device, if it can find one
    pub fn name(&self) -> Option<String> {
        match self.device.name() {
            Ok(n) => Some(n),
            Err(_) => None,   
        }
    }
}

impl Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Device {}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// This test assumes that the computer that runs the tests at least have 1
    /// device with a name
    fn list_device_names_works() {
        assert_ne!(Device::list_device_names().len(), 0)
    }

    #[test]
    fn new_from_name_works() {
        let device_name = &Device::list_device_names()[0];
        assert!(Device::new_from_name(device_name).is_some())
    }
}