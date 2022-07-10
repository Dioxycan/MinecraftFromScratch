use ash::vk;
use ash::extensions::khr::Surface as SurfaceLoader;
use std::ops::Drop;

pub struct Surface {
    pub surface:vk::SurfaceKHR,
    pub surface_loader:SurfaceLoader, 
}
impl Surface{
    pub fn new(
        entry:&ash::Entry,
        instance:&ash::Instance,
        window:&winit::window::Window)->Self{
       let surface_loader = SurfaceLoader::new(entry,instance);
       let surface = unsafe{
        ash_window::create_surface(
            entry,
            instance, 
            window, 
            None)
            .expect("Failed to create surface")
       };
         Surface{
              surface,
              surface_loader,
         }
    }
}
impl Drop for Surface{
    fn drop(&mut self){
        unsafe{
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }
}