use ash::vk;
use ash::extensions::khr;
use crate::game::core::Core;

pub struct SwapChain<'a>{
    core:&'a Core,
    window_extent:vk::Extent2D,
    swap_chain_loader: khr::Swapchain,
    swap_chain:vk::SwapchainKHR,
    images:Vec<vk::Image>,
    image_format:vk::Format,
    extent:vk::Extent2D,
    image_views:Vec<vk::ImageView>,
}
impl<'a> SwapChain<'a>{
    pub fn new(core:&'a Core,window_extent:vk::Extent2D)->Self{
        let swap_chain_loader = khr::Swapchain::new(&core.instance,&core.logical_device);
           SwapChain{
            core,
            swap_chain_loader,
            window_extent,
            swap_chain:vk::SwapchainKHR::null(),
            images:Vec::new(),
            image_format:vk::Format::default(),
            extent:vk::Extent2D::default(),
            image_views:Vec::new(),
           }
    }
    pub fn create_swap_chain(&mut self){
        let swap_chain_support = self.core.swap_chain_support;
    
        let surface_format = choose_swap_surface_format(&swap_chain_support.formats);
        let present_mode = choose_swap_present_mode(&swap_chain_support.present_modes);
        let extent = choose_swap_extent(&swap_chain_support.capabilities,&self.window_extent);
        
        let mut image_count = swap_chain_support.capabilities.min_image_count +1;
        if swap_chain_support.capabilities.max_image_count>0 && image_count>swap_chain_support.capabilities.max_image_count {
            image_count = swap_chain_support.capabilities.max_image_count;
        }
        let image_count = image_count;
    
        let indices = self.core.queue_families.queue_family_indices.to_vec();
        let mut create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(self.core.surface.surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .pre_transform(swap_chain_support.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(self.swap_chain)
            .build();
        if self.core.queue_families.queue_family_indices.graphics_family != self.core.queue_families.queue_family_indices.present_family {
            create_info.image_sharing_mode =vk::SharingMode::CONCURRENT;
            create_info.p_queue_family_indices =indices.as_slice().as_ptr();
        }else{
            create_info.image_sharing_mode = vk::SharingMode::EXCLUSIVE;
        }
        let create_info = create_info;
        self.swap_chain = unsafe{
            self.swap_chain_loader.create_swapchain(&create_info,None).expect("Failed to create swap chain")
        };
        self.images = unsafe{
            self.swap_chain_loader.get_swapchain_images(self.swap_chain).expect("Failed to get swap chain images")
        };
        self.image_format = surface_format.format;
    }
    fn create_image_views(&mut self){
        for image in self.images.iter(){
            let view_info = vk::ImageViewCreateInfo::builder()
                .image(*image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(self.image_format)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                })
                .build();
            let image_view = unsafe{
                self.core.logical_device.create_image_view(&view_info, None).expect("Failed to create image view")
            };
            self.image_views.push(image_view);
        }
    }
    pub fn create_render_pass(&mut self) -> vk::RenderPass {
        let depth_format= self.core.find_supported_format(
            vec![vk::Format::D32_SFLOAT, vk::Format::D32_SFLOAT_S8_UINT, vk::Format::D24_UNORM_S8_UINT],
            vk::ImageTiling::OPTIMAL,
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
        );
        let color_attachment = vk::AttachmentDescription {
            format: self.image_format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            ..Default::default()
        };
        let color_attachment_ref = vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };
        let depth_attachment = vk::AttachmentDescription::builder()
            .format(depth_format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::DONT_CARE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .build();
        let depth_attachment_ref = vk::AttachmentReference::builder()
            .attachment(1)
            .layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .build();
        let subpass = vk::SubpassDescription {
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            p_color_attachments: &color_attachment_ref,
            color_attachment_count:1,
            p_depth_stencil_attachment: &depth_attachment_ref,
            ..Default::default()
        };
        let subpass_dependency = vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            src_access_mask: vk::AccessFlags::empty(),
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
            ..Default::default()
        };
        let create_info = vk::RenderPassCreateInfo {
            attachment_count: 2,
            p_attachments: [color_attachment, depth_attachment].as_ptr(),
            subpass_count: 1,
            p_subpasses: &subpass,
            dependency_count: 1,
            p_dependencies: &subpass_dependency,
            ..Default::default()
        };
        unsafe {
            self.core.logical_device
                .create_render_pass(&create_info, None)
                .expect("Failed to create render pass")
        }
    }
}

fn choose_swap_surface_format(available_formats:&Vec<vk::SurfaceFormatKHR>)->vk::SurfaceFormatKHR{
    for available_format in available_formats.iter(){
        if available_format.format == vk::Format::B8G8R8A8_SRGB && available_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR{
            return available_format.clone();
        }
    }   
    available_formats[0].clone()
}
fn choose_swap_present_mode(available_present_modes:&Vec<vk::PresentModeKHR>)->vk::PresentModeKHR{
    for available_present_mode in available_present_modes.iter(){
        if *available_present_mode == vk::PresentModeKHR::MAILBOX{
            return available_present_mode.clone();
        }
    }
    vk::PresentModeKHR::FIFO
}
fn choose_swap_extent(capabilities:&vk::SurfaceCapabilitiesKHR,window_extent:&vk::Extent2D)->vk::Extent2D{
    if capabilities.current_extent.width != std::u32::MAX {
        return capabilities.current_extent;
    }else {
        return vk::Extent2D {
            width: window_extent.width,
            height: window_extent.height,
        };
    }
}