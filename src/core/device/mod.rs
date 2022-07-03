mod surface;
mod queue;
use ash::Device as LogicalDevice;
use ash::Instance;
use ash::vk;
use ash::Entry;
use winit::window::Window;

use self::surface::Surface;
use self::queue::QueueFamilyIndices;
use std::ffi::CStr;

const device_extensions: [&str; 1] = [
    "VK_KHR_swapchain",
];
#[derive(Debug)]
pub struct SwapChainSupportDetails {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

pub fn query_swap_chain_support(
    physical_device:&vk::PhysicalDevice,
    surface: &Surface,
) -> SwapChainSupportDetails {
    unsafe {
        let capabilities = surface
            .surface_loader
            .get_physical_device_surface_capabilities(*physical_device, surface.surface)
            .expect("Failed to query for surface capabilities.");
        let formats = surface
            .surface_loader
            .get_physical_device_surface_formats(*physical_device, surface.surface)
            .expect("Failed to query for surface formats.");
        let present_modes = surface
            .surface_loader
            .get_physical_device_surface_present_modes(*physical_device, surface.surface)
            .expect("Failed to query for surface present mode.");

        SwapChainSupportDetails {
            capabilities,
            formats,
            present_modes,
        }
    }
}

pub struct Device{
   // pub logical_device:LogicalDevice,
    pub physical_device:vk::PhysicalDevice,
}
impl Device{
    pub fn new(entry:&Entry,instance:&Instance,window:&Window)->Self{
        let surface = Surface::new(entry,instance,window);
        let physical_device = pick_physical_device(instance,&surface);
        Device{
            physical_device
        }
    }

    
}
fn pick_physical_device(instance:&Instance,surface:&Surface)->vk::PhysicalDevice{
    let physical_devices = unsafe{
        instance.enumerate_physical_devices()
            .expect("Failed to enumerate physical devices")
    };
    if physical_devices.len() == 0 {
        panic!("No physical devices available");
    }
    for physical_device in physical_devices{
        let properties = unsafe{
            instance.get_physical_device_properties(physical_device)
        };
        println!("{:?}",unsafe{CStr::from_ptr(properties.device_name.as_ptr())});
        if is_device_is_suitable(instance,&physical_device,surface){
            return physical_device;
        }
    }
    panic!("Failed to find a suitable GPU");
}
fn is_device_is_suitable(instance:&Instance,physical_device:&vk::PhysicalDevice,surface:&Surface)->bool{
    let indices = QueueFamilyIndices::find_queue_families(instance,physical_device,surface);
    let extensions_supported = check_device_extension_support(instance,physical_device);
    let swap_chain_adequate = if extensions_supported {
        let swap_chain_support = query_swap_chain_support(physical_device,surface);
        !(swap_chain_support.formats.is_empty() && swap_chain_support.present_modes.is_empty())
    }else{
        false
    };

    let supported_features = unsafe{
        instance.get_physical_device_features(*physical_device)
    };
    return indices.is_complete() 
    && supported_features.sampler_anisotropy == vk::TRUE 
    && swap_chain_adequate 
    && extensions_supported;
}
fn check_device_extension_support(instance:&Instance,physical_device:&vk::PhysicalDevice)->bool{
    let available_extensions = unsafe{
        instance.enumerate_device_extension_properties(*physical_device)
            .expect("Failed to enumerate device extensions")
    };
    device_extensions.iter()
        .all(|extension| {
            available_extensions.iter().any(|&i| {
                *extension ==  unsafe{ CStr::from_ptr(i.extension_name.as_ptr())}
                .to_str()
                .unwrap()
            })
        })
}