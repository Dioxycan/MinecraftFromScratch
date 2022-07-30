use super::pipeline::{Pipeline, PipelineConfig};
use super::RenderSystem;
use crate::core::Core;
use ash::vk;
use nalgebra_glm as glm;
use std::mem;
use std::rc::Rc;
#[derive(Debug)]
pub struct PushConstant {
    pub proj_view: glm::Mat4,
}
impl PushConstant {
    pub fn new(proj_view: glm::Mat4) -> Self {
        PushConstant { proj_view }
    }
    pub fn as_u8(&self) -> Vec<u8> {
        unsafe {
            let mut proj_view = self.proj_view.as_slice().align_to::<u8>().1.to_owned();
            proj_view
        }
    }
}

pub struct MainRenderSystem {
    core: Rc<Core>,
    pipeline_layout: vk::PipelineLayout,
    pipeline: Pipeline,
}
impl MainRenderSystem {
    fn new(
        core: Rc<Core>,
        render_pass: &vk::RenderPass,
        attribute_descriptions: &Vec<vk::VertexInputAttributeDescription>,
        binding_descriptions: &Vec<vk::VertexInputBindingDescription>,
    ) -> Self {
        let mut render_system = MainRenderSystem {
            pipeline: Pipeline::new(core.clone()),
            pipeline_layout: vk::PipelineLayout::default(),
            core,
        };
        render_system.create_pipeline_layout();
        render_system.create_pipeline(render_pass, attribute_descriptions, binding_descriptions);
        render_system
    }
    fn create_pipeline_layout(&mut self) {
        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder()
            .push_constant_ranges(&[vk::PushConstantRange::builder()
                .stage_flags(vk::ShaderStageFlags::VERTEX)
                .offset(0)
                .size(mem::size_of::<PushConstant>() as u32)
                .build()])
            .build();

        let pipeline_layout = unsafe {
            self.core
                .logical_device
                .create_pipeline_layout(&pipeline_layout_info, None)
                .expect("Failed to create pipeline layout")
        };
        self.pipeline_layout = pipeline_layout;
    }
    fn create_pipeline(
        &mut self,
        render_pass: &vk::RenderPass,
        attribute_descriptions: &Vec<vk::VertexInputAttributeDescription>,
        binding_descriptions: &Vec<vk::VertexInputBindingDescription>,
    ) {
        assert!(
            self.pipeline_layout != vk::PipelineLayout::default(),
            "Cannot create pipeline before pipeline layout"
        );
        let mut pipeline_config = PipelineConfig::default();
        pipeline_config.init();
        pipeline_config.render_pass = render_pass.clone();
        pipeline_config.pipeline_layout = self.pipeline_layout;
        self.pipeline.create_graphic_pipeline(
            "shaders/shader.vert.spv",
            "shaders/shader.frag.spv",
            pipeline_config,
            binding_descriptions.as_slice(),
            attribute_descriptions.as_slice(),
        );
    }
    pub fn bind(&mut self, command_buffer: &vk::CommandBuffer, push_constant: PushConstant) {
        let pipeline_bind_point = vk::PipelineBindPoint::GRAPHICS;
        unsafe {
            self.core.logical_device.cmd_bind_pipeline(
                *command_buffer,
                pipeline_bind_point,
                self.pipeline.graphic_pipeline,
            );
            //bind vertex buffer to cmd
            self.core.logical_device.cmd_push_constants(
                *command_buffer,
                self.pipeline_layout,
                vk::ShaderStageFlags::VERTEX,
                0,
                push_constant.as_u8().as_slice(),
            )
        }
    }
}
impl RenderSystem for MainRenderSystem {
    fn new(
        core: Rc<Core>,
        render_pass: &vk::RenderPass,
        attribute_descriptions: &Vec<vk::VertexInputAttributeDescription>,
        binding_descriptions: &Vec<vk::VertexInputBindingDescription>,
    ) -> Self {
        Self::new(
            core,
            render_pass,
            attribute_descriptions,
            binding_descriptions,
        )
    }
    fn bind(&mut self, command_buffer: &vk::CommandBuffer, push_constant: PushConstant) {
        self.bind(command_buffer, push_constant);
    }
}
impl Drop for MainRenderSystem {
    fn drop(&mut self) {
        unsafe {
            self.core
                .logical_device
                .destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}
