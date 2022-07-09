use ash::vk;
use std::default::Default;
pub struct PipelineConfig {
    pub viewport_state: vk::PipelineViewportStateCreateInfo,
    pub input_assembly_state: vk::PipelineInputAssemblyStateCreateInfo,
    pub rasterization_state: vk::PipelineRasterizationStateCreateInfo,
    pub multisample_state: vk::PipelineMultisampleStateCreateInfo,
    pub color_blend_state: vk::PipelineColorBlendStateCreateInfo,
    color_blend_attachment: vk::PipelineColorBlendAttachmentState,
    pub depth_stencil_state: vk::PipelineDepthStencilStateCreateInfo,
    pub dynamic_state: vk::PipelineDynamicStateCreateInfo,
    pub dynamic_state_enables: Vec<vk::DynamicState>,
    pub pipeline_layout: vk::PipelineLayout,
    pub render_pass: vk::RenderPass,
    pub subpass: u32,
}
impl PipelineConfig {
    pub fn init(&mut self) {
        self.viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewport_count(1)
            .scissor_count(1)
            .build();
        self.input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false)
            .build();
        self.rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false)
            .depth_bias_constant_factor(0.0)
            .depth_bias_clamp(0.0)
            .depth_bias_slope_factor(0.0)
            .build();
        self.multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1)
            .sample_shading_enable(false)
            .min_sample_shading(1.0)
            .alpha_to_coverage_enable(false)
            .alpha_to_one_enable(false)
            .build();
        self.color_blend_attachment = vk::PipelineColorBlendAttachmentState{
            blend_enable: vk::FALSE,
            src_color_blend_factor: vk::BlendFactor::ONE,
            dst_color_blend_factor: vk::BlendFactor::ZERO,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::R|vk::ColorComponentFlags::G|vk::ColorComponentFlags::B|vk::ColorComponentFlags::A,
        };
        self.color_blend_state = vk::PipelineColorBlendStateCreateInfo{
            logic_op_enable: vk::FALSE,
            logic_op: vk::LogicOp::COPY,
            attachment_count: 1,
            p_attachments: &self.color_blend_attachment,
            blend_constants: [0.0, 0.0, 0.0, 0.0],
            ..Default::default()
        };
        self.depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(false)
            .depth_write_enable(false)
            .depth_compare_op(vk::CompareOp::LESS)
            .depth_bounds_test_enable(false)
            .stencil_test_enable(false)
            .front(
                vk::StencilOpState::default(), // builder()
                                               // .fail_op(vk::StencilOp::KEEP)
                                               // .pass_op(vk::StencilOp::KEEP)
                                               // .depth_fail_op(vk::StencilOp::KEEP)
                                               // .compare_op(vk::CompareOp::ALWAYS)
                                               // .compare_mask(0)
                                               // .write_mask(0)
                                               // .reference(0)
                                               // .build()
            )
            .back(
                vk::StencilOpState::default(), // builder()
                                               // .fail_op(vk::StencilOp::KEEP)
                                               // .pass_op(vk::StencilOp::KEEP)
                                               // .depth_fail_op(vk::StencilOp::KEEP)
                                               // .compare_op(vk::CompareOp::ALWAYS)
                                               // .compare_mask(0)
                                               // .write_mask(0)
                                               // .reference(0)
                                               // .build()
            )
            .build();
        self.dynamic_state_enables = vec![vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        self.dynamic_state = vk::PipelineDynamicStateCreateInfo::builder()
            .dynamic_states(&[vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR])
            .build();
    }
}
impl Default for PipelineConfig {
    fn default() -> Self {
        PipelineConfig {
            viewport_state: vk::PipelineViewportStateCreateInfo::default(),
            input_assembly_state: vk::PipelineInputAssemblyStateCreateInfo::default(),
            rasterization_state: vk::PipelineRasterizationStateCreateInfo::default(),
            multisample_state: vk::PipelineMultisampleStateCreateInfo::default(),
            color_blend_state: vk::PipelineColorBlendStateCreateInfo::default(),
            color_blend_attachment: vk::PipelineColorBlendAttachmentState::default(),
            depth_stencil_state: vk::PipelineDepthStencilStateCreateInfo::default(),
            dynamic_state: vk::PipelineDynamicStateCreateInfo::default(),
            dynamic_state_enables: Vec::new(),
            pipeline_layout: vk::PipelineLayout::default(),
            render_pass: vk::RenderPass::null(),
            subpass: 0,
        }
    }
}
