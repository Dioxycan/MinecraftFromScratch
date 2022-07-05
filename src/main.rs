mod core;
mod game;
use winit::event_loop::EventLoop;
use crate::game::Game;

pub const IS_VALIDATION_LAYERS_ENABLED: bool = true;
fn main() {
    let mut event_loop = EventLoop::new();
    Game::new(&event_loop).run(&mut event_loop); 
}
