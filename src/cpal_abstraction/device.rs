use std::fmt::Debug;

use cpal::{self, traits::HostTrait, traits::DeviceTrait, Host};

pub struct Device {
    device: cpal::Device,
}

impl Device {
    fn new(device: cpal::Device) -> Device {
        Device {
            device
        }
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