use super::pipeline::{Pipeline, PipelineConfig};
use crate::game::core::Core;
use crate::offset_of;
use ash::vk;
use nalgebra_glm as glm;
use std::mem::{self, align_of};
use std::rc::Rc;
#[derive(Debug)]
pub struct PushConstant {
    pub offset: glm::Vec3,
}
impl PushConstant {
    pub fn new(transform: glm::Mat2, offset: glm::Vec3) -> Self {
        PushConstant {  offset }
    }
    pub fn to_u8(&self) -> Vec<u8> {
        unsafe { self.offset.as_slice().align_to::<u8>().1.to_owned() }
    }
}
#[derive(Debug)]
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
pub struct MainRenderSystem {
    core: Rc<Core>,
    pipeline_layout: vk::PipelineLayout,
    pipeline: Pipeline,
}
impl MainRenderSystem {
    pub fn new(core: Rc<Core>) -> Self {
        MainRenderSystem {
            pipeline: Pipeline::new(core.clone()),
            pipeline_layout: vk::PipelineLayout::default(),
            core,
        }
    }
    pub fn init(&mut self, render_pass: &vk::RenderPass) {
        self.create_pipeline_layout();
        self.create_pipeline(render_pass);
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
    fn create_pipeline(&mut self, render_pass: &vk::RenderPass) {
        assert!(
            self.pipeline_layout != vk::PipelineLayout::default(),
            "Cannot create pipeline before pipeline layout"
        );
        let mut pipeline_config = PipelineConfig::default();
        pipeline_config.init();
        pipeline_config.render_pass = render_pass.clone();
        pipeline_config.pipeline_layout = self.pipeline_layout;
        let binding_description = Vertex::get_binding_description();
        let attribute_descriptions = Vertex::get_attribute_descriptions();
        self.pipeline.create_graphic_pipeline(
            "shaders/shader.vert.spv",
            "shaders/shader.frag.spv",
            pipeline_config,
            &[binding_description],
            &attribute_descriptions,
        );
    }

    //     std::vector<LveModel::Vertex> vertices{

    //       // left face (white)
    //       {{-.5f, -.5f, -.5f}, {.9f, .9f, .9f}},
    //       {{-.5f, .5f, .5f}, {.9f, .9f, .9f}},
    //       {{-.5f, -.5f, .5f}, {.9f, .9f, .9f}},
    //       {{-.5f, -.5f, -.5f}, {.9f, .9f, .9f}},
    //       {{-.5f, .5f, -.5f}, {.9f, .9f, .9f}},
    //       {{-.5f, .5f, .5f}, {.9f, .9f, .9f}},

    //       // right face (yellow)
    //       {{.5f, -.5f, -.5f}, {.8f, .8f, .1f}},
    //       {{.5f, .5f, .5f}, {.8f, .8f, .1f}},
    //       {{.5f, -.5f, .5f}, {.8f, .8f, .1f}},
    //       {{.5f, -.5f, -.5f}, {.8f, .8f, .1f}},
    //       {{.5f, .5f, -.5f}, {.8f, .8f, .1f}},
    //       {{.5f, .5f, .5f}, {.8f, .8f, .1f}},

    //       // top face (orange, remember y axis points down)
    //       {{-.5f, -.5f, -.5f}, {.9f, .6f, .1f}},
    //       {{.5f, -.5f, .5f}, {.9f, .6f, .1f}},
    //       {{-.5f, -.5f, .5f}, {.9f, .6f, .1f}},
    //       {{-.5f, -.5f, -.5f}, {.9f, .6f, .1f}},
    //       {{.5f, -.5f, -.5f}, {.9f, .6f, .1f}},
    //       {{.5f, -.5f, .5f}, {.9f, .6f, .1f}},

    //       // bottom face (red)
    //       {{-.5f, .5f, -.5f}, {.8f, .1f, .1f}},
    //       {{.5f, .5f, .5f}, {.8f, .1f, .1f}},
    //       {{-.5f, .5f, .5f}, {.8f, .1f, .1f}},
    //       {{-.5f, .5f, -.5f}, {.8f, .1f, .1f}},
    //       {{.5f, .5f, -.5f}, {.8f, .1f, .1f}},
    //       {{.5f, .5f, .5f}, {.8f, .1f, .1f}},

    //       // nose face (blue)
    //       {{-.5f, -.5f, 0.5f}, {.1f, .1f, .8f}},
    //       {{.5f, .5f, 0.5f}, {.1f, .1f, .8f}},
    //       {{-.5f, .5f, 0.5f}, {.1f, .1f, .8f}},
    //       {{-.5f, -.5f, 0.5f}, {.1f, .1f, .8f}},
    //       {{.5f, -.5f, 0.5f}, {.1f, .1f, .8f}},
    //       {{.5f, .5f, 0.5f}, {.1f, .1f, .8f}},

    //       // tail face (green)
    //       {{-.5f, -.5f, -0.5f}, {.1f, .8f, .1f}},
    //       {{.5f, .5f, -0.5f}, {.1f, .8f, .1f}},
    //       {{-.5f, .5f, -0.5f}, {.1f, .8f, .1f}},
    //       {{-.5f, -.5f, -0.5f}, {.1f, .8f, .1f}},
    //       {{.5f, -.5f, -0.5f}, {.1f, .8f, .1f}},
    //       {{.5f, .5f, -0.5f}, {.1f, .8f, .1f}},

    //   };
    
    pub fn bind(&mut self, command_buffer: vk::CommandBuffer) {
        let pipeline_bind_point = vk::PipelineBindPoint::GRAPHICS;
        let push_constant = PushConstant {
            offset: glm::Vec3::new(0.0, -0.25,0.0),
        };
        unsafe {
            self.core.logical_device.cmd_bind_pipeline(
                command_buffer,
                pipeline_bind_point,
                self.pipeline.graphic_pipeline,
            );
            //bind vertex buffer to cmd

            self.core.logical_device.cmd_bind_vertex_buffers(
                command_buffer,
                0,
                &[self.vertex_buffer],
                &[0],
            );
            self.core.logical_device.cmd_push_constants(
                command_buffer,
                self.pipeline_layout,
                vk::ShaderStageFlags::VERTEX,
                0,
                push_constant.to_u8().as_slice(),
            )
        }
    }
    pub fn draw(&mut self, command_buffer: vk::CommandBuffer) {
        unsafe {
            self.core
                .logical_device
                .cmd_draw(command_buffer, self.vertex_count as u32, 1, 0, 0);
        }
    }
}

impl Drop for MainRenderSystem {
    fn drop(&mut self) {
        unsafe {
            self.core
                .logical_device
                .destroy_buffer(self.vertex_buffer, None);
            self.core
                .logical_device
                .free_memory(self.vertex_buffer_memory, None);
            self.core
                .logical_device
                .destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}
