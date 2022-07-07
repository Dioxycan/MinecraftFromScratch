use std::rc::Rc;
use std::mem::{align_of,self};
use ash::vk;
use nalgebra;
use crate::game::core::Core;
use crate::offset_of;
use super::pipeline::{PipelineConfig,Pipeline};

pub struct PushConstant{
    pub transform:nalgebra::Matrix4<f32>,
}
pub struct Vertex{
    pub position:nalgebra::Vector3<f32>,
    pub color:nalgebra::Vector3<f32>,
}
impl Vertex{
    pub fn new(position:nalgebra::Vector3<f32>,color:nalgebra::Vector3<f32>)->Self{
        Vertex{
            position,
            color,
        }
    }
    pub fn get_binding_description()->vk::VertexInputBindingDescription{
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(mem::size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }
    pub fn get_attribute_descriptions()->Vec<vk::VertexInputAttributeDescription>{
        vec![
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Vertex,position) as u32)
                .build(),
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Vertex,color) as u32)
                .build(),
        ]
    }
}
pub struct MainRenderSystem{
    core:Rc<Core>,
    pipeline_layout: vk::PipelineLayout,
    pipeline: Pipeline,
    vertex_buffer:vk::Buffer,
    vertex_buffer_memory:vk::DeviceMemory,
    vertex_count:u32,
}
impl MainRenderSystem{
    pub fn new(core:Rc<Core>)->Self{
        MainRenderSystem{
            pipeline: Pipeline::new(core.clone()),
            pipeline_layout: vk::PipelineLayout::default(),
            vertex_buffer:vk::Buffer::null(),
            vertex_buffer_memory:vk::DeviceMemory::null(),
            vertex_count:0,
            core,
        }
    }
    pub fn init(&mut self,render_pass:&vk::RenderPass){
        self.create_pipeline_layout();
        self.create_pipeline(render_pass);
    }

    fn create_pipeline_layout(&mut self){
        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder()
            .push_constant_ranges(&[vk::PushConstantRange::builder()
                .stage_flags(vk::ShaderStageFlags::VERTEX)
                .offset(0)
                .size(mem::size_of::<PushConstant>() as u32)
                .build()
            ])
            .build();
        
        let pipeline_layout = unsafe{
            self.core.logical_device.create_pipeline_layout(&pipeline_layout_info, None)
                .expect("Failed to create pipeline layout")
        };
        self.pipeline_layout = pipeline_layout;
    }
    fn create_pipeline(&mut self,render_pass:&vk::RenderPass){
        assert!(self.pipeline_layout != vk::PipelineLayout::default(),"Cannot create pipeline before pipeline layout"); 
        let mut pipeline_config = PipelineConfig::default();
        pipeline_config.render_pass = render_pass.clone();
        pipeline_config.pipeline_layout = self.pipeline_layout;
        self.pipeline.create_graphic_pipeline( "shaders/shader.vert.spv", "shaders/shader.frag.spv", pipeline_config);
    }
    fn create_vertex_buffer(&mut self){

    }
}
