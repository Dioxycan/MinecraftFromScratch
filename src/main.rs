mod game;
mod utils;
mod memory;
mod core;
mod render_systems;
mod window;
mod renderer;
mod command;
use winit::event_loop;

use crate::game::Game;
pub const MAX_FRAMES_IN_FLIGHT: u32 = 2;
pub const IS_VALIDATION_LAYERS_ENABLED: bool = true;
fn main() {
    let mut event_loop = event_loop::EventLoop::new();
    Game::new(&event_loop).run(&mut event_loop);

   
}
