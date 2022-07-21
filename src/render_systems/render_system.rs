use std::ops::Drop;
use ash::vk;
pub trait RenderSystem:Drop{
    fn bind(&mut self,command_buffer:vk::CommandBuffer);
}