mod command_buffer;
mod command_pool;
use crate::core::Core;
use crate::MAX_FRAMES_IN_FLIGHT;
use ash::vk;
use self::command_buffer::create_command_buffers;
use self::command_pool::create_command_pool;

pub struct Command {
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,
}

impl Command {
    pub fn new(core: &Core) -> Self {
        let command_pool = create_command_pool(core);
        let command_buffers = create_command_buffers(core, &command_pool);
        Command {
            command_pool,
            command_buffers,
        }
    }
}

