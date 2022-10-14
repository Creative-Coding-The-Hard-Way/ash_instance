# Creative Coding The Hard Way - Ash Instance

A small library for handling the boilerplate associated with creating a
Vulkan instance with extensions and layers.

## Examples

```rust
use ccthw_ash_instance::{
    VulkanInstance,
    PhysicalDeviceFeatures,
    PhysicalDevice
};

let instance = unsafe { VulkanInstance::new(&[], &[]).unwrap() };

let mut physical_device_features = PhysicalDeviceFeatures::default();
physical_device_features.maintenance4_mut().maintenance4 = ash::vk::TRUE;

PhysicalDevice::enumerate_supported_devices(
    &instance,
    &PhysicalDeviceFeatures::default(),
)
.unwrap()
.iter()
.for_each(|device| {
    log::info!("Found device {}", device.name());
});
```
