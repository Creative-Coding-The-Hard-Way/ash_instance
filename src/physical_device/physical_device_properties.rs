use {crate::VulkanInstance, ash::vk};

/// An owned set of physical device features.
#[derive(Copy, Clone, Debug, Default)]
pub struct PhysicalDeviceProperties {
    physical_device_properties: vk::PhysicalDeviceProperties2,
}

unsafe impl Send for PhysicalDeviceProperties {}

impl PhysicalDeviceProperties {
    /// Get the properties from a physical device.
    pub fn from_physical_device(
        instance: &VulkanInstance,
        physical_device: &vk::PhysicalDevice,
    ) -> Self {
        let mut properties = Self::default();
        unsafe {
            instance.ash().get_physical_device_properties2(
                *physical_device,
                properties.link_p_next_chain(),
            )
        };
        properties
    }

    pub fn properties(&self) -> &vk::PhysicalDeviceProperties {
        &self.physical_device_properties.properties
    }

    pub fn properties_mut(&mut self) -> &mut vk::PhysicalDeviceProperties {
        &mut self.physical_device_properties.properties
    }

    /// Link all of the contained device feature structs using their p_next
    /// pointers.
    ///
    /// # Safety
    ///
    /// The linked pointers are invalid if the owning struct is moved.
    pub unsafe fn link_p_next_chain(
        &mut self,
    ) -> &mut vk::PhysicalDeviceProperties2 {
        // this library doesn't currently support any other p_next types
        // so nothing to link up here
        &mut self.physical_device_properties
    }
}
