mod game;
mod utils;
mod memory;
mod core;
mod render_systems;
mod window;
mod renderer;
use winit::event_loop;

use crate::game::Game;

pub const IS_VALIDATION_LAYERS_ENABLED: bool = true;
fn main() {
    let mut event_loop = event_loop::EventLoop::new();
    Game::new(&event_loop).run(&mut event_loop);

   
}
