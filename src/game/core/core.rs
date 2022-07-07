use ash::Instance;
use ash::vk;
use ash::extensions::ext::DebugUtils;
use crate::game::window::Window;
use super::logical_device::create_logical_device;
use super::surface::Surface;
use super::physical_device::pick_physical_device;
use super::queue::{QueueFamilies,QueueFamilyIndices};
use super::swap_chain_support::SwapChainSupportDetails;
use super::instance::create_instance;
use super::debug::Debug;
use crate::IS_VALIDATION_LAYERS_ENABLED;
pub struct Core{
    pub entry: ash::Entry,
    pub instance: Instance,
    extension_names: Vec<*const i8>,
    pub surface:Surface,
    pub physical_device:vk::PhysicalDevice,
    pub logical_device:ash::Device,
    pub queue_families:QueueFamilies,
    pub swap_chain_support:SwapChainSupportDetails,
    pub debug:Option<Debug>,
}
impl Core{
    pub fn new(window:&Window)->Self{
        let entry = ash::Entry::linked();
        let mut extension_names = window.enumerate_window_extensions();
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
        let surface = Surface::new(&entry,&instance,&window.window);
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
        Core{
            entry,
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
    pub fn find_supported_format(&self,formats:Vec<vk::Format>,tiling:vk::ImageTiling,features:vk::FormatFeatureFlags)->vk::Format{
        for format in formats{
            let format_properties = unsafe{self.instance.get_physical_device_format_properties(self.physical_device,format)};
            if tiling == vk::ImageTiling::LINEAR && format_properties.linear_tiling_features.contains(features){
                return format;
            }else if tiling == vk::ImageTiling::OPTIMAL && format_properties.optimal_tiling_features.contains(features){
                return format;
            }
            }
        panic!("Failed to find supported format");
    }
    pub fn find_memory_type(&self,type_bits:u32,properties:vk::MemoryPropertyFlags)->Option<u32>{
        let memory_properties = unsafe{self.instance.get_physical_device_memory_properties(self.physical_device)};
        memory_properties.memory_types.iter().enumerate().find(|&(index,memory_type)|{
            type_bits & (1 << index) != 0 && memory_type.property_flags.contains(properties)
        }).map(|(index,_)|index as u32) 
    }
}
    
impl Drop for Core{
    fn drop(&mut self) {
        println!("dropping core");
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
