pub mod main_render_system;
mod pipeline;
use ash::vk;
pub use main_render_system::MainRenderSystem;
use std::ops::Drop;
use crate::core::Core;
use std::rc::Rc;
use main_render_system::PushConstant;
pub trait RenderSystem: Drop {
    fn new(
        core: Rc<Core>,
        render_pass: &vk::RenderPass,
        attribute_descriptions: &Vec<vk::VertexInputAttributeDescription>,
        binding_descriptions: &Vec<vk::VertexInputBindingDescription>,
    ) -> Self
    where
        Self: Sized;
    fn bind(&mut self, command_buffer: &vk::CommandBuffer,push_constant:PushConstant);
}

#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = mem::zeroed();
            (&b.$field as *const _ as isize) - (&b as *const _ as isize)
        }
    }};
}
