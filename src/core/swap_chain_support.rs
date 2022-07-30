use ash::vk;
use super::surface::Surface;

#[derive(Debug)]
pub struct SwapChainSupportDetails {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}
impl SwapChainSupportDetails {
    pub fn query_swap_chain_support(
        physical_device:&vk::PhysicalDevice,
        surface: &Surface,
    ) -> Self {
        unsafe {
            let capabilities = surface
                .surface_loader
                .get_physical_device_surface_capabilities(*physical_device, surface.surface)
                .expect("Failed to query for surface capabilities.");
            let formats = surface
                .surface_loader
                .get_physical_device_surface_formats(*physical_device, surface.surface)
                .expect("Failed to query for surface formats.");
            let present_modes = surface
                .surface_loader
                .get_physical_device_surface_present_modes(*physical_device, surface.surface)
                .expect("Failed to query for surface present mode.");
    
            SwapChainSupportDetails {
                capabilities,
                formats,
                present_modes,
            }
        }
    }
}
