
pub mod queue;
pub mod core;
mod physical_device;
mod swap_chain_support;
mod logical_device;
mod instance;
mod debug;
mod surface;
pub use self::core::Core;

const DEVICE_EXTENSIONS: [&'static str; 1] = ["VK_KHR_swapchain"];