use ash::vk;
use super::command_pool::create_command_pool;
use crate::game::core::Core;
pub struct Command{
    pub command_pool: vk::CommandPool,
    //pub command_buffer: vk::CommandBuffer,
}
impl Command{
    pub fn new(core:&Core)->Self{
        let command_pool = create_command_pool(&core.logical_device, &core.queue_families.queue_family_indices);
        Command{
            command_pool,
            
        }
    }
}