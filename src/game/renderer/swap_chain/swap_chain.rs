use std::rc::Rc;

use crate::game::core::Core;
use ash::extensions::khr;
use ash::vk;
const MAX_FRAMES_IN_FLIGHT : usize = 2;
pub struct SwapChain {
    core: Rc<Core>,
    window_extent: vk::Extent2D,
    swap_chain_loader: khr::Swapchain,
    swap_chain: vk::SwapchainKHR,
    images: Vec<vk::Image>,
    image_format: vk::Format,
    extent: vk::Extent2D,
    image_views: Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    depth_images: Vec<vk::Image>,
    depth_image_views: Vec<vk::ImageView>,
    depth_image_memories: Vec<vk::DeviceMemory>,
    swap_chain_extent: vk::Extent2D,
    frame_buffers: Vec<vk::Framebuffer>,
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    inflight_fences: Vec<vk::Fence>,
    image_in_flight: Vec<vk::Fence>,
    current_frame: usize,
}
impl SwapChain {
    pub fn new(core: Rc<Core>, window_extent: vk::Extent2D) -> Self {
        let swap_chain_loader = khr::Swapchain::new(&core.instance, &core.logical_device);
        SwapChain {
            core,
            swap_chain_loader,
            window_extent,
            swap_chain: vk::SwapchainKHR::null(),
            images: Vec::new(),
            image_format: vk::Format::default(),
            extent: vk::Extent2D::default(),
            image_views: Vec::new(),
            render_pass: vk::RenderPass::null(),
            depth_images: Vec::new(),
            depth_image_views: Vec::new(),
            depth_image_memories: Vec::new(),
            swap_chain_extent: vk::Extent2D::default(),
            frame_buffers: Vec::new(),
            image_available_semaphores: Vec::new(),
            render_finished_semaphores: Vec::new(),
            inflight_fences: Vec::new(),
            image_in_flight: Vec::new(),
            current_frame: 0,
        }.init()
    }
    fn init(mut self)->Self{
        self.create_swap_chain();
        self.create_image_views();
        self.create_render_pass();
        self.create_depth_resources();
        self.create_frame_buffer();
        self.create_sync_objects();
        self
    }
    fn create_swap_chain(&mut self) {
        let swap_chain_support = &self.core.swap_chain_support;

        let surface_format = choose_swap_surface_format(&swap_chain_support.formats);
        let present_mode = choose_swap_present_mode(&swap_chain_support.present_modes);
        let extent = choose_swap_extent(&swap_chain_support.capabilities, &self.window_extent);

        let mut image_count = swap_chain_support.capabilities.min_image_count + 1;
        if swap_chain_support.capabilities.max_image_count > 0
            && image_count > swap_chain_support.capabilities.max_image_count
        {
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
        if self
            .core
            .queue_families
            .queue_family_indices
            .graphics_family
            != self.core.queue_families.queue_family_indices.present_family
        {
            create_info.image_sharing_mode = vk::SharingMode::CONCURRENT;
            create_info.p_queue_family_indices = indices.as_slice().as_ptr();
        } else {
            create_info.image_sharing_mode = vk::SharingMode::EXCLUSIVE;
        }
        let create_info = create_info;
        self.swap_chain = unsafe {
            self.swap_chain_loader
                .create_swapchain(&create_info, None)
                .expect("Failed to create swap chain")
        };
        self.images = unsafe {
            self.swap_chain_loader
                .get_swapchain_images(self.swap_chain)
                .expect("Failed to get swap chain images")
        };
        self.image_format = surface_format.format;
        self.swap_chain_extent = extent;
    }
    fn create_image_views(&mut self) {
        for image in self.images.iter() {
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
            let image_view = unsafe {
                self.core
                    .logical_device
                    .create_image_view(&view_info, None)
                    .expect("Failed to create image view")
            };
            self.image_views.push(image_view);
        }
    }
    fn create_render_pass(&mut self) {
        let depth_format = self.core.find_supported_format(
            vec![
                vk::Format::D32_SFLOAT,
                vk::Format::D32_SFLOAT_S8_UINT,
                vk::Format::D24_UNORM_S8_UINT,
            ],
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
            color_attachment_count: 1,
            p_depth_stencil_attachment: &depth_attachment_ref,
            ..Default::default()
        };
        let subpass_dependency = vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            src_access_mask: vk::AccessFlags::empty(),
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE
                | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
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
        self.render_pass = unsafe {
            self.core
                .logical_device
                .create_render_pass(&create_info, None)
                .expect("Failed to create render pass")
        };
    }
    fn create_depth_resources(&mut self) {
        let depth_format = self.core.find_supported_format(
            vec![
                vk::Format::D32_SFLOAT,
                vk::Format::D32_SFLOAT_S8_UINT,
                vk::Format::D24_UNORM_S8_UINT,
            ],
            vk::ImageTiling::OPTIMAL,
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
        );
        let extent = &self.swap_chain_extent;
        for i in 0..self.images.len() {
            let image_create_info = vk::ImageCreateInfo::builder()
                .image_type(vk::ImageType::TYPE_2D)
                .extent(vk::Extent3D {
                    width: extent.width,
                    height: extent.height,
                    depth: 1,
                })
                .mip_levels(1)
                .array_layers(1)
                .format(depth_format)
                .tiling(vk::ImageTiling::OPTIMAL)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
                .sharing_mode(vk::SharingMode::EXCLUSIVE)
                .samples(vk::SampleCountFlags::TYPE_1)
                .build();
            let image = unsafe {
                self.core
                    .logical_device
                    .create_image(&image_create_info, None)
                    .expect("Failed to create image")
            };
            let mut mem_req: vk::MemoryRequirements = unsafe {
                self.core
                    .logical_device
                    .get_image_memory_requirements(image)
            };
            let mem_index = self.core.find_memory_type(
                mem_req.memory_type_bits,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            );
            let alloc_info = vk::MemoryAllocateInfo::builder()
                .allocation_size(mem_req.size)
                .memory_type_index(mem_index.unwrap())
                .build();
            let image_memory = unsafe {
                self.core
                    .logical_device
                    .allocate_memory(&alloc_info, None)
                    .expect("Failed to allocate image memory")
            };
            unsafe {
                self.core
                    .logical_device
                    .bind_image_memory(image, image_memory, 0)
                    .expect("Failed to bind")
            };
            self.depth_images.push(image);
            self.depth_image_memories.push(image_memory);
            let view_info = vk::ImageViewCreateInfo::builder()
                .image(image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(depth_format)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::DEPTH,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                })
                .build();
            let depth_image_view = unsafe {
                self.core
                    .logical_device
                    .create_image_view(&view_info, None)
                    .expect("Failed to create image view")
            };
            self.depth_image_views.push(depth_image_view);
        }
    }
    fn create_frame_buffer(&mut self){
        for i in 0..self.images.len() {
            let attachments = [self.image_views[i],self.depth_image_views[i]];
            let create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(self.render_pass)
                .attachments(&attachments)
                .width(self.swap_chain_extent.width)
                .height(self.swap_chain_extent.height)
                .layers(1)
                .build();
            let frame_buffer = unsafe {
                self.core
                    .logical_device
                    .create_framebuffer(&create_info, None)
                    .expect("Failed to create frame buffer")
            };
            self.frame_buffers.push(frame_buffer);
        }
    }
    fn create_sync_objects(&mut self) {
        self.image_available_semaphores.resize(MAX_FRAMES_IN_FLIGHT, vk::Semaphore::null());
        self.render_finished_semaphores.resize(MAX_FRAMES_IN_FLIGHT, vk::Semaphore::null());
        self.inflight_fences.resize(MAX_FRAMES_IN_FLIGHT, vk::Fence::null());
        self.image_in_flight.resize(self.images.len(),vk::Fence::null());

        let semaphore_info =vk::SemaphoreCreateInfo::default();
        let fence_info = vk::FenceCreateInfo::builder()
            .flags(vk::FenceCreateFlags::SIGNALED)
            .build();
        for i in 0..MAX_FRAMES_IN_FLIGHT {
            let semaphore = unsafe {
                self.core
                    .logical_device
                    .create_semaphore(&semaphore_info, None)
                    .expect("Failed to create semaphore")
            };
            self.image_available_semaphores.push(semaphore);
            let semaphore = unsafe {
                self.core
                    .logical_device
                    .create_semaphore(&semaphore_info, None)
                    .expect("Failed to create semaphore")
            };
            self.render_finished_semaphores.push(semaphore);
            let fence = unsafe {
                self.core
                    .logical_device
                    .create_fence(&fence_info, None)
                    .expect("Failed to create fence")
            };
            self.inflight_fences.push(fence);
        }
    }
}

fn choose_swap_surface_format(
    available_formats: &Vec<vk::SurfaceFormatKHR>,
) -> vk::SurfaceFormatKHR {
    for available_format in available_formats.iter() {
        if available_format.format == vk::Format::B8G8R8A8_SRGB
            && available_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        {
            return available_format.clone();
        }
    }
    available_formats[0].clone()
}
fn choose_swap_present_mode(
    available_present_modes: &Vec<vk::PresentModeKHR>,
) -> vk::PresentModeKHR {
    for available_present_mode in available_present_modes.iter() {
        if *available_present_mode == vk::PresentModeKHR::MAILBOX {
            return available_present_mode.clone();
        }
    }
    vk::PresentModeKHR::FIFO
}
fn choose_swap_extent(
    capabilities: &vk::SurfaceCapabilitiesKHR,
    window_extent: &vk::Extent2D,
) -> vk::Extent2D {
    if capabilities.current_extent.width != std::u32::MAX {
        return capabilities.current_extent;
    } else {
        return vk::Extent2D {
            width: window_extent.width,
            height: window_extent.height,
        };
    }
}
