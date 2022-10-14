//! A small library for handling the boilerplate associated with creating a
//! Vulkan instance with extensions and layers.
//!
//! # Examples
//!
//! ```
//! use ccthw_ash_instance::{VulkanInstance, PhysicalDeviceFeatures};
//!
//! let instance = unsafe { VulkanInstance::new(&[], &[]).unwrap() };
//!
//! let mut physical_device_features = PhysicalDeviceFeatures::default();
//! physical_device_features.maintenance4_mut().maintenance4 = ash::vk::TRUE;
//!
//! let devices: Vec<ash::vk::PhysicalDevice> = physical_device_features
//!     .enumerate_supported_devices(&instance)
//!     .unwrap();
//! ```

mod error;
mod physical_device;
mod vulkan_instance;

pub use self::{
    error::{InstanceError, InstanceResult},
    physical_device::PhysicalDeviceFeatures,
    vulkan_instance::VulkanInstance,
};
