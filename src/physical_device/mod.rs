mod physical_device_features;
mod physical_device_properties;

use {
    crate::{ffi, InstanceResult, VulkanHandle, VulkanInstance},
    ash::vk,
    indoc::indoc,
};

pub use self::{
    physical_device_features::PhysicalDeviceFeatures,
    physical_device_properties::PhysicalDeviceProperties,
};

/// A Vulkan physical device along with its properties and requested features.
///
/// Physical devices are purely descriptive and can be cloned without concern
/// for underlying GPU resources.
#[derive(Clone)]
pub struct PhysicalDevice {
    properties: PhysicalDeviceProperties,
    features: PhysicalDeviceFeatures,
    available_extensions: Vec<vk::ExtensionProperties>,
    available_extension_names: Vec<String>,
    queue_family_properties: Vec<vk::QueueFamilyProperties>,
    physical_device: vk::PhysicalDevice,
}

impl PhysicalDevice {
    /// Properties for all queue families supported by this device.
    pub fn queue_family_properties(&self) -> &[vk::QueueFamilyProperties] {
        &self.queue_family_properties
    }

    /// The set of all extensions available on this device.
    pub fn available_extensions(&self) -> &[vk::ExtensionProperties] {
        &self.available_extensions
    }

    /// The set of all extension names available on this device.
    pub fn available_extension_names(&self) -> &[String] {
        &self.available_extension_names
    }

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
        ffi::string_from_i8(&self.properties().properties().device_name)
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
        log::trace!(
            "Looking for a device with the following features:\n{:#?}",
            required_features
        );
        let all_supported_devices: Vec<vk::PhysicalDevice> =
            unsafe { instance.ash().enumerate_physical_devices()? }
                .into_iter()
                .filter(|physical_device| {
                    let available_features =
                        PhysicalDeviceFeatures::from_physical_device(
                            instance,
                            physical_device,
                        );
                    log::trace!(
                        "Physical Device {:?}\nHas features: {:#?}",
                        physical_device,
                        available_features
                    );
                    required_features.is_supported_by(&available_features)
                })
                .collect();

        let mut devices_with_requested_features = vec![];
        for physical_device in all_supported_devices {
            let properties = PhysicalDeviceProperties::from_physical_device(
                instance,
                &physical_device,
            );
            let extension_properties = unsafe {
                instance
                    .ash()
                    .enumerate_device_extension_properties(physical_device)?
            };
            let extension_names: Vec<String> = extension_properties
                .iter()
                .map(|props| ffi::string_from_i8(&props.extension_name))
                .filter_map(|name| name.ok())
                .collect();
            let queue_family_properties = unsafe {
                instance.ash().get_physical_device_queue_family_properties(
                    physical_device,
                )
            };
            devices_with_requested_features.push(Self {
                properties,
                features: *required_features,
                available_extensions: extension_properties,
                available_extension_names: extension_names,
                queue_family_properties,
                physical_device,
            });
        }

        Ok(devices_with_requested_features)
    }
}

impl std::fmt::Debug for PhysicalDevice {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if formatter.alternate() {
            formatter
                .debug_struct("PhysicalDevice")
                .field("properties", &self.properties)
                .field("features", &self.features)
                .field("available_extensions", &self.available_extensions)
                .field("queue_family_properties", &self.queue_family_properties)
                .finish()
        } else {
            formatter.write_str(&self.name())
        }
    }
}

impl std::fmt::Display for PhysicalDevice {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_fmt(format_args!(indoc!("{:?}"), self.name(),))?;
        Ok(())
    }
}

impl VulkanHandle for PhysicalDevice {
    type Handle = vk::PhysicalDevice;

    unsafe fn raw(&self) -> &Self::Handle {
        &self.physical_device
    }
}
