mod game;
use crate::game::Game;

pub const IS_VALIDATION_LAYERS_ENABLED: bool = true;
fn main() {
    Game::new().run(); 
}
