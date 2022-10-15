mod common;

use {
    anyhow::Result,
    assert2::{check, let_assert},
    ccthw_ash_instance::{InstanceError, VulkanInstance},
};

#[test]
pub fn create_instance() -> Result<()> {
    common::setup_logger();

    let mut instance = unsafe { VulkanInstance::new(&[], &[])? };

    log::info!("Successfully Created Instance - {}", instance);

    unsafe { instance.destroy() };

    Ok(())
}

#[test]
pub fn missing_extensions_should_fail() {
    common::setup_logger();

    let_assert!(
        Err(InstanceError::MissingExtensions(extensions)) = unsafe {
            VulkanInstance::new(&["bogus_extension_name".to_owned()], &[])
        }
    );
    check!(extensions.contains(&"bogus_extension_name".to_owned()));
}

#[test]
pub fn missing_layers_should_fail() {
    common::setup_logger();

    let_assert!(
        Err(e) = unsafe {
            VulkanInstance::new(&[], &["bogus_layer_name".to_owned()])
        }
    );
    let_assert!(InstanceError::MissingLayers(missing_layers) = e);
    check!(missing_layers.contains(&"bogus_layer_name".to_owned()));
}
