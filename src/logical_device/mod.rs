use {
    crate::{
        ffi, InstanceResult, PhysicalDevice, VulkanHandle, VulkanInstance,
    },
    ash::vk,
    indoc::indoc,
};

mod queue_family_info;

pub use self::queue_family_info::QueueFamilyInfo;

/// The logical device and information about the backing physical device.
///
/// Basically everything done with Vulkan requires a logical device. This type
/// helps with the creation of the logical device and keeps the relevant
/// information about the underlying physical device at hand for debugging and
/// referenc.
pub struct LogicalDevice {
    physical_device: PhysicalDevice,
    active_physical_device_extensions: Vec<String>,
    device: ash::Device,
}

impl LogicalDevice {
    /// Get the physical device being controlled by this logical device.
    pub fn physical_device(&self) -> &PhysicalDevice {
        &self.physical_device
    }

    /// Get all of the device extensions used when creating the device.
    pub fn active_physical_device_extensions(&self) -> &[String] {
        &self.active_physical_device_extensions
    }

    /// Create a logical device for interfacing with a given physical device.
    ///
    /// # Params
    ///
    /// * `instance` - the Vulkan instance which provides access to the physical
    ///   device.
    /// * `physical_device` - the actual physical device which this logical
    ///   device will control.
    /// * `physical_device_extensions` - the extentions to enable for the
    ///   physical device.
    /// * `queue_family_infos` - a slice of structs which control how many
    ///   device queues to create.
    ///
    /// # Safety
    ///
    /// Unsafe because the logical device must be dropped before the instance
    /// used to create it.
    pub unsafe fn new(
        instance: &VulkanInstance,
        physical_device: PhysicalDevice,
        physical_device_extensions: &[String],
        queue_family_infos: &[QueueFamilyInfo],
    ) -> InstanceResult<Self> {
        let (_c_layer_names, layer_name_ptrs) = unsafe {
            // SAFE because the name strings are not dropped until after
            // the call to create device.
            ffi::to_os_ptrs(instance.layers())
        };
        let (_c_ext_names, ext_name_ptrs) = unsafe {
            // SAFE because the name strings are not dropped until after
            // the call to create device.
            ffi::to_os_ptrs(physical_device_extensions)
        };

        let mut features = *physical_device.features();
        let physical_device_features_v2 = unsafe {
            // SAFE because the features struct is not moved and is not
            // dropped until after the call to create device.
            features.link_p_next_chain()
        };

        let queue_create_infos: Vec<vk::DeviceQueueCreateInfo> =
            queue_family_infos
                .iter()
                .map(|family_info| unsafe {
                    // SAFE because the family infos are not dropped until
                    // after the call to create device and no more calls to
                    // add_queue_priority can be made to the family infos.
                    family_info.as_queue_create_info()
                })
                .collect();

        let create_info = vk::DeviceCreateInfo {
            p_next: physical_device_features_v2
                as *mut vk::PhysicalDeviceFeatures2
                as *mut std::ffi::c_void,
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            enabled_layer_count: layer_name_ptrs.len() as u32,
            pp_enabled_layer_names: layer_name_ptrs.as_ptr(),
            enabled_extension_count: ext_name_ptrs.len() as u32,
            pp_enabled_extension_names: ext_name_ptrs.as_ptr(),

            // Enabled Features is null because PhysicalDeviceFeatures2 is
            // provided through the p_next pointer.
            p_enabled_features: std::ptr::null(),

            ..Default::default()
        };

        let device = unsafe {
            // SAFE because the logical device handle will be owned by the
            // logical device.
            instance.ash().create_device(
                *physical_device.raw(),
                &create_info,
                None,
            )?
        };

        Ok(Self {
            physical_device,
            active_physical_device_extensions: physical_device_extensions
                .to_vec(),
            device,
        })
    }
}

impl Drop for LogicalDevice {
    /// Destroy the logical device.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    ///   - The logical device must be dropped before the instance is destroyed.
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}

impl std::fmt::Debug for LogicalDevice {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LogicalDevice")
            .field("physical_device", &self.physical_device)
            .field(
                "active_physical_device_extensions",
                &self.active_physical_device_extensions,
            )
            .field("device", &unsafe { self.raw().handle() })
            .finish()
    }
}

impl std::fmt::Display for LogicalDevice {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_fmt(format_args!(
            indoc!(
                "
                LogicalDevice {{
                    physical_device: {},
                    active_physical_device_extensions: {:?}
                }}"
            ),
            self.physical_device,
            self.active_physical_device_extensions(),
        ))?;
        Ok(())
    }
}

impl VulkanHandle for LogicalDevice {
    type Handle = ash::Device;

    unsafe fn raw(&self) -> &Self::Handle {
        &self.device
    }
}
