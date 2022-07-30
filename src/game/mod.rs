pub mod game_objects;
use self::game_objects::{camera::Camera, GameObject, Vertex};
use crate::core::Core;
use crate::memory::Memory;
use crate::render_systems::main_render_system::PushConstant;
use crate::render_systems::MainRenderSystem;
use crate::renderer::Renderer;
use crate::window::Window;
use ash::vk;
use nalgebra_glm as glm;
use std::mem;
use std::rc::Rc;
use std::time;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::{
    event_loop::{self, ControlFlow},
    platform::run_return::EventLoopExtRunReturn,
};
pub const STATIC_MOVE_SPEED: f32 = 1000.0;
use crate::memory::AllocationType;
use crate::render_systems::RenderSystem;
use game_objects::key_event::{handle_key_event, key_handler};

pub struct Game {
    core: Rc<Core>,
    pub window: Window,
    pub renderer: Renderer,
    render_system: MainRenderSystem,
    memory: Memory,
    camera: Camera,
    pub delta_time: time::Duration,
    pub time: time::Instant,
    key_handler: key_handler,
    index_count: u32,
    game_objects: Vec<Box<dyn GameObject>>,
}
impl Game {
    pub fn new(event_loop: &event_loop::EventLoop<()>) -> Self {
        let mut window = Window::new(event_loop);
        let core = Rc::new(Core::new(&mut window));
        let renderer = Renderer::new(core.clone(), window.get_window_extent());
        let mut memory = Memory::new(core.clone());
        let render_system = MainRenderSystem::new(
            core.clone(),
            &renderer.get_render_pass(),
            &Vertex::get_attribute_descriptions(),
            &vec![Vertex::get_binding_description()],
        );
        // create a cube

        let vertices = vec![
            Vertex::new(glm::vec3(-0.5, -0.5, -0.5), glm::vec3(1.0, 0.0, 0.0)),
            Vertex::new(glm::vec3(-0.5, 0.5, 0.5), glm::vec3(0.0, 1.0, 0.0)),
            Vertex::new(glm::vec3(-0.5, -0.5, 0.5), glm::vec3(0.0, 0.0, 1.0)),
            Vertex::new(glm::vec3(-0.5, 0.5, -0.5), glm::vec3(1.0, 1.0, 1.0)),
            Vertex::new(glm::vec3(0.5, -0.5, -0.5), glm::vec3(1.0, 0.0, 0.0)),
            Vertex::new(glm::vec3(0.5, 0.5, 0.5), glm::vec3(0.0, 1.0, 0.0)),
            Vertex::new(glm::vec3(0.5, -0.5, 0.5), glm::vec3(0.0, 0.0, 1.0)),
            Vertex::new(glm::vec3(0.5, 0.5, -0.5), glm::vec3(1.0, 1.0, 1.0)),
        ];
        let indices = vec![
            0, 1, 2, 0, 3, 1, 4, 5, 6, 4, 7, 5, 0, 6, 2, 0, 4, 6, 3, 5, 1, 3, 7, 5, 2, 5, 1, 2, 6,
            5, 0, 7, 3, 0, 4, 7,
        ];
        let index_count = indices.len() as u32;
        memory.create_allocator(
            10000,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            256,
        );
        let vertex_buffer_index = memory.create_buffer(
            vertices.len() as u64 * std::mem::size_of::<Vertex>() as u64,
            AllocationType::Buffer,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
        );
        let index_buffer_index = memory.create_buffer(
            (indices.len() * mem::size_of::<u32>()) as u64,
            AllocationType::Buffer,
            vk::BufferUsageFlags::INDEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
        );
        memory.copy_memory(
            None,
            vertex_buffer_index,
            0,
            (vertices.len() * mem::size_of::<Vertex>()) as u64,
            vertices.as_ptr() as *const u8,
        );
        memory.copy_memory(
            None,
            index_buffer_index,
            0,
            (indices.len() * mem::size_of::<u32>()) as u64,
            indices.as_ptr() as *const u8,
        );
        let mut camera = Camera::new();

        Game {
            core,
            window,
            renderer,
            render_system,
            memory,
            camera,
            delta_time: time::Duration::new(0, 0),
            time: time::Instant::now(),
            key_handler: key_handler {
                position: glm::vec3(-5.0, -1.0, 2.0),
                target: glm::vec3(0.0, 0.0, 0.0),
            },
            index_count,
            game_objects: vec![],
        }
    }
    pub fn reset_perspective(&mut self) {
        self.camera.set_perspective_projection(
            50_f32.to_radians(),
            self.renderer.swap_chain.swap_chain_extent.width as f32
                / self.renderer.swap_chain.swap_chain_extent.height as f32,
            0.1,
            1000.0,
        );
    }
    pub fn draw(&mut self) {
        self.camera.set_view_direction(
            &self.key_handler.position,
            &self.key_handler.target,
            &glm::vec3(0.0, 1.0, 0.0),
        );

        let push = PushConstant {
            proj_view: self.camera.projection * self.camera.view,
        };

        let command_buffer = self.renderer.begin_frame();
        if command_buffer != vk::CommandBuffer::null() {
            self.renderer.begin_render_pass(command_buffer);

            for game_object in &mut self.game_objects {
                game_object.bind(&command_buffer);
                game_object.draw();
                game_object.update();
            }
            self.render_system.bind(&command_buffer, push);
            unsafe {
                self.core.logical_device.cmd_bind_vertex_buffers(
                    command_buffer,
                    0,
                    &[self.memory.buffers[0].handle],
                    &[0],
                );
                self.core.logical_device.cmd_bind_index_buffer(
                    command_buffer,
                    self.memory.buffers[1].handle,
                    0,
                    vk::IndexType::UINT32,
                );
                self.core.logical_device.cmd_draw_indexed(
                    command_buffer,
                    self.index_count,
                    1,
                    0,
                    0,
                    0,
                );
                
            }
            self.renderer.end_render_pass(command_buffer);
            self.renderer.end_frame();
        }
    }
    pub fn run(&mut self, event_loop: &mut event_loop::EventLoop<()>) {
        event_loop.run_return(move |event, _, control_flow| {
            let new_time = time::Instant::now();
            self.delta_time = new_time.duration_since(self.time);
            self.time = new_time;
            match event {
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
                            _ => {
                                handle_key_event(&input, &self.delta_time, &mut self.key_handler);
                            }
                        },
                    },
                    WindowEvent::Resized(_new_size) => {
                        self.reset_perspective();
                        unsafe {
                            self.core.logical_device.device_wait_idle().unwrap();
                        }
                        self.renderer.recreate_swap_chain(vk::Extent2D {
                            width: _new_size.width,
                            height: _new_size.height,
                        });
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {
                    self.window.window.request_redraw();
                }
                Event::RedrawRequested(_window_id) => {
                    self.draw();
                }
                _ => (),
            }
        });
    }
}
impl Drop for Game {
    fn drop(&mut self) {
        println!("dropping game");
    }
}
