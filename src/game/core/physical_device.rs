use super::queue::QueueFamilyIndices;
use std::ffi::CStr;
use ash::Instance;
use ash::vk;
use super::surface::Surface;
use super::swap_chain_support::SwapChainSupportDetails;
use super::DEVICE_EXTENSIONS;

pub fn pick_physical_device(instance:&Instance,surface:&Surface,indices:&mut QueueFamilyIndices)->vk::PhysicalDevice{
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
        let p = unsafe{
            instance.get_physical_device_memory_properties(physical_device)
        };
        println!("{:?}",unsafe{CStr::from_ptr(properties.device_name.as_ptr())});
        if is_device_is_suitable(instance,&physical_device,surface,indices){
            return physical_device;
        }
    }
    panic!("Failed to find a suitable GPU");
}
fn is_device_is_suitable(instance:&Instance,physical_device:&vk::PhysicalDevice,surface:&Surface,_indices:&mut QueueFamilyIndices)->bool{
    let indices = QueueFamilyIndices::find_queue_families(instance,physical_device,surface);
    let extensions_supported = check_device_extension_support(instance,physical_device);
    let swap_chain_adequate = if extensions_supported {
        let swap_chain_support = SwapChainSupportDetails::query_swap_chain_support(physical_device,surface);
        !(swap_chain_support.formats.is_empty() && swap_chain_support.present_modes.is_empty())
    }else{
        false
    };

    let supported_features = unsafe{
        instance.get_physical_device_features(*physical_device)
    };
    if indices.is_complete() 
    && supported_features.sampler_anisotropy == vk::TRUE 
    && swap_chain_adequate 
    && extensions_supported
    {
        *_indices = indices;
        return true;
    }
    false
}
fn check_device_extension_support(instance:&Instance,physical_device:&vk::PhysicalDevice)->bool{
    let available_extensions = unsafe{
        instance.enumerate_device_extension_properties(*physical_device)
            .expect("Failed to enumerate device extensions")
    };
    DEVICE_EXTENSIONS.iter()
        .all(|extension| {
            available_extensions.iter().any(|&i| {
                *extension ==  unsafe{ CStr::from_ptr(i.extension_name.as_ptr())}
                .to_str()
                .unwrap()
            })
        })
}