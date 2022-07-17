use super::core::Core;
use super::window::Window;
use super::renderer::Renderer;
use std::rc::Rc;
use super::render_systems::MainRenderSystem;
use ash::vk;
use winit::{event_loop::{self,ControlFlow}, platform::run_return::EventLoopExtRunReturn};
use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use super::game_objects::GameObjects;
pub struct Game{
    core:Rc<Core>,
    pub window:Window,
    pub renderer:Renderer,
    game_objects:GameObjects,
    render_system:MainRenderSystem,
}
impl Game{
    pub fn new(event_loop:&event_loop::EventLoop<()>)->Self{
        let mut window = Window::new(event_loop);
        let core = Rc::new(Core::new(&mut window));
        let renderer=Renderer::new(core.clone(),window.get_window_extent());
        let mut render_system = MainRenderSystem::new(core.clone());
        render_system.init(renderer.get_render_pass());
        let game_objects = GameObjects::new();
        let vertex_buffer_info = vk::BufferCreateInfo::builder()
            .size()
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .build();
        let vertex_buffer = unsafe{
            core.logical_device.create_buffer(

                , None)
        }
        unsafe{
            println!("{:?}",core.logical_device.get_buffer_memory_requirements(vk::Buffer::null()));
        }
        Game{
            core,
            window,
            renderer,
            game_objects,
            render_system
        }
    }
    pub fn draw(&mut self){
        let command_buffer = self.renderer.begin_frame();
        if command_buffer != vk::CommandBuffer::null() {
            self.renderer.begin_render_pass(command_buffer);
            self.render_system.bind(command_buffer);                                                                                         
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
                            if _new_size.width ==0 && _new_size.height == 0{
                                println!("new size {:?}",_new_size);
                            }else{
                            unsafe {
                                self.core.logical_device.device_wait_idle().unwrap();
                            }
                            self.renderer.recreate_swap_chain(vk::Extent2D{width:_new_size.width,height:_new_size.height});
                        }   
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