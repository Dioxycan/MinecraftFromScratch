use std::rc::Rc;

use super::MAX_FRAMES_IN_FLIGHT;
use crate::game::core::Core;
use crate::game::surface::Surface;
use ash::extensions::khr;
use ash::prelude::*;
use ash::vk;
use std::ops::Drop;
pub struct SwapChain {
    core: Rc<Core>,
    pub swap_chain_loader: khr::Swapchain,
    pub swap_chain: vk::SwapchainKHR,
    images: Vec<vk::Image>,
    image_format: vk::Format,
    extent: vk::Extent2D,
    image_views: Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    depth_images: Vec<vk::Image>,
    depth_image_views: Vec<vk::ImageView>,
    depth_image_memories: Vec<vk::DeviceMemory>,
    pub swap_chain_extent: vk::Extent2D,
    pub frame_buffers: Vec<vk::Framebuffer>,
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub inflight_fences: Vec<vk::Fence>,
    pub image_in_flight: Vec<vk::Fence>,
    pub current_frame: usize,
}
impl SwapChain {
    pub fn new(core: Rc<Core>) -> Self {
        let swap_chain_loader = khr::Swapchain::new(&core.instance, &core.logical_device);
        SwapChain {
            core,
            swap_chain_loader,
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
        }
    }
    pub fn init(&mut self, window_extent:&vk::Extent2D, old_swap_chain: Option<vk::SwapchainKHR>) {
        self.create_swap_chain(window_extent,old_swap_chain);
        self.create_image_views();
        self.create_render_pass();
        self.create_depth_resources();
        self.create_frame_buffer();
        self.create_sync_objects();
    }
    fn create_swap_chain(
        &mut self,
        window_extent:&vk::Extent2D,
        old_swap_chain: Option<vk::SwapchainKHR>,
    ) {
        let old_swap_chain = old_swap_chain.unwrap_or(vk::SwapchainKHR::null());
        let swap_chain_support = &self.core.swap_chain_support;
        let surface_format = choose_swap_surface_format(&swap_chain_support.formats);
        let present_mode = choose_swap_present_mode(&swap_chain_support.present_modes);
        let extent = choose_swap_extent(&swap_chain_support.capabilities, window_extent);
        println!("surface format {:?}",extent);
        let mut image_count = swap_chain_support.capabilities.min_image_count + 1;
        if swap_chain_support.capabilities.max_image_count > 0
            && image_count > swap_chain_support.capabilities.max_image_count
        {
            image_count = swap_chain_support.capabilities.max_image_count;
        }
        let image_count = image_count;
        let indices = self.core.queue_families.queue_family_indices.to_vec();
        let mut create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(self.surface.surface)
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
            .old_swapchain(old_swap_chain)
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
    fn create_frame_buffer(&mut self) {
        for i in 0..self.images.len() {
            let attachments = [self.image_views[i], self.depth_image_views[i]];
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
        self.image_in_flight = vec![vk::Fence::null(); self.images.len()];
        let semaphore_info = vk::SemaphoreCreateInfo::default();
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
    pub fn get_render_pass(&self) -> &vk::RenderPass {
        &self.render_pass
    }
    pub fn acquire_next_image(&self) -> VkResult<(u32, bool)> {
        unsafe {
            self.core
                .logical_device
                .wait_for_fences(&self.inflight_fences, true, std::u64::MAX)
                .expect("Failed to wait for fence");
        }
        unsafe {
            self.swap_chain_loader.acquire_next_image(
                self.swap_chain,
                std::u64::MAX,
                self.image_available_semaphores[self.current_frame],
                vk::Fence::null(),
            )
        }
    }
    pub fn submit_command_buffer(
        &mut self,
        command_buffer: &vk::CommandBuffer,
        image_index: &u32,
    ) -> VkResult<(bool)> {
        if (self.image_in_flight[*image_index as usize] != vk::Fence::null()) {
            unsafe {
                self.core
                    .logical_device
                    .wait_for_fences(
                        &[self.image_in_flight[*image_index as usize]],
                        true,
                        std::u64::MAX,
                    )
                    .expect("Failed to wait for fence");
            }
        }
        self.image_in_flight[*image_index as usize] = self.inflight_fences[self.current_frame];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&[self.image_available_semaphores[self.current_frame]])
            .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
            .command_buffers(&[*command_buffer])
            .signal_semaphores(&[self.render_finished_semaphores[self.current_frame]])
            .build();
        unsafe {
            self.core
                .logical_device
                .reset_fences(&[self.inflight_fences[self.current_frame]])
                .expect("Failed to reset fence");
            self.core
                .logical_device
                .queue_submit(
                    self.core.queue_families.graphics_queue,
                    &[submit_info],
                    self.inflight_fences[self.current_frame],
                )
                .expect("Failed to submit command buffer");
        }
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&[self.render_finished_semaphores[self.current_frame]])
            .swapchains(&[self.swap_chain])
            .image_indices(&[*image_index])
            .build();
        let result =unsafe {
            self.swap_chain_loader
                .queue_present(self.core.queue_families.present_queue, &present_info)
        };
        self.current_frame =( self.current_frame + 1 ) % MAX_FRAMES_IN_FLIGHT;
        result
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
    // if capabilities.current_extent.width != std::u32::MAX {
    //     return capabilities.current_extent;
    // } else {
        return vk::Extent2D {
            width: window_extent.width,
            height: window_extent.height,
        };
    // } 
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        println!("droping swap chain");
        unsafe {
            for view in self.image_views.iter() {
                self.core.logical_device.destroy_image_view(*view, None);
            }

            self.swap_chain_loader
                .destroy_swapchain(self.swap_chain, None);
           for (i,depth_image) in self.depth_images.iter().enumerate() {
                self.core.logical_device.destroy_image_view(self.depth_image_views[i], None);
                self.core.logical_device.destroy_image(*depth_image, None);
                self.core.logical_device.free_memory(self.depth_image_memories[i], None);
            }
            for framebuffer in self.frame_buffers.iter() {
                self.core
                    .logical_device
                    .destroy_framebuffer(*framebuffer, None);
            }
            self.core
                .logical_device
                .destroy_render_pass(self.render_pass, None);
            for i in 0..MAX_FRAMES_IN_FLIGHT{
                self.core.logical_device.destroy_semaphore(self.image_available_semaphores[i], None);
                self.core.logical_device.destroy_semaphore(self.render_finished_semaphores[i], None);
                self.core.logical_device.destroy_fence(self.inflight_fences[i], None);
            }
        }
    }
}
