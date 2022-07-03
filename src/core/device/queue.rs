use ash::vk;
use ash::{Instance};
use super::surface::Surface;

use std::collections::HashSet;

#[derive(Debug)]
pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
    pub present_family: Option<u32>,
}
impl QueueFamilyIndices {
    pub fn new()->Self{
        QueueFamilyIndices{
            graphics_family:None,
            present_family:None,
        }
    }
    pub fn find_queue_families(
        instance: &Instance,
        physical_device: &vk::PhysicalDevice,
        surface: &Surface,
    ) -> Self {
        let mut indices = QueueFamilyIndices::new();
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(*physical_device) };
        for (i, queue_family) in queue_families.iter().enumerate() {
            if queue_family.queue_count > 0
                && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
            {
                indices.graphics_family = Some(i as u32);
            }
            let is_present_supported = unsafe {
                surface
                    .surface_loader
                    .get_physical_device_surface_support(
                        *physical_device,
                        i as u32,
                        surface.surface,
                    )
                    .expect("Failed to get physical device surface support")
            };
            if is_present_supported {
                indices.present_family = Some(i as u32);
            }
            if indices.is_complete() {
                return indices;
            }
        }
        panic!("Failed to find queue families");
    }
    pub fn is_complete(&self)->bool{
        self.graphics_family.is_some() && self.present_family.is_some()
    }
    pub fn to_unique(&self)->HashSet<u32>{
        let mut set = HashSet::new();
        if let Some(graphics_family) = self.graphics_family{
            set.insert(graphics_family);
        }
        if let Some(present_family) = self.present_family{
            set.insert(present_family);
        }
        set
    }
}
