use super::pipeline::{Pipeline, PipelineConfig};
use crate::game::core::Core;
use crate::offset_of;
use ash::vk;
use nalgebra;
use std::mem::{self, align_of};
use std::rc::Rc;

pub struct PushConstant {
    pub transform: nalgebra::Matrix4<f32>,
}
pub struct Vertex {
    pub position: nalgebra::Vector3<f32>,
    pub color: nalgebra::Vector3<f32>,
}
impl Vertex {
    pub fn new(position: nalgebra::Vector3<f32>, color: nalgebra::Vector3<f32>) -> Self {
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
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    vertex_count: u32,
}
impl MainRenderSystem {
    pub fn new(core: Rc<Core>) -> Self {
        MainRenderSystem {
            pipeline: Pipeline::new(core.clone()),
            pipeline_layout: vk::PipelineLayout::default(),
            vertex_buffer: vk::Buffer::null(),
            vertex_buffer_memory: vk::DeviceMemory::null(),
            vertex_count: 0,
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
    fn create_vertex_buffer(&mut self) {
        let vertices = vec![
            Vertex::new(
                nalgebra::Vector3::new(-0.5, -0.5, 0.0),
                nalgebra::Vector3::new(1.0, 0.0, 0.0),
            ),
            Vertex::new(
                nalgebra::Vector3::new(0.5, -0.5, 0.0),
                nalgebra::Vector3::new(0.0, 1.0, 0.0),
            ),
            Vertex::new(
                nalgebra::Vector3::new(0.0, 0.5, 0.0),
                nalgebra::Vector3::new(0.0, 0.0, 1.0),
            ),
        ];
        let vertex_buffer_info = vk::BufferCreateInfo::builder()
            .size(vertices.len() as u64 * mem::size_of::<Vertex>() as u64)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .build();
        let vertex_buffer = unsafe {
            self.core
                .logical_device
                .create_buffer(&vertex_buffer_info, None)
                .expect("Failed to create vertex buffer")
        };
        let vertex_buffer_memory_requirements = unsafe {
            self.core
                .logical_device
                .get_buffer_memory_requirements(vertex_buffer)
        };
        let vertex_buffer_memory_allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(vertex_buffer_memory_requirements.size)
            .memory_type_index(self.core.find_memory_type(
                vertex_buffer_memory_requirements.memory_type_bits,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            ).unwrap())
            .build();
        let vertex_buffer_memory = unsafe {
            self.core
                .logical_device
                .allocate_memory(&vertex_buffer_memory_allocate_info, None)
                .expect("Failed to allocate vertex buffer memory")
        };
        unsafe {
            self.core
                .logical_device
                .bind_buffer_memory(vertex_buffer, vertex_buffer_memory, 0)
                .expect("Failed to bind vertex buffer memory")
        };
        let mapped_memory = unsafe {
            self.core
                .logical_device
                .map_memory(
                    vertex_buffer_memory,
                    0,
                    vertex_buffer_memory_requirements.size,
                    vk::MemoryMapFlags::empty(),
                )
                .expect("Failed to map vertex buffer memory")
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices.as_ptr() as *const u8,
                mapped_memory as *mut u8,
                vertices.len() * mem::size_of::<Vertex>(),
            );
            self.core
                .logical_device
                .unmap_memory(vertex_buffer_memory);
        }
    }
    pub fn bind(&mut self, command_buffer: vk::CommandBuffer) {
        let pipeline_bind_point = vk::PipelineBindPoint::GRAPHICS;
        unsafe {
            self.core.logical_device.cmd_bind_pipeline(
                command_buffer,
                pipeline_bind_point,
                self.pipeline.graphic_pipeline,
            );
            //bind vertex buffer to cmd
            println!("vertex_buffer: {:?}", self.vertex_buffer);

            self.core
                .logical_device
                .cmd_bind_vertex_buffers(
                    command_buffer,
                    0,
                    &[self.vertex_buffer],
                    &[0],
                );
        }
    }
}
