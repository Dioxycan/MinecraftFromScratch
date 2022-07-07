use ash::vk;
use std::cell::RefCell;
use winit::{window,event_loop, platform::run_return::EventLoopExtRunReturn};
pub struct Window{
    pub window:window::Window,
    pub event_loop:RefCell<event_loop::EventLoop<()>>,
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
            event_loop:RefCell::new(event_loop),
        }
    }
    pub fn run<F,CF>(&self,draw_f:F,close_f:CF)where F:'static + Fn(),CF:'static + Fn(){
        self.event_loop.borrow_mut().run_return( move |event, _, control_flow| {
            // handle event
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        close_f();
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    winit::event::WindowEvent::KeyboardInput { input, .. } => match input {
                        winit::event::KeyboardInput {
                            virtual_keycode,
                            state,
                            ..
                        } => match (virtual_keycode, state) {
                            (
                                Some(winit::event::VirtualKeyCode::Escape),
                                winit::event::ElementState::Pressed,
                            ) => {
                                dbg!();
                                close_f();
                                *control_flow = winit::event_loop::ControlFlow::Exit;
                            }
                            _ => {}
                        },
                    },
                    _ => {}
                },
                winit::event::Event::MainEventsCleared => {
                    draw_f();
                }
                winit::event::Event::RedrawRequested(_window_id) => {
                    
                }
                _ => (),
            }
        });
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
