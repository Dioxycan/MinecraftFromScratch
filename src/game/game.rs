use super::core::Core;
use super::window::Window;
use super::renderer::Renderer;
use std::rc::Rc;
use std::cell::RefCell;

use super::render_systems::MainRenderSystem;
use ash::vk;
use winit::{event_loop, platform::run_return::EventLoopExtRunReturn};
pub const MAX_FRAMES_IN_FLIGHT : usize = 2;
pub struct Game{
    core:Rc<Core>,
    pub window:Window,
    pub renderer:Renderer,
    render_system:MainRenderSystem,
    event_loop:Rc<RefCell<event_loop::EventLoop<()>>>
}

impl Game{
    pub fn new()->Self{
        let event_loop = event_loop::EventLoop::new();
        let window = Window::new();
        let core = Rc::new(Core::new(&window));
        let renderer=Renderer::new(core.clone(),window.get_window_extent());
        let mut render_system = MainRenderSystem::new(core.clone());
        render_system.init(renderer.get_render_pass());
        Game{
            core,
            window,
            renderer,
            render_system,
            event_loop:Rc::new(RefCell::new(event_loop))
        }
    }
    pub fn draw(&mut self){
        let command_buffer = self.renderer.begin_frame();
        if command_buffer != vk::CommandBuffer::null() {
            self.renderer.begin_render_pass(command_buffer);
            self.render_system.bind(command_buffer);
            unsafe{self.core.logical_device.cmd_draw(command_buffer, 3, 1, 0, 0);}
            self.renderer.end_render_pass(command_buffer);
            self.renderer.end_frame();
        }
    }
    pub fn run(&mut self){
        self.event_loop.clone().borrow_mut().run_return( move |event, _, control_flow| {
            // handle event
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    },
                    winit::event::WindowEvent::Resized(physical_size) => {
                       self.renderer.is_window_resized = true;
                        self.renderer.window_extent = self.window.get_window_extent();
                        *control_flow = winit::event_loop::ControlFlow::Wait;
                    },
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
                    self.window.window.request_redraw();
                }
                winit::event::Event::RedrawRequested(_window_id) => {
                    self.draw();
                }
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