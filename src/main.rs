mod game;
use winit::event_loop;

use crate::game::Game;

pub const IS_VALIDATION_LAYERS_ENABLED: bool = true;
fn main() {
    let event_loop = event_loop::EventLoop::new();
    let mut game =Game::new(&event_loop);
    event_loop.run( move |event, _, control_flow| {
        // handle event
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                },
                winit::event::WindowEvent::Resized(physical_size) => {
                    game.renderer.is_window_resized = true;
                    game.renderer.window_extent = game.window.get_window_extent();
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
            }
            winit::event::Event::RedrawRequested(_window_id) => {
                
            }
            _ => (),
        }
    });
}
