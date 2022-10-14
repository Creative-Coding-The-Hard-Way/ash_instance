mod physical_device_features;
mod physical_device_properties;

use {
    crate::{InstanceResult, VulkanHandle, VulkanInstance},
    ash::vk,
};

pub use self::{
    physical_device_features::PhysicalDeviceFeatures,
    physical_device_properties::PhysicalDeviceProperties,
};

/// A Vulkan physical device along with its properties and requested features.
#[derive(Debug)]
pub struct PhysicalDevice {
    properties: PhysicalDeviceProperties,
    features: PhysicalDeviceFeatures,
    physical_device: vk::PhysicalDevice,
}

impl PhysicalDevice {
    /// The properties for this physical device.
    pub fn properties(&self) -> &PhysicalDeviceProperties {
        &self.properties
    }

    /// The features requested when picking this device.
    pub fn features(&self) -> &PhysicalDeviceFeatures {
        &self.features
    }

    /// The physical device name from the device properties struct.
    pub fn name(&self) -> String {
        String::from_utf8(
            self.properties()
                .properties()
                .device_name
                .into_iter()
                .filter(|&c| c != 0)
                .map(|c| c as u8)
                .collect(),
        )
        .unwrap()
    }

    /// Enumerate all physical devices which support the required featuers.
    ///
    /// # Params
    ///
    /// * `instance` - the Vulkan instance which will provide access to the
    ///   physical devices.
    /// * `features` - the features the device must support.
    pub fn enumerate_supported_devices(
        instance: &VulkanInstance,
        required_features: &PhysicalDeviceFeatures,
    ) -> InstanceResult<Vec<Self>> {
        let devices_with_requested_features: Vec<Self> =
            unsafe { instance.ash().enumerate_physical_devices()? }
                .into_iter()
                .filter(|physical_device| {
                    required_features.is_supported_by(
                        &PhysicalDeviceFeatures::from_physical_device(
                            instance,
                            physical_device,
                        ),
                    )
                })
                .map(|physical_device| {
                    let properties =
                        PhysicalDeviceProperties::from_physical_device(
                            instance,
                            &physical_device,
                        );
                    Self {
                        properties,
                        features: *required_features,
                        physical_device,
                    }
                })
                .collect();
        Ok(devices_with_requested_features)
    }
}

impl VulkanHandle for PhysicalDevice {
    type Handle = vk::PhysicalDevice;

    unsafe fn raw(&self) -> &Self::Handle {
        &self.physical_device
    }
}
