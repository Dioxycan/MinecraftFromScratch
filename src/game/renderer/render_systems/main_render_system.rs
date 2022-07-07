use ash::vk;
pub struct MainRenderSystem{
    pipelineLayout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
}
impl MainRenderSystem{
    fn new()->Self{
        MainRenderSystem{
            pipelineLayout: vk::PipelineLayout::default(),
            pipeline: vk::Pipeline::default(),
        }.init()
    }
    fn init(mut self)->Self{
        self.create_pipeline_layout();
        self.create_pipeline();
        self
    }

    fn create_pipeline_layout(&mut self){
        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder()
            .push_constant_ranges(&[vk::PushConstantRange::builder()
                .stage_flags(vk::ShaderStageFlags::VERTEX)
                .offset(0)
                .size(std::mem::size_of::<u32>() as u32)
                .build()
            ])
            .set_layouts(&[vk::DescriptorSetLayout::default()]);
        let pipeline_layout = unsafe{
            self.core.logical_device.create_pipeline_layout(&pipeline_layout_info, None)
                .expect("Failed to create pipeline layout")
        };
        self.pipelineLayout = pipeline_layout;
    }
}
