use super::core::Core;
use super::window::Window;
use super::renderer::Renderer;
use std::rc::Rc;
use super::render_systems::MainRenderSystem;
use ash::vk;
use winit::{event_loop, platform::run_return::EventLoopExtRunReturn};
use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::ControlFlow;
pub struct Game{
    core:Rc<Core>,
    pub window:Window,
    pub renderer:Renderer,
    render_system:MainRenderSystem,
}

impl Game{
    pub fn new(event_loop:&event_loop::EventLoop<()>)->Self{
        let mut window = Window::new(event_loop);
        let core = Rc::new(Core::new(&mut window));
        let renderer=Renderer::new(core.clone(),window.get_window_extent());
        let mut render_system = MainRenderSystem::new(core.clone());
        render_system.init(renderer.get_render_pass());
        Game{
            core,
            window,
            renderer,
            render_system,
        }
    }
    pub fn draw(&mut self){
        let command_buffer = self.renderer.begin_frame();
        if command_buffer != vk::CommandBuffer::null() {
            self.renderer.begin_render_pass(command_buffer);
            self.render_system.bind(command_buffer);
            self.render_system.draw(command_buffer);
            self.renderer.end_render_pass(command_buffer);
            self.renderer.end_frame();
        }
    }
    pub fn run(&mut self,event_loop:&mut event_loop::EventLoop<()>){
        event_loop.run_return( move |event, _, control_flow| {
            match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            unsafe {
                                self.core.logical_device.device_wait_idle().unwrap();
                            }
                            *control_flow = ControlFlow::Exit
                        },
                        | WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                        | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                            unsafe {
                                                self.core.logical_device.device_wait_idle().unwrap();
                                            }
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                        },
                        | WindowEvent::Resized(_new_size) => {
                            *control_flow = ControlFlow::Wait;
                            unsafe {
                                self.core.logical_device.device_wait_idle().unwrap();
                            }
                            self.renderer.recreate_swap_chain(vk::Extent2D{width:_new_size.width,height:_new_size.height});

                        },
                        | _ => {},
                    }
                },
                | Event::MainEventsCleared => {
                    self.window.window.request_redraw();
                },
                | Event::RedrawRequested(_window_id) => {
                    self.draw();
                },
                | Event::LoopDestroyed => {
                    unsafe {
                        self.core.logical_device.device_wait_idle().unwrap();
                    }
                },
                _ => (),
            }
        });
    }
}
impl Drop for Game{
    fn drop(&mut self){
        println!("dropping game");
    }
}