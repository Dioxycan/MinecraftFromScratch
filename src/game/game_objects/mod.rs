pub mod camera;
pub mod key_event;
pub mod block;
use crate::offset_of;
use nalgebra_glm as glm;
use ash::vk;
use std::mem;
use crate::render_systems::RenderSystem;
pub (super) use super::STATIC_MOVE_SPEED;
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub color: glm::Vec3,
}
impl Vertex {
    pub fn new(position: glm::Vec3, color: glm::Vec3) -> Self {
        Vertex { position, color }
    }
    pub fn get_binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(mem::size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }
    pub fn get_attribute_descriptions() -> Vec<vk::VertexInputAttributeDescription> {
        vec![
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Vertex, position) as u32)
                .build(),
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Vertex, color) as u32)
                .build(),
        ]
    }
}

pub trait GameObject{
    fn new()->Self where Self: Sized;
    fn bind(&self,command_buffer:& vk::CommandBuffer);
    fn draw(&self);
    fn update(&self);
}


