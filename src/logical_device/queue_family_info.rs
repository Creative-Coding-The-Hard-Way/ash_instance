use ash::vk;

/// The logical device constructor uses an array of these structs to build the
/// Vulkan DeviceQueueCreateInfo structs.
///
/// It's convenient to use this wrapper instead of the raw DeviceQueueCreateInfo
/// structs because the raw structs include a pointer to an array of priorities.
/// It can be unwieldy to handle the raw pointer without introducing memory
/// safety problems, so this struct owns a vector of priority values.
///
/// # Examples
///
/// ```
/// use ccthw_ash_instance::QueueFamilyInfo;
///
/// let mut queue_family_info = QueueFamilyInfo::new(2);
/// queue_family_info.add_queue_priority(1.0);
///
/// // NOTE - create_info is only valid while QueueFamilyInfo exists and no
/// //        additional calls to add_queue_priority are made.
/// let create_info = unsafe { queue_family_info.as_queue_create_info() };
/// println!("{:#?}", create_info);
/// ```
#[derive(Debug, Clone)]
pub struct QueueFamilyInfo {
    queue_family_index: u32,
    queue_priorities: Vec<f32>,
}

impl QueueFamilyInfo {
    /// Create a new instance with no queue priorities.
    ///
    /// # Params
    ///
    /// * `queue_family_index` - index for the underlying queue family. This
    ///   comes from enumerating the physical devices queue family properties.
    pub fn new(queue_family_index: u32) -> Self {
        Self {
            queue_family_index,
            queue_priorities: vec![],
        }
    }

    /// Add a queue with the given priority.
    ///
    /// # Params
    ///
    /// * `priority` - should be between 0.0 and 1.0. Implementations are
    ///   allowed to give more resources to higher-priority queues, but it isn't
    ///   required. If you don't have a good reason to do otherwise, 1.0 is a
    ///   reasonable choice.
    pub fn add_queue_priority(&mut self, priority: f32) {
        self.queue_priorities.push(priority);
    }

    /// Get a DeviceQueueCreateInfo struct based on the number of queue
    /// priorities specified for this queue family.
    ///
    /// # Safety
    ///
    /// Unsafe because the device queue create info struct contains a pointer
    /// to the queue_priorities vector. This means that if any queue priorities
    /// are added AFTER calling this function, then using the struct will cause
    /// undefined behavior.
    pub unsafe fn as_queue_create_info(&self) -> vk::DeviceQueueCreateInfo {
        vk::DeviceQueueCreateInfo {
            queue_family_index: self.queue_family_index,
            queue_count: self.queue_priorities.len() as u32,
            p_queue_priorities: self.queue_priorities.as_ptr(),
            ..Default::default()
        }
    }
}
