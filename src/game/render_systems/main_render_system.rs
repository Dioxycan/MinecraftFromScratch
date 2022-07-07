use std::rc::Rc;
use std::mem;
use ash::vk;
use nalgebra;
use crate::game::core::Core;
use super::pipeline::{PipelineConfig,Pipeline};
pub struct PushConstant{
    pub transform:nalgebra::Matrix4<f32>,
}
pub struct MainRenderSystem{
    core:Rc<Core>,
    pipeline_layout: vk::PipelineLayout,
    pipeline: Pipeline,
}
impl MainRenderSystem{
    pub fn new(core:Rc<Core>)->Self{
        MainRenderSystem{
            pipeline: Pipeline::new(core.clone()),
            pipeline_layout: vk::PipelineLayout::default(),
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
}
