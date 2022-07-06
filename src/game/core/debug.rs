use ash::{extensions::ext, vk};
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::borrow::Cow;

pub const LAYER_NAME: [&'static str; 1] = ["VK_LAYER_KHRONOS_validation"];

pub struct Debug {
    pub debug_utils_loader: ext::DebugUtils,
    pub debug_utils_messenger: vk::DebugUtilsMessengerEXT,
}

fn raw_to_str(raw_str: &[c_char]) -> String {
    let p = raw_str.as_ptr();
    unsafe { CStr::from_ptr(p).to_string_lossy().into_owned() }
}

pub fn check_validation_layer_support(entry: &ash::Entry) -> bool {
    let layer_properties = entry
        .enumerate_instance_layer_properties()
        .expect("Failed to enumerate Instance Layers Properties!");
    if layer_properties.len() <= 0 {
        eprintln!("No available layers.");
        return false;
    } else {
        println!("Instance Available Layers: ");
        for layer in layer_properties.iter() {
            let layer_name = raw_to_str(&layer.layer_name);
            println!("\t{}", layer_name);
        }
    }

    println!("Checking required Validation Layers: ");
    let mut is_required_validation_layers_found = true;
    'outer: for required_validation_layer in LAYER_NAME.iter() {
        for layer_property in layer_properties.iter() {
            let layer_name = raw_to_str(&layer_property.layer_name);
            if layer_name == *(required_validation_layer) {
                println!("\t{} is available", required_validation_layer);
                continue 'outer;
            }
        }
        is_required_validation_layers_found = false;
        eprintln!("\t{} is not available", required_validation_layer);
    }
    is_required_validation_layers_found
}
unsafe extern "system" fn debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number: i32 = callback_data.message_id_number as i32;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{:?}:\n{:?} [{} ({})] : {}\n",
        message_severity,
        message_type,
        message_id_name,
        &message_id_number.to_string(),
        message,
    );

    vk::FALSE
}
pub fn populate_debug_messenger_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
    vk::DebugUtilsMessengerCreateInfoEXT {
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
            | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
            | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
        message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        pfn_user_callback: Some(debug_utils_callback),
        p_user_data: ptr::null_mut(),
        ..Default::default()

    }
}
pub fn setup_debug_messenger(
    debug_utils_loader : &ext::DebugUtils,
) -> vk::DebugUtilsMessengerEXT {
        let create_info = populate_debug_messenger_create_info();
        let debug_utils_messenger = unsafe {
            debug_utils_loader
                .create_debug_utils_messenger(&create_info, None)
                .expect("Debug Utils Callback not found")
        };
        debug_utils_messenger

}

impl Debug {
    pub fn new(entry: &ash::Entry,instance: &ash::Instance) -> Self {
        let debug_utils_loader = ext::DebugUtils::new(entry, instance);
        let debug_utils_messenger = setup_debug_messenger(&debug_utils_loader);
        Debug {
            debug_utils_loader:  ext::DebugUtils::new(entry, instance),
            debug_utils_messenger,
        }
    }
}

