use super::core::Core;
use super::window::Window;
use super::renderer::Renderer;
use std::rc::Rc;
pub struct Game{
    core:Rc<Core>,
    window:Window,
    renderer:Renderer,
}
impl Game{
    pub fn new()->Self{
        let window = Window::new();
        let core = Rc::new(Core::new(&window));
        let renderer=Renderer::new(core.clone(),window.get_window_extent());

        Game{
            core,
            window,
            renderer,
        }
    }
    pub fn draw(&mut self){}
    pub fn close(&mut self){}
    pub fn run(&mut self){
        let draw_f = ||{
        };
        let close_f = ||{
        };
        self.window.run(draw_f,close_f);
    }
}
impl Drop for Game{
    fn drop(&mut self){
        println!("dropping game");
    }
}