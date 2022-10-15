use {
    crate::error::InstanceResult,
    ash::{extensions::ext::DebugUtils, vk},
    std::fmt::Debug,
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

    /// Destroy the Vulkan instance.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    ///   - all resources which were created with this instance must be
    ///     destroyed prior to calling this function
    ///   - the ash instance must not be used after calling this function
    pub unsafe fn destroy(&mut self) {
        if self.debug_utils.is_some() {
            self.debug_utils
                .as_ref()
                .unwrap()
                .destroy_debug_utils_messenger(
                    self.debug_messenger.unwrap(),
                    None,
                );
        }
        self.ash.destroy_instance(None);
    }
}

impl Debug for VulkanInstance {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VulkanInstance")
            .field("layers", &self.layers)
            .field("extensions", &self.extensions)
            .field("is_debug_enabled", &cfg!(debug_assertions))
            .finish()
    }
}

impl std::fmt::Display for VulkanInstance {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_fmt(format_args!("{:#?}", self))
    }
}
