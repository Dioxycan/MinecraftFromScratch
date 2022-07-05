use ash::Instance;
use ash::vk;
use ash::Entry;
use winit::window::Window;

use super::surface::Surface;
use super::physical_device::pick_physical_device;

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
