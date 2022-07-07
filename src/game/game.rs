use super::core::Core;
use super::window::Window;
use super::renderer::Renderer;
use std::rc::Rc;
use std::cell::RefCell;

use super::render_systems::MainRenderSystem;
use ash::vk;
use winit::{event_loop, platform::run_return::EventLoopExtRunReturn};
pub struct Game{
    core:Rc<Core>,
    pub window:Window,
    pub renderer:Renderer,
    render_system:MainRenderSystem,
}
impl Game{
    pub fn new(event_loop:&event_loop::EventLoop<()>)->Self{
        let mut window = Window::new(event_loop);
        let core = Rc::new(Core::new(&window));
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

            self.renderer.end_render_pass(command_buffer);
            self.renderer.end_frame();
        }
       
    }
}
impl Drop for Game{
    fn drop(&mut self){
        println!("dropping game");
    }
}