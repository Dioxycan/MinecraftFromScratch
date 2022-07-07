use std::rc::Rc;

use super::swap_chain::SwapChain;
use crate::game::core::Core;
use ash::vk;
pub struct Renderer{
    swap_chain: SwapChain,
}
impl Renderer{
    pub fn new(core: Rc<Core>,window_extent:vk::Extent2D)->Self{
        let swap_chain = SwapChain::new(core,window_extent);
        Renderer{
            swap_chain,
        }
    }
    pub fn get_render_pass(&self)->&vk::RenderPass{
        self.swap_chain.get_render_pass()
    }
}