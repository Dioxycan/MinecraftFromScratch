pub mod allocators;
mod buffers;
mod image;
use ash::vk;

use crate::core::Core;
use allocators::Allocator;
use buffers::Buffer;
use std::rc::Rc;

pub struct Memory {
    core: Rc<Core>,
    allocators: Vec<Allocator>,
    pub buffers: Vec<Buffer>,
}
impl Memory {
    pub fn new(core: Rc<Core>) -> Self {
        Self {
            core,
            allocators: Vec::new(),
            buffers: Vec::new(),
        }
    }
    pub fn create_allocator(
        &mut self,
        size: vk::DeviceSize,
        memory_flags: vk::MemoryPropertyFlags,
    ) {
        let mut allocator = Allocator::new(self.core.clone(), size, memory_flags);
        if memory_flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE){
            allocator.map_memory();
        }
        self.allocators.push(allocator);
    }
    pub fn create_buffer(
        &mut self,
        size: vk::DeviceSize,
        usage_flags: vk::BufferUsageFlags,
        memory_flags: vk::MemoryPropertyFlags,
    )->usize {
        let buffer = Buffer::new(
            self.core.clone(),
            vk::BufferCreateInfo {
                size,
                usage: usage_flags,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                ..Default::default()
            },
            memory_flags,
        );
        self.buffers.push(buffer);
        self.buffers.len() - 1
    }
    pub fn allocate_memory(&mut self, buffer_index: usize) {
        let allocator_index = self
            .find_suitable_allocator(
                self.buffers[buffer_index].memory_type,
                self.buffers[buffer_index].size,
                self.buffers[buffer_index]
                    .memory_requirements
                    .memory_type_bits,
            )
            .unwrap();
        self.allocators[allocator_index].allocate_memory(&self.buffers[buffer_index], buffer_index);
        self.buffers[buffer_index].allocator_index = Some(allocator_index);
    }
    fn find_suitable_allocator(
        &mut self,
        memory_type: vk::MemoryPropertyFlags,
        size: vk::DeviceSize,
        memory_type_index: u32,
    ) -> Option<usize> {
        self.allocators
            .iter_mut()
            .enumerate()
            .find(|(index, allocator)| {
                allocator.memory_flags.contains(memory_type)
                    && allocator.free_memory >= size
                    && memory_type_index & (1 << index) != 0
            })
            .map(|(index, allocator)| index)
    }
    pub fn copy_memory(
        &mut self,
        command_buffer: vk::CommandBuffer,
        buffer_index: usize,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        data: *const u8,
    ) {
        if let Some(allocator_index) = self.buffers[buffer_index].allocator_index {
            match self.buffers[buffer_index].memory_type {
                vk::MemoryPropertyFlags::HOST_VISIBLE => {
                    match self.allocators[allocator_index].host_data_ptr{
                        Some(host_data_ptr)=>{
                            unsafe{
                                std::ptr::copy_nonoverlapping(data,host_data_ptr as *mut u8,size as usize);
                            }

                        },
                        None=>{
                            eprintln!("Failed to find host data ptr");
                        }
                    }
                }
                vk::MemoryPropertyFlags::DEVICE_LOCAL => {}
                _ => {}
            }
        }
    }
    pub fn find_suitable_buffer(
        &mut self,
        size: vk::DeviceSize,
        usage_flags: vk::BufferUsageFlags,
    ) -> Option<usize> {
        self.buffers
            .iter_mut()
            .enumerate()
            .find(|(index, buffer)| {
                buffer.buffer_type == usage_flags
                    && buffer.free_memory >= size
                    && buffer.memory_type == vk::MemoryPropertyFlags::HOST_VISIBLE
            })
            .map(|(index, buffer)| index)
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        for allocator in self.allocators.iter_mut() {
            unsafe {
                self.core.logical_device.free_memory(allocator.handle, None);
            }
        }
    }
}
// impl Memory{
//     pub fn new(core:Rc<Core>)->Self{
//         Self{
//             core,
//             allocator,
//         }
//     }
// }
// // // fn create_vertex_buffer() {
// // //     let vertices = vec![

// // //         Vertex::new(glm::Vec3::new(0.25, -0.25,0.5), glm::Vec3::new(1.0, 0.0, 0.0)),
// // //         Vertex::new(glm::Vec3::new(0.75, 0.75,0.5), glm::Vec3::new(1.0, 0.0, 0.0)),
// // //         Vertex::new(glm::Vec3::new(-0.25, 0.75,0.5), glm::Vec3::new(1.0, 0.0, 0.0)),""

// // //     ];

// // //     let vertex_count = mem::size_of::<Vertex>() as u64;
// // //     let vertex_buffer_info = vk::BufferCreateInfo::builder()
// // //         .size(vertex_count * vertices.len() as u64)
// // //         .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
// // //         .sharing_mode(vk::SharingMode::EXCLUSIVE)
// // //         .build();
// // //     let vertex_buffer = unsafe {
// // //         self.core
// // //             .logical_device
// // //             .create_buffer(&vertex_buffer_info, None)
// // //             .expect("Failed to create vertex buffer")
// // //     };
// // //     let vertex_buffer_memory_requirements = unsafe {
// // //         self.core
// // //             .logical_device
// // //             .get_buffer_memory_requirements(self.vertex_buffer)
// // //     };
// // //     let vertex_buffer_memory_allocate_info = vk::MemoryAllocateInfo::builder()
// // //         .allocation_size(vertex_buffer_memory_requirements.size)
// // //         .memory_type_index(
// // //             self.core
// // //                 .find_memory_type(
// // //                     vertex_buffer_memory_requirements.memory_type_bits,
// // //                     vk::MemoryPropertyFlags::HOST_VISIBLE
// // //                         | vk::MemoryPropertyFlags::HOST_COHERENT,
// // //                 )
// // //                 .unwrap(),
// // //         )
// // //         .build();
// // //     self.vertex_buffer_memory = unsafe {
// // //         self.core
// // //             .logical_device
// // //             .allocate_memory(&vertex_buffer_memory_allocate_info, None)
// // //             .expect("Failed to allocate vertex buffer memory")
// // //     };
// // //     unsafe {
// // //         self.core
// // //             .logical_device
// // //             .bind_buffer_memory(self.vertex_buffer, self.vertex_buffer_memory, 0)
// // //             .expect("Failed to bind vertex buffer memory")
// // //     };
// // //     let mapped_memory = unsafe {
// // //         self.core
// // //             .logical_device
// // //             .map_memory(
// // //                 self.vertex_buffer_memory,
// // //                 0,
// // //                 vertex_buffer_memory_requirements.size,
// // //                 vk::MemoryMapFlags::empty(),
// // //             )
// // //             .expect("Failed to map vertex buffer memory")
// // //     };
// // //     unsafe {
// // //         std::ptr::copy_nonoverlapping(
// // //             vertices.as_ptr() as *const u8,
// // //             mapped_memory as *mut u8,
// // //             vertices.len() * mem::size_of::<Vertex>(),
// // //         );
// // //         self.core
// // //             .logical_device
// // //             .unmap_memory(self.vertex_buffer_memory);
// // //     }
// // // }
// // // pub fn draw(&mut self, command_buffer: vk::CommandBuffer) {
// // //     unsafe {
// // //         self.core
// // //             .logical_device
// // //             .cmd_draw(command_buffer, self.vertex_count as u32, 1, 0, 0);
// // //     }
// // // }
