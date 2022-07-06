mod surface;
pub mod queue;
pub mod device;
mod physical_device;
mod swap_chain_support;
mod logical_device;
mod instance;
mod debug;
pub use device::Device;

const DEVICE_EXTENSIONS: [&'static str; 1] = ["VK_KHR_swapchain"];