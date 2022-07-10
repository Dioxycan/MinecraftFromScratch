use std::cell::RefCell;
use std::rc::Rc;
use ash::vk;
use winit::{window,event_loop,platform::run_return::EventLoopExtRunReturn};



pub struct Window{
    pub window:window::Window,
    
}
impl Window{
    pub fn new()->Self{
        let event_loop = event_loop::EventLoop::new();
        let window = window::WindowBuilder::new()
            .with_title("Hello World!")
            .build(&event_loop)
            .expect("Failed to create window");
        Window{
            window,
        }
    }
    pub fn get_window_extent(&self)->vk::Extent2D{
        let window_size = self.window.inner_size();
        vk::Extent2D{
            width:window_size.width as u32,
            height:window_size.height as u32,
        }
    }
    pub fn enumerate_window_extensions(&self)->Vec<*const i8>{
        ash_window::enumerate_required_extensions(&self.window).expect("Failed to enumerate window extensions").to_owned()
    }
}
