use ash::vk;
use crate::game::core::device::queue::QueueFamilyIndices;

pub fn create_command_pool(device:&ash::Device,indices:&QueueFamilyIndices)->vk::CommandPool{
    let pool_info = vk::CommandPoolCreateInfo::builder()
        .queue_family_index(indices.graphics_family.unwrap())
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER | vk::CommandPoolCreateFlags::TRANSIENT)  
        .build();
    unsafe{
        device.create_command_pool(&pool_info, None).expect("Failed to create command pool")
    }
}