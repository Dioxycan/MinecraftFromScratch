
use std::rc::Rc;
use ash::vk;
use crate::game::core::Core;
use std::io::prelude::*;
use std::ffi::CStr;
pub struct Pipeline{
    core:Rc<Core>,
    graphic_pipeline:vk::Pipeline,
    vert_shader_module:vk::ShaderModule,
    frag_shader_module:vk::ShaderModule,
}
impl Pipeline{
    pub fn new(core:Rc<Core>,vert_file_path:&'static str,frag_file_path:&'static str,config_info:PipelineConfig)->Self{
        Pipeline{
            core,
            graphic_pipeline:vk::Pipeline::null(),
            vert_shader_module:vk::ShaderModule::null(),
            frag_shader_module:vk::ShaderModule::null(),
        }.create_graphic_pipeline(vert_file_path,frag_file_path,config_info)
    }
    fn create_graphic_pipeline(mut self,vert_file_path:&'static str,frag_file_path:&'static str,config_info:PipelineConfig)->Self{
        assert!(config_info.pipeline_layout != vk::PipelineLayout::null(),"Cannot create pipeline without pipeline layout");
        assert!(config_info.render_pass != vk::RenderPass::null(),"Cannot create pipeline without render pass");
        let vert_code = read_shader_code(vert_file_path);
        let frag_code = read_shader_code(frag_file_path);
        self.vert_shader_module = self.create_shader_mode(vert_code.as_slice());
        self.frag_shader_module = self.create_shader_mode(frag_code.as_slice());
        let mut shader_stage:Vec<vk::PipelineShaderStageCreateInfo>= vec![];
        shader_stage[0] = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(self.vert_shader_module)
            .name(unsafe{CStr::from_bytes_with_nul_unchecked(b"main\0")})
            .build();
        shader_stage[1] = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(self.frag_shader_module)
            .name(unsafe{CStr::from_bytes_with_nul_unchecked(b"main\0")})
            .build();

        let binding_descriptions = [vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(10)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()];
        let attribute_descriptions = [vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(0)
            .build(),
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(12)
                .build()];
        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&binding_descriptions)
            .vertex_attribute_descriptions(&attribute_descriptions)
            .build();
        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stage)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&config_info.input_assembly_state)
            .viewport_state(&config_info.viewport_state)
            .rasterization_state(&config_info.rasterization_state)
            .multisample_state(&config_info.multisample_state)
            .depth_stencil_state(&config_info.depth_stencil_state)
            .color_blend_state(&config_info.color_blend_state)
            .dynamic_state(&config_info.dynamic_state)
            .layout(config_info.pipeline_layout)
            .render_pass(config_info.render_pass)
            .subpass(config_info.subpass)
            .base_pipeline_index(-1)
            .base_pipeline_handle(vk::Pipeline::null())
            .build();
        self.graphic_pipeline = unsafe{
            self.core.logical_device.create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
            .expect("Failed to create graphics pipeline")[0]
        };
        self
    }
    fn create_shader_mode(&mut self,shader_code:&[u8])->vk::ShaderModule{
        let create_info = vk::ShaderModuleCreateInfo{
            p_code:shader_code.as_ptr() as *const _,
            code_size:shader_code.len() as _,
            ..Default::default()
        };
        unsafe{
            self.core.logical_device.create_shader_module(&create_info, None).expect("Failed to create shader module")
        }
    }

}
fn read_shader_code(file_path:&'static str)->Vec<u8>{
    let mut file = std::fs::File::open(file_path).expect("Failed to open shader file");
    let mut code = Vec::new();
    file.read_to_end(&mut code).expect("Failed to read shader file");
    code
}
pub struct PipelineConfig{
    pub viewport_state:vk::PipelineViewportStateCreateInfo,
    pub input_assembly_state:vk::PipelineInputAssemblyStateCreateInfo,
    pub rasterization_state:vk::PipelineRasterizationStateCreateInfo,
    pub multisample_state:vk::PipelineMultisampleStateCreateInfo,
    pub color_blend_state:vk::PipelineColorBlendStateCreateInfo,
    pub color_blend_attachments:vk::PipelineColorBlendAttachmentState,
    pub depth_stencil_state:vk::PipelineDepthStencilStateCreateInfo,
    pub dynamic_state:vk::PipelineDynamicStateCreateInfo,
    pub dynamic_state_enables:Vec<vk::DynamicState>,
    pub pipeline_layout : vk::PipelineLayout,
    pub render_pass : vk::RenderPass,
    pub subpass : u32,
}