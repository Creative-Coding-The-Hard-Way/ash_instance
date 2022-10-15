![build, test, and docs](https://github.com/Creative-Coding-The-Hard-Way/ash_instance/actions/workflows/build_test_upload_docs.yml/badge.svg)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)

# [Creative Coding The Hard Way - Ash Instance](https://creative-coding-the-hard-way.github.io/ash_instance/ccthw_ash_instance/index.html)

A small library for handling the boilerplate associated with creating a
Vulkan instance with extensions and layers.

## Examples

```rust
use {
    ash::vk,
    ccthw_ash_instance::{
        VulkanInstance,
        PhysicalDeviceFeatures,
        PhysicalDevice,
        QueueFamilyInfo,
        LogicalDevice,
    },
};

// Create a Vulkan instance.
let mut instance = unsafe { VulkanInstance::new(&[], &[]).unwrap() };

// Pick a suitable physical device
let physical_device = PhysicalDevice::enumerate_supported_devices(
    &instance,
    &PhysicalDeviceFeatures::default(),
)
.unwrap()
.into_iter()
.find(|device| {
    // Find a device which has at least one queue family that supports
    // compute operations.
    device
        .queue_family_properties()
        .iter()
        .any(|family_properties| {
            family_properties
                .queue_flags
                .contains(vk::QueueFlags::COMPUTE)
        })
})
.unwrap();

let compute_queue_index = physical_device
    .queue_family_properties()
    .iter()
    .enumerate()
    .find(|(_, properties)| {
        properties.queue_flags.contains(vk::QueueFlags::COMPUTE)
    })
    .map(|(queue_family_index, _)| queue_family_index)
    .unwrap();

let mut family_info = QueueFamilyInfo::new(compute_queue_index as u32);
family_info.add_queue_priority(1.0);

let mut logical_device = unsafe {
    LogicalDevice::new(&instance, physical_device, &[], &[family_info])
        .unwrap()
};

log::info!("Created Logical device! {}", logical_device);

unsafe { logical_device.destroy() };
unsafe { instance.destroy() };
```
