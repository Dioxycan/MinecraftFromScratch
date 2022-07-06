use crate::IS_VALIDATION_LAYERS_ENABLED;

use super::queue::QueueFamilyIndices;
use super::surface::Surface;
use ash::vk;
use std::ffi::CString;
use super::debug::LAYER_NAME;
use super::DEVICE_EXTENSIONS;

pub fn create_logical_device(
    instance: &ash::Instance,
    physical_device: &vk::PhysicalDevice,
    surface: &Surface,
) -> ash::Device {
    let mut queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = vec![];
    let unique_indices =
        QueueFamilyIndices::find_queue_families(instance, physical_device, surface).to_unique();
    for uinque_indice in unique_indices {
        let queue_create_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(uinque_indice)
            .queue_priorities(&[1.0])
            .build();
        queue_create_infos.push(queue_create_info);
    }
    let queue_create_infos = queue_create_infos;

    let device_features = vk::PhysicalDeviceFeatures::builder()
        .sampler_anisotropy(true)
        .build();

    let requred_validation_layer_raw_names: Vec<CString> = LAYER_NAME
        .iter()
        .map(|layer_name| CString::new(*layer_name).unwrap())
        .collect();
    let enabled_layer_names: Vec<*const i8> = requred_validation_layer_raw_names
        .iter()
        .map(|layer_name| layer_name.as_ptr())
        .collect();

    let device_extensions_raw_names: Vec<CString> = DEVICE_EXTENSIONS
        .iter()
        .map(|extension| CString::new(*extension).unwrap())
        .collect();
    let device_extensions_names: Vec<*const i8> = device_extensions_raw_names
        .iter()
        .map(|extension| extension.as_ptr())
        .collect();

    let mut create_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_features(&device_features)
        .enabled_extension_names(&device_extensions_names)
        .build();
    if IS_VALIDATION_LAYERS_ENABLED {
        create_info.enabled_layer_count = 1;
        create_info.pp_enabled_layer_names = enabled_layer_names.as_ptr();
    } else {
        create_info.enabled_layer_count = 0;
        create_info.pp_enabled_layer_names = std::ptr::null();
    }

    let create_info = create_info;

    let device:ash::Device = unsafe {
        instance
            .create_device(*physical_device, &create_info, None)
            .expect("Failed to create logical device")
    };
    device
}
