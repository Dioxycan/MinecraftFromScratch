use ash::vk;
use crate::core::Core;
use crate::MAX_FRAMES_IN_FLIGHT;
pub fn create_command_buffers(
    core: &Core,
    command_pool: &vk::CommandPool,
) -> Vec<vk::CommandBuffer> {
    let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_buffer_count(MAX_FRAMES_IN_FLIGHT)
        .command_pool(command_pool.clone())
        .level(vk::CommandBufferLevel::PRIMARY)
        .build();
    unsafe {
        core.logical_device
            .allocate_command_buffers(&command_buffer_allocate_info)
            .expect("Failed to allocate command buffers")
    }
}
