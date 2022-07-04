mod debug;
mod device;
use ash::{vk, Entry, Instance,extensions::ext::DebugUtils};
use winit::{self, event_loop::EventLoop, window::WindowBuilder};

use self::debug::{
    check_validation_layer_support, populate_debug_messenger_create_info, Debug,LAYER_NAME
};
use self::device::Device;

use crate::IS_VALIDATION_LAYERS_ENABLED;
use std::ffi::{c_void, CStr, CString};
use std::ptr;
pub struct Core {
    pub window: winit::window::Window,
    entry: Entry,
    instance: Instance,
    extension_names: Vec<*const i8>,
    debug: Option<Debug>,
    device: Device,
}

impl Core {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let entry = Entry::linked();
        let window = WindowBuilder::new()
            .with_title("RustCraft")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
            .build(&event_loop)
            .unwrap();

        let mut extension_names = ash_window::enumerate_required_extensions(&window)
            .expect("Failed to enumerate required window extensions")
            .to_vec();
        if IS_VALIDATION_LAYERS_ENABLED{
            extension_names.push(DebugUtils::name().as_ptr());
        }
        extension_names.iter().for_each(|name| {
            println!("{}", unsafe { CStr::from_ptr(*name).to_str().unwrap() });
        });
        if IS_VALIDATION_LAYERS_ENABLED && check_validation_layer_support(&entry) == false {
            panic!("Validation layers requested, but not available!");
        }
        let instance = Core::create_instance(&entry, &extension_names);
        let debug = if IS_VALIDATION_LAYERS_ENABLED {
            Some(Debug::new(&entry, &instance))
        } else {
            None
        };
        let device = Device::new(&entry, &instance, &window);
        Core {
            entry: entry,
            window,
            extension_names,
            instance,
            debug,
            device,
        }
    }
    fn create_instance(entry: &Entry, extension_names: &Vec<*const i8>) -> Instance {
        let app_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"RustCraft\0") };
        let engine_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"No Engine\0") };

        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: 0,
            p_engine_name: engine_name.as_ptr(),
            engine_version: 0,
            api_version: vk::API_VERSION_1_0,
        };

            let requred_validation_layer_raw_names: Vec<CString> = LAYER_NAME
                .iter()
                .map(|layer_name| CString::new(*layer_name).unwrap())
                .collect();
            let enable_layer_names: Vec<*const i8> = requred_validation_layer_raw_names
                .iter()
                .map(|layer_name| {
                    layer_name.as_ptr()
                })
                .collect();
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,

            enabled_extension_count: extension_names.len() as u32,
            pp_enabled_extension_names: extension_names.as_ptr(),

            pp_enabled_layer_names: if IS_VALIDATION_LAYERS_ENABLED {
                enable_layer_names.as_ptr()
            } else {
                ptr::null()
            },
            enabled_layer_count: if IS_VALIDATION_LAYERS_ENABLED {
                enable_layer_names.len()
            } else {
                0
            } as u32,
            p_next: if IS_VALIDATION_LAYERS_ENABLED {
                &populate_debug_messenger_create_info()
                    as *const vk::DebugUtilsMessengerCreateInfoEXT as *const c_void
            } else {
                ptr::null()
            },
            ..Default::default()
        };
        unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance")
        }
    }
}
