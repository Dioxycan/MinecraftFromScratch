use ash::extensions::ext::DebugUtils;
use winit::{self, event_loop::EventLoop, window::WindowBuilder};
use super::debug::Debug;
use super::device::Device;
use std::ffi::{CStr};
use crate::IS_VALIDATION_LAYERS_ENABLED;
use super::instance::create_instance;
pub struct Core {
    pub window: winit::window::Window,
    entry: ash::Entry,
    instance: ash::Instance,
    extension_names: Vec<*const i8>,
    debug: Option<Debug>,
    device: Device,
}

impl Core {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let entry = ash::Entry::linked();
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


        let instance = create_instance(&entry, &extension_names);
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
}

impl Drop for Core{
    fn drop(&mut self) {
        unsafe {
            if Some(self.debug){

            }
            self.instance.destroy_instance(None);
        }
    }
}