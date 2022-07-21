use ash::vk;
use crate::core::Core;
use crate::MAX_FRAMES_IN_FLIGHT;
pub fn create_command_pool(core: &Core) -> vk::CommandPool {
    let pool_info = vk::CommandPoolCreateInfo::builder()
        .queue_family_index(
            core.queue_families
                .queue_family_indices
                .graphics_family
                .unwrap(),
        )
        .flags(
            vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER
                | vk::CommandPoolCreateFlags::TRANSIENT,
        )
        .build();
    unsafe {
        core.logical_device
            .create_command_pool(&pool_info, None)
            .expect("Failed to create command pool")
    }
}
