use {
    crate::{InstanceResult, VulkanInstance},
    ash::vk,
    std::ffi::c_void,
};

mod is_supported_by;

/// An owned set of physical device features.
#[derive(Copy, Clone, Debug, Default)]
pub struct PhysicalDeviceFeatures {
    physical_device_features2: vk::PhysicalDeviceFeatures2,
    descriptor_indexing_features: vk::PhysicalDeviceDescriptorIndexingFeatures,
    maintenance4: vk::PhysicalDeviceMaintenance4Features,
}

impl VulkanInstance {}

impl PhysicalDeviceFeatures {
    /// Enumerate all physical devices which support all of the desired
    /// features.
    ///
    /// # Params
    pub fn enumerate_supported_devices(
        &self,
        instance: &VulkanInstance,
    ) -> InstanceResult<Vec<vk::PhysicalDevice>> {
        let devices_with_requested_features =
            unsafe { instance.ash().enumerate_physical_devices()? }
                .into_iter()
                .filter(|physical_device| {
                    self.is_supported_by(&Self::from_physical_device(
                        instance,
                        physical_device,
                    ))
                })
                .collect();
        Ok(devices_with_requested_features)
    }

    /// Get the physical device features for a given device.
    ///
    /// # Params
    ///
    /// * `instance` - the instance which supports the physical device
    /// * `physical_device` - the physical device to query for available
    ///   features
    pub fn from_physical_device(
        instance: &VulkanInstance,
        physical_device: &vk::PhysicalDevice,
    ) -> PhysicalDeviceFeatures {
        let mut results = PhysicalDeviceFeatures::default();
        unsafe {
            instance.ash().get_physical_device_features2(
                *physical_device,
                results.link_p_next_chain(),
            )
        }
        results
    }

    pub fn features(&self) -> &vk::PhysicalDeviceFeatures {
        &self.physical_device_features2.features
    }

    pub fn features_mut(&mut self) -> &mut vk::PhysicalDeviceFeatures {
        &mut self.physical_device_features2.features
    }

    pub fn descriptor_indexing_features(
        &self,
    ) -> &vk::PhysicalDeviceDescriptorIndexingFeatures {
        &self.descriptor_indexing_features
    }

    pub fn descriptor_indexing_features_mut(
        &mut self,
    ) -> &mut vk::PhysicalDeviceDescriptorIndexingFeatures {
        &mut self.descriptor_indexing_features
    }

    pub fn maintenance4(&self) -> &vk::PhysicalDeviceMaintenance4Features {
        &self.maintenance4
    }

    pub fn maintenance4_mut(
        &mut self,
    ) -> &mut vk::PhysicalDeviceMaintenance4Features {
        &mut self.maintenance4
    }

    /// Link all of the contained device feature structs using their p_next
    /// pointers.
    ///
    /// # Safety
    ///
    /// The linked pointers are invalid if the owning struct is moved.
    pub unsafe fn link_p_next_chain(
        &mut self,
    ) -> &mut vk::PhysicalDeviceFeatures2 {
        // write the p_next pointer chain
        self.physical_device_features2.p_next = &mut self
            .descriptor_indexing_features
            as *mut vk::PhysicalDeviceDescriptorIndexingFeatures
            as *mut c_void;
        self.descriptor_indexing_features.p_next = &mut self.maintenance4
            as *mut vk::PhysicalDeviceMaintenance4Features
            as *mut c_void;
        &mut self.physical_device_features2
    }
}
