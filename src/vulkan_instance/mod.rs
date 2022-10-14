use {
    crate::error::InstanceResult,
    ash::{extensions::ext::DebugUtils, vk},
    indoc::indoc,
};

mod create_instance;
mod debug_callback;

/// The Ash instance, entry, and additional data provided when the instance was
/// created.
pub struct VulkanInstance {
    layers: Vec<String>,
    extensions: Vec<String>,

    debug_messenger: Option<vk::DebugUtilsMessengerEXT>,
    debug_utils: Option<DebugUtils>,

    entry: ash::Entry,
    ash: ash::Instance,
}

impl VulkanInstance {
    /// Create a new Vulkan instance.
    ///
    /// # Params
    ///
    /// * `required_extensions` - All of the extension names required by this
    ///   application. The DebugUtils extension is added automatically when
    ///   compiled with debug assertions enabled.
    /// * `required_layers` - All of the layers required by this application.
    ///
    /// # Returns
    ///
    /// The Vulkan Instance or an InstanceError if any of the extensions or
    /// layers are unavailable.
    ///
    /// # Safety
    ///
    /// The Application must ensure that all device resources created with the
    /// instance are destroyed proior to dropping the returned struct.
    pub unsafe fn new(
        required_extensions: &[String],
        required_layers: &[String],
    ) -> InstanceResult<Self> {
        let actual_required_extensions =
            Self::with_additional_extensions(required_extensions);

        let (entry, ash) = Self::create_instance(
            &actual_required_extensions,
            required_layers,
        )?;

        let mut vulkan_instance = Self {
            layers: required_layers.to_vec(),
            extensions: actual_required_extensions.to_vec(),
            debug_messenger: None,
            debug_utils: None,
            entry,
            ash,
        };

        vulkan_instance.setup_debug_logger()?;

        Ok(vulkan_instance)
    }

    /// The raw Ash Entry.
    pub fn entry(&self) -> &ash::Entry {
        &self.entry
    }

    /// The raw Ash library instance.
    pub fn ash(&self) -> &ash::Instance {
        &self.ash
    }

    /// The layers used to create this Vulkan Instance.
    pub fn layers(&self) -> &[String] {
        &self.layers
    }

    /// The extensions used to creat this Vulkan Instance.
    pub fn extensions(&self) -> &[String] {
        &self.extensions
    }

    /// Set the debug name for an object owned by the provided logical device.
    ///
    /// This is a no-op for release builds.
    ///
    /// # Params
    ///
    /// * `logical_device` - the logical Vulkan device used to create the object
    ///   referenced by the name info struct.
    /// * `name_info` - the name info struct containing the targeted object and
    ///   its new name.
    #[cfg(debug_assertions)]
    pub fn debug_utils_set_object_name(
        &self,
        logical_device: &ash::Device,
        name_info: &vk::DebugUtilsObjectNameInfoEXT,
    ) {
        let result = unsafe {
            self.debug_utils
                .as_ref()
                .unwrap()
                .debug_utils_set_object_name(logical_device.handle(), name_info)
        };
        if result.is_err() {
            log::warn!(
                "Unable to set debug name for device! {:#?} {:#?}",
                name_info,
                result.err().unwrap()
            );
        }
    }

    /// Set the debug name for an object owned by the provided logical device.
    ///
    /// This is a no-op for release builds.
    ///
    /// # Params
    ///
    /// * `logical_device` - the logical Vulkan device used to create the object
    ///   referenced by the name info struct.
    /// * `name_info` - the name info struct containing the targeted object and
    ///   its new name.
    #[cfg(not(debug_assertions))]
    pub fn debug_utils_set_object_name(
        &self,
        _logical_device: &ash::Device,
        _name_info: &vk::DebugUtilsObjectNameInfoEXT,
    ) {
        // no-op
    }

    // /// Create the logical device with the requested queues.
    // pub fn create_logical_device(
    //     &self,
    //     physical_device: &vk::PhysicalDevice,
    //     physical_device_extensions: &[String],
    //     queue_create_infos: &[vk::DeviceQueueCreateInfo],
    //     physical_device_features: PhysicalDeviceFeatures,
    // ) -> Result<ash::Device, VulkanError> {
    //     let (_c_layer_names, layer_name_ptrs) =
    //         unsafe { to_os_ptrs(&self.layers) };
    //     let (_c_ext_names, ext_name_ptrs) =
    //         unsafe { to_os_ptrs(physical_device_extensions) };

    //     let mut maintenance4_features =
    //         vk::PhysicalDeviceMaintenance4Features {
    //             ..physical_device_features.maintenance4
    //         };
    //     let mut descriptor_indexing_features =
    //         vk::PhysicalDeviceDescriptorIndexingFeatures {
    //             p_next: &mut maintenance4_features
    //                 as *mut vk::PhysicalDeviceMaintenance4Features
    //                 as *mut c_void,
    //             ..physical_device_features.descriptor_indexing_features
    //         };
    //     let physical_device_features_v2 = vk::PhysicalDeviceFeatures2 {
    //         p_next: &mut descriptor_indexing_features
    //             as *mut vk::PhysicalDeviceDescriptorIndexingFeatures
    //             as *mut c_void,
    //         features: physical_device_features.features,
    //         ..Default::default()
    //     };
    //     let create_info = vk::DeviceCreateInfo {
    //         p_next: &physical_device_features_v2
    //             as *const vk::PhysicalDeviceFeatures2
    //             as *const c_void,
    //         queue_create_info_count: queue_create_infos.len() as u32,
    //         p_queue_create_infos: queue_create_infos.as_ptr(),
    //         p_enabled_features: std::ptr::null(),
    //         pp_enabled_layer_names: layer_name_ptrs.as_ptr(),
    //         enabled_layer_count: layer_name_ptrs.len() as u32,
    //         pp_enabled_extension_names: ext_name_ptrs.as_ptr(),
    //         enabled_extension_count: ext_name_ptrs.len() as u32,
    //         ..Default::default()
    //     };

    //     unsafe {
    //         self.ash
    //             .create_device(*physical_device, &create_info, None)
    //             .map_err(VulkanError::UnableToCreateLogicalDevice)
    //     }
    // }
}

impl std::fmt::Display for VulkanInstance {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_fmt(format_args!(
            indoc!(
                "VulkanInstance
                  -> Layers: {:?}
                  -> Extensions: {:?}"
            ),
            self.layers(),
            self.extensions()
        ))
    }
}

impl Drop for VulkanInstance {
    fn drop(&mut self) {
        unsafe {
            self.ash.destroy_instance(None);
        }
    }
}
