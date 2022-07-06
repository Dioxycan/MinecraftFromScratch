
use winit::{self, event_loop::EventLoop, window::WindowBuilder};
use super::device::Device;
pub struct Core {
    pub window: winit::window::Window,
    pub entry: ash::Entry,
    pub device: Device,
}

impl Core {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let entry = ash::Entry::linked();
        let window = WindowBuilder::new()
            .with_title("RustCraft")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
            .build(&event_loop)
            .unwrap();
        let device = Device::new(&entry, &window);
        Core {
            entry: entry,
            window,
            device,
        }
    }
}
