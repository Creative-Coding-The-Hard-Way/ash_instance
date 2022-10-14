use {ash::vk, thiserror::Error};

pub type InstanceResult<T> = Result<T, InstanceError>;

#[derive(Debug, Error)]
pub enum InstanceError {
    #[error(transparent)]
    CannotLoadVulkanEntry(#[from] ash::LoadingError),

    #[error("Missing Vulkan extensions {0:?}")]
    MissingExtensions(Vec<String>),

    #[error("Missing Vulkan layers {0:?}")]
    MissingLayers(Vec<String>),

    #[error("Unexpected Vulkan error! {0:?}")]
    UnexpectedVulkanError(#[from] vk::Result),
}
