use ccthw_ash_instance::VulkanInstance;

mod common;

use {
    anyhow::Result,
    ash::vk,
    assert2::assert,
    ccthw_ash_instance::{PhysicalDevice, PhysicalDeviceFeatures},
};

#[test]
pub fn get_physical_device_with_features() -> Result<()> {
    common::setup_logger();

    let instance = unsafe { VulkanInstance::new(&[], &[])? };

    // It's not possible to know what features or what devices might be
    // available on the testing machine. It's good enough to just verify
    // that this method doesn't fail for now.
    PhysicalDevice::enumerate_supported_devices(
        &instance,
        &PhysicalDeviceFeatures::default(),
    )?
    .iter()
    .for_each(|device| {
        log::info!("Found device {}", device);
    });

    Ok(())
}

#[test]
pub fn feature_should_not_be_supported_by_default() {
    common::setup_logger();

    let mut desired_features = PhysicalDeviceFeatures::default();
    desired_features.features_mut().full_draw_index_uint32 = vk::TRUE;
    assert!(
        !desired_features.is_supported_by(&PhysicalDeviceFeatures::default())
    );
}

#[test]
pub fn feature_should_not_be_supported_when_explicitly_disabled() {
    common::setup_logger();

    let mut desired_features = PhysicalDeviceFeatures::default();
    desired_features.vulkan_13_features_mut().maintenance4 = vk::TRUE;

    let mut available_features = PhysicalDeviceFeatures::default();
    available_features.vulkan_13_features_mut().maintenance4 = vk::FALSE;

    assert!(!desired_features.is_supported_by(&available_features));
}

#[test]
pub fn features_should_be_supported_when_explicitly_enabled() {
    common::setup_logger();

    let mut desired_features = PhysicalDeviceFeatures::default();
    desired_features
        .descriptor_indexing_features_mut()
        .shader_input_attachment_array_dynamic_indexing = vk::TRUE;

    let mut available_features = PhysicalDeviceFeatures::default();
    available_features
        .descriptor_indexing_features_mut()
        .shader_input_attachment_array_dynamic_indexing = vk::TRUE;

    assert!(desired_features.is_supported_by(&available_features));
}

#[test]
pub fn send_physical_device() -> Result<()> {
    common::setup_logger();

    let instance = unsafe { VulkanInstance::new(&[], &[])? };

    let device = PhysicalDevice::enumerate_supported_devices(
        &instance,
        &PhysicalDeviceFeatures::default(),
    )?
    .into_iter()
    .next()
    .unwrap();

    let thread = std::thread::spawn(move || {
        log::info!("Got device: {}", device);
    });

    thread.join().unwrap();

    Ok(())
}
