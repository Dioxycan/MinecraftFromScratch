use ash::Instance;
use ash::vk;
use ash::Entry;
use ash::extensions::ext::DebugUtils;
use winit::window::Window;

use super::logical_device::create_logical_device;
use super::surface::Surface;
use super::physical_device::pick_physical_device;
use super::queue::{QueueFamilies,QueueFamilyIndices};
use super::swap_chain_support::SwapChainSupportDetails;
use super::instance::create_instance;
use super::debug::Debug;
use crate::IS_VALIDATION_LAYERS_ENABLED;
pub struct Device{
    pub instance: Instance,
    extension_names: Vec<*const i8>,
    pub surface:Surface,
    pub physical_device:vk::PhysicalDevice,
    pub logical_device:ash::Device,
    pub queue_families:QueueFamilies,
    pub swap_chain_support:SwapChainSupportDetails,
    pub debug:Option<Debug>,
}
impl Device{
    pub fn new(entry:&Entry,window:&Window)->Self{
        let mut extension_names = ash_window::enumerate_required_extensions(&window)
        .expect("Failed to enumerate required window extensions")
        .to_vec();
        if IS_VALIDATION_LAYERS_ENABLED{
            extension_names.push(DebugUtils::name().as_ptr());
        }
        let instance = create_instance(&entry, &extension_names);
        let mut indices = QueueFamilyIndices{
            graphics_family:None,
            present_family:None,
        };
        let debug = if IS_VALIDATION_LAYERS_ENABLED {
            Some(Debug::new(&entry, &instance))
        } else {
            None
        };
        let surface = Surface::new(entry,&instance,window);
        let physical_device = pick_physical_device(&instance,&surface,&mut indices);
        let logical_device = create_logical_device(&instance,&physical_device,&surface);
        let queue_families = QueueFamilies::from(
            indices,
            &logical_device,
        );
        let swap_chain_support = SwapChainSupportDetails::query_swap_chain_support(
            &physical_device,
            &surface,
        );
        Device{
            instance,
            debug,
            extension_names,
            surface,
            physical_device,
            logical_device,
            queue_families,
            swap_chain_support
        }
    }

    
}
impl Drop for Device{
    fn drop(&mut self) {
        unsafe {

            self.logical_device.destroy_device(None);
            self.surface.surface_loader.destroy_surface(self.surface.surface, None);
            match self.debug{
                Some(ref debug) => {
                    debug.debug_utils_loader.destroy_debug_utils_messenger(debug.debug_utils_messenger, None);
                },
                None => {},
            }
           
            self.instance.destroy_instance(None);
        }
    }
}
