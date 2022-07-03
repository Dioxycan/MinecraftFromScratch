mod debug;
mod device;
use ash::{vk, Entry, Instance};
use winit::{self, event_loop::EventLoop, window::WindowBuilder};

use self::debug::{Debug,LAYER_NAME};
use self::device::{Device};

use std::ffi::{CStr, CString};
use crate::IS_VALIDATION_LAYERS_ENABLED;

pub struct Core {
    pub window: winit::window::Window,
    entry: Entry,
    instance: Instance,
    extension_names: Vec<*const i8>,
    debug:Option<Debug>,
    device:Device
}

impl Core {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let entry = Entry::linked();
        let window = WindowBuilder::new()
            .with_title("RustCraft")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
            .build(&event_loop)
            .unwrap();

        let extension_names = ash_window::enumerate_required_extensions(&window)
            .expect("Failed to enumerate required window extensions")
            .to_vec();

        let instance = Core::create_instance(&entry, &extension_names);

        let debug = if IS_VALIDATION_LAYERS_ENABLED {
            Some(Debug::new(&entry, &instance))
        } else {
            None
        };
        let device = Device::new(&entry, &instance,&window);
        Core {
            entry: entry,
            window,
            extension_names,
            instance,
            debug,
            device
        }
    }
    fn create_instance(entry: &Entry, extension_names: &Vec<*const i8>) -> Instance {
        let app_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"RustCraft\0") };
        let engine_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"No Engine\0") };

        let app_info = vk::ApplicationInfo::builder()
            .application_name(app_name)
            .application_version(0)
            .engine_name(engine_name)
            .engine_version(0)
            .api_version(vk::API_VERSION_1_0);

        let requred_validation_layer_raw_names: Vec<CString> = LAYER_NAME
            .iter()
            .map(|layer_name| CString::new(*layer_name).unwrap())
            .collect();
        let enable_layer_names: Vec<*const i8> = requred_validation_layer_raw_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(extension_names)
            .enabled_layer_names(enable_layer_names.as_slice().as_ref());

        unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance")
        }
    }
}
