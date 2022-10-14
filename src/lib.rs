//! A small library for handling the boilerplate associated with creating a
//! Vulkan instance with extensions and layers.
//!
//! # Examples
//!
//! ```
//! use ccthw_ash_instance::{
//!     VulkanInstance,
//!     PhysicalDeviceFeatures,
//!     PhysicalDevice
//! };
//!
//! let instance = unsafe { VulkanInstance::new(&[], &[]).unwrap() };
//!
//! let mut physical_device_features = PhysicalDeviceFeatures::default();
//! physical_device_features.maintenance4_mut().maintenance4 = ash::vk::TRUE;
//!
//! PhysicalDevice::enumerate_supported_devices(
//!     &instance,
//!     &PhysicalDeviceFeatures::default(),
//! )
//! .unwrap()
//! .iter()
//! .for_each(|device| {
//!     log::info!("Found device {}", device.name());
//! });
//! ```

mod error;
mod physical_device;
mod vulkan_instance;

pub use self::{
    error::{InstanceError, InstanceResult},
    physical_device::{
        PhysicalDevice, PhysicalDeviceFeatures, PhysicalDeviceProperties,
    },
    vulkan_instance::VulkanInstance,
};

/// Types which implement this trait can provide the raw Vulkan resource handle
/// as provided by Ash.
pub trait VulkanHandle {
    type Handle;

    /// Get the raw resource handle.
    ///
    /// # Safety
    ///
    /// Unsafe because ownership is not transferred when accessing the raw
    /// handle. The caller is responsible for ensuring no copies of the handle
    /// outlive the original instance.
    unsafe fn raw(&self) -> &Self::Handle;
}
