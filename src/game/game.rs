use super::core::Core;
use super::window::Window;

pub struct Game{
    core:Core,
    window:Window,
}
impl Game{
    pub fn new()->Self{
        let window = Window::new();
        let core = Core::new(&window);
        Game{
            core,
            window,
        }
    }
    pub fn run(&self){
        
    }
}