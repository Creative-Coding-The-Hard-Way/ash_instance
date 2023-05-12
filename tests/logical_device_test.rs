use {
    anyhow::Result,
    ash::vk,
    ccthw_ash_instance::{
        LogicalDevice, PhysicalDevice, PhysicalDeviceFeatures, QueueFamilyInfo,
        VulkanInstance,
    },
};

mod common;

#[test]
pub fn create_logical_device() -> Result<()> {
    common::setup_logger();

    // Create a Vulkan instance.
    let instance = unsafe { VulkanInstance::new(&[], &[])? };

    // Pick a suitable physical device
    let physical_device = PhysicalDevice::enumerate_supported_devices(
        &instance,
        &PhysicalDeviceFeatures::default(),
    )?
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

    let logical_device = unsafe {
        LogicalDevice::new(&instance, physical_device, &[], &[family_info])
            .unwrap()
    };

    log::info!("Created Logical device! {:#?}", logical_device);

    Ok(())
}
