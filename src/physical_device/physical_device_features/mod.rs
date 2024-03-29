use {crate::VulkanInstance, ash::vk, std::ffi::c_void};

mod is_supported_by;

/// An owned set of physical device features.
#[derive(Copy, Clone, Debug, Default)]
pub struct PhysicalDeviceFeatures {
    physical_device_features2: vk::PhysicalDeviceFeatures2,
    descriptor_indexing_features: vk::PhysicalDeviceDescriptorIndexingFeatures,
    physical_device_vulkan_13_features: vk::PhysicalDeviceVulkan13Features,
}

impl PhysicalDeviceFeatures {
    /// Get the physical device features for a given device.
    ///
    /// # Params
    ///
    /// * `instance` - the instance which provides access to the physical device
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

    pub fn vulkan_13_features(&self) -> &vk::PhysicalDeviceVulkan13Features {
        &self.physical_device_vulkan_13_features
    }

    pub fn vulkan_13_features_mut(
        &mut self,
    ) -> &mut vk::PhysicalDeviceVulkan13Features {
        &mut self.physical_device_vulkan_13_features
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
        self.descriptor_indexing_features.p_next = &mut self
            .physical_device_vulkan_13_features
            as *mut vk::PhysicalDeviceVulkan13Features
            as *mut c_void;
        &mut self.physical_device_features2
    }
}
