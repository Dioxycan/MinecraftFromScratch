use ash::vk;
use crate::core::Core;
use std::rc::Rc;
pub struct Buffer{
    core: Rc<Core>,
    pub handle:vk::Buffer,
    pub offsets:Vec<vk::DeviceSize>,
    pub buffer_type:vk::BufferUsageFlags,
    pub memory_type:vk::MemoryPropertyFlags,
    pub memory_requirements:vk::MemoryRequirements,
    pub size:vk::DeviceSize,
    pub allocator_index:Option<usize>,
}
impl Buffer{
    pub fn new(core:Rc<Core>,buffer_info:vk::BufferCreateInfo,memory_type:vk::MemoryPropertyFlags)->Self{
        let handle = unsafe {
            core.logical_device.create_buffer(&buffer_info, None).expect("Failed to create buffer")
        };
        let memory_requirements = unsafe {
            core.logical_device.get_buffer_memory_requirements(handle)
        };
        Self{
            core,
            handle,
            offsets:Vec::new(),
            buffer_type:buffer_info.usage,
            memory_type,
            memory_requirements,
            size:buffer_info.size,
            allocator_index:None,
        }
    }
   
}
impl Drop for Buffer{
    fn drop(&mut self){
        unsafe {
            self.core.logical_device.destroy_buffer(self.handle, None);
        }
    }
}