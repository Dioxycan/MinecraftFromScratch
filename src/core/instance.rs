use super::debug::{
    check_validation_layer_support, populate_debug_messenger_create_info, LAYER_NAME,
};
use crate::IS_VALIDATION_LAYERS_ENABLED;
use ash::vk;
use std::ffi::{CStr, CString};

pub fn create_instance(entry: &ash::Entry, extension_names: &Vec<*const i8>) -> ash::Instance {
    if IS_VALIDATION_LAYERS_ENABLED && check_validation_layer_support(&entry) == false {
        panic!("Validation layers requested, but not available!");
    }
    let app_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"RustCraft\0") };
    let engine_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"No Engine\0") };

    let app_info = vk::ApplicationInfo::builder()
        .application_name(app_name)
        .application_version(0)
        .engine_name(engine_name)
        .engine_version(0)
        .api_version(vk::API_VERSION_1_0)
        .build();

    let mut debug_create_info: vk::DebugUtilsMessengerCreateInfoEXT =
        populate_debug_messenger_create_info();
    let mut _raw_names: Vec<CString> = vec![];
    let mut enabled_layer_names: Vec<*const i8> = vec![];

    if IS_VALIDATION_LAYERS_ENABLED {
        _raw_names = LAYER_NAME
            .iter()
            .map(|layer_name| CString::new(*layer_name).unwrap())
            .collect();

        enabled_layer_names = _raw_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();
    }
    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names)
        .enabled_layer_names(&enabled_layer_names);
    unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("Failed to create instance")
    }
}
