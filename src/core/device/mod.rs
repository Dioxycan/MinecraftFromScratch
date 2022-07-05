mod surface;
mod queue;
pub mod device;
mod physical_device;
mod swap_chain_support;

pub use device::Device;

const device_extensions: [&str; 1] = [
    "VK_KHR_swapchain",
];