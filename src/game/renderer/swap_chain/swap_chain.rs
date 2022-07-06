use ash::vk;
use ash::extensions::khr;
use std::ptr;
use crate::core::device::Device;
use crate::core::device::queue::QueueFamilyIndices;

pub struct SwapChain{
    swap_chain_loader: khr::Swapchain,
    swap_chain:vk::SwapchainKHR,
    old_swap_chain:vk::SwapchainKHR
}
impl SwapChain{
    pub fn new(device:&Device)->Self{
        let swap_chain_loader = khr::Swapchain::new(&device.instance,&device.logical_device);
           SwapChain{
            swap_chain_loader,
                swap_chain:vk::SwapchainKHR::null(),
                old_swap_chain:vk::SwapchainKHR::null(),
           }
    }
    pub fn create_swap_chain(&mut self,device:&Device,window:&winit::window::Window){
        let swap_chain_support = &device.swap_chain_support;
    
        let surface_format = choose_swap_surface_format(&swap_chain_support.formats);
        let present_mode = choose_swap_present_mode(&swap_chain_support.present_modes);
        let extent = choose_swap_extent(&swap_chain_support.capabilities,window);
        
        let mut image_count = swap_chain_support.capabilities.min_image_count +1;
        if swap_chain_support.capabilities.max_image_count>0 && image_count>swap_chain_support.capabilities.max_image_count {
            image_count = swap_chain_support.capabilities.max_image_count;
        }
        let image_count = image_count;
    
        let indices = device.queue_families.queue_family_indices.to_vec();
        let mut create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(device.surface.surface)
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
        if device.queue_families.queue_family_indices.graphics_family != device.queue_families.queue_family_indices.present_family {
            create_info.image_sharing_mode =vk::SharingMode::CONCURRENT;
            create_info.p_queue_family_indices =indices.as_slice().as_ptr();
        }else{
            create_info.image_sharing_mode = vk::SharingMode::EXCLUSIVE;
        }
        let create_info = create_info;
        self.swap_chain = unsafe{
            self.swap_chain_loader.create_swapchain(&create_info,None).expect("Failed to create swap chain")
        };
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
fn choose_swap_extent(capabilities:&vk::SurfaceCapabilitiesKHR,window:&winit::window::Window)->vk::Extent2D{
    if capabilities.current_extent.width != std::u32::MAX {
        return capabilities.current_extent;
    }else {
    use num::clamp;

    let window_size = window
        .inner_size();
    println!(
        "\t\tInner Window Size: ({}, {})",
        window_size.width, window_size.height
    );

    vk::Extent2D {
        width: clamp(
            window_size.width as u32,
            capabilities.min_image_extent.width,
            capabilities.max_image_extent.width,
        ),
        height: clamp(
            window_size.height as u32,
            capabilities.min_image_extent.height,
            capabilities.max_image_extent.height,
        ),
    }
    }
}