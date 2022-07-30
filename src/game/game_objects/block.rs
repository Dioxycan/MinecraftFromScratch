use ash::vk;
use super::GameObject;
struct Block{

}
impl GameObject for Block{
    fn new()->Self{
        Block{
        }
    }
    fn bind(&self,command_buffer:& vk::CommandBuffer){
    }
    fn draw(&self){
    }
    fn update(&self){
    }
}
