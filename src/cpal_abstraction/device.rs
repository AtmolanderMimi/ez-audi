//use std::fmt::Debug;
//
//use cpal::{self, traits::{HostTrait, DeviceTrait}, Host};
//
//use super::{config, Samples, Sample, Stream};
//
//pub struct Device {
//    device: cpal::Device,
//}
//
//impl Device {
//    fn new(device: cpal::Device) -> Device {
//        Device {
//            device
//        }
//    }
//
//    pub fn play_default_output<T: Sample>(samples: Samples<T>) -> Stream {
//        let device = Device::default_output()
//            .expect("no default output device on the default host");
//    
//        device.play(samples)
//    }
//
//    pub fn create_stream<T: Sample>(&self, samples: Samples<T>) -> Stream {
//        let config_range = self.inner_device().supported_output_configs()
//            .expect("default output device of default host has no output configs");
//
//        let metadata = (&samples.metadata).into();
//        let config = config::best_fitting_stream_config(&metadata, config_range)
//            .expect("default output device of default host has no matching output configs");
//
//        let sample_rate = cpal::SampleRate(samples.metadata.sample_rate);
//        let config = config.with_sample_rate(sample_rate);
//
//        let mut samples_iter = samples.samples.into_iter();
//        let data_callback = move |samples_out: &mut [T], _info: &_| {
//            for sample in samples_out {
//                *sample = match samples_iter.next() {
//                    Some(s) => s,
//                    None => T::EQUILIBRIUM,
//                }
//            }
//        };
//
//        let error_callback = |err| {
//            // FIXME: Here too,
//            panic!("{:?}", err);
//        };
//        
//        let stream = self
//            .inner_device()
//            .build_output_stream(&config.config(), data_callback, error_callback, None)
//            // FIXME: I will puke uncontrolably if this is not removed within a reasonable amount of time :)
//            .unwrap();
//
//        stream.into()
//    }
//
//    pub fn play<T: Sample>(&self, samples: Samples<T>) -> Stream {
//        let stream = self.create_stream(samples);
//        stream.start();
//
//        stream.into()
//    }
//
//    /// Returns all devices from all hosts
//    fn list_cpal_devices() -> Vec<cpal::Device> {
//        // We like Iterators, You like Iterators, Everybody likes Iterators!
//        let hosts = cpal::available_hosts();
//
//        // Gets all hosts, discards ones that cause an error
//        let hosts = hosts.into_iter()
//            .map(|id| cpal::host_from_id(id))
//            .filter(|r| r.is_ok())
//            .map(|r| r.unwrap())
//            .collect::<Vec<Host>>();
//
//        // Gets all devices form all hosts, discards ones that cause an error
//        hosts.into_iter()
//            .map(|h| h.devices())
//            .filter(|r| r.is_ok())
//            .map(|r| r.unwrap().collect::<Vec<cpal::Device>>())
//            .flatten()
//            .collect()
//    } 
//
//    /// Returns the default output device of the default host.
//    /// Be aware that there may be none
//    pub fn default_output() -> Option<Device> {
//        let inner_device = cpal::default_host().default_output_device()?;
//
//        Some(Device::new(inner_device))
//    }
//
//    /// Gives the name of all devices on all hosts
//    pub fn list_device_names() -> Vec<String> {
//        let devices = Device::list_cpal_devices();
//
//        // Gets all device names, discards ones that cause an error 
//        devices.into_iter()
//            .map(|d| d.name())
//            .filter(|r| r.is_ok())
//            .map(|r| r.unwrap())
//            .collect()
//    }
//
//    pub fn new_from_name(device_name: &str) -> Option<Device> {
//        let devices = Device::list_cpal_devices();
//
//        let the_device = devices.into_iter()
//            .filter(|d| d.name().unwrap_or("".to_string()) == device_name)
//            .next()?;
//
//        Some(Device::new(the_device))
//    }
//
//    #[doc(hidden)]
//    /// Gives a reference to the inner cpal device struct
//    pub fn inner_device(&self) -> &cpal::Device {
//        &self.device
//    }
//}
//
//impl Device {
//    pub fn name(&self) -> Option<String> {
//        match self.device.name() {
//            Ok(n) => Some(n),
//            Err(_) => None,   
//        }
//    }
//}
//
//impl Debug for Device {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        f.write_str("Device {}")
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    /// This test assumes that the computer that runs the tests at least have 1
//    /// device with a name
//    fn list_device_names_works() {
//        assert_ne!(Device::list_device_names().len(), 0)
//    }
//
//    #[test]
//    fn new_from_name_works() {
//        let device_name = &Device::list_device_names()[0];
//        assert!(Device::new_from_name(device_name).is_some())
//    }
//}

use std::{fmt::Debug, sync::{Mutex, Arc}};

use cpal::{self, traits::{HostTrait, DeviceTrait}, Host};

use crate::traits::AudioMetadataTrait;

use super::{config, Samples, Sample, Stream, SamplesPlayerTrait};

pub struct Device {
    device: cpal::Device,
}

impl Device {
    fn new(device: cpal::Device) -> Device {
        Device {
            device
        }
    }

    pub fn play_default_output(player: &mut impl SamplesPlayerTrait) {
        let device = Device::default_output()
            .expect("no default output device on the default host");
    
        player.play_on_device(device);
    }


    pub fn create_stream<T: Sample>(&self, metadata: &impl AudioMetadataTrait, samples: Arc<Mutex<Samples<T>>>) -> Stream {
        let config_range = self.inner_device().supported_output_configs()
            .expect("default output device of default host has no output configs");

        let metadata = config::new_supported_stream_config(
            metadata.channels() as u16,
            metadata.sample_rate(),
            metadata.sample_type().unwrap_or_else(|| todo!("make error here")).into(),
        );
        let config = config::best_fitting_stream_config(&metadata, config_range)
            .expect("default output device of default host has no matching output configs");

        let sample_rate = metadata.sample_rate();
        let config = config.with_sample_rate(sample_rate);

        let mut index = 0;
        let data_callback = move |samples_out: &mut [T], _info: &_| {
            // TODO: This should maybe not crash
            let samples = samples.lock().expect("samples are inaccessible to audio stream");
            for sample in samples_out {
                *sample = match samples.samples.get(index) {
                    Some(s) => *s,
                    None => T::EQUILIBRIUM,
                };
                index += 1;
            }
        };

        let error_callback = |err| {
            // FIXME: Here too,
            panic!("{:?}", err);
        };
        
        let stream = self
            .inner_device()
            .build_output_stream(&config.config(), data_callback, error_callback, None)
            // FIXME: I will puke uncontrolably if this is not removed within a reasonable amount of time :)
            .unwrap();

        stream.into()
    }

    pub fn play<T: Sample>(self, player: &mut impl SamplesPlayerTrait) {
        player.play_on_device(self);
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