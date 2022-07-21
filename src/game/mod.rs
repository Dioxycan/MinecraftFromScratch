mod game_objects;
use self::game_objects::{camera::Camera, Vertex};
use crate::core::Core;
use crate::memory::Memory;
use crate::render_systems::MainRenderSystem;
use crate::renderer::Renderer;
use crate::window::Window;
use crate::render_systems::main_render_system::PushConstant;
use ash::vk;
use nalgebra_glm as glm;
use std::mem;
use std::rc::Rc;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::{
    event_loop::{self, ControlFlow},
    platform::run_return::EventLoopExtRunReturn,
};

pub struct Game {
    core: Rc<Core>,
    pub window: Window,
    pub renderer: Renderer,
    render_system: MainRenderSystem,
    memory: Memory,
    camera: Camera,
}
impl Game {
    pub fn new(event_loop: &event_loop::EventLoop<()>) -> Self {
        let mut window = Window::new(event_loop);
        let core = Rc::new(Core::new(&mut window));
        let renderer = Renderer::new(core.clone(), window.get_window_extent());
        let mut memory = Memory::new(core.clone());
        let mut render_system = MainRenderSystem::new(core.clone());
        render_system.init(
            renderer.get_render_pass(),
            &Vertex::get_attribute_descriptions(),
            &vec![Vertex::get_binding_description()],
        );
        let vertices = vec![
            Vertex::new(glm::vec3(0.0, -0.5, 0.0), glm::vec3(1.0, 0.0, 0.0)),
            Vertex::new(glm::vec3(0.5, 0.5, 0.0), glm::vec3(0.0, 1.0, 0.0)),
            Vertex::new(glm::vec3(-0.5, 0.5, 0.0), glm::vec3(0.0, 0.0, 1.0)),
        ];
        memory.create_allocator(
            256,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );
        let vertex_buffer_index = memory.create_buffer(
            100,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
        );
        memory.allocate_memory(vertex_buffer_index);
        memory.copy_memory(
            renderer.command.command_buffers[0],
            vertex_buffer_index,
            0,
            (vertices.len() * mem::size_of::<Vertex>()) as u64,
            vertices.as_ptr() as *const u8,
        );
        let mut camera = Camera::new();
        Game {
            core,
            window,
            renderer,
            render_system,
            memory,
            camera,
        }
    }
    pub fn draw(&mut self) {
        self.camera.set_perspective_projection(
            45.0f32.to_radians(),
            self.renderer.swap_chain.swap_chain_extent.width as f32
                / self.renderer.swap_chain.swap_chain_extent.height as f32,
            0.0,
            100.0,
        );
        let push = PushConstant{
            view: self.camera.view,
            proj: self.camera.projection,
        };
        let command_buffer = self.renderer.begin_frame();
        if command_buffer != vk::CommandBuffer::null() {
            self.renderer.begin_render_pass(command_buffer);
            self.render_system.bind(command_buffer,push);
            unsafe {
                self.core.logical_device.cmd_bind_vertex_buffers(
                    command_buffer,
                    0,
                    &[self.memory.buffers[0].handle],
                    &[0],
                );
                self.core.logical_device.cmd_draw(
                    command_buffer,
                    3,
                    1,
                    0,
                    0,
                );
            }
            self.renderer.end_render_pass(command_buffer);
            self.renderer.end_frame();
        }
    }
    pub fn run(&mut self, event_loop: &mut event_loop::EventLoop<()>) {
        event_loop.run_return(move |event, _, control_flow| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    unsafe {
                        self.core.logical_device.device_wait_idle().unwrap();
                    }
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        virtual_keycode,
                        state,
                        ..
                    } => match (virtual_keycode, state) {
                        (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                            unsafe {
                                self.core.logical_device.device_wait_idle().unwrap();
                            }
                            *control_flow = ControlFlow::Exit
                        }
                        _ => {}
                    },
                },
                WindowEvent::Resized(_new_size) => {
                    if _new_size.width == 0 && _new_size.height == 0 {
                        println!("new size {:?}", _new_size);
                    } else {
                        unsafe {
                            self.core.logical_device.device_wait_idle().unwrap();
                        }
                        self.renderer.recreate_swap_chain(vk::Extent2D {
                            width: _new_size.width,
                            height: _new_size.height,
                        });
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                self.window.window.request_redraw();
            }
            Event::RedrawRequested(_window_id) => {
                self.draw();
            }
            Event::LoopDestroyed => {}
            _ => (),
        });
    }
}
impl Drop for Game {
    fn drop(&mut self) {
        println!("dropping game");
    }
}
