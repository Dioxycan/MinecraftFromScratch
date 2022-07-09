use super::pipeline_config::PipelineConfig;
use crate::game::core::Core;
use ash::vk;
use std::ffi::CStr;
use std::io::prelude::*;
use std::rc::Rc;
pub struct Pipeline {
    core: Rc<Core>,
    pub graphic_pipeline: vk::Pipeline,
    vert_shader_module: vk::ShaderModule,
    frag_shader_module: vk::ShaderModule,
}
impl Pipeline {
    pub fn new(core: Rc<Core>) -> Self {
        Pipeline {
            core,
            graphic_pipeline: vk::Pipeline::null(),
            vert_shader_module: vk::ShaderModule::null(),
            frag_shader_module: vk::ShaderModule::null(),
        }
    }
    pub fn create_graphic_pipeline(
        &mut self,
        vert_file_path: &'static str,
        frag_file_path: &'static str,
        config_info: PipelineConfig,
        binding_descriptions: &[vk::VertexInputBindingDescription],
        attribute_descriptions: &[vk::VertexInputAttributeDescription],
    ) {
        assert!(
            config_info.pipeline_layout != vk::PipelineLayout::null(),
            "Cannot create pipeline without pipeline layout"
        );
        assert!(
            config_info.render_pass != vk::RenderPass::null(),
            "Cannot create pipeline without render pass"
        );
        let vert_code = read_shader_code(vert_file_path);
        let frag_code = read_shader_code(frag_file_path);
        self.vert_shader_module = self.create_shader_mode(vert_code.as_slice());
        self.frag_shader_module = self.create_shader_mode(frag_code.as_slice());
        let mut shader_stage: Vec<vk::PipelineShaderStageCreateInfo> = vec![];
        shader_stage.push(
            vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(self.vert_shader_module)
                .name(unsafe { CStr::from_bytes_with_nul_unchecked(b"main\0") })
                .build(),
        );
        shader_stage.push(
            vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(self.frag_shader_module)
                .name(unsafe { CStr::from_bytes_with_nul_unchecked(b"main\0") })
                .build(),
        );
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
        self.graphic_pipeline = unsafe {
            self.core
                .logical_device
                .create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
                .expect("Failed to create graphics pipeline")[0]
        };
    }
    fn create_shader_mode(&mut self, shader_code: &[u8]) -> vk::ShaderModule {
        let create_info = vk::ShaderModuleCreateInfo {
            p_code: shader_code.as_ptr() as *const _,
            code_size: shader_code.len() as _,
            ..Default::default()
        };
        unsafe {
            self.core
                .logical_device
                .create_shader_module(&create_info, None)
                .expect("Failed to create shader module")
        }
    }
}
fn read_shader_code(file_path: &'static str) -> Vec<u8> {
    let mut file = std::fs::File::open(file_path).expect("Failed to open shader file");
    let mut code = Vec::new();
    file.read_to_end(&mut code)
        .expect("Failed to read shader file");
    code
}
