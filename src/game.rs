use crate::core::Core;
use winit::{self,event_loop::EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
pub struct Game{
    core:Core,
}
impl Game{
    pub fn new(event_loop:&EventLoop<()>)->Self{
        Game{
            core:Core::new(event_loop),
        }
    }
    pub fn run(&self,event_loop:&mut EventLoop<()>){
        event_loop.run_return( move |event, _, control_flow| {
            // handle event
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
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

                                *control_flow = winit::event_loop::ControlFlow::Exit;
                            }
                            _ => {}
                        },
                    },
                    _ => {}
                },
                winit::event::Event::MainEventsCleared => {
                    self.core.window.request_redraw();
                }
                winit::event::Event::RedrawRequested(_window_id) => {
                    
                }
                _ => (),
            }
        });
    }
}