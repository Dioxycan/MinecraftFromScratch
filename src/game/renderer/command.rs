use ash::vk;
use crate::game::core::Core;
use crate::game::renderer::swap_chain::MAX_FRAMES_IN_FLIGHT;

pub struct Command{
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,
}
impl Command{
    pub fn new(core:&Core)->Self{
        let command_pool = create_command_pool(core);
        let command_buffers =create_command_buffers(core, &command_pool);
        Command{
            command_pool,
            command_buffers
        }
    }
}
pub fn create_command_pool(core:&Core)->vk::CommandPool{
    let pool_info = vk::CommandPoolCreateInfo::builder()
        .queue_family_index(core.queue_families.queue_family_indices.graphics_family.unwrap())
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER | vk::CommandPoolCreateFlags::TRANSIENT)  
        .build();
    unsafe{
        core.logical_device.create_command_pool(&pool_info, None).expect("Failed to create command pool")
    }
}

pub fn create_command_buffers(core:&Core,command_pool:&vk::CommandPool)->Vec<vk::CommandBuffer>{
    let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_buffer_count(MAX_FRAMES_IN_FLIGHT as u32)
        .command_pool(command_pool.clone())
        .level(vk::CommandBufferLevel::PRIMARY)
        .build();
    unsafe{
        core.logical_device.allocate_command_buffers(&command_buffer_allocate_info).expect("Failed to allocate command buffers")
    }
}

