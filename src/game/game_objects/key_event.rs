use std::time;
use winit::event;
use nalgebra_glm as glm;
use super::STATIC_MOVE_SPEED;
#[derive(Debug, Clone, Copy)]
pub struct  key_handler {
    pub position: glm::Vec3,
    pub target: glm::Vec3,
}
pub fn handle_key_event(
    event: &event::KeyboardInput,
    delta_time: &time::Duration,
    key_handler: &mut key_handler,
) {
    let rotate = glm::Vec3::new(0.0, 0.0, 0.0);
    match event {

       | event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::W),
            ..
        } => {

            key_handler.position.z += STATIC_MOVE_SPEED * delta_time.as_secs_f64() as f32;
        }
        |event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::S),
            ..
        } => {
            key_handler.position.z -= STATIC_MOVE_SPEED * delta_time.as_secs_f64() as f32;
        }
        |event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::A),
            ..
        } => {
            key_handler.position.x -= STATIC_MOVE_SPEED * delta_time.as_secs_f64() as f32;
        }
        |event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::D),
            ..
        } => {
            key_handler.position.x += STATIC_MOVE_SPEED * delta_time.as_secs_f64() as f32;
        }
        |event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::Space),
            ..
        } => {
            key_handler.position.y += STATIC_MOVE_SPEED * delta_time.as_secs_f64() as f32;
        }
        |event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::LControl),
            ..
        } => {
            key_handler.position.y -= STATIC_MOVE_SPEED * delta_time.as_secs_f64() as f32;
        }
        |event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::Up),
            ..
        } => {
            key_handler.target.y += STATIC_MOVE_SPEED * delta_time.as_secs_f64() as f32;
        }
        |event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::Down),
            ..
        } => {
            key_handler.target.y -= STATIC_MOVE_SPEED * delta_time.as_secs_f64() as f32;
        }
        |event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::Left),
            ..
        } => {
            key_handler.target.x -= STATIC_MOVE_SPEED * delta_time.as_secs_f64() as f32;
        }
        |event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::Right),
            ..
        } => {
            key_handler.target.x += STATIC_MOVE_SPEED * delta_time.as_secs_f64() as f32;
        }
        |event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::RShift),
            ..
        } => {
            key_handler.target.z += STATIC_MOVE_SPEED * delta_time.as_secs_f64() as f32;
        }
        |event::KeyboardInput {
            state: event::ElementState::Pressed,
            virtual_keycode: Some(event::VirtualKeyCode::RControl),
            ..
        } => {
            key_handler.target.z -= STATIC_MOVE_SPEED * delta_time.as_secs_f64() as f32;
        }

        _ => {}
    }
}